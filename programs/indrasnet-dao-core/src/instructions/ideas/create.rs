//! Idea creation handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::enums::IdeaStatus;
use crate::state::member::role::role_permissions;
use crate::utils::{assert_role, verify_ed25519_signature, compute_canonical_embedding_hash};

use super::helpers::{normalize_idea_text, compute_idea_hash};

/// Create a new idea
///
/// This handler creates a new idea account with the provided title and description.
/// New ideas start with status Draft.
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~5,000 CU
/// - Account initialization: ~25,000 CU
#[allow(clippy::too_many_arguments)]
pub fn create_idea_handler(
    ctx: Context<crate::CreateIdea>,
    idea_id: u64,
    title: String,
    description: String,
    // Track B: Optional embedding parameters (B1)
    embedding_hash: Option<[u8; 32]>,
    embedding_signature: Option<[u8; 64]>,
    embedding_provider: Option<String>,
    embedding_model: Option<String>,
    embedding_model_version: Option<String>,
    embedding_provider_pubkey: Option<Pubkey>,
) -> Result<()> {
    let author = ctx.accounts.author.key();
    let dao_config = &ctx.accounts.dao_config;
    let current_time = Clock::get()?.unix_timestamp;
    
    // SEC-INV-8: Rate limit check - 1 idea per day per author
    if let Some(ref mut rate_limit_tracker) = ctx.accounts.rate_limit_tracker {
        const RATE_LIMIT_WINDOW_SECONDS: i64 = 86400; // 24 hours
        
        // Initialize tracker if needed
        if rate_limit_tracker.user == Pubkey::default() {
            rate_limit_tracker.user = author;
            rate_limit_tracker.operation_type = "create_idea".to_string();
            rate_limit_tracker.last_operation_at = 0;
            rate_limit_tracker.operation_count = 0;
            rate_limit_tracker.window_start = current_time;
            // Bump is set automatically by Anchor's init_if_needed macro
        }
        
        // Check rate limit
        rate_limit_tracker.check_time_based_rate_limit(current_time, RATE_LIMIT_WINDOW_SECONDS)?;
        msg!("SEC-INV-8: Rate limit check passed for author {} (window: {}s)", author, RATE_LIMIT_WINDOW_SECONDS);
    } else if !dao_config.dev_mode {
        return err!(IndrasError::AccountNotFound);
    } else {
        msg!("SEC-INV-8: Rate limit check skipped (dev_mode)");
    }
    
    // SECURITY: Check permission - author must be DAO authority OR have CAN_CREATE_IDEA permission
    if author != dao_config.authority {
        let author_role = ctx
            .accounts
            .author_role
            .as_ref()
            .ok_or(error!(IndrasError::Unauthorized))?;

        assert_role(
            &author_role.to_account_info(),
            &author,
            role_permissions::CAN_CREATE_IDEA,
            ctx.program_id,
        )?;
    }
    
    // Validate inputs
    require!(!title.is_empty(), IndrasError::InvalidInput);
    require!(!description.is_empty(), IndrasError::InvalidInput);
    require!(title.len() <= 100, IndrasError::InvalidInput);
    require!(description.len() <= 500, IndrasError::InvalidInput);
    
    // Get idea key BEFORE any borrows (needed for anti-duplication check)
    let idea_key = ctx.accounts.idea.key();
    
    // Clone embedding values BEFORE processing to avoid move issues
    let embedding_hash_clone = embedding_hash;
    let embedding_signature_clone = embedding_signature;
    let embedding_provider_clone = embedding_provider.clone();
    let embedding_model_clone = embedding_model.clone();
    let embedding_model_version_clone = embedding_model_version.clone();
    
    // Track B: Process optional embedding parameters (B1) - BEFORE mutable borrow
    // This includes all validation and anti-duplication checks
    if let (Some(emb_hash), Some(emb_sig), Some(ref emb_provider), Some(emb_provider_pk)) = 
        (embedding_hash_clone, embedding_signature_clone, embedding_provider_clone, embedding_provider_pubkey) {
        
        // Validate embedding parameters
        require!(emb_hash != [0u8; 32], IndrasError::EmbeddingHashMismatch);
        require!(emb_sig != [0u8; 64], IndrasError::EmbeddingSignatureInvalid);
        require!(!emb_provider.is_empty(), IndrasError::InvalidEmbeddingProvider);
        require!(emb_provider.len() <= 50, IndrasError::StringTooLong);
        
        if let Some(ref model) = embedding_model_clone {
            require!(model.len() <= 100, IndrasError::StringTooLong);
        }
        if let Some(ref model_version) = embedding_model_version_clone {
            require!(model_version.len() <= 50, IndrasError::StringTooLong);
        }
        
        // SEC-INV-10: Verify embedding signature via CPI to ed25519_program
        // Use canonical hashing for consistent hash computation
        let current_time = Clock::get()?.unix_timestamp;
        let message_hash = compute_canonical_embedding_hash(
            &emb_hash,
            idea_id,
            current_time,
            embedding_model_version_clone.as_deref(),
        );
        
        // CRITICAL: Verify signature via CPI to ed25519_program
        // Without this, semantic layer is vulnerable to poisoning & spoofing attacks
        verify_ed25519_signature(&message_hash, &emb_provider_pk, &emb_sig)?;
        
        // Use reference for logging (emb_provider is already a reference)
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
            if let Some(ref model) = embedding_model_clone {
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
            // If registry not provided, only DAO authority can submit embeddings
            require!(
                emb_provider_pk == ctx.accounts.dao_config.authority,
                IndrasError::InvalidEmbeddingProvider
            );
            msg!("SEC-INV-11: Provider verified as DAO authority (registry not provided)");
        }
        
        // Anti-duplication: Check if embedding already exists for this idea with same model_version
        // Seeds: [b"embedding_dedup", idea.key(), embedding_hash, model_version.as_bytes()]
        // This prevents the same embedding+model_version from being stored twice for the same idea
        // NOTE: Same embedding with different model_version is allowed (model upgrade)
        // Get all dedup data and deserialize BEFORE any mutable borrows
        let model_version_str = embedding_model_version_clone.as_deref().unwrap_or("");
        let expected_seeds = &[
            b"embedding_dedup",
            idea_key.as_ref(),
            emb_hash.as_ref(),
            model_version_str.as_bytes(),
        ];
        let (expected_pda, _bump) = Pubkey::find_program_address(expected_seeds, ctx.program_id);
        
        // Full anti-duplication: Deserialize and verify duplicate check
        // This ensures the same embedding+model_version cannot be stored twice for the same entity
        if let Some(ref dedup_info) = ctx.accounts.embedding_deduplication {
            let dedup_key = dedup_info.key();
            require!(
                dedup_key == expected_pda,
                IndrasError::InvalidInput // PDA seeds mismatch
            );
            
            // Deserialize EmbeddingDeduplication manually from UncheckedAccount
            let dedup = crate::utils::account_helpers::deserialize_embedding_deduplication(dedup_info)?;
            let is_duplicate = dedup.is_duplicate(
                idea_id,
                emb_hash,
                model_version_str
            );
            
            require!(
                !is_duplicate,
                IndrasError::InvalidInput // Duplicate embedding+model_version for same entity
            );
            
            msg!("Anti-duplication: Full check passed - no duplicate detected");
        }
    }
    
    // NOW take mutable borrow for idea account initialization
    let idea = &mut ctx.accounts.idea;
    idea.id = idea_id;
    idea.author = ctx.accounts.author.key();
    idea.title = title.clone();
    idea.description = description.clone();
    idea.status = IdeaStatus::Draft; // New ideas start as Draft
    idea.rights_transferred_to_ev = None; // Rights not transferred at creation
    
    // Compute idea_hash for AI analysis verification
    // Normalize idea text: title + description
    let idea_text = format!("{}\n{}", title.trim(), description.trim());
    let normalized_text = normalize_idea_text(&idea_text);
    let idea_hash = compute_idea_hash(&normalized_text);
    idea.idea_hash = Some(idea_hash);
    
    // Track B: Store embedding metadata if provided
    // Use original values (not clones) since we validated above
    if embedding_hash.is_some() && embedding_signature.is_some() && embedding_provider.is_some() {
        // Store embedding metadata (values already validated above)
        let current_time = Clock::get()?.unix_timestamp;
        idea.embedding_hash = embedding_hash;
        idea.embedding_signature = embedding_signature;
        idea.embedding_provider = embedding_provider;
        idea.embedding_model = embedding_model;
        idea.embedding_model_version = embedding_model_version;
        
        // Track C: Update telemetry fields (C3)
        if idea.embedding_created_at.is_none() {
            idea.embedding_created_at = Some(current_time);
        }
        idea.embedding_updated_at = Some(current_time);
        idea.embedding_update_count = idea.embedding_update_count.saturating_add(1);
    } else {
        // No embedding provided - set to None
        idea.embedding_hash = None;
        idea.embedding_signature = None;
        idea.embedding_provider = None;
        idea.embedding_model = None;
        idea.embedding_model_version = None;
        // Track C: Reset telemetry fields when embedding is removed
        idea.embedding_created_at = None;
        idea.embedding_updated_at = None;
        idea.embedding_update_count = 0;
    }
    
    idea.bump = ctx.bumps.idea;
    
    msg!("Idea {} created by {}", idea_id, ctx.accounts.author.key());
    msg!("Idea hash computed: {}", hex::encode(idea_hash));
    
    Ok(())
}
