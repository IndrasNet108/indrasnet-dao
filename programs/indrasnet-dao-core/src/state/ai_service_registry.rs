//! AI Service Registry
//!
//! Registry of authorized AI services that can submit analysis results.
//! Provides Proof-of-Presence for AI service authenticity.

use anchor_lang::prelude::*;

/// AI Service Registry
/// 
/// Stores list of authorized AI services that can submit analysis results.
/// Services must be registered by DAO authority.
#[account]
#[derive(InitSpace)]
pub struct AIServiceRegistry {
    #[max_len(5)]
    pub services: Vec<AIService>,
    pub authority: Pubkey,
    pub bump: u8,
}

/// AI Service metadata
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AIService {
    pub pubkey: Pubkey,                    // Service identity
    pub model_ids: Vec<String>,           // Supported models (max 20 items, each max 100 chars)
    pub is_active: bool,
    pub is_suspended: bool,               // Temporary suspension (can be reactivated)
    pub stake_amount: u64,                // Economic security (future)
    pub registered_at: i64,
}

impl anchor_lang::Space for AIService {
    const INIT_SPACE: usize = 32 + 4 + (5 * (4 + 100)) + 1 + 1 + 8 + 8; // pubkey + vec overhead + model_ids (5 items * (4 bytes len + 100 chars)) + is_active + is_suspended + stake + registered_at
}

impl AIServiceRegistry {
    /// Check if service is authorized (active and not suspended)
    pub fn is_service_authorized(&self, service_pubkey: &Pubkey) -> bool {
        self.services.iter()
            .any(|s| s.pubkey == *service_pubkey && s.is_active && !s.is_suspended)
    }
    
    /// Get service by pubkey
    pub fn get_service(&self, service_pubkey: &Pubkey) -> Option<&AIService> {
        self.services.iter()
            .find(|s| s.pubkey == *service_pubkey)
    }
    
    /// Check if service supports model
    pub fn supports_model(&self, service_pubkey: &Pubkey, model_id: &str) -> bool {
        if let Some(service) = self.get_service(service_pubkey) {
            // Check if any model_id in the vector matches (case-sensitive)
            service.model_ids.iter().any(|id| id == model_id)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_registry() -> AIServiceRegistry {
        let service1_pubkey = Pubkey::new_unique();
        let service2_pubkey = Pubkey::new_unique();
        
        AIServiceRegistry {
            services: vec![
                AIService {
                    pubkey: service1_pubkey,
                    model_ids: vec!["model1".to_string(), "model2".to_string()],
                    is_active: true,
                    is_suspended: false,
                    stake_amount: 1000,
                    registered_at: 1000,
                },
                AIService {
                    pubkey: service2_pubkey,
                    model_ids: vec!["model3".to_string()],
                    is_active: false, // Inactive
                    is_suspended: false,
                    stake_amount: 2000,
                    registered_at: 2000,
                },
            ],
            authority: Pubkey::new_unique(),
            bump: 255,
        }
    }

    #[test]
    fn test_is_service_authorized_active() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[0].pubkey;
        
        assert!(registry.is_service_authorized(&service_pubkey));
    }

    #[test]
    fn test_is_service_authorized_inactive() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[1].pubkey;
        
        assert!(!registry.is_service_authorized(&service_pubkey));
    }

    #[test]
    fn test_is_service_authorized_not_found() {
        let registry = create_test_registry();
        let unknown_pubkey = Pubkey::new_unique();
        
        assert!(!registry.is_service_authorized(&unknown_pubkey));
    }

    #[test]
    fn test_get_service() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[0].pubkey;
        
