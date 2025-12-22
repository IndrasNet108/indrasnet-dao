//! Termination module
//!
//! Partnership termination management
//!
//! On-chain: Metadata for termination process
//! Off-chain: Actual termination workflow execution

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Termination status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TerminationStatus {
    /// Termination initiated
    Initiated,
    /// Termination in progress
    InProgress,
    /// Termination completed
    Completed,
    /// Termination cancelled
    Cancelled,
}

/// Partnership termination metadata (on-chain)
///
/// Stores metadata for partnership termination
#[account]
#[derive(InitSpace)]
pub struct PartnershipTerminationMetadata {
    /// Termination ID
    pub termination_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Status
    pub status: TerminationStatus,
    /// Created at
    pub created_at: i64,
    /// Completed at
    pub completed_at: Option<i64>,
    /// Termination data hash
    pub termination_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for termination
pub mod onchain {
    use super::*;

    /// Initialize partnership termination
    pub fn initialize_partnership_termination(
        termination: &mut PartnershipTerminationMetadata,
        termination_id: u64,
        partnership_id: u64,
        termination_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(termination_id > 0, IndrasError::InvalidInput);
        
        termination.termination_id = termination_id;
        termination.partnership_id = partnership_id;
        termination.status = TerminationStatus::Initiated;
        termination.created_at = current_time;
        termination.completed_at = None;
        termination.termination_data_hash = termination_data_hash;
        termination.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for termination
pub mod offchain {
    /// Execute termination
    pub fn execute_termination(_termination_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
