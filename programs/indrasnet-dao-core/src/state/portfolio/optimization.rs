//! Portfolio Optimization module
//!
//! Portfolio optimization
//!
//! On-chain: Metadata for portfolio optimization
//! Off-chain: Actual optimization, rebalancing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Optimization method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PortfolioOptimizationMethod {
    /// Mean-variance optimization
    MeanVariance,
    /// Black-Litterman model
    BlackLitterman,
    /// Risk parity
    RiskParity,
    /// Custom method
    Custom,
}

/// Optimization status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PortfolioOptimizationStatus {
    /// Optimization pending
    Pending,
    /// Optimization in progress
    InProgress,
    /// Optimization completed
    Completed,
}

/// Portfolio optimization metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PortfolioOptimizationMetadata {
    /// Optimization ID
    pub optimization_id: u64,
    /// Portfolio ID
    pub portfolio_id: u64,
    /// Optimization method
    pub optimization_method: PortfolioOptimizationMethod,
    /// Status
    pub status: PortfolioOptimizationStatus,
    /// Created at
    pub created_at: i64,
    /// Optimization data hash
    pub optimization_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_portfolio_optimization(
        optimization: &mut PortfolioOptimizationMetadata,
        optimization_id: u64,
        portfolio_id: u64,
        optimization_method: PortfolioOptimizationMethod,
        optimization_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(optimization_id > 0, IndrasError::InvalidInput);
        optimization.optimization_id = optimization_id;
        optimization.portfolio_id = portfolio_id;
        optimization.optimization_method = optimization_method;
        optimization.status = PortfolioOptimizationStatus::Pending;
        optimization.created_at = current_time;
        optimization.optimization_data_hash = optimization_data_hash;
        optimization.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn optimize_portfolio(_optimization_id: u64) -> Vec<u8> {
        vec![]
    }
}
