//! Financial Data Quality module
//!
//! Financial data quality management
//!
//! On-chain: Metadata for data quality
//! Off-chain: Actual quality checks, validation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Quality check type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataQualityCheckType {
    /// Completeness check
    Completeness,
    /// Accuracy check
    Accuracy,
    /// Consistency check
    Consistency,
    /// Custom check
    Custom,
}

/// Quality status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataQualityStatus {
    /// Quality check active
    Active,
    /// Quality check paused
    Paused,
    /// Quality check disabled
    Disabled,
}

/// Financial data quality metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataQualityMetadata {
    /// Quality ID
    pub quality_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Quality check type
    pub quality_check_type: FinancialDataQualityCheckType,
    /// Status
    pub status: FinancialDataQualityStatus,
    /// Created at
    pub created_at: i64,
    /// Quality config hash
    pub quality_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_quality(
        quality: &mut FinancialDataQualityMetadata,
        quality_id: u64,
        entity_id: u64,
        quality_check_type: FinancialDataQualityCheckType,
        quality_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(quality_id > 0, IndrasError::InvalidInput);
        quality.quality_id = quality_id;
        quality.entity_id = entity_id;
        quality.quality_check_type = quality_check_type;
        quality.status = FinancialDataQualityStatus::Active;
        quality.created_at = current_time;
        quality.quality_config_hash = quality_config_hash;
        quality.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn check_data_quality(_quality_id: u64) -> bool {
        false
    }
}
