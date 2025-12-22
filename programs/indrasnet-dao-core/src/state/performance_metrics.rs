//! Performance Metrics module
//!
//! Performance metrics and KPIs
//!
//! On-chain: Metadata for performance metrics
//! Off-chain: Actual metrics calculation, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Metric type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PerformanceMetricType {
    /// Return on investment
    ROI,
    /// Sharpe ratio
    SharpeRatio,
    /// Alpha
    Alpha,
    /// Custom metric
    Custom,
}

/// Metric status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PerformanceMetricStatus {
    /// Metric active
    Active,
    /// Metric paused
    Paused,
    /// Metric disabled
    Disabled,
}

/// Performance metrics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PerformanceMetricsMetadata {
    /// Metric ID
    pub metric_id: u64,
    /// Portfolio ID
    pub portfolio_id: u64,
    /// Metric type
    pub metric_type: PerformanceMetricType,
    /// Status
    pub status: PerformanceMetricStatus,
    /// Created at
    pub created_at: i64,
    /// Metric config hash
    pub metric_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_performance_metrics(
        metric: &mut PerformanceMetricsMetadata,
        metric_id: u64,
        portfolio_id: u64,
        metric_type: PerformanceMetricType,
        metric_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(metric_id > 0, IndrasError::InvalidInput);
        metric.metric_id = metric_id;
        metric.portfolio_id = portfolio_id;
        metric.metric_type = metric_type;
        metric.status = PerformanceMetricStatus::Active;
        metric.created_at = current_time;
        metric.metric_config_hash = metric_config_hash;
        metric.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn calculate_performance_metrics(_metric_id: u64) -> Vec<u8> {
        vec![]
    }
}
