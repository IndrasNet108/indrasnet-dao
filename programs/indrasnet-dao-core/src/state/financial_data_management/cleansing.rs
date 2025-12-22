//! Financial Data Cleansing module
//!
//! Financial data cleansing
//!
//! On-chain: Metadata for data cleansing
//! Off-chain: Actual cleansing, correction

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Cleansing type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataCleansingType {
    /// Duplicate removal
    DuplicateRemoval,
    /// Missing value handling
    MissingValueHandling,
    /// Outlier detection
    OutlierDetection,
    /// Custom cleansing
    Custom,
}

/// Cleansing status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataCleansingStatus {
    /// Cleansing active
    Active,
    /// Cleansing paused
    Paused,
    /// Cleansing disabled
    Disabled,
}

/// Financial data cleansing metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataCleansingMetadata {
    /// Cleansing ID
    pub cleansing_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Cleansing type
    pub cleansing_type: FinancialDataCleansingType,
    /// Status
    pub status: FinancialDataCleansingStatus,
    /// Created at
    pub created_at: i64,
    /// Cleansing config hash
    pub cleansing_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_cleansing(
        cleansing: &mut FinancialDataCleansingMetadata,
        cleansing_id: u64,
        entity_id: u64,
        cleansing_type: FinancialDataCleansingType,
        cleansing_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(cleansing_id > 0, IndrasError::InvalidInput);
        cleansing.cleansing_id = cleansing_id;
        cleansing.entity_id = entity_id;
        cleansing.cleansing_type = cleansing_type;
        cleansing.status = FinancialDataCleansingStatus::Active;
        cleansing.created_at = current_time;
        cleansing.cleansing_config_hash = cleansing_config_hash;
        cleansing.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn cleanse_financial_data(_cleansing_id: u64) -> Vec<u8> {
        vec![]
    }
}
