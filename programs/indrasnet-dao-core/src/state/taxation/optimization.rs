//! Tax Optimization module
//!
//! Tax optimization strategies
//!
//! On-chain: Metadata for tax optimization
//! Off-chain: Actual optimization, tax planning

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Optimization strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxOptimizationStrategy {
    /// Tax-loss harvesting
    TaxLossHarvesting,
    /// Asset location optimization
    AssetLocationOptimization,
    /// Timing optimization
    TimingOptimization,
    /// Custom strategy
    Custom,
}

/// Optimization status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxOptimizationStatus {
    /// Optimization active
    Active,
    /// Optimization paused
    Paused,
    /// Optimization disabled
    Disabled,
}

/// Tax optimization metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct TaxOptimizationMetadata {
    /// Optimization ID
    pub optimization_id: u64,
    /// Portfolio ID
    pub portfolio_id: u64,
    /// Optimization strategy
    pub optimization_strategy: TaxOptimizationStrategy,
    /// Status
    pub status: TaxOptimizationStatus,
    /// Created at
    pub created_at: i64,
    /// Optimization config hash
    pub optimization_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_tax_optimization(
        optimization: &mut TaxOptimizationMetadata,
        optimization_id: u64,
        portfolio_id: u64,
        optimization_strategy: TaxOptimizationStrategy,
        optimization_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(optimization_id > 0, IndrasError::InvalidInput);
        optimization.optimization_id = optimization_id;
        optimization.portfolio_id = portfolio_id;
        optimization.optimization_strategy = optimization_strategy;
        optimization.status = TaxOptimizationStatus::Active;
        optimization.created_at = current_time;
        optimization.optimization_config_hash = optimization_config_hash;
        optimization.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn optimize_taxes(_optimization_id: u64) -> Vec<u8> {
        vec![]
    }
}
