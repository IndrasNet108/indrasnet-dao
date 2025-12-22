//! Asset Performance module
//!
//! Asset performance tracking
//!
//! On-chain: Metadata for asset performance
//! Off-chain: Actual tracking, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Performance metric
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AssetPerformanceMetric {
    /// Return metric
    Return,
    /// Risk metric
    Risk,
    /// Volatility metric
    Volatility,
    /// Custom metric
    Custom,
}

/// Performance status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AssetPerformanceStatus {
    /// Performance tracking active
    Active,
    /// Performance tracking paused
    Paused,
    /// Performance tracking disabled
    Disabled,
}

/// Asset performance metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct AssetPerformanceMetadata {
    /// Performance ID
    pub performance_id: u64,
    /// Asset ID
    pub asset_id: u64,
    /// Performance metric
    pub performance_metric: AssetPerformanceMetric,
    /// Status
    pub status: AssetPerformanceStatus,
    /// Created at
    pub created_at: i64,
    /// Performance config hash
    pub performance_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_asset_performance(
        performance: &mut AssetPerformanceMetadata,
        performance_id: u64,
        asset_id: u64,
        performance_metric: AssetPerformanceMetric,
        performance_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(performance_id > 0, IndrasError::InvalidInput);
        performance.performance_id = performance_id;
        performance.asset_id = asset_id;
        performance.performance_metric = performance_metric;
        performance.status = AssetPerformanceStatus::Active;
        performance.created_at = current_time;
        performance.performance_config_hash = performance_config_hash;
        performance.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn track_asset_performance(_performance_id: u64) -> Vec<u8> {
        vec![]
    }
}
