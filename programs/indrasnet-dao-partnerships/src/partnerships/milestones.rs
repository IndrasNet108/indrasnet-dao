//! Milestones module
//!
//! Partnership milestone management
//!
//! On-chain: Metadata for milestones
//! Off-chain: Actual milestone tracking, notifications

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Milestone status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MilestoneStatus {
    /// Milestone planned
    Planned,
    /// Milestone in progress
    InProgress,
    /// Milestone completed
    Completed,
    /// Milestone delayed
    Delayed,
}

/// Partnership milestone metadata (on-chain)
///
/// Stores metadata for partnership milestones
#[account]
#[derive(InitSpace)]
pub struct PartnershipMilestoneMetadata {
    /// Milestone ID
    pub milestone_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Milestone name
    #[max_len(100)]
    pub name: String,
    /// Status
    pub status: MilestoneStatus,
    /// Created at
    pub created_at: i64,
    /// Target date
    pub target_date: Option<i64>,
    /// Completed at
    pub completed_at: Option<i64>,
    /// Milestone data hash
    pub milestone_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for milestones
pub mod onchain {
    use super::*;

    /// Initialize partnership milestone
    pub fn initialize_partnership_milestone(
        milestone: &mut PartnershipMilestoneMetadata,
        milestone_id: u64,
        partnership_id: u64,
        name: String,
        milestone_data_hash: [u8; 32],
        target_date: Option<i64>,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(milestone_id > 0, IndrasError::InvalidInput);
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= 100, IndrasError::InvalidInput);
        
        milestone.milestone_id = milestone_id;
        milestone.partnership_id = partnership_id;
        milestone.name = name;
        milestone.status = MilestoneStatus::Planned;
        milestone.created_at = current_time;
        milestone.target_date = target_date;
        milestone.completed_at = None;
        milestone.milestone_data_hash = milestone_data_hash;
        milestone.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for milestones
pub mod offchain {
    /// Track milestone
    pub fn track_milestone(_milestone_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
