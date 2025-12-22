//! Financial Valuation module
//!
//! Financial valuation
//!
//! On-chain: Metadata for valuation
//! Off-chain: Actual valuation, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Valuation method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialValuationMethod {
    /// DCF method
    DCF,
    /// Comparable companies
    ComparableCompanies,
    /// Asset-based
    AssetBased,
    /// Custom method
    Custom,
}

/// Valuation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialValuationStatus {
    /// Valuation pending
    Pending,
    /// Valuation in progress
    InProgress,
    /// Valuation completed
    Completed,
}

/// Financial valuation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialValuationMetadata {
    /// Valuation ID
    pub valuation_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Valuation method
    pub valuation_method: FinancialValuationMethod,
    /// Status
    pub status: FinancialValuationStatus,
    /// Created at
    pub created_at: i64,
    /// Valuation data hash
    pub valuation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_valuation(
        valuation: &mut FinancialValuationMetadata,
        valuation_id: u64,
        entity_id: u64,
        valuation_method: FinancialValuationMethod,
        valuation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(valuation_id > 0, IndrasError::InvalidInput);
        valuation.valuation_id = valuation_id;
        valuation.entity_id = entity_id;
        valuation.valuation_method = valuation_method;
        valuation.status = FinancialValuationStatus::Pending;
        valuation.created_at = current_time;
        valuation.valuation_data_hash = valuation_data_hash;
        valuation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn calculate_valuation(_valuation_id: u64) -> Vec<u8> {
        vec![]
    }
}
