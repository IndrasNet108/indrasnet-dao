//! Financial Bankruptcy module
//!
//! Financial bankruptcy management
//!
//! On-chain: Metadata for bankruptcy
//! Off-chain: Actual bankruptcy, process management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Bankruptcy type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBankruptcyType {
    /// Chapter 7
    Chapter7,
    /// Chapter 11
    Chapter11,
    /// Chapter 13
    Chapter13,
    /// Custom type
    Custom,
}

/// Bankruptcy status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBankruptcyStatus {
    /// Bankruptcy pending
    Pending,
    /// Bankruptcy in progress
    InProgress,
    /// Bankruptcy resolved
    Resolved,
}

/// Financial bankruptcy metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialBankruptcyMetadata {
    /// Bankruptcy ID
    pub bankruptcy_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Bankruptcy type
    pub bankruptcy_type: FinancialBankruptcyType,
    /// Status
    pub status: FinancialBankruptcyStatus,
    /// Created at
    pub created_at: i64,
    /// Bankruptcy data hash
    pub bankruptcy_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_bankruptcy(
        bankruptcy: &mut FinancialBankruptcyMetadata,
        bankruptcy_id: u64,
        entity_id: u64,
        bankruptcy_type: FinancialBankruptcyType,
        bankruptcy_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(bankruptcy_id > 0, IndrasError::InvalidInput);
        bankruptcy.bankruptcy_id = bankruptcy_id;
        bankruptcy.entity_id = entity_id;
        bankruptcy.bankruptcy_type = bankruptcy_type;
        bankruptcy.status = FinancialBankruptcyStatus::Pending;
        bankruptcy.created_at = current_time;
        bankruptcy.bankruptcy_data_hash = bankruptcy_data_hash;
        bankruptcy.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_bankruptcy(_bankruptcy_id: u64) -> Vec<u8> {
        vec![]
    }
}
