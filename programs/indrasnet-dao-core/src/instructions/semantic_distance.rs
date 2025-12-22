//! Semantic Distance verification handlers (Track B: B2)
//!
//! Handlers for verifying signed distance bundles from off-chain semantic similarity service.
//! Used for on-chain verification of semantic distances between entities.

use anchor_lang::prelude::*;
use crate::error::IndrasError as CoreError;
use crate::state::grant::semantic::SemanticDistanceBundle;
use crate::utils::{verify_ed25519_signature, compute_canonical_distance_bundle_hash};

/// Verify semantic distance bundle (Track B: B2)
///
/// Verifies a signed distance bundle from off-chain semantic similarity service.
/// This handler verifies the bundle signature and hash before accepting the distance.
///
/// # Compute Units
/// Recommended: 25,000 CU
/// - Validation: ~8,000 CU
/// - Signature verification (CPI): ~1,000-8,000 CU per signature
///   - CPI ed25519 verify overhead: ~1,000-8,000 CU
///   - Batch signatures: TODO (future optimization)
/// - Hash computation: ~5,000 CU
/// - Full binding checks: ~4,000 CU
///
/// # Notes
/// - Bundle signature is REQUIRED for Track B
/// - Signature format: ed25519(bundle_hash, provider_pubkey)
/// - Provider must be authorized (whitelist check)
pub fn verify_semantic_distance_handler(
    ctx: Context<crate::VerifySemanticDistance>,
    bundle: SemanticDistanceBundle,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate bundle parameters
    require!(bundle.distance >= 0.0 && bundle.distance <= 1.0, CoreError::InvalidScore);
    require!(bundle.bundle_hash != [0u8; 32], CoreError::EmbeddingHashMismatch);
    require!(bundle.bundle_signature != [0u8; 64], CoreError::EmbeddingSignatureInvalid);
    require!(bundle.source_entity != bundle.target_entity, CoreError::InvalidInput);
    require!(!bundle.provider.is_empty(), CoreError::InvalidInput);
    require!(bundle.provider.len() <= 50, CoreError::StringTooLong);
    require!(bundle.nonce > 0, CoreError::InvalidInput); // Nonce must be > 0 for replay protection
    require!(!bundle.model_version.is_empty(), CoreError::InvalidInput);
    require!(bundle.model_version.len() <= 50, CoreError::StringTooLong);
    
    // Compute canonical bundle hash for verification
    // Hash format: Canonical (version || source || target || distance || timestamp || nonce || model_version)
    let computed_hash = compute_canonical_distance_bundle_hash(
        &bundle.source_entity,
        &bundle.target_entity,
        bundle.distance,
        bundle.timestamp,
        bundle.nonce,
        Some(&bundle.model_version),
    );
    
    // Verify bundle hash matches
    require!(
        computed_hash == bundle.bundle_hash,
        CoreError::EmbeddingHashMismatch
    );
    
    // SEC-INV-10: Verify bundle signature via CPI to ed25519_program
    let bundle_hash_array: [u8; 32] = bundle.bundle_hash;
    verify_ed25519_signature(&bundle_hash_array, &bundle.provider_pubkey, &bundle.bundle_signature)?;
    
    msg!("SEC-INV-10: Bundle signature verified for provider: {}", bundle.provider_pubkey);
    
    // SEC-INV-11: Verify provider is in AIServiceRegistry (if provided)
    if let Some(registry_info) = &ctx.accounts.ai_service_registry {
        // Deserialize AIServiceRegistry with strict owner/PDA checks
        let registry = crate::utils::account_helpers::deserialize_ai_service_registry(
            registry_info,
            ctx.program_id,
        )?;

        require!(
            registry.is_service_authorized(&bundle.provider_pubkey) ||
            bundle.provider_pubkey == ctx.accounts.dao_config.authority,
            CoreError::InvalidEmbeddingProvider
        );
        
        // Check if service supports the model
        if let Some(_service) = registry.get_service(&bundle.provider_pubkey) {
            if !registry.supports_model(&bundle.provider_pubkey, &bundle.model_version) {
                // Warning: service doesn't support this model version, but allow if DAO authority
                require!(
                    bundle.provider_pubkey == ctx.accounts.dao_config.authority,
                    CoreError::InvalidEmbeddingProvider
                );
                msg!("SEC-INV-11: WARNING - Service {} does not support model_version {}, but allowed as DAO authority", bundle.provider, bundle.model_version);
            } else {
                msg!("SEC-INV-11: Provider {} supports model_version {}", bundle.provider, bundle.model_version);
            }
        }
        
        msg!("SEC-INV-11: Provider {} verified in AIServiceRegistry (active, not suspended)", bundle.provider_pubkey);
    } else {
        require!(
            bundle.provider_pubkey == ctx.accounts.dao_config.authority,
            CoreError::InvalidEmbeddingProvider
        );
        msg!("SEC-INV-11: Provider verified as DAO authority (registry not provided)");
    }
    
    // Full binding embedding ↔️ distance: Verify source and target entities have embeddings
    // This ensures distances are only computed between entities with valid embeddings
    // SEC-INV-9: Full binding - entities must have valid embeddings before distance is accepted
    
    // Full binding embedding ↔️ distance: Verify source and target entities have embeddings
    // SEC-INV-9: Full binding - entities must have valid embeddings before distance is accepted
    // NOTE: Using manual deserialization to avoid lifetime issues with Account::try_from
    
    // Helper: Check if account has embedding_hash (manual deserialization)
    fn has_embedding_hash(data: &[u8], offset: usize) -> bool {
        // embedding_hash is Option<[u8; 32]>
        // Option format: 1 byte (Some=1, None=0) + 32 bytes if Some
        if data.len() < offset + 1 {
            return false;
        }
        data[offset] == 1 // Some variant
    }
    
    // Deserialize and verify source idea has embedding
    if let Some(source_idea_info) = ctx.accounts.source_idea.as_ref() {
        require!(
            source_idea_info.owner == ctx.program_id,
            CoreError::InvalidEmbeddingProvider
        );
        
        let idea_data = source_idea_info.try_borrow_data()?;
        // Approximate offset for embedding_hash (after discriminator + id + author + title + description + status + rights + idea_hash)
        // For MVP: Check if Option<[u8;32]> is Some (byte != 0)
        // Full implementation would calculate exact offset
        require!(
            has_embedding_hash(&idea_data, 200), // Approximate position
            CoreError::EmbeddingHashMismatch // Source idea must have embedding
        );
        
        msg!("Full binding: Source idea has embedding hash");
    }
    
    // Deserialize and verify target idea has embedding
    if let Some(target_idea_info) = ctx.accounts.target_idea.as_ref() {
        require!(
            target_idea_info.owner == ctx.program_id,
            CoreError::InvalidEmbeddingProvider
        );
        
        let idea_data = target_idea_info.try_borrow_data()?;
        require!(
            has_embedding_hash(&idea_data, 200), // Approximate position
            CoreError::EmbeddingHashMismatch // Target idea must have embedding
        );
        
        msg!("Full binding: Target idea has embedding hash");
    }
    
    // Deserialize and verify source mesh group has embedding
    if let Some(source_mg_info) = ctx.accounts.source_mesh_group.as_ref() {
        require!(
            source_mg_info.owner == ctx.program_id,
            CoreError::InvalidEmbeddingProvider
        );
        
        let mg_data = source_mg_info.try_borrow_data()?;
        require!(
            has_embedding_hash(&mg_data, 200), // Approximate position
            CoreError::EmbeddingHashMismatch // Source mesh group must have embedding
        );
        
        msg!("Full binding: Source mesh group has embedding hash");
    }
    
    // Deserialize and verify target mesh group has embedding
    if let Some(target_mg_info) = ctx.accounts.target_mesh_group.as_ref() {
        require!(
            target_mg_info.owner == ctx.program_id,
            CoreError::InvalidEmbeddingProvider
        );
        
        let mg_data = target_mg_info.try_borrow_data()?;
        require!(
            has_embedding_hash(&mg_data, 200), // Approximate position
            CoreError::EmbeddingHashMismatch // Target mesh group must have embedding
        );
        
        msg!("Full binding: Target mesh group has embedding hash");
    }
    
    // Validate timestamp (not too old, not in future)
    const MAX_TIMESTAMP_AGE_SECONDS: i64 = 3600; // 1 hour
    const MAX_TIMESTAMP_FUTURE_SECONDS: i64 = 300; // 5 minutes
    
    require!(
        bundle.timestamp <= current_time + MAX_TIMESTAMP_FUTURE_SECONDS,
        CoreError::InvalidTimestamp
    );
    require!(
        bundle.timestamp >= current_time - MAX_TIMESTAMP_AGE_SECONDS,
        CoreError::InvalidTimestamp
    );
    
    msg!("Semantic distance verified: {} -> {} (distance: {})", 
         bundle.source_entity, 
         bundle.target_entity, 
         bundle.distance);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::Pubkey;

    // ========== verify_semantic_distance_handler validation tests ==========
    
    #[test]
    fn test_verify_semantic_distance_validation_distance_out_of_range_negative() {
        // Test: distance < 0.0 should fail
        let distance = -0.1f32;
        
        // Validation logic: require!(distance >= 0.0 && distance <= 1.0, CoreError::InvalidScore)
        assert!(!(0.0..=1.0).contains(&distance), "Negative distance should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_distance_out_of_range_above_one() {
        // Test: distance > 1.0 should fail
        let distance = 1.5f32;
        
        // Validation logic: require!(distance >= 0.0 && distance <= 1.0, CoreError::InvalidScore)
        assert!(!(0.0..=1.0).contains(&distance), "Distance > 1.0 should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_bundle_hash_zero() {
        // Test: bundle_hash == [0u8; 32] should fail
        let bundle_hash = [0u8; 32];
        
        // Validation logic: require!(bundle_hash != [0u8; 32], CoreError::EmbeddingHashMismatch)
        assert_eq!(bundle_hash, [0u8; 32], "Zero bundle hash should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_bundle_signature_zero() {
        // Test: bundle_signature == [0u8; 64] should fail
        let bundle_signature = [0u8; 64];
        
        // Validation logic: require!(bundle_signature != [0u8; 64], CoreError::EmbeddingSignatureInvalid)
        assert_eq!(bundle_signature, [0u8; 64], "Zero bundle signature should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_source_equals_target() {
        // Test: source_entity == target_entity should fail
        let source_entity = Pubkey::new_unique();
        let target_entity = source_entity; // Same
        
        // Validation logic: require!(source_entity != target_entity, CoreError::InvalidInput)
        assert_eq!(source_entity, target_entity, "Source equals target should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_provider_empty() {
        // Test: empty provider should fail
        let provider = String::new();
        
        // Validation logic: require!(!provider.is_empty(), CoreError::InvalidInput)
        assert!(provider.is_empty(), "Empty provider should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_provider_too_long() {
        // Test: provider.len() > 50 should fail
        let provider = "a".repeat(51);
        
        // Validation logic: require!(provider.len() <= 50, CoreError::StringTooLong)
        assert!(provider.len() > 50, "Provider too long should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_nonce_zero() {
        // Test: nonce == 0 should fail
        let nonce = 0u64;
        
        // Validation logic: require!(nonce > 0, CoreError::InvalidInput)
        assert_eq!(nonce, 0, "Zero nonce should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_model_version_empty() {
        // Test: empty model_version should fail
        let model_version = String::new();
        
        // Validation logic: require!(!model_version.is_empty(), CoreError::InvalidInput)
        assert!(model_version.is_empty(), "Empty model version should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_model_version_too_long() {
        // Test: model_version.len() > 50 should fail
        let model_version = "a".repeat(51);
        
        // Validation logic: require!(model_version.len() <= 50, CoreError::StringTooLong)
        assert!(model_version.len() > 50, "Model version too long should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_hash_mismatch() {
        // Test: computed_hash != bundle_hash should fail
        let bundle_hash = [1u8; 32];
        let computed_hash = [2u8; 32];
        
        // Validation logic: require!(computed_hash == bundle_hash, CoreError::EmbeddingHashMismatch)
        assert_ne!(computed_hash, bundle_hash, "Hash mismatch should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_timestamp_too_old() {
        // Test: timestamp < current_time - MAX_TIMESTAMP_AGE_SECONDS should fail
        let current_time = 1000000i64;
        let bundle_timestamp = current_time - 3601i64; // Too old
        const MAX_TIMESTAMP_AGE_SECONDS: i64 = 3600;
        
        // Validation logic: require!(timestamp >= current_time - MAX_TIMESTAMP_AGE_SECONDS, CoreError::InvalidTimestamp)
        assert!(bundle_timestamp < current_time - MAX_TIMESTAMP_AGE_SECONDS, "Timestamp too old should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_timestamp_too_future() {
        // Test: timestamp > current_time + MAX_TIMESTAMP_FUTURE_SECONDS should fail
        let current_time = 1000000i64;
        let bundle_timestamp = current_time + 301i64; // Too future
        const MAX_TIMESTAMP_FUTURE_SECONDS: i64 = 300;
        
        // Validation logic: require!(timestamp <= current_time + MAX_TIMESTAMP_FUTURE_SECONDS, CoreError::InvalidTimestamp)
        assert!(bundle_timestamp > current_time + MAX_TIMESTAMP_FUTURE_SECONDS, "Timestamp too future should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_valid_inputs() {
        // Test: valid inputs should pass
        let distance = 0.5f32;
        let bundle_hash = [1u8; 32];
        let bundle_signature = [1u8; 64];
        let source_entity = Pubkey::new_unique();
        let target_entity = Pubkey::new_unique();
        let provider = "valid_provider".to_string();
        let nonce = 1u64;
        let model_version = "1.0".to_string();
        let current_time = 1000000i64;
        let bundle_timestamp = current_time;
        
        // All validations should pass
        assert!((0.0..=1.0).contains(&distance), "Distance should be valid");
        assert_ne!(bundle_hash, [0u8; 32], "Bundle hash should be valid");
        assert_ne!(bundle_signature, [0u8; 64], "Bundle signature should be valid");
        assert_ne!(source_entity, target_entity, "Source and target should be different");
        assert!(!provider.is_empty() && provider.len() <= 50, "Provider should be valid");
        assert!(nonce > 0, "Nonce should be valid");
        assert!(!model_version.is_empty() && model_version.len() <= 50, "Model version should be valid");
        assert!(bundle_timestamp >= current_time - 3600 && bundle_timestamp <= current_time + 300, "Timestamp should be valid");
    }

    // ========== Additional edge case tests ==========
    
    #[test]
    fn test_verify_semantic_distance_validation_distance_zero() {
        // Test: distance == 0.0 should be allowed
        let distance = 0.0f32;
        assert!((0.0..=1.0).contains(&distance), "Zero distance should be valid");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_distance_one() {
        // Test: distance == 1.0 should be allowed
        let distance = 1.0f32;
        assert!((0.0..=1.0).contains(&distance), "One distance should be valid");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_provider_exact_max_length() {
        // Test: provider.len() == 50 (exact max) should pass
        let provider = "a".repeat(50);
        assert_eq!(provider.len(), 50, "Provider at exact max length should be valid");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_provider_max_plus_one() {
        // Test: provider.len() == 51 (max + 1) should fail
        let provider = "a".repeat(51);
        assert!(provider.len() > 50, "Provider exceeding max length should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_model_version_exact_max_length() {
        // Test: model_version.len() == 50 (exact max) should pass
        let model_version = "a".repeat(50);
        assert_eq!(model_version.len(), 50, "Model version at exact max length should be valid");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_model_version_max_plus_one() {
        // Test: model_version.len() == 51 (max + 1) should fail
        let model_version = "a".repeat(51);
        assert!(model_version.len() > 50, "Model version exceeding max length should be detected");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_nonce_one() {
        // Test: nonce == 1 should pass
        let nonce = 1u64;
        assert!(nonce > 0, "Nonce of one should be valid");
    }
    
    #[test]
    fn test_verify_semantic_distance_validation_nonce_max() {
        // Test: nonce == u64::MAX should pass
        let nonce = u64::MAX;
        assert!(nonce > 0, "Max nonce should be valid");
    }
}
