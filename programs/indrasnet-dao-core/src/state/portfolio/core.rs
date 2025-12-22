//! Portfolio module
//!
//! Portfolio management
//!
//! On-chain: Metadata for portfolios
//! Off-chain: Actual portfolio calculations, analytics

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Portfolio status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PortfolioStatus {
    /// Portfolio active
    Active,
    /// Portfolio closed
    Closed,
}

/// Portfolio metadata (on-chain)
///
/// Stores metadata for portfolios
#[account]
#[derive(InitSpace)]
pub struct PortfolioMetadata {
    /// Portfolio ID
    pub portfolio_id: u64,
    /// Owner pubkey
    pub owner_pubkey: Pubkey,
    /// Total value (in smallest unit)
    pub total_value: u64,
    /// Status
    pub status: PortfolioStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Portfolio data hash
    pub portfolio_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for portfolio
pub mod onchain {
    use super::*;

    /// Initialize portfolio
    pub fn initialize_portfolio(
        portfolio: &mut PortfolioMetadata,
        portfolio_id: u64,
        owner_pubkey: Pubkey,
        portfolio_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(portfolio_id > 0, IndrasError::InvalidInput);
        
        portfolio.portfolio_id = portfolio_id;
        portfolio.owner_pubkey = owner_pubkey;
        portfolio.total_value = 0;
        portfolio.status = PortfolioStatus::Active;
        portfolio.created_at = current_time;
        portfolio.updated_at = current_time;
        portfolio.portfolio_data_hash = portfolio_data_hash;
        portfolio.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for portfolio
pub mod offchain {
    /// Calculate portfolio value
    pub fn calculate_portfolio_value(_portfolio_id: u64) -> u64 {
        // Implementation in off-chain service
        0
    }
}
