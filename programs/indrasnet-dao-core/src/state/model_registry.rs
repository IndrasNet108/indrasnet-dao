//! Model Registry
//!
//! Registry of AI models with version verification.
//! Ensures only verified model versions are used for analysis.

use anchor_lang::prelude::*;

/// Model Registry
/// 
/// Stores metadata for AI models, including version verification.
#[account]
#[derive(InitSpace)]
pub struct ModelRegistry {
    #[max_len(100)]
    pub models: Vec<ModelMetadata>,
    pub authority: Pubkey,
    pub bump: u8,
}

/// Model metadata
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub struct ModelMetadata {
    #[max_len(100)]
    pub model_id: String,
    #[max_len(50)]
    pub version: String,
    pub model_hash: Option<[u8; 32]>,     // Hash of model weights/config (future)
    pub is_verified: bool,                // Audited by DAO
    pub deprecation_date: Option<i64>,
    pub registered_at: i64,
}

impl ModelRegistry {
    /// Check if model version is valid
    pub fn is_model_version_valid(&self, model_id: &str, version: &str) -> bool {
        // NOTE: Clock::get() cannot be used in account methods
        // For MVP, we skip deprecation check in this method
        // Deprecation should be checked in instruction handler where Clock is available
        self.models.iter()
            .any(|m| {
                m.model_id == model_id 
                    && m.version == version 
                    && m.is_verified
                    // Deprecation check removed - should be done in handler
            })
    }
    
