//! Financial Quality Management module
//!
//! Financial quality management
//!
//! On-chain: Metadata for quality management
//! Off-chain: Actual quality, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Quality standard
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialQualityStandard {
    /// ISO 9001
    ISO9001,
    /// Six Sigma
    SixSigma,
    /// Total Quality Management
    TQM,
    /// Custom standard
    Custom,
}

/// Quality status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialQualityStatus {
    /// Quality active
    Active,
    /// Quality paused
    Paused,
    /// Quality certified
    Certified,
}

/// Financial quality management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialQualityManagementMetadata {
    /// Quality ID
    pub quality_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Quality standard
    pub quality_standard: FinancialQualityStandard,
    /// Status
    pub status: FinancialQualityStatus,
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
    pub fn initialize_financial_quality_management(
        quality: &mut FinancialQualityManagementMetadata,
        quality_id: u64,
        entity_id: u64,
        quality_standard: FinancialQualityStandard,
        quality_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(quality_id > 0, IndrasError::InvalidInput);
        quality.quality_id = quality_id;
        quality.entity_id = entity_id;
        quality.quality_standard = quality_standard;
        quality.status = FinancialQualityStatus::Active;
        quality.created_at = current_time;
        quality.quality_config_hash = quality_config_hash;
        quality.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_quality(_quality_id: u64) -> Vec<u8> {
        vec![]
    }
}
