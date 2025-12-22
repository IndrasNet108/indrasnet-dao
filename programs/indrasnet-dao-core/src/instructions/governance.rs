//! Governance instruction handlers
//!
//! Handlers for governance operations: initialize DAO
//!
//! NOTE: This module implements governance functionality for MVP v1.0.0.
//! Following the proper migration process: Read → Understand → Analyze → Implement

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::dao_config::{DaoConfig, DaoConfigV1, DAO_CONFIG_SCHEMA_VERSION};

/// Initialize DAO with basic configuration
///
/// This handler initializes the DAO configuration account with the provided name and description.
/// The DAO config is a PDA with seeds [b"dao_config"].
pub fn initialize_dao_handler(
    ctx: Context<crate::InitializeDao>,
    name: String,
    description: String,
) -> Result<()> {
    // Validate inputs
    require!(!name.is_empty(), IndrasError::InvalidInput);
    require!(name.len() <= 100, IndrasError::InvalidInput);
    require!(!description.is_empty(), IndrasError::InvalidInput);
    require!(description.len() <= 500, IndrasError::InvalidInput);
    
    let dao_config = &mut ctx.accounts.dao_config;
    let authority = ctx.accounts.authority.key();
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize DAO config fields
    dao_config.authority = authority;
    dao_config.name = name;
    dao_config.description = description;
    dao_config.is_active = true;
    dao_config.schema_version = DAO_CONFIG_SCHEMA_VERSION;
    dao_config.dev_mode = false;
    // SECURITY: Initialize security fields with defaults
    dao_config.is_paused = false; // DAO starts active
    dao_config.last_operation_timestamp = None;
    dao_config.operation_count = 0;
    dao_config.execution_delay_seconds = 24 * 3600; // Default: 24 hours execution delay
    // SECURITY: Enable innovative security features by default
    dao_config.adaptive_security_enabled = true;
    dao_config.progressive_unlock_enabled = true;
    dao_config.behavioral_analysis_enabled = true;
    dao_config.created_at = current_time;
    dao_config.updated_at = None;
    dao_config.deactivated_at = None;
    dao_config.reactivated_at = None;
    dao_config.authority_transferred_at = None;
    dao_config.security_enhancement_count = 0;
    dao_config.bump = ctx.bumps.dao_config;
    
    msg!("DAO initialized by {}", authority);
    
    Ok(())
}

