//! Idea embedding management handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::utils::{verify_ed25519_signature, compute_canonical_embedding_hash};

pub fn update_idea_embedding_handler(
    ctx: Context<crate::UpdateIdeaEmbedding>,
    idea_id: u64,
    embedding_hash: [u8; 32],
    embedding_signature: [u8; 64],
    embedding_provider: String,
    embedding_model: Option<String>,
    embedding_model_version: Option<String>,
    embedding_provider_pubkey: Pubkey,
) -> Result<()> {
    // Get idea key BEFORE mutable borrow (needed for anti-duplication check)
    let _idea_key = ctx.accounts.idea.key();
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate embedding parameters
    require!(embedding_hash != [0u8; 32], IndrasError::EmbeddingHashMismatch);
    require!(embedding_signature != [0u8; 64], IndrasError::EmbeddingSignatureInvalid);
    require!(!embedding_provider.is_empty(), IndrasError::InvalidEmbeddingProvider);
    require!(embedding_provider.len() <= 50, IndrasError::StringTooLong);
    
    if let Some(ref model) = embedding_model {
        require!(model.len() <= 100, IndrasError::StringTooLong);
    }
    if let Some(ref model_version) = embedding_model_version {
        require!(model_version.len() <= 50, IndrasError::StringTooLong);
    }
    
    // SEC-INV-10: Verify embedding signature via CPI to ed25519_program
    // Use canonical hashing for consistent hash computation
    let message_hash = compute_canonical_embedding_hash(
        &embedding_hash,
        idea_id,
        current_time,
        embedding_model_version.as_deref(),
    );
    
    // CRITICAL: Verify signature via CPI to ed25519_program
    verify_ed25519_signature(&message_hash, &embedding_provider_pubkey, &embedding_signature)?;
    
    msg!("SEC-INV-10: Embedding signature verified for provider: {}", embedding_provider);
    
    // SEC-INV-11: Verify provider is in AIServiceRegistry (if provided)
    if let Some(registry_info) = &ctx.accounts.ai_service_registry {
        // Deserialize AIServiceRegistry manually from UncheckedAccount
        let registry = crate::utils::account_helpers::deserialize_ai_service_registry(
            registry_info,
            ctx.program_id,
        )?;
        require!(
            registry.is_service_authorized(&embedding_provider_pubkey) ||
            embedding_provider_pubkey == ctx.accounts.dao_config.authority,
            IndrasError::InvalidEmbeddingProvider
        );
        
        // Check if service supports the model (if model specified)
        if let Some(ref model) = embedding_model {
            if let Some(_service) = registry.get_service(&embedding_provider_pubkey) {
                if !registry.supports_model(&embedding_provider_pubkey, model) {
                    // Warning: service doesn't support this model, but allow if DAO authority
                    require!(
                        embedding_provider_pubkey == ctx.accounts.dao_config.authority,
                        IndrasError::InvalidEmbeddingProvider
                    );
                    msg!("SEC-INV-11: WARNING - Service {} does not support model {}, but allowed as DAO authority", embedding_provider, model);
                } else {
                    msg!("SEC-INV-11: Provider {} supports model {}", embedding_provider, model);
                }
            }
        }
        
        msg!("SEC-INV-11: Provider {} verified in AIServiceRegistry (active, not suspended)", embedding_provider);
    } else {
        require!(
            embedding_provider_pubkey == ctx.accounts.dao_config.authority,
            IndrasError::InvalidEmbeddingProvider
        );
        msg!("SEC-INV-11: Provider verified as DAO authority (registry not provided)");
    }
    
    // Anti-duplication: Check if embedding already exists for this idea with same model_version
    // Seeds: [b"embedding_dedup", idea.key(), embedding_hash, model_version.as_bytes()]
    // Same embedding with different model_version is allowed (model upgrade)
    // Get idea_key BEFORE mutable borrow
    let idea_key = ctx.accounts.idea.key();
    
    // Check dedup account BEFORE mutable borrow
    let model_version_str = embedding_model_version.as_deref().unwrap_or("");
    let expected_seeds = &[
        b"embedding_dedup",
        idea_key.as_ref(),
        embedding_hash.as_ref(),
        model_version_str.as_bytes(),
    ];
    let (expected_pda, _bump) = Pubkey::find_program_address(expected_seeds, ctx.program_id);
    
        // Full anti-duplication: Deserialize and verify duplicate check
        // This ensures the same embedding+model_version cannot be stored twice for the same entity
        if let Some(dedup_info) = &ctx.accounts.embedding_deduplication {
            let dedup_key = dedup_info.key();
            require!(
                dedup_key == expected_pda,
                IndrasError::InvalidInput // PDA seeds mismatch
            );
            
            // Deserialize EmbeddingDeduplication manually from UncheckedAccount
            let dedup = crate::utils::account_helpers::deserialize_embedding_deduplication(dedup_info)?;
            let is_duplicate = dedup.is_duplicate(
                idea_id,
                embedding_hash,
                model_version_str
            );
        
        require!(
            !is_duplicate,
            IndrasError::InvalidInput // Duplicate embedding+model_version for same entity
        );
        
        msg!("Anti-duplication: Full check passed - no duplicate detected");
    }
    
    // NOW take mutable borrow for idea account update
    let idea = &mut ctx.accounts.idea;
    
    // Validate idea ID matches
    require!(idea.id == idea_id, IndrasError::InvalidInput);
    
    // Update embedding metadata
    idea.embedding_hash = Some(embedding_hash);
    idea.embedding_signature = Some(embedding_signature);
    idea.embedding_provider = Some(embedding_provider);
    
    // Track C: Update telemetry fields (C3)
    if idea.embedding_created_at.is_none() {
        idea.embedding_created_at = Some(current_time);
    }
    idea.embedding_updated_at = Some(current_time);
    idea.embedding_update_count = idea.embedding_update_count.saturating_add(1);
    idea.embedding_model = embedding_model;
    idea.embedding_model_version = embedding_model_version;
    
    msg!("Idea {} embedding updated by provider {}", idea_id, embedding_provider_pubkey);
    
    Ok(())
}
