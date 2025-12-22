//! Staking module
//!
//! DAO staking management
//!
//! On-chain: Metadata for staking
//! Off-chain: Actual staking calculations, rewards

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Staking status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum StakingStatus {
    /// Staking active
    Active,
    /// Staking unstaking
    Unstaking,
    /// Staking completed
    Completed,
}

/// Staking metadata (on-chain)
///
/// Stores metadata for DAO staking
#[account]
#[derive(InitSpace)]
pub struct StakingMetadata {
    /// Staking ID
    pub staking_id: u64,
    /// Staker pubkey
    pub staker_pubkey: Pubkey,
    /// Amount staked (in smallest unit)
    pub amount_staked: u64,
    /// Status
    pub status: StakingStatus,
    /// Created at
    pub created_at: i64,
    /// Unstaked at
    pub unstaked_at: Option<i64>,
    /// Staking data hash
    pub staking_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for staking
pub mod onchain {
    use super::*;

    /// Initialize staking
    pub fn initialize_staking(
        staking: &mut StakingMetadata,
        staking_id: u64,
        staker_pubkey: Pubkey,
        amount_staked: u64,
        staking_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(staking_id > 0, IndrasError::InvalidInput);
        require!(amount_staked > 0, IndrasError::InvalidInput);
        
        staking.staking_id = staking_id;
        staking.staker_pubkey = staker_pubkey;
        staking.amount_staked = amount_staked;
        staking.status = StakingStatus::Active;
        staking.created_at = current_time;
        staking.unstaked_at = None;
        staking.staking_data_hash = staking_data_hash;
        staking.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for staking
pub mod offchain {
    /// Calculate staking rewards
    pub fn calculate_staking_rewards(_staking_id: u64) -> u64 {
        // Implementation in off-chain service
        0
    }
}