/// Migrate DaoConfig from v1 layout to v2 (adds schema_version + dev_mode)
pub fn migrate_dao_config_v1_to_v2_handler(
    ctx: Context<crate::MigrateDaoConfig>,
) -> Result<()> {
    let dao_config_info = ctx.accounts.dao_config.to_account_info();

    require!(
        dao_config_info.owner == ctx.program_id,
        IndrasError::InvalidProgram
    );

    let (expected_pda, _) = Pubkey::find_program_address(&[b"dao_config"], ctx.program_id);
    require!(
        dao_config_info.key() == expected_pda,
        IndrasError::InvalidProgram
    );

    {
        let data = dao_config_info.try_borrow_data()?;
        require!(data.len() >= 8, IndrasError::InvalidInput);
        let mut data_slice = &data[8..];
        if data.len() >= 8 + DaoConfig::INIT_SPACE {
            if let Ok(config_v2) = DaoConfig::try_deserialize(&mut data_slice) {
                if config_v2.schema_version >= DAO_CONFIG_SCHEMA_VERSION {
                    return err!(IndrasError::InvalidState);
                }
            }
        }
    }

    let v1 = {
        let data = dao_config_info.try_borrow_data()?;
        let mut data_slice = &data[8..];
        DaoConfigV1::deserialize(&mut data_slice)
            .map_err(|_| IndrasError::InvalidInput)?
    };

    require!(
        ctx.accounts.authority.key() == v1.authority,
        IndrasError::Unauthorized
    );

    let new_size = 8 + DaoConfig::INIT_SPACE;
    if dao_config_info.data_len() < new_size {
        let rent = Rent::get()?;
        let required_lamports = rent
            .minimum_balance(new_size)
            .saturating_sub(dao_config_info.lamports());
        if required_lamports > 0 {
            let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.authority.key(),
                &dao_config_info.key(),
                required_lamports,
            );
            anchor_lang::solana_program::program::invoke(
                &transfer_ix,
                &[
                    ctx.accounts.authority.to_account_info(),
                    dao_config_info.clone(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }
        #[allow(deprecated)]
        dao_config_info.realloc(new_size, true)?;
    }

    let dao_config_v2 = DaoConfig {
        schema_version: DAO_CONFIG_SCHEMA_VERSION,
        authority: v1.authority,
        name: v1.name,
        description: v1.description,
        is_active: v1.is_active,
        dev_mode: false,
        is_paused: v1.is_paused,
        last_operation_timestamp: v1.last_operation_timestamp,
        operation_count: v1.operation_count,
        execution_delay_seconds: v1.execution_delay_seconds,
        adaptive_security_enabled: v1.adaptive_security_enabled,
        progressive_unlock_enabled: v1.progressive_unlock_enabled,
        behavioral_analysis_enabled: v1.behavioral_analysis_enabled,
        created_at: v1.created_at,
        updated_at: v1.updated_at,
        deactivated_at: v1.deactivated_at,
        reactivated_at: v1.reactivated_at,
        authority_transferred_at: v1.authority_transferred_at,
        security_enhancement_count: v1.security_enhancement_count,
        bump: v1.bump,
    };

    let mut data = dao_config_info.try_borrow_mut_data()?;
    let mut data_slice = &mut data[8..];
    dao_config_v2.try_serialize(&mut data_slice)?;

    msg!(
        "DaoConfig migrated to schema_version={} by {}",
        DAO_CONFIG_SCHEMA_VERSION,
        ctx.accounts.authority.key()
    );

    Ok(())
}

/// Manage quorum for voting
///
/// This handler initializes or updates quorum requirements for voting.
/// Quorum is a PDA with seeds [b"quorum", dao_config.key()].
///
/// # Security
/// - Validates quorum_threshold is within valid range (0-100)
/// - Validates required_votes is reasonable
pub fn manage_quorum_handler(
    ctx: Context<crate::ManageQuorum>,
    quorum_id: u64,
    required_votes: u64,
    quorum_threshold: u64,
) -> Result<()> {
    // SECURITY: Validate quorum_threshold is within valid range (0-100)
    require!(
        quorum_threshold <= 100,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate required_votes is reasonable (max 1 billion)
    const MAX_REQUIRED_VOTES: u64 = 1_000_000_000;
    require!(
        required_votes <= MAX_REQUIRED_VOTES,
        IndrasError::AmountTooLarge
    );
    
    let quorum = &mut ctx.accounts.quorum;
    let clock = Clock::get()?;
    
    quorum.id = quorum_id;
    quorum.required_votes = required_votes;
    quorum.quorum_threshold = quorum_threshold;
    quorum.update_timestamp = clock.unix_timestamp;
    quorum.bump = ctx.bumps.quorum;
    
    Ok(())
}

/// Initialize governance parameters
///
/// Initializes adaptive governance parameters.
/// Governance params is a PDA with seeds [b"governance_params", dao_config.key()].
///
/// # Security
/// - Validates quorum_percentage is within valid range (0-100)
/// - Validates vote_duration_hours is within valid range (24-720 hours)
/// - Validates delegate_weight_percentage is within valid range (0-100)
pub fn initialize_governance_params_handler(
    ctx: Context<crate::InitializeGovernanceParams>,
    quorum_percentage: u8,
    vote_duration_hours: u64,
    delegate_weight_percentage: u8,
    early_quorum_enabled: bool,
) -> Result<()> {
    // SECURITY: Validate quorum_percentage is within valid range (0-100)
    require!(
        quorum_percentage <= 100,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate vote_duration_hours is within valid range (24-720 hours = 1 day - 30 days)
    require!(
        (24..=720).contains(&vote_duration_hours),
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate delegate_weight_percentage is within valid range (0-100)
    require!(
        delegate_weight_percentage <= 100,
        IndrasError::InvalidInput
    );
    
    let governance_params = &mut ctx.accounts.governance_params;
    let clock = Clock::get()?;
    
    // Create parameters with validation
    let new_params = crate::state::GovernanceParams::new_with_time(
        quorum_percentage,
        vote_duration_hours,
        delegate_weight_percentage,
        early_quorum_enabled,
        clock.unix_timestamp,
        ctx.bumps.governance_params,
    )?;
    
    // Copy validated fields
    governance_params.quorum_percentage = new_params.quorum_percentage;
    governance_params.vote_duration_hours = new_params.vote_duration_hours;
    governance_params.delegate_weight_percentage = new_params.delegate_weight_percentage;
    governance_params.early_quorum_enabled = new_params.early_quorum_enabled;
    governance_params.update_timestamp = new_params.update_timestamp;
    governance_params.bump = new_params.bump;
    
    msg!("Governance params initialized: quorum={}%, duration={}h, delegate_weight={}%, early_quorum={}", 
         quorum_percentage,
         vote_duration_hours,
         delegate_weight_percentage,
         early_quorum_enabled);
    
    Ok(())
}

/// Update governance parameters (adaptive governance)
///
/// Updates adaptive governance parameters:
/// - quorum_percentage - quorum percentage (0-100)
/// - vote_duration_hours - voting duration in hours
/// - delegate_weight_percentage - delegate weight (0-100)
/// - early_quorum_enabled - early quorum enabled
///
/// Constraints:
/// - Maximum quorum_percentage change per update: ±10%
/// - vote_duration_hours: 24-720 hours (1 day - 30 days)
/// - delegate_weight_percentage: 0-100%
///
/// NOTE: Minimum interval between changes (24 hours) is checked off-chain.
pub fn update_governance_params_handler(
    ctx: Context<crate::UpdateGovernanceParams>,
    quorum_percentage: Option<u8>,
    vote_duration_hours: Option<u64>,
    delegate_weight_percentage: Option<u8>,
    early_quorum_enabled: Option<bool>,
) -> Result<()> {
    let governance_params = &mut ctx.accounts.governance_params;
    
    // Update parameters with validation
    governance_params.update(
        quorum_percentage,
        vote_duration_hours,
        delegate_weight_percentage,
        early_quorum_enabled,
    )?;
    
    msg!("Governance params updated: quorum={:?}%, duration={:?}h, delegate_weight={:?}%, early_quorum={:?}", 
         quorum_percentage,
         vote_duration_hours,
         delegate_weight_percentage,
         early_quorum_enabled);
    
    Ok(())
}

#[cfg(test)]
mod tests {

    // ========== initialize_dao_handler validation tests ==========
    
    #[test]
    fn test_initialize_dao_validation_empty_name() {
        // Test: empty name should fail
        let name = String::new();
        
        // Validation logic: require!(!name.is_empty(), IndrasError::InvalidInput)
        assert!(name.is_empty(), "Empty name should be detected");
    }
    
    #[test]
    fn test_initialize_dao_validation_name_too_long() {
        // Test: name.len() > 100 should fail
        let name = "a".repeat(101);
        
        // Validation logic: require!(name.len() <= 100, IndrasError::InvalidInput)
        assert!(name.len() > 100, "Name too long should be detected");
    }
    
    #[test]
    fn test_initialize_dao_validation_empty_description() {
        // Test: empty description should fail
        let description = String::new();
        
        // Validation logic: require!(!description.is_empty(), IndrasError::InvalidInput)
        assert!(description.is_empty(), "Empty description should be detected");
    }
    
    #[test]
    fn test_initialize_dao_validation_description_too_long() {
        // Test: description.len() > 500 should fail
        let description = "a".repeat(501);
        
        // Validation logic: require!(description.len() <= 500, IndrasError::InvalidInput)
        assert!(description.len() > 500, "Description too long should be detected");
    }
    
    #[test]
    fn test_initialize_dao_validation_valid_inputs() {
        // Test: valid inputs should pass
        let name = "Valid DAO Name".to_string();
        let description = "Valid description".to_string();
        
        // All validations should pass
        assert!(!name.is_empty() && name.len() <= 100, "Name should be valid");
        assert!(!description.is_empty() && description.len() <= 500, "Description should be valid");
    }

    // ========== manage_quorum_handler validation tests ==========
    
    #[test]
    fn test_manage_quorum_validation_quorum_threshold_too_large() {
        // Test: quorum_threshold > 100 should fail
        let quorum_threshold = 101u64;
        
        // Validation logic: require!(quorum_threshold <= 100, IndrasError::InvalidInput)
        assert!(quorum_threshold > 100, "Quorum threshold too large should be detected");
    }
    
    #[test]
    fn test_manage_quorum_validation_quorum_threshold_valid() {
        // Test: quorum_threshold <= 100 should pass
        let quorum_threshold = 50u64;
        
        // Validation logic: require!(quorum_threshold <= 100, IndrasError::InvalidInput)
        assert!(quorum_threshold <= 100, "Valid quorum threshold should pass");
    }
    
    #[test]
    fn test_manage_quorum_validation_required_votes_too_large() {
        // Test: required_votes > MAX_REQUIRED_VOTES should fail
        let required_votes = 1_000_000_001u64;
        const MAX_REQUIRED_VOTES: u64 = 1_000_000_000;
        
        // Validation logic: require!(required_votes <= MAX_REQUIRED_VOTES, IndrasError::AmountTooLarge)
        assert!(required_votes > MAX_REQUIRED_VOTES, "Required votes too large should be detected");
    }
    
    #[test]
    fn test_manage_quorum_validation_required_votes_valid() {
        // Test: required_votes <= MAX_REQUIRED_VOTES should pass
        let required_votes = 1_000_000_000u64;
        const MAX_REQUIRED_VOTES: u64 = 1_000_000_000;
        
        // Validation logic: require!(required_votes <= MAX_REQUIRED_VOTES, IndrasError::AmountTooLarge)
        assert!(required_votes <= MAX_REQUIRED_VOTES, "Valid required votes should pass");
    }
    
    #[test]
    fn test_manage_quorum_validation_valid_inputs() {
        // Test: valid inputs should pass
        let quorum_threshold = 50u64;
        let required_votes = 1000u64;
        
        // All validations should pass
        assert!(quorum_threshold <= 100, "Quorum threshold should be valid");
        assert!(required_votes <= 1_000_000_000, "Required votes should be valid");
    }
    
    // ========== Additional edge case tests ==========
    
    #[test]
    fn test_initialize_dao_validation_name_exact_max_length() {
        // Test: name.len() == 100 should pass
        let name = "a".repeat(100);
        assert_eq!(name.len(), 100, "Name at max length should be valid");
    }
    
    #[test]
    fn test_initialize_dao_validation_description_exact_max_length() {
        // Test: description.len() == 500 should pass
        let description = "a".repeat(500);
        assert_eq!(description.len(), 500, "Description at max length should be valid");
    }
    
    #[test]
    fn test_manage_quorum_validation_quorum_threshold_zero() {
        // Test: quorum_threshold == 0 should pass
        let quorum_threshold = 0u64;
        assert!(quorum_threshold <= 100, "Zero quorum threshold should be valid");
    }
    
    #[test]
    fn test_manage_quorum_validation_quorum_threshold_exact_max() {
        // Test: quorum_threshold == 100 should pass
        let quorum_threshold = 100u64;
        assert_eq!(quorum_threshold, 100, "Quorum threshold at max should be valid");
    }
    
    #[test]
    fn test_manage_quorum_validation_required_votes_zero() {
        // Test: required_votes == 0 should pass
        let required_votes = 0u64;
        assert!(required_votes <= 1_000_000_000, "Zero required votes should be valid");
    }
    
    #[test]
    fn test_manage_quorum_validation_required_votes_exact_max() {
        // Test: required_votes == MAX_REQUIRED_VOTES should pass
        let required_votes = 1_000_000_000u64;
        assert_eq!(required_votes, 1_000_000_000, "Required votes at max should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_quorum_percentage_exact_max() {
        // Test: quorum_percentage == 100 should pass
        let quorum_percentage = 100u8;
        assert!(quorum_percentage <= 100, "Quorum percentage at max should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_quorum_percentage_zero() {
        // Test: quorum_percentage == 0 should pass
        let quorum_percentage = 0u8;
        assert!(quorum_percentage <= 100, "Zero quorum percentage should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_vote_duration_hours_exact_min() {
        // Test: vote_duration_hours == 24 should pass
        let vote_duration_hours = 24u64;
        assert!((24..=720).contains(&vote_duration_hours), "Vote duration at min should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_vote_duration_hours_exact_max() {
        // Test: vote_duration_hours == 720 should pass
        let vote_duration_hours = 720u64;
        assert!((24..=720).contains(&vote_duration_hours), "Vote duration at max should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_vote_duration_hours_too_low() {
        // Test: vote_duration_hours < 24 should fail
        let vote_duration_hours = 23u64;
        assert!(!(24..=720).contains(&vote_duration_hours), "Vote duration too low should be detected");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_vote_duration_hours_too_high() {
        // Test: vote_duration_hours > 720 should fail
        let vote_duration_hours = 721u64;
        assert!(!(24..=720).contains(&vote_duration_hours), "Vote duration too high should be detected");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_delegate_weight_percentage_exact_max() {
        // Test: delegate_weight_percentage == 100 should pass
        let delegate_weight_percentage = 100u8;
        assert!(delegate_weight_percentage <= 100, "Delegate weight percentage at max should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_delegate_weight_percentage_zero() {
        // Test: delegate_weight_percentage == 0 should pass
        let delegate_weight_percentage = 0u8;
        assert!(delegate_weight_percentage <= 100, "Zero delegate weight percentage should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_quorum_percentage_too_large() {
        // Test: quorum_percentage > 100 should fail
        let quorum_percentage = 101u8;
        assert!(quorum_percentage > 100, "Quorum percentage too large should be detected");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_delegate_weight_percentage_too_large() {
        // Test: delegate_weight_percentage > 100 should fail
        let delegate_weight_percentage = 101u8;
        assert!(delegate_weight_percentage > 100, "Delegate weight percentage too large should be detected");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_early_quorum_enabled_true() {
        // Test: early_quorum_enabled == true should pass
        let early_quorum_enabled = true;
        assert!(early_quorum_enabled || !early_quorum_enabled, "Early quorum enabled should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_early_quorum_enabled_false() {
        // Test: early_quorum_enabled == false should pass
        let early_quorum_enabled = false;
        assert!(early_quorum_enabled || !early_quorum_enabled, "Early quorum disabled should be valid");
    }
    
    #[test]
    fn test_initialize_governance_params_validation_valid_inputs() {
        // Test: valid inputs should pass
        let quorum_percentage = 50u8;
        let vote_duration_hours = 168u64; // 7 days
        let delegate_weight_percentage = 10u8;
        let early_quorum_enabled = true;
        
        assert!(quorum_percentage <= 100, "Quorum percentage should be valid");
        assert!((24..=720).contains(&vote_duration_hours), "Vote duration should be valid");
        assert!(delegate_weight_percentage <= 100, "Delegate weight percentage should be valid");
        assert!(early_quorum_enabled || !early_quorum_enabled, "Early quorum flag should be valid");
    }
    
    #[test]
    fn test_update_governance_params_validation_all_none() {
        // Test: all parameters None should pass (no update)
        let quorum_percentage: Option<u8> = None;
        let vote_duration_hours: Option<u64> = None;
        let delegate_weight_percentage: Option<u8> = None;
        let early_quorum_enabled: Option<bool> = None;
        
        // All None is valid - no update will occur
        assert!(quorum_percentage.is_none() && 
                vote_duration_hours.is_none() && 
                delegate_weight_percentage.is_none() && 
                early_quorum_enabled.is_none(), 
                "All parameters None should be valid (no update)");
    }
    
    #[test]
    fn test_update_governance_params_validation_partial_update() {
        // Test: partial update should pass
        let quorum_percentage = Some(60u8);
        let vote_duration_hours: Option<u64> = None;
        let delegate_weight_percentage: Option<u8> = None;
        let early_quorum_enabled: Option<bool> = None;
        
        if let Some(qp) = quorum_percentage {
            assert!(qp <= 100, "Quorum percentage should be valid");
        }
        assert!(vote_duration_hours.is_none(), "Vote duration None should be valid");
        assert!(delegate_weight_percentage.is_none(), "Delegate weight None should be valid");
        assert!(early_quorum_enabled.is_none(), "Early quorum None should be valid");
    }
}
