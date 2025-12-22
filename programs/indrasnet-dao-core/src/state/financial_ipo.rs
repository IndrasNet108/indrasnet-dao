//! Financial IPO module
//!
//! Financial IPO management
//!
//! On-chain: Metadata for IPO
//! Off-chain: Actual IPO, process management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// IPO stage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIPOStage {
    /// Pre-IPO
    PreIPO,
    /// IPO process
    IPOProcess,
    /// Post-IPO
    PostIPO,
    /// Custom stage
    Custom,
}

/// IPO status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIPOStatus {
    /// IPO pending
    Pending,
    /// IPO in progress
    InProgress,
    /// IPO completed
    Completed,
}

/// Financial IPO metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialIPOMetadata {
    /// IPO ID
    pub ipo_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// IPO stage
    pub ipo_stage: FinancialIPOStage,
    /// Status
    pub status: FinancialIPOStatus,
    /// Created at
    pub created_at: i64,
    /// IPO data hash
    pub ipo_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_ipo(
        ipo: &mut FinancialIPOMetadata,
        ipo_id: u64,
        entity_id: u64,
        ipo_stage: FinancialIPOStage,
        ipo_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(ipo_id > 0, IndrasError::InvalidInput);
        ipo.ipo_id = ipo_id;
        ipo.entity_id = entity_id;
        ipo.ipo_stage = ipo_stage;
        ipo.status = FinancialIPOStatus::Pending;
        ipo.created_at = current_time;
        ipo.ipo_data_hash = ipo_data_hash;
        ipo.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_ipo(_ipo_id: u64) -> Vec<u8> {
        vec![]
    }
}