        let service = registry.get_service(&service_pubkey);
        assert!(service.is_some());
        assert_eq!(service.unwrap().pubkey, service_pubkey);
    }

    #[test]
    fn test_get_service_not_found() {
        let registry = create_test_registry();
        let unknown_pubkey = Pubkey::new_unique();
        
        assert!(registry.get_service(&unknown_pubkey).is_none());
    }

    #[test]
    fn test_supports_model() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[0].pubkey;
        
        assert!(registry.supports_model(&service_pubkey, "model1"));
        assert!(registry.supports_model(&service_pubkey, "model2"));
        assert!(!registry.supports_model(&service_pubkey, "model3"));
    }

    #[test]
    fn test_supports_model_service_not_found() {
        let registry = create_test_registry();
        let unknown_pubkey = Pubkey::new_unique();
        
        assert!(!registry.supports_model(&unknown_pubkey, "model1"));
    }

    #[test]
    fn test_is_service_authorized_suspended() {
        let mut registry = create_test_registry();
        // Create a suspended service
        let suspended_pubkey = Pubkey::new_unique();
        registry.services.push(AIService {
            pubkey: suspended_pubkey,
            model_ids: vec!["model4".to_string()],
            is_active: true,
            is_suspended: true, // Suspended
            stake_amount: 3000,
            registered_at: 3000,
        });
        
        // Suspended service should not be authorized
        assert!(!registry.is_service_authorized(&suspended_pubkey));
    }

    #[test]
    fn test_is_service_authorized_active_not_suspended() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[0].pubkey;
        
        // Active and not suspended - should be authorized
        assert!(registry.is_service_authorized(&service_pubkey));
        assert!(registry.services[0].is_active);
        assert!(!registry.services[0].is_suspended);
    }

    #[test]
    fn test_ai_service_registry_structure() {
        let authority = Pubkey::new_unique();
        let registry = AIServiceRegistry {
            services: vec![],
            authority,
            bump: 200,
        };
        
        assert_eq!(registry.services.len(), 0);
        assert_eq!(registry.authority, authority);
        assert_eq!(registry.bump, 200);
    }

    #[test]
    fn test_ai_service_structure() {
        let service_pubkey = Pubkey::new_unique();
        let service = AIService {
            pubkey: service_pubkey,
            model_ids: vec!["model-a".to_string(), "model-b".to_string()],
            is_active: true,
            is_suspended: false,
            stake_amount: 5000,
            registered_at: 4000,
        };
        
        assert_eq!(service.pubkey, service_pubkey);
        assert_eq!(service.model_ids.len(), 2);
        assert_eq!(service.model_ids[0], "model-a");
        assert_eq!(service.model_ids[1], "model-b");
        assert!(service.is_active);
        assert!(!service.is_suspended);
        assert_eq!(service.stake_amount, 5000);
        assert_eq!(service.registered_at, 4000);
    }

    #[test]
    fn test_get_service_all_fields() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[0].pubkey;
        
        let service = registry.get_service(&service_pubkey);
        assert!(service.is_some());
        let s = service.unwrap();
        assert_eq!(s.pubkey, service_pubkey);
        assert_eq!(s.model_ids.len(), 2);
        assert_eq!(s.stake_amount, 1000);
        assert_eq!(s.registered_at, 1000);
    }

    #[test]
    fn test_supports_model_case_sensitive() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[0].pubkey;
        
        // Case-sensitive matching
        assert!(registry.supports_model(&service_pubkey, "model1"));
        assert!(!registry.supports_model(&service_pubkey, "Model1")); // Different case
        assert!(!registry.supports_model(&service_pubkey, "MODEL1")); // Different case
    }

    #[test]
    fn test_supports_model_empty_model_ids() {
        let mut registry = create_test_registry();
        let empty_service_pubkey = Pubkey::new_unique();
        registry.services.push(AIService {
            pubkey: empty_service_pubkey,
            model_ids: vec![], // Empty
            is_active: true,
            is_suspended: false,
            stake_amount: 0,
            registered_at: 5000,
        });
        
        // Service with no models should not support any model
        assert!(!registry.supports_model(&empty_service_pubkey, "any-model"));
    }

    #[test]
    fn test_is_service_authorized_multiple_services() {
        let registry = create_test_registry();
        
        // First service is active
        assert!(registry.is_service_authorized(&registry.services[0].pubkey));
        
        // Second service is inactive
        assert!(!registry.is_service_authorized(&registry.services[1].pubkey));
    }

    #[test]
    fn test_supports_model_multiple_models() {
        let registry = create_test_registry();
        let service_pubkey = registry.services[0].pubkey;
        
        // Service supports multiple models
        assert!(registry.supports_model(&service_pubkey, "model1"));
        assert!(registry.supports_model(&service_pubkey, "model2"));
        assert!(!registry.supports_model(&service_pubkey, "model3")); // Different service
    }

    #[test]
    fn test_ai_service_inactive_and_suspended() {
        let service = AIService {
            pubkey: Pubkey::new_unique(),
            model_ids: vec!["test".to_string()],
            is_active: false,
            is_suspended: true,
            stake_amount: 0,
            registered_at: 1000,
        };
        
        // Both inactive and suspended
        assert!(!service.is_active);
        assert!(service.is_suspended);
    }

    #[test]
    fn test_ai_service_registry_empty_services() {
        let registry = AIServiceRegistry {
            services: vec![],
            authority: Pubkey::new_unique(),
            bump: 255,
        };
        
        let test_pubkey = Pubkey::new_unique();
        assert!(!registry.is_service_authorized(&test_pubkey));
        assert!(registry.get_service(&test_pubkey).is_none());
        assert!(!registry.supports_model(&test_pubkey, "any"));
    }

    #[test]
    fn test_ai_service_all_fields() {
        let service_pubkey = Pubkey::new_unique();
        let service = AIService {
            pubkey: service_pubkey,
            model_ids: vec!["model-a".to_string(), "model-b".to_string()],
            is_active: true,
            is_suspended: false,
            stake_amount: 5000,
            registered_at: 4000,
        };
        
        assert_eq!(service.pubkey, service_pubkey);
        assert_eq!(service.model_ids.len(), 2);
        assert_eq!(service.model_ids[0], "model-a");
        assert_eq!(service.model_ids[1], "model-b");
        assert!(service.is_active);
        assert!(!service.is_suspended);
        assert_eq!(service.stake_amount, 5000);
        assert_eq!(service.registered_at, 4000);
    }

    #[test]
    fn test_ai_service_clone() {
        let service1 = AIService {
            pubkey: Pubkey::new_unique(),
            model_ids: vec!["test".to_string()],
            is_active: true,
            is_suspended: false,
            stake_amount: 1000,
            registered_at: 1000,
        };
        
        let service2 = service1.clone();
        assert_eq!(service1.pubkey, service2.pubkey);
        assert_eq!(service1.model_ids, service2.model_ids);
        assert_eq!(service1.is_active, service2.is_active);
        assert_eq!(service1.stake_amount, service2.stake_amount);
    }

    #[test]
    fn test_ai_service_registry_all_fields() {
        let authority = Pubkey::new_unique();
        let service_pubkey = Pubkey::new_unique();
        let registry = AIServiceRegistry {
            services: vec![AIService {
                pubkey: service_pubkey,
                model_ids: vec!["model1".to_string()],
                is_active: true,
                is_suspended: false,
                stake_amount: 1000,
                registered_at: 1000,
            }],
            authority,
            bump: 128,
        };
        
        assert_eq!(registry.services.len(), 1);
        assert_eq!(registry.authority, authority);
        assert_eq!(registry.bump, 128);
    }
}
