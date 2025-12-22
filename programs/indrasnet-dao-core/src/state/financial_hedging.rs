//! Financial Hedging module
//!
//! Financial hedging
//!
//! On-chain: Metadata for hedging
//! Off-chain: Actual hedging, execution

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Hedging instrument
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialHedgingInstrument {
    /// Futures
    Futures,
    /// Options
    Options,
    /// Swaps
    Swaps,
    /// Custom instrument
    Custom,
}

/// Hedging status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialHedgingStatus {
    /// Hedging active
    Active,
    /// Hedging paused
    Paused,
    /// Hedging closed
    Closed,
}

/// Financial hedging metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialHedgingMetadata {
    /// Hedging ID
    pub hedging_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Hedging instrument
    pub hedging_instrument: FinancialHedgingInstrument,
    /// Status
    pub status: FinancialHedgingStatus,
    /// Created at
    pub created_at: i64,
    /// Hedging config hash
    pub hedging_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_hedging(
        hedging: &mut FinancialHedgingMetadata,
        hedging_id: u64,
        entity_id: u64,
        hedging_instrument: FinancialHedgingInstrument,
        hedging_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(hedging_id > 0, IndrasError::InvalidInput);
        hedging.hedging_id = hedging_id;
        hedging.entity_id = entity_id;
        hedging.hedging_instrument = hedging_instrument;
        hedging.status = FinancialHedgingStatus::Active;
        hedging.created_at = current_time;
        hedging.hedging_config_hash = hedging_config_hash;
        hedging.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_hedge(_hedging_id: u64) -> Vec<u8> {
        vec![]
    }
}
