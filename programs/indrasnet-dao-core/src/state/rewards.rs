//! Rewards module
//!
//! DAO rewards management
//!
//! On-chain: Metadata for rewards
//! Off-chain: Actual reward calculations, distribution

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Reward type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum RewardType {
    /// Participation reward
    Participation,
    /// Contribution reward
    Contribution,
    /// Voting reward
    Voting,
    /// Custom reward
    Custom,
}

/// Reward status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum RewardStatus {
    /// Reward pending
    Pending,
    /// Reward distributed
    Distributed,
    /// Reward cancelled
    Cancelled,
}

/// Reward metadata (on-chain)
///
/// Stores metadata for DAO rewards
#[account]
#[derive(InitSpace)]
pub struct RewardMetadata {
    /// Reward ID
    pub reward_id: u64,
    /// Recipient pubkey
    pub recipient_pubkey: Pubkey,
    /// Reward type
    pub reward_type: RewardType,
    /// Amount (in smallest unit)
    pub amount: u64,
    /// Status
    pub status: RewardStatus,
    /// Created at
    pub created_at: i64,
    /// Distributed at
    pub distributed_at: Option<i64>,
    /// Reward data hash
    pub reward_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for rewards
pub mod onchain {
    use super::*;

    /// Initialize reward
    pub fn initialize_reward(
        reward: &mut RewardMetadata,
        reward_id: u64,
        recipient_pubkey: Pubkey,
        reward_type: RewardType,
        amount: u64,
        reward_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(reward_id > 0, IndrasError::InvalidInput);
        require!(amount > 0, IndrasError::InvalidInput);
        
        reward.reward_id = reward_id;
        reward.recipient_pubkey = recipient_pubkey;
        reward.reward_type = reward_type;
        reward.amount = amount;
        reward.status = RewardStatus::Pending;
        reward.created_at = current_time;
        reward.distributed_at = None;
        reward.reward_data_hash = reward_data_hash;
        reward.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for rewards
pub mod offchain {
    /// Distribute reward
    pub fn distribute_reward(_reward_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
