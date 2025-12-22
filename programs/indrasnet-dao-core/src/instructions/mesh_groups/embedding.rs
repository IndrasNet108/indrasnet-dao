//! Mesh group embedding management handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::utils::{verify_ed25519_signature, compute_canonical_embedding_hash};

/// Update mesh group embedding (Track B: B1)
///
/// Updates the embedding metadata for a mesh group with on-chain signature verification.
/// SEC-INV-10: All embeddings MUST be cryptographically signed.
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~10,000 CU
/// - Signature verification (CPI): ~15,000 CU
/// - State update: ~5,000 CU
#[allow(clippy::too_many_arguments)]
pub fn update_mesh_group_embedding_handler(
    ctx: Context<crate::UpdateMeshGroupEmbedding>,
    mesh_group_id: u64,
    embedding_hash: [u8; 32],
    embedding_signature: [u8; 64],
    embedding_provider: String,
    embedding_model: Option<String>,
    embedding_model_version: Option<String>,
    embedding_provider_pubkey: Pubkey,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate mesh group ID matches
    require!(mesh_group.id == mesh_group_id, IndrasError::InvalidInput);
    
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
        mesh_group_id,
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
    
    // Update embedding metadata
    let current_time = Clock::get()?.unix_timestamp;
    mesh_group.embedding_hash = Some(embedding_hash);
    mesh_group.embedding_signature = Some(embedding_signature);
    mesh_group.embedding_provider = Some(embedding_provider);
    
    // Track C: Update telemetry fields (C3)
    if mesh_group.embedding_created_at.is_none() {
        mesh_group.embedding_created_at = Some(current_time);
    }
    mesh_group.embedding_updated_at = Some(current_time);
    mesh_group.embedding_update_count = mesh_group.embedding_update_count.saturating_add(1);
    mesh_group.embedding_model = embedding_model;
    mesh_group.embedding_model_version = embedding_model_version;
    
    msg!("Mesh Group {} embedding updated by provider {}", mesh_group_id, embedding_provider_pubkey);
    
    Ok(())
}
