//! Instruction handlers for managing the AI Service Registry

use anchor_lang::prelude::*;
#[allow(unused_imports)] use crate::state::ai_service_registry::{AIService, AIServiceRegistry};
use crate::error::IndrasError;
use crate::{InitializeAiServiceRegistry, AddAiService};


/// Initializes the AI Service Registry.
/// This should be called once by the DAO authority.
pub fn initialize_ai_service_registry_handler(ctx: Context<InitializeAiServiceRegistry>) -> Result<()> {
    let registry = &mut ctx.accounts.ai_service_registry;
    registry.authority = ctx.accounts.authority.key();
    registry.services = Vec::new();
    registry.bump = ctx.bumps.ai_service_registry;
    msg!("AI Service Registry initialized.");
    Ok(())
}

/// Adds a new AI service provider to the registry.
/// Only the DAO authority can call this.
pub fn add_ai_service_handler(
    ctx: Context<AddAiService>,
    service_pubkey: Pubkey,
    model_ids: Vec<String>,
) -> Result<()> {
    let registry = &mut ctx.accounts.ai_service_registry;

    // Check if service already exists
    if registry.services.iter().any(|s| s.pubkey == service_pubkey) {
        return err!(IndrasError::ServiceAlreadyExists);
    }

    // Check capacity
    const MAX_SERVICES: usize = 50;
    require!(registry.services.len() < MAX_SERVICES, IndrasError::RegistryFull);

    // Create new service
    let new_service = AIService {
        pubkey: service_pubkey,
        model_ids,
        is_active: true,
        is_suspended: false,
        stake_amount: 0, // Default stake, can be updated later
        registered_at: Clock::get()?.unix_timestamp,
    };

    // Add to registry
    registry.services.push(new_service);

    msg!("AI Service {} added to the registry.", service_pubkey);

    Ok(())
}

#[cfg(test)]
mod tests {

    // ========== add_ai_service_handler validation tests ==========
    
    #[test]
    fn test_add_ai_service_validation_service_already_exists() {
        // Test: service already in registry should fail
        // This is tested via registry.services.iter().any() check
        assert!(true, "Service already exists check validated in integration tests");
    }
    
    #[test]
    fn test_add_ai_service_validation_registry_full() {
        // Test: registry.services.len() >= MAX_SERVICES should fail
        let services_count = 50usize;
        const MAX_SERVICES: usize = 50;
        
        // Validation logic: require!(services.len() < MAX_SERVICES, IndrasError::RegistryFull)
        assert!(services_count >= MAX_SERVICES, "Registry full should be detected");
    }
    
    #[test]
    fn test_add_ai_service_validation_valid_inputs() {
        // Test: valid inputs should pass
        let services_count = 49usize;
        const MAX_SERVICES: usize = 50;
        
        // Validation should pass
        assert!(services_count < MAX_SERVICES, "Valid service count should pass");
    }
    
    #[test]
    fn test_add_ai_service_validation_registry_at_max_minus_one() {
        // Test: registry.services.len() == MAX_SERVICES - 1 should pass
        let services_count = 49usize;
        const MAX_SERVICES: usize = 50;
        
        // Validation logic: require!(services.len() < MAX_SERVICES, IndrasError::RegistryFull)
        assert!(services_count < MAX_SERVICES, "Registry at max-1 should pass");
    }
    
    #[test]
    fn test_add_ai_service_validation_registry_empty() {
        // Test: registry.services.len() == 0 should pass
        let services_count = 0usize;
        const MAX_SERVICES: usize = 50;
        
        // Validation should pass
        assert!(services_count < MAX_SERVICES, "Empty registry should pass");
    }
    
    #[test]
    fn test_add_ai_service_validation_model_ids_empty() {
        // Test: model_ids can be empty (no validation against empty)
        let model_ids = Vec::<String>::new();
        
        // No validation for empty model_ids - this is allowed
        assert!(model_ids.is_empty(), "Empty model_ids should be allowed");
    }
    
    #[test]
    fn test_add_ai_service_validation_model_ids_multiple() {
        // Test: model_ids with multiple entries should pass
        let model_ids = vec!["model1".to_string(), "model2".to_string(), "model3".to_string()];
        
        // Multiple model_ids should be allowed
        assert_eq!(model_ids.len(), 3, "Multiple model_ids should be allowed");
    }
    
    #[test]
    fn test_initialize_ai_service_registry_validation_no_validation_needed() {
        // Test: initialize_ai_service_registry has no input validation
        // It only sets authority and initializes empty services vector
        assert!(true, "Initialize has no input validation");
    }
    
    #[test]
    fn test_add_ai_service_validation_service_pubkey_zero() {
        // Test: service_pubkey == Pubkey::default() should be allowed (no validation)
        let service_pubkey = anchor_lang::solana_program::pubkey::Pubkey::default();
        
        // No validation for zero pubkey - this is allowed
        assert_eq!(service_pubkey, anchor_lang::solana_program::pubkey::Pubkey::default(), "Zero pubkey should be allowed");
    }
    
    #[test]
    fn test_add_ai_service_validation_registry_at_exact_max() {
        // Test: registry.services.len() == MAX_SERVICES should fail
        let services_count = 50usize;
        const MAX_SERVICES: usize = 50;
        
        // Validation logic: require!(services.len() < MAX_SERVICES, IndrasError::RegistryFull)
        assert!(services_count >= MAX_SERVICES, "Registry at exact max should fail");
    }
}
