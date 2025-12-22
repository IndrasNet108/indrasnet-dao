//! Rebalancing module
//!
//! Portfolio rebalancing
//!
//! On-chain: Metadata for rebalancing
//! Off-chain: Actual rebalancing, execution

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Rebalancing strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum RebalancingStrategy {
    /// Time-based rebalancing
    TimeBased,
    /// Threshold-based rebalancing
    ThresholdBased,
    /// Drift-based rebalancing
    DriftBased,
    /// Custom strategy
    Custom,
}

/// Rebalancing status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum RebalancingStatus {
    /// Rebalancing scheduled
    Scheduled,
    /// Rebalancing in progress
    InProgress,
    /// Rebalancing completed
    Completed,
}

/// Rebalancing metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct RebalancingMetadata {
    /// Rebalancing ID
    pub rebalancing_id: u64,
    /// Portfolio ID
    pub portfolio_id: u64,
    /// Rebalancing strategy
    pub rebalancing_strategy: RebalancingStrategy,
    /// Status
    pub status: RebalancingStatus,
    /// Created at
    pub created_at: i64,
    /// Rebalancing config hash
    pub rebalancing_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_rebalancing(
        rebalancing: &mut RebalancingMetadata,
        rebalancing_id: u64,
        portfolio_id: u64,
        rebalancing_strategy: RebalancingStrategy,
        rebalancing_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(rebalancing_id > 0, IndrasError::InvalidInput);
        rebalancing.rebalancing_id = rebalancing_id;
        rebalancing.portfolio_id = portfolio_id;
        rebalancing.rebalancing_strategy = rebalancing_strategy;
        rebalancing.status = RebalancingStatus::Scheduled;
        rebalancing.created_at = current_time;
        rebalancing.rebalancing_config_hash = rebalancing_config_hash;
        rebalancing.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_rebalancing(_rebalancing_id: u64) -> Vec<u8> {
        vec![]
    }
}
