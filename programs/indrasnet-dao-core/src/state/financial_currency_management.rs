//! Financial Currency Management module
//!
//! Financial currency management
//!
//! On-chain: Metadata for currency management
//! Off-chain: Actual management, conversion

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Currency operation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCurrencyOperationType {
    /// Currency conversion
    Conversion,
    /// Currency hedging
    Hedging,
    /// Currency reporting
    Reporting,
    /// Custom operation
    Custom,
}

/// Currency status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCurrencyStatus {
    /// Currency management active
    Active,
    /// Currency management paused
    Paused,
    /// Currency management disabled
    Disabled,
}

/// Financial currency management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCurrencyManagementMetadata {
    /// Currency management ID
    pub currency_management_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Currency operation type
    pub currency_operation_type: FinancialCurrencyOperationType,
    /// Status
    pub status: FinancialCurrencyStatus,
    /// Created at
    pub created_at: i64,
    /// Currency config hash
    pub currency_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_currency_management(
        currency: &mut FinancialCurrencyManagementMetadata,
        currency_management_id: u64,
        entity_id: u64,
        currency_operation_type: FinancialCurrencyOperationType,
        currency_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(currency_management_id > 0, IndrasError::InvalidInput);
        currency.currency_management_id = currency_management_id;
        currency.entity_id = entity_id;
        currency.currency_operation_type = currency_operation_type;
        currency.status = FinancialCurrencyStatus::Active;
        currency.created_at = current_time;
        currency.currency_config_hash = currency_config_hash;
        currency.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_currency(_currency_management_id: u64) -> Vec<u8> {
        vec![]
    }
}
