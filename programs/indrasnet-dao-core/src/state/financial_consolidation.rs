//! Financial Consolidation module
//!
//! Financial consolidation and aggregation
//!
//! On-chain: Metadata for financial consolidation
//! Off-chain: Actual consolidation, aggregation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Consolidation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialConsolidationType {
    /// Entity consolidation
    Entity,
    /// Period consolidation
    Period,
    /// Currency consolidation
    Currency,
    /// Custom consolidation
    Custom,
}

/// Consolidation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialConsolidationStatus {
    /// Consolidation pending
    Pending,
    /// Consolidation in progress
    InProgress,
    /// Consolidation completed
    Completed,
}

/// Financial consolidation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialConsolidationMetadata {
    /// Consolidation ID
    pub consolidation_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Consolidation type
    pub consolidation_type: FinancialConsolidationType,
    /// Status
    pub status: FinancialConsolidationStatus,
    /// Created at
    pub created_at: i64,
    /// Consolidation data hash
    pub consolidation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_consolidation(
        consolidation: &mut FinancialConsolidationMetadata,
        consolidation_id: u64,
        entity_id: u64,
        consolidation_type: FinancialConsolidationType,
        consolidation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(consolidation_id > 0, IndrasError::InvalidInput);
        consolidation.consolidation_id = consolidation_id;
        consolidation.entity_id = entity_id;
        consolidation.consolidation_type = consolidation_type;
        consolidation.status = FinancialConsolidationStatus::Pending;
        consolidation.created_at = current_time;
        consolidation.consolidation_data_hash = consolidation_data_hash;
        consolidation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn consolidate_financials(_consolidation_id: u64) -> Vec<u8> {
        vec![]
    }
}
