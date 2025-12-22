//! Partnership Analytics Retention module
//!
//! Partnership analytics retention
//!
//! On-chain: Metadata for retention
//! Off-chain: Actual retention, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Retention metric
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipRetentionMetric {
    /// Customer retention
    Customer,
    /// Revenue retention
    Revenue,
    /// Engagement retention
    Engagement,
    /// Custom metric
    Custom,
}

/// Retention status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipRetentionStatus {
    /// Retention measuring
    Measuring,
    /// Retention measured
    Measured,
    /// Retention optimized
    Optimized,
}

/// Partnership analytics retention metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsRetentionMetadata {
    /// Retention ID
    pub retention_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Retention metric
    pub retention_metric: PartnershipRetentionMetric,
    /// Status
    pub status: PartnershipRetentionStatus,
    /// Created at
    pub created_at: i64,
    /// Retention data hash
    pub retention_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_retention(
        retention: &mut PartnershipAnalyticsRetentionMetadata,
        retention_id: u64,
        partnership_id: u64,
        retention_metric: PartnershipRetentionMetric,
        retention_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(retention_id > 0, IndrasError::InvalidInput);
        retention.retention_id = retention_id;
        retention.partnership_id = partnership_id;
        retention.retention_metric = retention_metric;
        retention.status = PartnershipRetentionStatus::Measuring;
        retention.created_at = current_time;
        retention.retention_data_hash = retention_data_hash;
        retention.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn measure_retention(_retention_id: u64) -> Vec<u8> {
        vec![]
    }
}
