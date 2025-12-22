//! Canonical Hashing Utilities
//!
//! Provides canonical (normalized) hashing for embeddings and distance bundles.
//! Ensures consistent hash computation regardless of input format.

use anchor_lang::prelude::*;
use sha2::{Sha256, Digest};

/// Canonical hash version prefix
/// 
/// Used to version hash formats. If hash format changes, increment this.
const CANONICAL_HASH_VERSION: u8 = 1;

/// Compute canonical hash for embedding
///
/// Canonical format ensures consistent hashing:
/// - Version prefix (1 byte)
/// - Embedding hash (32 bytes, little-endian)
/// - Entity ID (8 bytes, little-endian)
/// - Timestamp (8 bytes, little-endian)
/// - Model version length (1 byte) + Model version bytes (if present)
///
/// This ensures:
/// - Same embedding always produces same hash
/// - Different model versions produce different hashes
/// - Timestamp included for replay protection
pub fn compute_canonical_embedding_hash(
    embedding_hash: &[u8; 32],
    entity_id: u64,
    timestamp: i64,
    model_version: Option<&str>,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    
    // Version prefix
    hasher.update([CANONICAL_HASH_VERSION]);
    
    // Embedding hash (32 bytes, little-endian - already in correct format)
    hasher.update(embedding_hash);
    
    // Entity ID (8 bytes, little-endian)
    hasher.update(entity_id.to_le_bytes());
    
    // Timestamp (8 bytes, little-endian)
    hasher.update(timestamp.to_le_bytes());
    
    // Model version (if present)
    if let Some(version) = model_version {
        // Length (1 byte) + bytes
        hasher.update([version.len() as u8]);
        hasher.update(version.as_bytes());
    } else {
        // Zero length indicates no model version
        hasher.update([0u8]);
    }
    
    hasher.finalize().into()
}

