//! Mesh group creation handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::mesh_group::{GroupStatus, GroupType, GroupRole, DevelopmentStage};
use crate::state::enums::IdeaStatus;
use crate::state::member::role::role_permissions;
use crate::utils::{assert_role, verify_ed25519_signature, compute_canonical_embedding_hash};

use super::helpers::verify_ai_analysis;

/// Create mesh group
///
/// According to updated logic:
/// - AI checks idea for DAO compliance
/// - AI confirms mesh group creation
/// - Mesh group is created after AI confirmation
///
/// Mesh group can have 1-7 members (if more needed, create additional mesh group).
///
/// # Compute Units
/// Recommended: 45,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~35,000 CU
///
/// # CRITICAL
/// If idea is provided when creating group, it must:
/// 1. Be in Approved status (after AI analysis)
/// 2. Have AI analysis that confirms idea can enter mesh group
#[allow(clippy::too_many_arguments)]
pub fn create_mesh_group_handler(
    ctx: Context<crate::CreateMeshGroup>,
    mesh_group_id: u64,
    name: String,
    description: String,
    group_type: GroupType,
    // Track B: Optional embedding parameters (B1)
    embedding_hash: Option<[u8; 32]>,
    embedding_signature: Option<[u8; 64]>,
    embedding_provider: Option<String>,
    embedding_model: Option<String>,
    embedding_model_version: Option<String>,
    embedding_provider_pubkey: Option<Pubkey>,
) -> Result<()> {
    let creator = ctx.accounts.creator.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // SECURITY: Check permission - creator must be DAO authority OR have CAN_MANAGE_MESH_GROUPS permission
    if creator != dao_config.authority {
        let creator_role = ctx
            .accounts
            .creator_role
            .as_ref()
            .ok_or(error!(IndrasError::Unauthorized))?;

        assert_role(
            &creator_role.to_account_info(),
            &creator,
            role_permissions::CAN_MANAGE_MESH_GROUPS,
            ctx.program_id,
        )?;
    }
    
    // Validate input data
    require!(name.len() <= 100, IndrasError::StringTooLong);
    require!(description.len() <= 500, IndrasError::StringTooLong);
    require!(!name.is_empty(), IndrasError::InvalidInput);
    
    // CRITICAL: Check AI confirmation for mesh group creation
    // If idea is provided, it must pass AI analysis and be Approved
    if let Some(idea) = &ctx.accounts.idea {
        // Check 1: Idea must be in Approved status
        require!(
            idea.status == IdeaStatus::Approved,
            IndrasError::InvalidState
        );
        
        // Check 2: CRITICAL - AI analysis REQUIRED and must approve idea
        // Without AI analysis, idea cannot enter mesh group
        // CRITICAL: Check not only that account exists, but also that it's not empty
        // With allow-missing-optionals, Anchor may create Some(UncheckedAccount) with empty data
        let analysis_account = ctx.accounts.ai_analysis.as_ref()
            .ok_or(IndrasError::InvalidInput)?;
        let analysis_record = ctx.accounts.ai_analysis_record.as_ref()
            .ok_or(IndrasError::InvalidInput)?;
        
        // Check that account is not empty (has data)
        require!(
            !analysis_account.data_is_empty(),
            IndrasError::InvalidInput
        );
        
        // Check 3: Verify AI analysis - decision must be Approve
        verify_ai_analysis(analysis_account, idea.id, &idea.key(), analysis_record)?;
        
        msg!("Mesh Group creation confirmed by AI analysis for idea {} (AI analysis verified: decision=Approve)", idea.id);
    } else {
        // Mesh group can be created without idea (idea added later)
        // But when idea is added, it must pass AI analysis
        msg!("Mesh Group created without idea (idea can be added later via add_idea_to_mesh_group, but must pass AI compliance)");
    }
    
    // Mesh group can have maximum 7 members
    // If more needed, create additional mesh group
    let max_members = 7u8;
    let min_members = 1u8; // Can be one genius user
    
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize mesh group directly in account
    let group_type_str = format!("{:?}", group_type);
    let mesh_group_account = &mut ctx.accounts.mesh_group;
    mesh_group_account.id = mesh_group_id;
    mesh_group_account.name = name;
    mesh_group_account.description = description;
    mesh_group_account.group_type = group_type;
    mesh_group_account.status = GroupStatus::Forming;
    mesh_group_account.leader = ctx.accounts.creator.key();
    mesh_group_account.created_by = ctx.accounts.creator.key();
    mesh_group_account.created_at = current_time;
    mesh_group_account.members = Vec::new();
    mesh_group_account.ideas = Vec::new();
    mesh_group_account.grants = Vec::new();
    mesh_group_account.phenomena = Vec::new();
    mesh_group_account.max_members = max_members;
    mesh_group_account.min_members = min_members;
    mesh_group_account.parent_group = None;
    mesh_group_account.supporting_groups = Vec::new();
    mesh_group_account.stage_deadline = None;
    mesh_group_account.current_stage = DevelopmentStage::Planning;
    mesh_group_account.started_at = None;
    mesh_group_account.completed_at = None;
    mesh_group_account.total_contributions = 0;
    mesh_group_account.total_reputation = 0;
    mesh_group_account.bump = ctx.bumps.mesh_group;
    
    // Initialize protocol fields
    mesh_group_account.protocol = crate::state::mesh_group::OperatingProtocol::default();
    mesh_group_account.last_meeting_at = None;
    mesh_group_account.last_contribution_at = current_time;
    
    // v1.1: Initialize rate limiting fields (SEC-INV-9)
    mesh_group_account.last_member_added_at = None;
    mesh_group_account.last_group_created_at = Some(current_time);
    
    // v1.1: Initialize Sybil protection fields (SEC-INV-15)
    mesh_group_account.member_reputation_required = 10;  // Default: 10 reputation points
    mesh_group_account.member_cooldown_days = 30;         // Default: 30 days cooldown
    
    // v1.1: Initialize critical moment fields (SEC-INV-16)
    mesh_group_account.is_in_critical_moment = false;
    mesh_group_account.critical_moment_until = None;
    
    // Track B: Process optional embedding parameters (B1)
    if let (Some(emb_hash), Some(emb_sig), Some(emb_provider), Some(emb_provider_pk)) = 
        (embedding_hash, embedding_signature, embedding_provider, embedding_provider_pubkey) {
        
        // Validate embedding parameters
        require!(emb_hash != [0u8; 32], IndrasError::EmbeddingHashMismatch);
        require!(emb_sig != [0u8; 64], IndrasError::EmbeddingSignatureInvalid);
        require!(!emb_provider.is_empty(), IndrasError::InvalidEmbeddingProvider);
        require!(emb_provider.len() <= 50, IndrasError::StringTooLong);
        
        if let Some(ref model) = embedding_model {
            require!(model.len() <= 100, IndrasError::StringTooLong);
        }
        if let Some(ref model_version) = embedding_model_version {
            require!(model_version.len() <= 50, IndrasError::StringTooLong);
        }
        
        // SEC-INV-10: Verify embedding signature via CPI to ed25519_program
        // Use canonical hashing for consistent hash computation
        let message_hash = compute_canonical_embedding_hash(
            &emb_hash,
            mesh_group_id,
            current_time,
            embedding_model_version.as_deref(),
        );
        
        // CRITICAL: Verify signature via CPI to ed25519_program
        verify_ed25519_signature(&message_hash, &emb_provider_pk, &emb_sig)?;
        
        msg!("SEC-INV-10: Embedding signature verified for provider: {}", emb_provider);
        
        // SEC-INV-11: Verify provider is in AIServiceRegistry (if provided)
        if let Some(registry_info) = &ctx.accounts.ai_service_registry {
            // Deserialize AIServiceRegistry manually from UncheckedAccount
            let registry = crate::utils::account_helpers::deserialize_ai_service_registry(
                registry_info,
                ctx.program_id,
            )?;
            require!(
                registry.is_service_authorized(&emb_provider_pk) ||
                emb_provider_pk == ctx.accounts.dao_config.authority,
                IndrasError::InvalidEmbeddingProvider
            );
            
            // Check if service supports the model (if model specified)
            if let Some(ref model) = embedding_model {
                if let Some(_service) = registry.get_service(&emb_provider_pk) {
                    if !registry.supports_model(&emb_provider_pk, model) {
                        // Warning: service doesn't support this model, but allow if DAO authority
                        require!(
                            emb_provider_pk == ctx.accounts.dao_config.authority,
                            IndrasError::InvalidEmbeddingProvider
                        );
                        msg!("SEC-INV-11: WARNING - Service {} does not support model {}, but allowed as DAO authority", emb_provider, model);
                    } else {
                        msg!("SEC-INV-11: Provider {} supports model {}", emb_provider, model);
                    }
                }
            }
            
            msg!("SEC-INV-11: Provider {} verified in AIServiceRegistry (active, not suspended)", emb_provider);
        } else {
            require!(
                emb_provider_pk == ctx.accounts.dao_config.authority,
                IndrasError::InvalidEmbeddingProvider
            );
            msg!("SEC-INV-11: Provider verified as DAO authority (registry not provided)");
        }
        
        // Store embedding metadata
        let current_time = Clock::get()?.unix_timestamp;
        mesh_group_account.embedding_hash = Some(emb_hash);
        mesh_group_account.embedding_signature = Some(emb_sig);
        mesh_group_account.embedding_provider = Some(emb_provider);
        
        // Track C: Update telemetry fields (C3)
        if mesh_group_account.embedding_created_at.is_none() {
            mesh_group_account.embedding_created_at = Some(current_time);
        }
        mesh_group_account.embedding_updated_at = Some(current_time);
        mesh_group_account.embedding_update_count = mesh_group_account.embedding_update_count.saturating_add(1);
        mesh_group_account.embedding_model = embedding_model;
        mesh_group_account.embedding_model_version = embedding_model_version;
    } else {
        // No embedding provided - set to None
        mesh_group_account.embedding_hash = None;
        mesh_group_account.embedding_signature = None;
        mesh_group_account.embedding_provider = None;
        mesh_group_account.embedding_model = None;
        mesh_group_account.embedding_model_version = None;
    }
    
    // SEC-INV-9: Rate limit check - 1 group per week per creator
    if let Some(ref mut rate_limit_tracker) = ctx.accounts.rate_limit_tracker {
        const RATE_LIMIT_WINDOW_SECONDS: i64 = 7 * 24 * 3600; // 7 days (1 week)
        
        // Initialize tracker if needed
        if rate_limit_tracker.user == Pubkey::default() {
            rate_limit_tracker.user = ctx.accounts.creator.key();
            rate_limit_tracker.operation_type = "create_mesh_group".to_string();
            rate_limit_tracker.last_operation_at = 0;
            rate_limit_tracker.operation_count = 0;
            rate_limit_tracker.window_start = current_time;
            // Bump is set automatically by Anchor's init_if_needed macro
        }
        
        // Check rate limit
        rate_limit_tracker.check_time_based_rate_limit(current_time, RATE_LIMIT_WINDOW_SECONDS)?;
        msg!("SEC-INV-9: Rate limit check passed for creator {} (window: {}s)", ctx.accounts.creator.key(), RATE_LIMIT_WINDOW_SECONDS);
    } else if !ctx.accounts.dao_config.dev_mode {
        return err!(IndrasError::AccountNotFound);
    } else {
        msg!("SEC-INV-9: Rate limit check skipped (dev_mode)");
    }
    
    // Add creator as leader (Owner role for Track A)
    let leader_member = crate::state::mesh_group::GroupMember {
        pubkey: ctx.accounts.creator.key(),
        role: GroupRole::Leader,  // Track A: Leader role
        joined_at: current_time,
        contributions: 0,
        reputation: 0,
        is_active: true,
    };
    mesh_group_account.members.push(leader_member);
    
    // Group activates automatically (min_members = 1, creator already added)
    if mesh_group_account.members.len() >= mesh_group_account.min_members as usize {
        mesh_group_account.status = GroupStatus::Active;
        mesh_group_account.started_at = Some(current_time);
    }
    
    msg!("Mesh Group {} created by {} (type: {}, max_members: {})", 
         mesh_group_account.name, 
         ctx.accounts.creator.key(),
         group_type_str,
         max_members);
    
    Ok(())
}
/// Create supporting mesh group when main group is full (7 members)
///
/// Supporting groups work on same ideas as main group.
/// Mesh group can have maximum 7 members. If more needed, create additional mesh group.
pub fn create_supporting_mesh_group_handler(
    ctx: Context<crate::CreateSupportingMeshGroup>,
    supporting_group_id: u64,
    name: String,
    description: String,
) -> Result<()> {
    let main_group = &mut ctx.accounts.main_group;
    let supporting_group = &mut ctx.accounts.supporting_group;
    
    // Check that main group is full (7 members)
    require!(
        main_group.members.len() >= main_group.max_members as usize,
        IndrasError::InvalidState
    );
    
    // Check supporting groups limit (maximum 10 groups = 70 people)
    require!(
        main_group.supporting_groups.len() < 10,
        IndrasError::TooManySupportingGroups
    );
    
    // Validate input data
    require!(name.len() <= 100, IndrasError::StringTooLong);
    require!(description.len() <= 500, IndrasError::StringTooLong);
    require!(!name.is_empty(), IndrasError::InvalidInput);
    
    // Create supporting mesh group
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize supporting mesh group directly in account
    supporting_group.id = supporting_group_id;
    supporting_group.name = name;
    supporting_group.description = description;
    supporting_group.group_type = main_group.group_type.clone(); // Supporting group of same type
    supporting_group.status = GroupStatus::Forming;
    supporting_group.leader = ctx.accounts.creator.key();
    supporting_group.created_by = ctx.accounts.creator.key();
    supporting_group.created_at = current_time;
    supporting_group.members = Vec::new();
    supporting_group.ideas = main_group.ideas.clone(); // Copy ideas from main group
    supporting_group.grants = Vec::new();
    supporting_group.phenomena = Vec::new();
    supporting_group.max_members = 7; // Same maximum capacity for mesh group
    supporting_group.min_members = 1;
    supporting_group.parent_group = Some(main_group.key()); // Set parent group
    supporting_group.supporting_groups = Vec::new();
    supporting_group.stage_deadline = None;
    supporting_group.current_stage = DevelopmentStage::Planning;
    supporting_group.started_at = None;
    supporting_group.completed_at = None;
    supporting_group.total_contributions = 0;
    supporting_group.total_reputation = 0;
    supporting_group.bump = ctx.bumps.supporting_group;
    
    // Add creator as leader of supporting group
    let leader_member = crate::state::mesh_group::GroupMember {
        pubkey: ctx.accounts.creator.key(),
        role: GroupRole::Leader,  // Track A: Leader role
        joined_at: current_time,
        contributions: 0,
        reputation: 0,
        is_active: true,
    };
    supporting_group.members.push(leader_member);
    
    // Activate group if min_members reached
    if supporting_group.members.len() >= supporting_group.min_members as usize {
        supporting_group.status = GroupStatus::Active;
        supporting_group.started_at = Some(current_time);
    }
    
    // Add supporting group to main group list
    // SECURITY: Check max_len limit (max_len(10) in struct definition)
    require!(
        main_group.supporting_groups.len() < 10,
        IndrasError::TooManySupportingGroups
    );
    
    main_group.supporting_groups.push(supporting_group.key());
    
    msg!("Supporting Mesh Group {} created for main group {} (main group has {} members, {} supporting groups)", 
         supporting_group.name, 
         main_group.name,
         main_group.members.len(),
         main_group.supporting_groups.len());
    
    Ok(())
}
