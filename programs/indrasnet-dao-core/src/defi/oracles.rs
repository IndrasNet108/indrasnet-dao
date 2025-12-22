//! Oracles module
//!
//! Oracle data management for DeFi operations
//!
//! On-chain: Metadata for oracle data, price feeds
//! Off-chain: Actual oracle data fetching, aggregation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Oracle source
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum OracleSource {
    /// Chainlink
    Chainlink,
    /// Pyth
    Pyth,
    /// Switchboard
    Switchboard,
    /// Custom
    Custom,
}

/// Oracle data status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum OracleDataStatus {
    /// Data valid
    Valid,
    /// Data stale
    Stale,
    /// Data invalid
    Invalid,
}

/// Oracle data metadata (on-chain)
///
/// Stores metadata for oracle price feeds
#[account]
#[derive(InitSpace)]
pub struct OracleDataMetadata {
    /// Oracle ID
    pub oracle_id: u64,
    /// Oracle source
    pub source: OracleSource,
    /// Token mint
    pub token_mint: Pubkey,
    /// Price (stored as u64, scaled)
    pub price: u64,
    /// Status
    pub status: OracleDataStatus,
    /// Last updated at
    pub last_updated_at: i64,
    /// Created at
    pub created_at: i64,
    /// Bump seed
    pub bump: u8,
}

impl OracleDataMetadata {
    /// Update oracle price
    pub fn update_price(&mut self, new_price: u64, current_time: i64) {
        self.price = new_price;
        self.last_updated_at = current_time;
        self.status = OracleDataStatus::Valid;
    }

    /// Mark oracle as stale
    pub fn mark_stale(&mut self) {
        self.status = OracleDataStatus::Stale;
    }
}

/// On-chain functions for oracles
pub mod onchain {
    use super::*;

    /// Initialize oracle data
    pub fn initialize_oracle_data(
        oracle: &mut OracleDataMetadata,
        oracle_id: u64,
        source: OracleSource,
        token_mint: Pubkey,
        price: u64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(oracle_id > 0, IndrasError::InvalidInput);
        
        oracle.oracle_id = oracle_id;
        oracle.source = source;
        oracle.token_mint = token_mint;
        oracle.price = price;
        oracle.status = OracleDataStatus::Valid;
        oracle.last_updated_at = current_time;
        oracle.created_at = current_time;
        oracle.bump = bump;
        
        Ok(())
    }

    /// Update oracle price
    pub fn update_oracle_price(
        oracle: &mut OracleDataMetadata,
        new_price: u64,
        current_time: i64,
    ) -> Result<()> {
        oracle.update_price(new_price, current_time);
        Ok(())
    }
}

/// Off-chain functions for oracles
///
/// These functions should be implemented in off-chain service
/// for actual oracle data fetching.
pub mod offchain {
    // Off-chain functions will be implemented in separate service
    
    /// Fetch oracle price
    pub fn fetch_price(_oracle_id: u64) -> Option<u64> {
        // Implementation in off-chain service
        // Fetches price from oracle source
        None
    }

    /// Aggregate oracle prices
    pub fn aggregate_prices(_token_mint: super::Pubkey) -> u64 {
        // Implementation in off-chain service
        // Aggregates prices from multiple oracle sources
        0
    }
}
