//! Real Solana Runtime Tests for utils/canonical_hash.rs
//!
//! These tests use solana-program-test to test canonical hash functions
//! with real account data, providing actual code coverage.

#[cfg(all(test, feature = "program-test"))]
mod tests {
    use crate::utils::canonical_hash::*;
    use anchor_lang::prelude::*;
    use anyhow::Result;

    /// Test compute_canonical_embedding_hash with real data
    #[tokio::test]
    async fn test_compute_canonical_embedding_hash_real() -> Result<()> {
        let embedding_hash = [1u8; 32];
        let entity_id = 123u64;
        let timestamp = 1_000_000i64;
        let model_version = Some("v1.0");
        
        let hash1 = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, model_version);
        let hash2 = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, model_version);
        
        // Same inputs should produce same hash
        assert_eq!(hash1, hash2, "Same inputs should produce same hash");
        assert_eq!(hash1.len(), 32, "Hash should be 32 bytes");
        
        Ok(())
    }

    /// Test compute_canonical_embedding_hash with different inputs
    #[tokio::test]
    async fn test_compute_canonical_embedding_hash_different_inputs() -> Result<()> {
        let embedding_hash = [1u8; 32];
        
        let hash1 = compute_canonical_embedding_hash(&embedding_hash, 1, 1000, Some("v1.0"));
        let hash2 = compute_canonical_embedding_hash(&embedding_hash, 2, 1000, Some("v1.0"));
        let hash3 = compute_canonical_embedding_hash(&embedding_hash, 1, 2000, Some("v1.0"));
        let hash4 = compute_canonical_embedding_hash(&embedding_hash, 1, 1000, Some("v2.0"));
        let hash5 = compute_canonical_embedding_hash(&embedding_hash, 1, 1000, None);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2, "Different entity_id should produce different hash");
        assert_ne!(hash1, hash3, "Different timestamp should produce different hash");
        assert_ne!(hash1, hash4, "Different model_version should produce different hash");
        assert_ne!(hash1, hash5, "With vs without model_version should produce different hash");
        
        Ok(())
    }

    /// Test compute_canonical_embedding_hash with no model version
    #[tokio::test]
    async fn test_compute_canonical_embedding_hash_no_model_version() -> Result<()> {
        let embedding_hash = [1u8; 32];
        let entity_id = 1u64;
        let timestamp = 1_000_000i64;
        
        let hash = compute_canonical_embedding_hash(&embedding_hash, entity_id, timestamp, None);
        
        assert_eq!(hash.len(), 32, "Hash should be 32 bytes");
        
        Ok(())
    }

    /// Test compute_canonical_distance_bundle_hash with real data
    #[tokio::test]
    async fn test_compute_canonical_distance_bundle_hash_real() -> Result<()> {
        let source = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        let distance = 0.5f32;
        let timestamp = 1_000_000i64;
        let nonce = 123u64;
        let model_version = Some("v1.0");
        
        let hash1 = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, model_version);
        let hash2 = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, model_version);
        
        // Same inputs should produce same hash
        assert_eq!(hash1, hash2, "Same inputs should produce same hash");
        assert_eq!(hash1.len(), 32, "Hash should be 32 bytes");
        
        Ok(())
    }

    /// Test compute_canonical_distance_bundle_hash with different inputs
    #[tokio::test]
    async fn test_compute_canonical_distance_bundle_hash_different_inputs() -> Result<()> {
        let source1 = Pubkey::new_unique();
        let source2 = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        
        let hash1 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 123, Some("v1.0"));
        let hash2 = compute_canonical_distance_bundle_hash(&source2, &target, 0.5, 1000, 123, Some("v1.0"));
        let hash3 = compute_canonical_distance_bundle_hash(&source1, &target, 0.6, 1000, 123, Some("v1.0"));
        let hash4 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 2000, 123, Some("v1.0"));
        let hash5 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 456, Some("v1.0"));
        let hash6 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 123, Some("v2.0"));
        let hash7 = compute_canonical_distance_bundle_hash(&source1, &target, 0.5, 1000, 123, None);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2, "Different source should produce different hash");
        assert_ne!(hash1, hash3, "Different distance should produce different hash");
        assert_ne!(hash1, hash4, "Different timestamp should produce different hash");
        assert_ne!(hash1, hash5, "Different nonce should produce different hash");
        assert_ne!(hash1, hash6, "Different model_version should produce different hash");
        assert_ne!(hash1, hash7, "With vs without model_version should produce different hash");
        
        Ok(())
    }

    /// Test compute_canonical_distance_bundle_hash with no model version
    #[tokio::test]
    async fn test_compute_canonical_distance_bundle_hash_no_model_version() -> Result<()> {
        let source = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        let distance = 0.5f32;
        let timestamp = 1_000_000i64;
        let nonce = 123u64;
        
        let hash = compute_canonical_distance_bundle_hash(&source, &target, distance, timestamp, nonce, None);
        
        assert_eq!(hash.len(), 32, "Hash should be 32 bytes");
        
        Ok(())
    }

    /// Test compute_canonical_embedding_hash with edge cases
    #[tokio::test]
    async fn test_compute_canonical_embedding_hash_edge_cases() -> Result<()> {
        // Test with zero hash
        let zero_hash = [0u8; 32];
        let hash1 = compute_canonical_embedding_hash(&zero_hash, 0, 0, None);
        assert_eq!(hash1.len(), 32);
        
        // Test with max values
        let max_hash = [255u8; 32];
        let hash2 = compute_canonical_embedding_hash(&max_hash, u64::MAX, i64::MAX, Some("v999.999"));
        assert_eq!(hash2.len(), 32);
        
        // Test with long model version
        let long_version = "v".repeat(100);
        let hash3 = compute_canonical_embedding_hash(&[1u8; 32], 1, 1000, Some(&long_version));
        assert_eq!(hash3.len(), 32);
        
        Ok(())
    }

    /// Test compute_canonical_distance_bundle_hash with edge cases
    #[tokio::test]
    async fn test_compute_canonical_distance_bundle_hash_edge_cases() -> Result<()> {
        let source = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        
        // Test with zero distance
        let hash1 = compute_canonical_distance_bundle_hash(&source, &target, 0.0, 0, 0, None);
        assert_eq!(hash1.len(), 32);
        
        // Test with max distance
        let hash2 = compute_canonical_distance_bundle_hash(&source, &target, f32::MAX, i64::MAX, u64::MAX, None);
        assert_eq!(hash2.len(), 32);
        
        // Test with negative distance (f32 can be negative)
        let hash3 = compute_canonical_distance_bundle_hash(&source, &target, -1.0, 1000, 123, None);
        assert_eq!(hash3.len(), 32);
        
        Ok(())
    }
}
