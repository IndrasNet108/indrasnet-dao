//! Asset Valuation module
//!
//! Asset valuation and pricing
//!
//! On-chain: Metadata for asset valuation
//! Off-chain: Actual valuation, pricing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Valuation method (basic and advanced)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ValuationMethod {
    /// Market value (basic)
    MarketValue,
    /// Book value (basic)
    BookValue,
    /// Discounted cash flow (basic)
    DiscountedCashFlow,
    /// DCF valuation (advanced)
    DCF,
    /// Comparable company analysis (advanced)
    ComparableCompany,
    /// Precedent transactions (advanced)
    PrecedentTransactions,
    /// Custom method
    Custom,
}

/// Valuation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AssetValuationStatus {
    /// Valuation pending
    Pending,
    /// Valuation in progress
    InProgress,
    /// Valuation completed
    Completed,
}

/// Asset valuation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct AssetValuationMetadata {
    /// Valuation ID
    pub valuation_id: u64,
    /// Asset ID
    pub asset_id: u64,
    /// Valuation method
    pub valuation_method: ValuationMethod,
    /// Status
    pub status: AssetValuationStatus,
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
    
    /// Initialize asset valuation (basic or advanced)
    pub fn initialize_asset_valuation(
        valuation: &mut AssetValuationMetadata,
        valuation_id: u64,
        asset_id: u64,
        valuation_method: ValuationMethod,
        valuation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(valuation_id > 0, IndrasError::InvalidInput);
        valuation.valuation_id = valuation_id;
        valuation.asset_id = asset_id;
        valuation.valuation_method = valuation_method;
        valuation.status = AssetValuationStatus::Pending;
        valuation.created_at = current_time;
        valuation.valuation_data_hash = valuation_data_hash;
        valuation.bump = bump;
        Ok(())
    }

    /// Initialize advanced asset valuation (alias for compatibility)
    pub fn initialize_advanced_asset_valuation(
        valuation: &mut AssetValuationMetadata,
        valuation_id: u64,
        asset_id: u64,
        valuation_method: ValuationMethod,
        valuation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        // Use same function, method enum distinguishes basic vs advanced
        initialize_asset_valuation(
            valuation,
            valuation_id,
            asset_id,
            valuation_method,
            valuation_data_hash,
            current_time,
            bump,
        )
    }
}

/// Off-chain functions
pub mod offchain {
    /// Value asset (basic)
    pub fn value_asset(_valuation_id: u64) -> Vec<u8> {
        vec![]
    }

    /// Value asset advanced (advanced methods)
    pub fn value_asset_advanced(_valuation_id: u64) -> Vec<u8> {
        vec![]
    }
}
