//! Grant creation handlers

use anchor_lang::prelude::*;
use borsh::BorshDeserialize;
use crate::error::IndrasError;
use crate::state::grant::{GrantStatus, GrantCategory, GrantType, GrantDisbursementType, VotingLayer};
use crate::state::grant::semantic::SemanticDomain;
use crate::state::enums::IdeaStatus;
use crate::state::member::role_permissions;
use crate::utils::{assert_role, is_idea_in_phenomenon, get_phenomenon_status, verify_ed25519_signature};
use crate::constants::ai_program_id;
use crate::state::phenomenon::PhenomenonStatus;
use crate::state::mesh_group::{DevelopmentStage, MeshGroup};
use crate::state::Idea;
use sha2::{Sha256, Digest};

/// Create grant request
///
/// This handler creates a grant request (status Pending) for a mesh group working on an idea.
/// Grant will be added to mesh group only after approval (approve_grant).
///
/// # Compute Units
/// Recommended: 50,000 CU
/// - Validation: ~10,000 CU
/// - State updates: ~15,000 CU
/// - Account initialization: ~25,000 CU
///
/// # Notes
/// - Grant has NO relation to ownership rights.
/// - Grant is only funding, not transfer of IP rights.
/// - Idea author remains copyright owner.
/// - e.V. is custodian of copyright (through user membership in e.V.), not owner.
/// - MeshGroup validation is now implemented. Phenomenon is NOT required (created AFTER grant for analytics).
#[allow(clippy::too_many_arguments)]
pub fn create_grant_handler(
    ctx: Context<crate::CreateGrant>,
    grant_id: u64,
    idea_id: u64,
    category: GrantCategory,
    grant_type: GrantType,
    disbursement_type: GrantDisbursementType, // Disbursement type
    base_amount: u64,
    reputation_bonus: u64,
    milestone_id: Option<u64>,
    // Track B: Optional semantic pre-filter parameters (B4)
    semantic_domain_account: Option<Pubkey>,
    semantic_distance: Option<f32>,
    phenomenon_membership: Option<Pubkey>,
) -> Result<()> {
    let creator = ctx.accounts.creator.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // SECURITY: Check permission - creator must be DAO authority OR have CAN_CREATE_GRANT permission
    if creator != dao_config.authority {
        let creator_role_info = ctx
            .accounts
            .creator_role
            .as_ref()
            .ok_or(error!(IndrasError::Unauthorized))?;

        assert_role(
            creator_role_info,
            &creator,
            role_permissions::CAN_CREATE_GRANT,
            ctx.program_id,
        )?;
    }
    
    let grant = &mut ctx.accounts.grant;
    
    // Deserialize accounts to reduce stack size (BPF limit)
    // Using manual deserialization to avoid stack overflow
    require!(
        ctx.accounts.idea.owner == ctx.program_id,
        IndrasError::InvalidProgram
    );
    let (expected_idea_pda, _) = Pubkey::find_program_address(
        &[b"idea", idea_id.to_le_bytes().as_ref()],
        ctx.program_id,
    );
    require!(
        ctx.accounts.idea.key() == expected_idea_pda,
        IndrasError::InvalidInput
    );

    require!(
        ctx.accounts.mesh_group.owner == ctx.program_id,
        IndrasError::InvalidProgram
    );

    let idea_data = ctx.accounts.idea.data.borrow();
    let idea: Idea = BorshDeserialize::try_from_slice(&idea_data[8..])?; // Skip discriminator
    
    let mesh_group_data = ctx.accounts.mesh_group.data.borrow();
    let mesh_group: MeshGroup = BorshDeserialize::try_from_slice(&mesh_group_data[8..])?; // Skip discriminator

    let (expected_mesh_group_pda, _) = Pubkey::find_program_address(
        &[b"mesh_group", mesh_group.id.to_le_bytes().as_ref()],
        ctx.program_id,
    );
    require!(
        ctx.accounts.mesh_group.key() == expected_mesh_group_pda,
        IndrasError::InvalidInput
    );
    
    // Validate inputs
    require!(base_amount > 0, IndrasError::InvalidInput);
    require!(base_amount <= 1_000_000_000, IndrasError::AmountTooLarge); // 1 SOL max
    require!(reputation_bonus <= base_amount / 2, IndrasError::AmountTooLarge); // Max 50% bonus
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // Security check via CPI (if Security program is provided)
    // NOTE: Temporarily disabled - Security in exclude, causes build issues
    // if let Some(security_program) = &ctx.accounts.security_program {
    //     // ... security check code ...
    // }
    
    // ===== CRITERION 1: Idea status =====
    // Idea must be in progress or approved
    require!(
        idea.status == IdeaStatus::InProgress || 
        idea.status == IdeaStatus::Approved,
        IndrasError::InvalidState
    );
    
    // ===== CRITERION 2: Mesh group is valid =====
    // Mesh group must be active and contain the idea
    
    require!(
        mesh_group.is_active(),
        IndrasError::InvalidState
    );
    require!(
        mesh_group.ideas.contains(&idea_id),
        IndrasError::InvalidInput
    );
    
    // ===== CRITERION 3: Development stage =====
    // Grant can only be requested at specific stage (not Planning)
    require!(
        mesh_group.current_stage != DevelopmentStage::Planning,
        IndrasError::InvalidState
    );
    
    // ===== CRITERION 4: Match grant_type and development stage =====
    // Initial Grant → InitialDevelopment
    // Core Grant → CoreDevelopment
    // Final Grant → Finalization
    match grant_type {
        GrantType::Initial => {
            require!(
                mesh_group.current_stage == DevelopmentStage::InitialDevelopment,
                IndrasError::InvalidState
            );
        }
        GrantType::Core => {
            require!(
                mesh_group.current_stage == DevelopmentStage::CoreDevelopment,
                IndrasError::InvalidState
            );
        }
        GrantType::Final => {
            require!(
                mesh_group.current_stage == DevelopmentStage::Finalization,
                IndrasError::InvalidState
            );
        }
    }
    
    // ===== CRITERION 5: Group progress =====
    // Group must show minimum progress (minimum 3 contributions)
    require!(
        mesh_group.total_contributions >= 3,
        IndrasError::InsufficientProgress
    );
    
    // ===== CRITERION 6: Phenomenon NOT required (created AFTER grant) =====
    // According to updated logic: phenomena are created AFTER grant for analytics
    // Therefore, phenomenon check is NOT required when creating grant
    // NOTE: Phenomenon will be created by AI AFTER grant approval
    
    // ===== CRITERION 6: AI analysis and DAO norm compliance =====
    // CRITICAL: Idea must pass AI analysis with decision == Approve
    // Without AI compliance, idea cannot request grant
    
    // Check that idea passed AI analysis and is approved
    require!(
        idea.status == IdeaStatus::Approved,
        IndrasError::InvalidState
    );
    
    // CRITICAL: AI Analysis account is REQUIRED and must have decision == Approve
    require!(
        !ctx.accounts.analysis.data_is_empty(),
        IndrasError::InvalidInput
    );
    
    // Check that AI Analysis account belongs to AI program
    require!(
        ctx.accounts.analysis.owner == &ai_program_id(),
        IndrasError::InvalidProgram
    );

    // Check that AI Analysis account is the AI program PDA for this idea
    let (expected_analysis, _) = Pubkey::find_program_address(
        &[b"ai_analysis", ctx.accounts.idea.key().as_ref()],
        &ai_program_id(),
    );
    require!(
        ctx.accounts.analysis.key() == expected_analysis,
        IndrasError::InvalidProgram
    );

    // Verify AI analysis registration record (CPI-guarded)
    require!(
        ctx.accounts.analysis_record.analysis == ctx.accounts.analysis.key(),
        IndrasError::InvalidInput
    );
    require!(
        ctx.accounts.analysis_record.idea_id == idea_id,
        IndrasError::InvalidInput
    );
    require!(
        ctx.accounts.analysis_record.ai_program == ai_program_id(),
        IndrasError::InvalidProgram
    );
    
    // Deserialize and check AI analysis manually (avoid circular dependency)
    let data = ctx.accounts.analysis.try_borrow_data()?;
    require!(data.len() > 48, IndrasError::InvalidInput); // Minimum 49 bytes
    
    // Check idea_id (bytes 8-15 after discriminator)
    let idea_id_bytes: [u8; 8] = data[8..16].try_into().map_err(|_| IndrasError::InvalidInput)?;
    let analysis_idea_id = u64::from_le_bytes(idea_id_bytes);
    require!(
        analysis_idea_id == idea_id,
        IndrasError::InvalidInput
    );
    
    // Check decision (byte 48 after discriminator)
    // AIReviewDecision::Approve = 0, Reject = 1, Appeal = 2
    let decision_byte = data[48];
    require!(
        decision_byte == 0, // Approve
        IndrasError::InvalidState
    );
    
    // Additional checks: scores for can_enter_mesh_group
    if data.len() >= 57 {
        let ethics_score = data[51];
        let legal_score = data[52];
        let uniqueness_score = data[54];
        let feasibility_score = data[56];
        
        // Check criteria for can_enter_mesh_group
        require!(ethics_score >= 50, IndrasError::InvalidState);
        require!(legal_score >= 50, IndrasError::InvalidState);
        require!(uniqueness_score >= 70, IndrasError::InvalidState);
        require!(feasibility_score >= 70, IndrasError::InvalidState);
        
        // Check artifacts_verified (approximately at position 120)
        if data.len() >= 121 {
            let artifacts_verified = data[120] != 0;
            require!(artifacts_verified, IndrasError::InvalidState);
        }
    }
    
    msg!("AI Analysis verified for idea {}: decision=Approve, can_enter_mesh_group=true", idea_id);
    
    // Track B: Semantic pre-filter validation (B4)
    if let Some(_semantic_domain_pubkey) = semantic_domain_account {
        // Verify semantic domain account exists and is valid
        require!(
            !ctx.accounts.semantic_domain.as_ref().map(|sd| sd.data_is_empty()).unwrap_or(true),
            IndrasError::InvalidSemanticDomain
        );
        
        // Verify semantic domain account belongs to Core program
        if let Some(semantic_domain_info) = &ctx.accounts.semantic_domain {
            require!(
                semantic_domain_info.owner == ctx.program_id,
                IndrasError::InvalidSemanticDomain
            );
            
            // Deserialize semantic domain manually to verify signature
            let domain_data = semantic_domain_info.try_borrow_data()?;
            require!(domain_data.len() > 8 + 8 + 50 + 32 + 64 + 50 + 8, IndrasError::InvalidSemanticDomain);
            
            // Deserialize SemanticDomain account
            let semantic_domain: SemanticDomain = borsh::BorshDeserialize::try_from_slice(&domain_data[8..])?;
            
            // SEC-INV-10: Verify embedding signature
            require!(
                semantic_domain.embedding_hash != [0u8; 32],
                IndrasError::EmbeddingHashMismatch
            );
            require!(
                semantic_domain.embedding_signature != [0u8; 64],
                IndrasError::EmbeddingSignatureInvalid
            );
            
            // SEC-INV-10: Verify embedding signature if AIServiceRegistry provided
            if let Some(registry_info) = &ctx.accounts.ai_service_registry {
                // Deserialize AIServiceRegistry
                let registry = crate::utils::account_helpers::deserialize_ai_service_registry(
                    registry_info,
                    ctx.program_id,
                )?;
                
                // Find active service that matches provider name (check if provider name matches any model_id)
                // NOTE: For SYNAPSE MVP, we use simplified matching - provider name should match a model_id
                // In production, AIService should have a name field for direct matching
                let provider_pubkey = registry.services.iter()
                    .find(|s| s.is_active && !s.is_suspended && 
                          s.model_ids.iter().any(|id| id == &semantic_domain.provider))
                    .map(|s| s.pubkey)
                    .or_else(|| {
                        // Fallback: use first active service if provider name doesn't match
                        registry.services.iter()
                            .find(|s| s.is_active && !s.is_suspended)
                            .map(|s| s.pubkey)
                    })
                    .ok_or(IndrasError::InvalidEmbeddingProvider)?;
                
                // Compute message hash: SHA256(embedding_hash || domain_id || created_at)
                let mut hasher = Sha256::new();
                hasher.update(semantic_domain.embedding_hash);
                hasher.update(semantic_domain.domain_id.to_le_bytes());
                hasher.update(semantic_domain.created_at.to_le_bytes());
                let message_hash: [u8; 32] = hasher.finalize().into();
                
                // Verify signature via CPI to ed25519_program
                verify_ed25519_signature(
                    &message_hash,
                    &provider_pubkey,
                    &semantic_domain.embedding_signature
                )?;
                
                msg!("Track B: Semantic domain signature verified for provider: {}", semantic_domain.provider);
            } else {
                // If registry not provided, only validate signature is not zero
                msg!("Track B: Semantic domain signature validation skipped (AIServiceRegistry not provided)");
            }
            
            // Validate semantic distance if provided
            if let Some(distance) = semantic_distance {
                require!((0.0..=1.0).contains(&distance), IndrasError::InvalidScore);
                msg!("Track B: Semantic distance: {}", distance);
            }
        }
    }
    
    // Track B: ENFORCE phenomenon membership if provided
    // If phenomenon_membership is specified, idea MUST belong to that phenomenon
    // Get idea pubkey BEFORE any mutable borrows (for borrow checker)
    let idea_pubkey = ctx.accounts.idea.key();
    
    // Check phenomenon membership BEFORE initializing grant (to avoid borrow conflicts)
    if let Some(phenomenon_pubkey) = phenomenon_membership {
        // Verify phenomenon account exists and is provided
        let phenomenon_account = ctx.accounts.phenomenon.as_ref()
            .ok_or(IndrasError::IdeaNotInPhenomenon)?;
        
        require!(
            !phenomenon_account.data_is_empty(),
            IndrasError::IdeaNotInPhenomenon
        );
        
        // Verify phenomenon account belongs to Core program
        require!(
            phenomenon_account.owner == ctx.program_id,
            IndrasError::IdeaNotInPhenomenon
        );
        
        // FULL DESERIALIZATION: Deserialize phenomenon to check membership and status
        let phenomenon_data = phenomenon_account.try_borrow_data()?;
        require!(phenomenon_data.len() >= 8 + 32 + 8, IndrasError::IdeaNotInPhenomenon);
        
        // 1. Verify phenomenon status is Active (not Proposed)
        let status = get_phenomenon_status(&phenomenon_data)?;
        require!(
            status == PhenomenonStatus::Active,
            IndrasError::IdeaNotInPhenomenon // Phenomenon must be Active
        );
        
        // 2. Deserialize related_ideas Vec and verify idea is a member
        let is_member = is_idea_in_phenomenon(&phenomenon_data, &idea_pubkey)?;
        require!(
            is_member,
            IndrasError::IdeaNotInPhenomenon // Idea must be in phenomenon's related_ideas
        );
        
        msg!("Track B: ENFORCED - Idea {} verified as member of phenomenon {} (status: Active)", idea_id, phenomenon_pubkey);
    } else {
        // If phenomenon_membership is NOT provided, grant can be created without phenomenon
        // (phenomenon will be created AFTER grant for analytics)
        msg!("Track B: No phenomenon_membership specified - grant can be created without phenomenon (will be created after grant)");
    }
    
    // ===== CRITERION 7: Phenomenon NOT required (created AFTER grant) =====
    // According to updated logic: phenomena are created AFTER grant for analytics
    // Therefore, phenomenon check is NOT required when creating grant
    // NOTE: Phenomenon will be created by AI AFTER grant approval
    
    // NOTE: disbursement_type is passed as parameter when creating grant
    // Automatic determination by category can be done off-chain
    
    // SECURITY: Use checked arithmetic to prevent overflow
    let total_amount = base_amount.checked_add(reputation_bonus)
        .ok_or(error!(IndrasError::Overflow))?;
    
    // Initialize Grant (REQUEST, not creation)
    grant.id = grant_id;
    grant.idea_id = idea_id;
    grant.mesh_group = ctx.accounts.mesh_group.key();
    grant.category = category;
    grant.grant_type = grant_type;
    grant.disbursement_type = disbursement_type; // Disbursement type
    grant.milestone_id = milestone_id;
    grant.status = GrantStatus::Pending; // Request awaits approval
    grant.base_amount = base_amount;
    grant.reputation_bonus = reputation_bonus;
    grant.total_amount = total_amount;
    grant.disbursed_amount = 0;
    grant.verification_status = crate::state::grant::VerificationStatus::Pending;
    // NOTE: When grant is approved, author MUST transfer commercialization rights to e.V.
    // Author remains copyright owner (does not transfer)
    // e.V. receives right to transfer Idea to commercial enterprise
    grant.commercialization_right_transferred = false; // Will be set on approval
    grant.created_at = Clock::get()?.unix_timestamp;
    // Initialize semantic voting fields (defaults for MVP)
    grant.semantic_domain = None;  // Will be set by off-chain service
    grant.grant_level = 1;         // Default to Level 1 (author only)
    grant.voting_layer = VotingLayer::AuthorOnly; // Default to author only
    
    // Track B: Initialize semantic Grant Voting fields (B4)
    grant.semantic_domain_account = semantic_domain_account;
    grant.semantic_distance = semantic_distance;
    grant.phenomenon_membership = phenomenon_membership;
    
    // Grant report fields - initialize
    grant.final_report_submitted = false;
    grant.final_report_approved = false;
    grant.final_report_submitted_at = None;
    grant.final_report_approved_at = None;
    grant.escrow_account = None; // Will be created on activation for Escrow type
    
    grant.bump = ctx.bumps.grant;
    
    // IMPORTANT: Grant is NOT added to group until approval!
    // This will be done in approve_grant_handler
    
    msg!("Grant requested: {} SOL for Mesh Group {} (Idea: {})", 
         total_amount as f64 / 1_000_000_000.0, 
         ctx.accounts.mesh_group.key(), 
         idea_id);
    
    Ok(())
}
