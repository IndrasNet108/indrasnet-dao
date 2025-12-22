//! Financial Data Archival module
//!
//! Financial data archival
//!
//! On-chain: Metadata for data archival
//! Off-chain: Actual archival, storage

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Archival strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataArchivalStrategy {
    /// Time-based archival
    TimeBased,
    /// Size-based archival
    SizeBased,
    /// Policy-based archival
    PolicyBased,
    /// Custom strategy
    Custom,
}

/// Archival status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataArchivalStatus {
    /// Archival active
    Active,
    /// Archival paused
    Paused,
    /// Archival disabled
    Disabled,
}

/// Financial data archival metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataArchivalMetadata {
    /// Archival ID
    pub archival_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Archival strategy
    pub archival_strategy: FinancialDataArchivalStrategy,
    /// Status
    pub status: FinancialDataArchivalStatus,
    /// Created at
    pub created_at: i64,
    /// Archival config hash
    pub archival_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_archival(
        archival: &mut FinancialDataArchivalMetadata,
        archival_id: u64,
        entity_id: u64,
        archival_strategy: FinancialDataArchivalStrategy,
        archival_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(archival_id > 0, IndrasError::InvalidInput);
        archival.archival_id = archival_id;
        archival.entity_id = entity_id;
        archival.archival_strategy = archival_strategy;
        archival.status = FinancialDataArchivalStatus::Active;
        archival.created_at = current_time;
        archival.archival_config_hash = archival_config_hash;
        archival.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn archive_financial_data(_archival_id: u64) -> Vec<u8> {
        vec![]
    }
}
