//! Financial Data Validation module
//!
//! Financial data validation
//!
//! On-chain: Metadata for data validation
//! Off-chain: Actual validation, verification

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Validation rule type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataValidationRuleType {
    /// Format validation
    Format,
    /// Range validation
    Range,
    /// Business rule validation
    BusinessRule,
    /// Custom validation
    Custom,
}

/// Validation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataValidationStatus {
    /// Validation active
    Active,
    /// Validation paused
    Paused,
    /// Validation disabled
    Disabled,
}

/// Financial data validation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataValidationMetadata {
    /// Validation ID
    pub validation_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Validation rule type
    pub validation_rule_type: FinancialDataValidationRuleType,
    /// Status
    pub status: FinancialDataValidationStatus,
    /// Created at
    pub created_at: i64,
    /// Validation config hash
    pub validation_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_validation(
        validation: &mut FinancialDataValidationMetadata,
        validation_id: u64,
        entity_id: u64,
        validation_rule_type: FinancialDataValidationRuleType,
        validation_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(validation_id > 0, IndrasError::InvalidInput);
        validation.validation_id = validation_id;
        validation.entity_id = entity_id;
        validation.validation_rule_type = validation_rule_type;
        validation.status = FinancialDataValidationStatus::Active;
        validation.created_at = current_time;
        validation.validation_config_hash = validation_config_hash;
        validation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn validate_financial_data(_validation_id: u64) -> bool {
        false
    }
}
