//! Financial Recovery module
//!
//! Financial recovery management
//!
//! On-chain: Metadata for recovery
//! Off-chain: Actual recovery, process management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Recovery strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRecoveryStrategy {
    /// Turnaround strategy
    Turnaround,
    /// Restructuring strategy
    Restructuring,
    /// Liquidation strategy
    Liquidation,
    /// Custom strategy
    Custom,
}

/// Recovery status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRecoveryStatus {
    /// Recovery pending
    Pending,
    /// Recovery in progress
    InProgress,
    /// Recovery completed
    Completed,
}

/// Financial recovery metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialRecoveryMetadata {
    /// Recovery ID
    pub recovery_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Recovery strategy
    pub recovery_strategy: FinancialRecoveryStrategy,
    /// Status
    pub status: FinancialRecoveryStatus,
    /// Created at
    pub created_at: i64,
    /// Recovery data hash
    pub recovery_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_recovery(
        recovery: &mut FinancialRecoveryMetadata,
        recovery_id: u64,
        entity_id: u64,
        recovery_strategy: FinancialRecoveryStrategy,
        recovery_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(recovery_id > 0, IndrasError::InvalidInput);
        recovery.recovery_id = recovery_id;
        recovery.entity_id = entity_id;
        recovery.recovery_strategy = recovery_strategy;
        recovery.status = FinancialRecoveryStatus::Pending;
        recovery.created_at = current_time;
        recovery.recovery_data_hash = recovery_data_hash;
        recovery.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_recovery(_recovery_id: u64) -> Vec<u8> {
        vec![]
    }
}
