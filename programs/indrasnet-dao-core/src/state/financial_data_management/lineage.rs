//! Financial Data Lineage module
//!
//! Financial data lineage tracking
//!
//! On-chain: Metadata for data lineage
//! Off-chain: Actual lineage tracking, tracing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Lineage tracking type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataLineageTrackingType {
    /// Source tracking
    Source,
    /// Transformation tracking
    Transformation,
    /// Usage tracking
    Usage,
    /// Custom tracking
    Custom,
}

/// Lineage status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataLineageStatus {
    /// Lineage tracking active
    Active,
    /// Lineage tracking paused
    Paused,
    /// Lineage tracking disabled
    Disabled,
}

/// Financial data lineage metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataLineageMetadata {
    /// Lineage ID
    pub lineage_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Lineage tracking type
    pub lineage_tracking_type: FinancialDataLineageTrackingType,
    /// Status
    pub status: FinancialDataLineageStatus,
    /// Created at
    pub created_at: i64,
    /// Lineage config hash
    pub lineage_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_lineage(
        lineage: &mut FinancialDataLineageMetadata,
        lineage_id: u64,
        entity_id: u64,
        lineage_tracking_type: FinancialDataLineageTrackingType,
        lineage_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(lineage_id > 0, IndrasError::InvalidInput);
        lineage.lineage_id = lineage_id;
        lineage.entity_id = entity_id;
        lineage.lineage_tracking_type = lineage_tracking_type;
        lineage.status = FinancialDataLineageStatus::Active;
        lineage.created_at = current_time;
        lineage.lineage_config_hash = lineage_config_hash;
        lineage.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn track_data_lineage(_lineage_id: u64) -> Vec<u8> {
        vec![]
    }
}
