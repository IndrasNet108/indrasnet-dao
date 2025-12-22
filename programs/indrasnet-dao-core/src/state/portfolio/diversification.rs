//! Portfolio Diversification module
//!
//! Portfolio diversification management
//!
//! On-chain: Metadata for portfolio diversification
//! Off-chain: Actual diversification, rebalancing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Diversification strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PortfolioDiversificationStrategy {
    /// Asset class diversification
    AssetClass,
    /// Geographic diversification
    Geographic,
    /// Sector diversification
    Sector,
    /// Custom strategy
    Custom,
}

/// Diversification status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PortfolioDiversificationStatus {
    /// Diversification active
    Active,
    /// Diversification paused
    Paused,
    /// Diversification disabled
    Disabled,
}

/// Portfolio diversification metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PortfolioDiversificationMetadata {
    /// Diversification ID
    pub diversification_id: u64,
    /// Portfolio ID
    pub portfolio_id: u64,
    /// Diversification strategy
    pub diversification_strategy: PortfolioDiversificationStrategy,
    /// Status
    pub status: PortfolioDiversificationStatus,
    /// Created at
    pub created_at: i64,
    /// Diversification config hash
    pub diversification_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_portfolio_diversification(
        diversification: &mut PortfolioDiversificationMetadata,
        diversification_id: u64,
        portfolio_id: u64,
        diversification_strategy: PortfolioDiversificationStrategy,
        diversification_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(diversification_id > 0, IndrasError::InvalidInput);
        diversification.diversification_id = diversification_id;
        diversification.portfolio_id = portfolio_id;
        diversification.diversification_strategy = diversification_strategy;
        diversification.status = PortfolioDiversificationStatus::Active;
        diversification.created_at = current_time;
        diversification.diversification_config_hash = diversification_config_hash;
        diversification.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn diversify_portfolio(_diversification_id: u64) -> Vec<u8> {
        vec![]
    }
}
