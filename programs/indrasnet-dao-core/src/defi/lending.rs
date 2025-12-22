//! DeFi Lending module
//!
//! Lending operations
//!
//! On-chain: Metadata for lending
//! Off-chain: Actual lending, borrowing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Lending type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LendingType {
    /// Collateralized lending
    Collateralized,
    /// Uncollateralized lending
    Uncollateralized,
    /// Flash loans
    FlashLoans,
    /// Custom type
    Custom,
}

/// Lending status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LendingStatus {
    /// Lending active
    Active,
    /// Lending paused
    Paused,
    /// Lending closed
    Closed,
}

/// Lending metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct LendingMetadata {
    /// Lending ID
    pub lending_id: u64,
    /// Protocol ID
    pub protocol_id: u64,
    /// Lending type
    pub lending_type: LendingType,
    /// Status
    pub status: LendingStatus,
    /// Created at
    pub created_at: i64,
    /// Lending config hash
    pub lending_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_lending(
        lending: &mut LendingMetadata,
        lending_id: u64,
        protocol_id: u64,
        lending_type: LendingType,
        lending_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(lending_id > 0, IndrasError::InvalidInput);
        lending.lending_id = lending_id;
        lending.protocol_id = protocol_id;
        lending.lending_type = lending_type;
        lending.status = LendingStatus::Active;
        lending.created_at = current_time;
        lending.lending_config_hash = lending_config_hash;
        lending.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_lending(_lending_id: u64) -> Vec<u8> {
        vec![]
    }
}
