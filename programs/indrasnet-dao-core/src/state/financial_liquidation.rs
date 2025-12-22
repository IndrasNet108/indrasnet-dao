//! Financial Liquidation module
//!
//! Financial liquidation
//!
//! On-chain: Metadata for liquidation
//! Off-chain: Actual liquidation, process management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Liquidation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialLiquidationType {
    /// Voluntary liquidation
    Voluntary,
    /// Involuntary liquidation
    Involuntary,
    /// Asset liquidation
    AssetLiquidation,
    /// Custom type
    Custom,
}

/// Liquidation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialLiquidationStatus {
    /// Liquidation pending
    Pending,
    /// Liquidation in progress
    InProgress,
    /// Liquidation completed
    Completed,
}

/// Financial liquidation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialLiquidationMetadata {
    /// Liquidation ID
    pub liquidation_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Liquidation type
    pub liquidation_type: FinancialLiquidationType,
    /// Status
    pub status: FinancialLiquidationStatus,
    /// Created at
    pub created_at: i64,
    /// Liquidation data hash
    pub liquidation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_liquidation(
        liquidation: &mut FinancialLiquidationMetadata,
        liquidation_id: u64,
        entity_id: u64,
        liquidation_type: FinancialLiquidationType,
        liquidation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(liquidation_id > 0, IndrasError::InvalidInput);
        liquidation.liquidation_id = liquidation_id;
        liquidation.entity_id = entity_id;
        liquidation.liquidation_type = liquidation_type;
        liquidation.status = FinancialLiquidationStatus::Pending;
        liquidation.created_at = current_time;
        liquidation.liquidation_data_hash = liquidation_data_hash;
        liquidation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_liquidation(_liquidation_id: u64) -> Vec<u8> {
        vec![]
    }
}