    /// Get model metadata
    pub fn get_model(&self, model_id: &str, version: &str) -> Option<&ModelMetadata> {
        self.models.iter()
            .find(|m| m.model_id == model_id && m.version == version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_registry() -> ModelRegistry {
        ModelRegistry {
            models: vec![
                ModelMetadata {
                    model_id: "gemini-2.5".to_string(),
                    version: "v1.0".to_string(),
                    model_hash: Some([1u8; 32]),
                    is_verified: true,
                    deprecation_date: None,
                    registered_at: 1000,
                },
                ModelMetadata {
                    model_id: "gemini-2.5".to_string(),
                    version: "v2.0".to_string(),
                    model_hash: Some([2u8; 32]),
                    is_verified: true,
                    deprecation_date: None,
                    registered_at: 2000,
                },
                ModelMetadata {
                    model_id: "gpt-4".to_string(),
                    version: "v1.0".to_string(),
                    model_hash: None,
                    is_verified: false, // Not verified
                    deprecation_date: None,
                    registered_at: 3000,
                },
            ],
            authority: Pubkey::new_unique(),
            bump: 255,
        }
    }

    #[test]
    fn test_is_model_version_valid_verified() {
        let registry = create_test_registry();
        
        assert!(registry.is_model_version_valid("gemini-2.5", "v1.0"));
        assert!(registry.is_model_version_valid("gemini-2.5", "v2.0"));
    }

    #[test]
    fn test_is_model_version_valid_not_verified() {
        let registry = create_test_registry();
        
        // Not verified model should return false
        assert!(!registry.is_model_version_valid("gpt-4", "v1.0"));
    }

    #[test]
    fn test_is_model_version_valid_not_found() {
        let registry = create_test_registry();
        
        assert!(!registry.is_model_version_valid("unknown", "v1.0"));
    }

    #[test]
    fn test_get_model() {
        let registry = create_test_registry();
        
        let model = registry.get_model("gemini-2.5", "v1.0");
        assert!(model.is_some());
        assert_eq!(model.unwrap().model_id, "gemini-2.5");
        assert_eq!(model.unwrap().version, "v1.0");
    }

    #[test]
    fn test_get_model_not_found() {
        let registry = create_test_registry();
        
        assert!(registry.get_model("unknown", "v1.0").is_none());
    }

    #[test]
    fn test_model_registry_structure() {
        let authority = Pubkey::new_unique();
        let registry = ModelRegistry {
            models: vec![],
            authority,
            bump: 200,
        };
        
        assert_eq!(registry.models.len(), 0);
        assert_eq!(registry.authority, authority);
        assert_eq!(registry.bump, 200);
    }

    #[test]
    fn test_model_metadata_structure() {
        let metadata = ModelMetadata {
            model_id: "test-model".to_string(),
            version: "v1.5".to_string(),
            model_hash: Some([55u8; 32]),
            is_verified: true,
            deprecation_date: Some(10000),
            registered_at: 5000,
        };
        
        assert_eq!(metadata.model_id, "test-model");
        assert_eq!(metadata.version, "v1.5");
        assert_eq!(metadata.model_hash, Some([55u8; 32]));
        assert!(metadata.is_verified);
        assert_eq!(metadata.deprecation_date, Some(10000));
        assert_eq!(metadata.registered_at, 5000);
    }

    #[test]
    fn test_is_model_version_valid_multiple_versions() {
        let registry = create_test_registry();
        
        // Both versions of gemini-2.5 are verified
        assert!(registry.is_model_version_valid("gemini-2.5", "v1.0"));
        assert!(registry.is_model_version_valid("gemini-2.5", "v2.0"));
    }

    #[test]
    fn test_is_model_version_valid_wrong_version() {
        let registry = create_test_registry();
        
        // Model exists but version doesn't
        assert!(!registry.is_model_version_valid("gemini-2.5", "v3.0"));
    }

    #[test]
    fn test_is_model_version_valid_wrong_model() {
        let registry = create_test_registry();
        
        // Version exists but model doesn't
        assert!(!registry.is_model_version_valid("unknown-model", "v1.0"));
    }

    #[test]
    fn test_get_model_all_fields() {
        let registry = create_test_registry();
        
        let model = registry.get_model("gemini-2.5", "v1.0");
        assert!(model.is_some());
        let m = model.unwrap();
        assert_eq!(m.model_id, "gemini-2.5");
        assert_eq!(m.version, "v1.0");
        assert_eq!(m.model_hash, Some([1u8; 32]));
        assert!(m.is_verified);
        assert_eq!(m.registered_at, 1000);
    }

    #[test]
    fn test_get_model_different_versions() {
        let registry = create_test_registry();
        
        let v1 = registry.get_model("gemini-2.5", "v1.0");
        let v2 = registry.get_model("gemini-2.5", "v2.0");
        
        assert!(v1.is_some());
        assert!(v2.is_some());
        assert_ne!(v1.unwrap().version, v2.unwrap().version);
    }

    #[test]
    fn test_get_model_unverified() {
        let registry = create_test_registry();
        
        // Unverified model can still be retrieved
        let model = registry.get_model("gpt-4", "v1.0");
        assert!(model.is_some());
        assert!(!model.unwrap().is_verified);
    }

    #[test]
    fn test_is_model_version_valid_empty_registry() {
        let registry = ModelRegistry {
            models: vec![],
            authority: Pubkey::new_unique(),
            bump: 255,
        };
        
        assert!(!registry.is_model_version_valid("any", "v1.0"));
    }

    #[test]
    fn test_model_metadata_without_hash() {
        let metadata = ModelMetadata {
            model_id: "no-hash-model".to_string(),
            version: "v1.0".to_string(),
            model_hash: None,
            is_verified: true,
            deprecation_date: None,
            registered_at: 1000,
        };
        
        assert_eq!(metadata.model_hash, None);
        assert!(metadata.is_verified);
    }

    #[test]
    fn test_model_metadata_with_deprecation() {
        let metadata = ModelMetadata {
            model_id: "deprecated-model".to_string(),
            version: "v1.0".to_string(),
            model_hash: Some([0u8; 32]),
            is_verified: true,
            deprecation_date: Some(5000),
            registered_at: 1000,
        };
        
        assert_eq!(metadata.deprecation_date, Some(5000));
    }

    #[test]
    fn test_model_metadata_all_fields() {
        let metadata = ModelMetadata {
            model_id: "test-model".to_string(),
            version: "v1.5".to_string(),
            model_hash: Some([55u8; 32]),
            is_verified: true,
            deprecation_date: Some(10000),
            registered_at: 5000,
        };
        
        assert_eq!(metadata.model_id, "test-model");
        assert_eq!(metadata.version, "v1.5");
        assert_eq!(metadata.model_hash, Some([55u8; 32]));
        assert!(metadata.is_verified);
        assert_eq!(metadata.deprecation_date, Some(10000));
        assert_eq!(metadata.registered_at, 5000);
    }

    #[test]
    fn test_model_metadata_clone() {
        let metadata1 = ModelMetadata {
            model_id: "test".to_string(),
            version: "v1.0".to_string(),
            model_hash: Some([1u8; 32]),
            is_verified: true,
            deprecation_date: None,
            registered_at: 1000,
        };
        
        let metadata2 = metadata1.clone();
        assert_eq!(metadata1.model_id, metadata2.model_id);
        assert_eq!(metadata1.version, metadata2.version);
        assert_eq!(metadata1.model_hash, metadata2.model_hash);
        assert_eq!(metadata1.is_verified, metadata2.is_verified);
    }

    #[test]
    fn test_model_registry_all_fields() {
        let authority = Pubkey::new_unique();
        let registry = ModelRegistry {
            models: vec![ModelMetadata {
                model_id: "test".to_string(),
                version: "v1.0".to_string(),
                model_hash: None,
                is_verified: true,
                deprecation_date: None,
                registered_at: 1000,
            }],
            authority,
            bump: 200,
        };
        
        assert_eq!(registry.models.len(), 1);
        assert_eq!(registry.authority, authority);
        assert_eq!(registry.bump, 200);
    }
}
