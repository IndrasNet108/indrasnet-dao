//! Embedding Deduplication Account
//!
//! Anti-duplication PDA to prevent duplicate embeddings for the same entity.
//! Protects against poisoning attacks where the same embedding is submitted multiple times.

use anchor_lang::prelude::*;

/// Embedding Deduplication Account
///
/// PDA account that tracks which embeddings have been stored for which entities.
/// Seeds: [b"embedding_dedup", entity.key(), embedding_hash, model_version.as_bytes()]
///
/// This prevents:
/// - Duplicate embedding submissions for the same entity with same model_version
/// - Replay attacks with the same embedding
/// - Poisoning attacks via duplicate embeddings
/// - Model version mismatch (same embedding with different model_version is allowed)
#[account]
#[derive(InitSpace)]
pub struct EmbeddingDeduplication {
    /// Entity type: "idea" or "mesh_group"
    #[max_len(20)]
    pub entity_type: String,
    /// Entity ID (idea_id or mesh_group_id)
    pub entity_id: u64,
    /// Entity pubkey (Idea or MeshGroup PDA)
    pub entity_pubkey: Pubkey,
    /// Embedding hash (32 bytes)
    pub embedding_hash: [u8; 32],
    /// Model version (e.g., "1.0.0", "1.0.1", "2024-12-20")
    /// Used to allow same embedding with different model versions
    #[max_len(50)]
    pub model_version: String,
    /// Provider pubkey who submitted this embedding
    pub provider_pubkey: Pubkey,
    /// Timestamp when embedding was stored
    pub created_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl EmbeddingDeduplication {
    /// Check if embedding already exists for entity with same model_version
    pub fn is_duplicate(&self, entity_id: u64, embedding_hash: [u8; 32], model_version: &str) -> bool {
        self.entity_id == entity_id 
            && self.embedding_hash == embedding_hash
            && self.model_version == model_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_deduplication() -> EmbeddingDeduplication {
        EmbeddingDeduplication {
            entity_type: "idea".to_string(),
            entity_id: 1,
            entity_pubkey: Pubkey::new_unique(),
            embedding_hash: [1u8; 32],
            model_version: "v1.0".to_string(),
            provider_pubkey: Pubkey::new_unique(),
            created_at: 1000,
            bump: 255,
        }
    }

    #[test]
    fn test_is_duplicate_same_all() {
        let dedup = create_test_deduplication();
        
        assert!(dedup.is_duplicate(1, [1u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_different_entity_id() {
        let dedup = create_test_deduplication();
        
        assert!(!dedup.is_duplicate(2, [1u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_different_hash() {
        let dedup = create_test_deduplication();
        
        assert!(!dedup.is_duplicate(1, [2u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_different_model_version() {
        let dedup = create_test_deduplication();
        
        // Different model version is NOT a duplicate (allowed)
        assert!(!dedup.is_duplicate(1, [1u8; 32], "v2.0"));
    }

    #[test]
    fn test_embedding_deduplication_structure() {
        let entity_pubkey = Pubkey::new_unique();
        let provider_pubkey = Pubkey::new_unique();
        let dedup = EmbeddingDeduplication {
            entity_type: "mesh_group".to_string(),
            entity_id: 999,
            entity_pubkey,
            embedding_hash: [99u8; 32],
            model_version: "v3.5".to_string(),
            provider_pubkey,
            created_at: 5000,
            bump: 128,
        };
        
        assert_eq!(dedup.entity_type, "mesh_group");
        assert_eq!(dedup.entity_id, 999);
        assert_eq!(dedup.entity_pubkey, entity_pubkey);
        assert_eq!(dedup.embedding_hash, [99u8; 32]);
        assert_eq!(dedup.model_version, "v3.5");
        assert_eq!(dedup.provider_pubkey, provider_pubkey);
        assert_eq!(dedup.created_at, 5000);
        assert_eq!(dedup.bump, 128);
    }

    #[test]
    fn test_is_duplicate_all_fields_match() {
        let dedup = create_test_deduplication();
        
        // All fields match - should be duplicate
        assert!(dedup.is_duplicate(1, [1u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_different_entity_id_and_hash() {
        let dedup = create_test_deduplication();
        
        // Both entity_id and hash different
        assert!(!dedup.is_duplicate(2, [2u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_different_entity_id_and_model() {
        let dedup = create_test_deduplication();
        
        // Entity ID and model version different
        assert!(!dedup.is_duplicate(2, [1u8; 32], "v2.0"));
    }

    #[test]
    fn test_is_duplicate_different_hash_and_model() {
        let dedup = create_test_deduplication();
        
        // Hash and model version different
        assert!(!dedup.is_duplicate(1, [2u8; 32], "v2.0"));
    }

    #[test]
    fn test_is_duplicate_same_hash_different_entity() {
        let dedup = create_test_deduplication();
        
        // Same hash but different entity - not duplicate
        assert!(!dedup.is_duplicate(2, [1u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_same_entity_different_hash() {
        let dedup = create_test_deduplication();
        
        // Same entity but different hash - not duplicate
        assert!(!dedup.is_duplicate(1, [2u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_same_entity_and_hash_different_model() {
        let dedup = create_test_deduplication();
        
        // Same entity and hash but different model - NOT duplicate (allowed)
        assert!(!dedup.is_duplicate(1, [1u8; 32], "v2.0"));
        assert!(!dedup.is_duplicate(1, [1u8; 32], "v3.0"));
    }

    #[test]
    fn test_embedding_deduplication_entity_types() {
        let dedup_idea = EmbeddingDeduplication {
            entity_type: "idea".to_string(),
            entity_id: 1,
            entity_pubkey: Pubkey::new_unique(),
            embedding_hash: [0u8; 32],
            model_version: "v1.0".to_string(),
            provider_pubkey: Pubkey::new_unique(),
            created_at: 1000,
            bump: 255,
        };
        
        assert_eq!(dedup_idea.entity_type, "idea");
        
        let dedup_mesh = EmbeddingDeduplication {
            entity_type: "mesh_group".to_string(),
            entity_id: 1,
            entity_pubkey: Pubkey::new_unique(),
            embedding_hash: [0u8; 32],
            model_version: "v1.0".to_string(),
            provider_pubkey: Pubkey::new_unique(),
            created_at: 1000,
            bump: 255,
        };
        
        assert_eq!(dedup_mesh.entity_type, "mesh_group");
    }

    #[test]
    fn test_is_duplicate_edge_cases() {
        let dedup = create_test_deduplication();
        
        // Zero entity ID
        assert!(!dedup.is_duplicate(0, [1u8; 32], "v1.0"));
        
        // Max entity ID
        assert!(!dedup.is_duplicate(u64::MAX, [1u8; 32], "v1.0"));
        
        // Zero hash
        assert!(!dedup.is_duplicate(1, [0u8; 32], "v1.0"));
        
        // Empty model version
        assert!(!dedup.is_duplicate(1, [1u8; 32], ""));
    }

    #[test]
    fn test_embedding_deduplication_all_fields() {
        let entity_pubkey = Pubkey::new_unique();
        let provider_pubkey = Pubkey::new_unique();
        let dedup = EmbeddingDeduplication {
            entity_type: "idea".to_string(),
            entity_id: 123,
            entity_pubkey,
            embedding_hash: [42u8; 32],
            model_version: "v2.1".to_string(),
            provider_pubkey,
            created_at: 5000,
            bump: 128,
        };
        
        assert_eq!(dedup.entity_type, "idea");
        assert_eq!(dedup.entity_id, 123);
        assert_eq!(dedup.entity_pubkey, entity_pubkey);
        assert_eq!(dedup.embedding_hash, [42u8; 32]);
        assert_eq!(dedup.model_version, "v2.1");
        assert_eq!(dedup.provider_pubkey, provider_pubkey);
        assert_eq!(dedup.created_at, 5000);
        assert_eq!(dedup.bump, 128);
    }

    #[test]
    fn test_is_duplicate_large_entity_id() {
        let mut dedup = create_test_deduplication();
        dedup.entity_id = u64::MAX;
        
        assert!(dedup.is_duplicate(u64::MAX, [1u8; 32], "v1.0"));
        assert!(!dedup.is_duplicate(1, [1u8; 32], "v1.0"));
    }

    #[test]
    fn test_is_duplicate_different_hash_patterns() {
        let dedup = create_test_deduplication();
        
        // Test various hash patterns
        let hashes = vec![
            [0u8; 32],
            [1u8; 32],
            [255u8; 32],
            [42u8; 32],
        ];
        
        for hash in hashes {
            if hash == [1u8; 32] {
                assert!(dedup.is_duplicate(1, hash, "v1.0"));
            } else {
                assert!(!dedup.is_duplicate(1, hash, "v1.0"));
            }
        }
    }

    #[test]
    fn test_is_duplicate_different_model_versions() {
        let dedup = create_test_deduplication();
        
        // Test various model versions
        let versions = vec!["v1.0", "v2.0", "v3.5", "2024-12-20", ""];
        
        for version in versions {
            if version == "v1.0" {
                assert!(dedup.is_duplicate(1, [1u8; 32], version));
            } else {
                assert!(!dedup.is_duplicate(1, [1u8; 32], version));
            }
        }
    }

    #[test]
    fn test_embedding_deduplication_clone() {
        let dedup1 = create_test_deduplication();
        let dedup2 = dedup1.clone();
        
        assert_eq!(dedup1.entity_id, dedup2.entity_id);
        assert_eq!(dedup1.embedding_hash, dedup2.embedding_hash);
        assert_eq!(dedup1.model_version, dedup2.model_version);
    }
}
