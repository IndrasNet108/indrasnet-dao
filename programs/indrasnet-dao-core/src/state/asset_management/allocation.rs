//! Asset Allocation module
//!
//! Asset allocation and rebalancing
//!
//! On-chain: Metadata for asset allocation
//! Off-chain: Actual allocation, rebalancing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Allocation strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AllocationStrategy {
    /// Equal allocation
    Equal,
    /// Risk-based allocation
    RiskBased,
    /// Market-cap weighted
    MarketCapWeighted,
    /// Custom strategy
    Custom,
}

/// Allocation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AssetAllocationStatus {
    /// Allocation active
    Active,
    /// Allocation paused
    Paused,
    /// Allocation rebalanced
    Rebalanced,
}

/// Asset allocation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct AssetAllocationMetadata {
    /// Allocation ID
    pub allocation_id: u64,
    /// Portfolio ID
    pub portfolio_id: u64,
    /// Allocation strategy
    pub allocation_strategy: AllocationStrategy,
    /// Status
    pub status: AssetAllocationStatus,
    /// Created at
    pub created_at: i64,
    /// Allocation config hash
    pub allocation_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_asset_allocation(
        allocation: &mut AssetAllocationMetadata,
        allocation_id: u64,
        portfolio_id: u64,
        allocation_strategy: AllocationStrategy,
        allocation_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(allocation_id > 0, IndrasError::InvalidInput);
        allocation.allocation_id = allocation_id;
        allocation.portfolio_id = portfolio_id;
        allocation.allocation_strategy = allocation_strategy;
        allocation.status = AssetAllocationStatus::Active;
        allocation.created_at = current_time;
        allocation.allocation_config_hash = allocation_config_hash;
        allocation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn allocate_assets(_allocation_id: u64) -> Vec<u8> {
        vec![]
    }
}