/// Compute canonical hash for distance bundle
///
/// Canonical format ensures consistent hashing:
/// - Version prefix (1 byte)
/// - Source entity (32 bytes, pubkey)
/// - Target entity (32 bytes, pubkey)
/// - Distance (4 bytes, f32, little-endian)
/// - Timestamp (8 bytes, little-endian)
/// - Nonce (8 bytes, little-endian)
/// - Model version length (1 byte) + Model version bytes (if present)
///
/// This ensures:
/// - Same distance bundle always produces same hash
/// - Different model versions produce different hashes
/// - Nonce included for replay protection
pub fn compute_canonical_distance_bundle_hash(
    source_entity: &Pubkey,
    target_entity: &Pubkey,
    distance: f32,
    timestamp: i64,
    nonce: u64,
    model_version: Option<&str>,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    
    // Version prefix
    hasher.update([CANONICAL_HASH_VERSION]);
    
    // Source entity (32 bytes, pubkey bytes)
    hasher.update(source_entity.as_ref());
    
    // Target entity (32 bytes, pubkey bytes)
    hasher.update(target_entity.as_ref());
    
    // Distance (4 bytes, f32, little-endian)
    hasher.update(distance.to_le_bytes());
    
    // Timestamp (8 bytes, little-endian)
    hasher.update(timestamp.to_le_bytes());
    
    // Nonce (8 bytes, little-endian)
    hasher.update(nonce.to_le_bytes());
    
    // Model version (if present)
    if let Some(version) = model_version {
        // Length (1 byte) + bytes
        hasher.update([version.len() as u8]);
        hasher.update(version.as_bytes());
    } else {
        // Zero length indicates no model version
        hasher.update([0u8]);
    }
    
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_compute_canonical_embedding_hash() {
        let embedding_hash = [1u8; 32];
        let entity_id = 123;
        let timestamp = 1000;
        let model_version = Some("v1.0");
        
        let hash1 = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, model_version);
        let hash2 = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, model_version);
        
        // Same inputs should produce same hash
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }

    #[test]
    fn test_compute_canonical_embedding_hash_different_inputs() {
        let embedding_hash = [1u8; 32];
        
        let hash1 = compute_canonical_embedding_hash(&embedding_hash, 1, 1000, Some("v1.0"));
        let hash2 = compute_canonical_embedding_hash(&embedding_hash, 2, 1000, Some("v1.0"));
        let hash3 = compute_canonical_embedding_hash(&embedding_hash, 1, 2000, Some("v1.0"));
        let hash4 = compute_canonical_embedding_hash(&embedding_hash, 1, 1000, Some("v2.0"));
        let hash5 = compute_canonical_embedding_hash(&embedding_hash, 1, 1000, None);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2); // Different entity_id
        assert_ne!(hash1, hash3); // Different timestamp
        assert_ne!(hash1, hash4); // Different model_version
        assert_ne!(hash1, hash5); // With vs without model_version
    }

    #[test]
    fn test_compute_canonical_embedding_hash_no_model_version() {
        let embedding_hash = [1u8; 32];
        let hash = compute_canonical_embedding_hash(&embedding_hash, 1, 1000, None);
        
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash() {
        let source = create_test_pubkey(1);
        let target = create_test_pubkey(2);
        let distance = 0.5;
        let timestamp = 1000;
        let nonce = 123;
        let model_version = Some("v1.0");
        
        let hash1 = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, model_version);
        let hash2 = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, model_version);
        
        // Same inputs should produce same hash
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash_different_inputs() {
        let source1 = create_test_pubkey(1);
        let source2 = create_test_pubkey(2);
        let target = create_test_pubkey(3);
        
        let hash1 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 123, Some("v1.0"));
        let hash2 = compute_canonical_distance_bundle_hash(&source2, &target, 0.5, 1000, 123, Some("v1.0"));
        let hash3 = compute_canonical_distance_bundle_hash(&source1, &target, 0.6, 1000, 123, Some("v1.0"));
        let hash4 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 2000, 123, Some("v1.0"));
        let hash5 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 456, Some("v1.0"));
        let hash6 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 123, Some("v2.0"));
        let hash7 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 123, None);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2); // Different source
        assert_ne!(hash1, hash3); // Different distance
        assert_ne!(hash1, hash4); // Different timestamp
        assert_ne!(hash1, hash5); // Different nonce
        assert_ne!(hash1, hash6); // Different model_version
        assert_ne!(hash1, hash7); // With vs without model_version
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash_no_model_version() {
        let source = create_test_pubkey(1);
        let target = create_test_pubkey(2);
        let hash = compute_canonical_distance_bundle_hash(&source, &target, 0.5, 1000, 123, None);
        
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_compute_canonical_embedding_hash_edge_cases() {
        // Test with zero values
        let hash1 = compute_canonical_embedding_hash(&[0u8; 32], 0, 0, None);
        assert_eq!(hash1.len(), 32);

        // Test with max values
        let hash2 = compute_canonical_embedding_hash(&[255u8; 32], u64::MAX, i64::MAX, Some("v1.0"));
        assert_eq!(hash2.len(), 32);

        // Test with empty model version string (should be equivalent to None)
        let hash3 = compute_canonical_embedding_hash(&[1u8; 32], 1, 1000, Some(""));
        assert_eq!(hash3.len(), 32);
        // Empty string model version should produce same hash as None (both have length 0)
        assert_eq!(hash3, compute_canonical_embedding_hash(&[1u8; 32], 1, 1000, None));
    }

    #[test]
    fn test_compute_canonical_embedding_hash_long_model_version() {
        let long_version = "v".repeat(255);
        let hash = compute_canonical_embedding_hash(&[1u8; 32], 1, 1000, Some(&long_version));
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_compute_canonical_embedding_hash_consistency() {
        let embedding_hash = [42u8; 32];
        let entity_id = 12345;
        let timestamp = 987654321;
        let model_version = Some("test-model-v1.2.3");

        // Compute hash multiple times
        let hash1 = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, model_version);
        let hash2 = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, model_version);
        let hash3 = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, model_version);

        // All should be identical
        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash_edge_cases() {
        let source = create_test_pubkey(1);
        let target = create_test_pubkey(2);

        // Test with zero values
        let hash1 = compute_canonical_distance_bundle_hash(&source, &target, 0.0, 0, 0, None);
        assert_eq!(hash1.len(), 32);

        // Test with max values
        let hash2 = compute_canonical_distance_bundle_hash(&source, &target, f32::MAX, i64::MAX, u64::MAX, Some("v1.0"));
        assert_eq!(hash2.len(), 32);

        // Test with negative distance
        let hash3 = compute_canonical_distance_bundle_hash(&source, &target, -1.0, 1000, 123, None);
        assert_eq!(hash3.len(), 32);

        // Test with empty model version string (should be equivalent to None)
        let hash4 = compute_canonical_distance_bundle_hash(&source, &target, 0.5, 1000, 123, Some(""));
        assert_eq!(hash4.len(), 32);
        // Empty string model version should produce same hash as None (both have length 0)
        assert_eq!(hash4, compute_canonical_distance_bundle_hash(&source, &target, 0.5, 1000, 123, None));
        
        // Test with non-empty model version string (should be different from None)
        let hash5 = compute_canonical_distance_bundle_hash(&source, &target, 0.5, 1000, 123, Some("v1.0"));
        assert_eq!(hash5.len(), 32);
        assert_ne!(hash5, compute_canonical_distance_bundle_hash(&source, &target, 0.5, 1000, 123, None));
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash_same_source_target() {
        let pubkey = create_test_pubkey(1);
        let hash = compute_canonical_distance_bundle_hash(&pubkey, &pubkey, 0.5, 1000, 123, None);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash_consistency() {
        let source = create_test_pubkey(1);
        let target = create_test_pubkey(2);
        let distance = 0.12345;
        let timestamp = 987654321;
        let nonce = 54321;
        let model_version = Some("test-model-v2.0");

        // Compute hash multiple times
        let hash1 = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, model_version);
        let hash2 = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, model_version);
        let hash3 = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, model_version);

        // All should be identical
        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash_float_precision() {
        let source = create_test_pubkey(1);
        let target = create_test_pubkey(2);

        // Test that different float representations produce different hashes
        let hash1 = compute_canonical_distance_bundle_hash(&source, &target, 0.1, 1000, 123, None);
        let hash2 = compute_canonical_distance_bundle_hash(&source, &target, 0.2, 1000, 123, None);
        let hash3 = compute_canonical_distance_bundle_hash(&source, &target, 0.1000001, 1000, 123, None);

        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_compute_canonical_distance_bundle_hash_long_model_version() {
        let source = create_test_pubkey(1);
        let target = create_test_pubkey(2);
        let long_version = "v".repeat(255);
        let hash = compute_canonical_distance_bundle_hash(&source, &target, 0.5, 1000, 123, Some(&long_version));
        assert_eq!(hash.len(), 32);
    }
}
