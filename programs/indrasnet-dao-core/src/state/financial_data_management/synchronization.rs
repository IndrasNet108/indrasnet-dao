//! Financial Data Synchronization module
//!
//! Financial data synchronization
//!
//! On-chain: Metadata for data synchronization
//! Off-chain: Actual synchronization, replication

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Synchronization type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataSynchronizationType {
    /// Real-time synchronization
    RealTime,
    /// Batch synchronization
    Batch,
    /// Event-based synchronization
    EventBased,
    /// Custom synchronization
    Custom,
}

/// Synchronization status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataSynchronizationStatus {
    /// Synchronization active
    Active,
    /// Synchronization paused
    Paused,
    /// Synchronization disabled
    Disabled,
}

/// Financial data synchronization metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataSynchronizationMetadata {
    /// Synchronization ID
    pub synchronization_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Synchronization type
    pub synchronization_type: FinancialDataSynchronizationType,
    /// Status
    pub status: FinancialDataSynchronizationStatus,
    /// Created at
    pub created_at: i64,
    /// Synchronization config hash
    pub synchronization_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_synchronization(
        synchronization: &mut FinancialDataSynchronizationMetadata,
        synchronization_id: u64,
        entity_id: u64,
        synchronization_type: FinancialDataSynchronizationType,
        synchronization_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(synchronization_id > 0, IndrasError::InvalidInput);
        synchronization.synchronization_id = synchronization_id;
        synchronization.entity_id = entity_id;
        synchronization.synchronization_type = synchronization_type;
        synchronization.status = FinancialDataSynchronizationStatus::Active;
        synchronization.created_at = current_time;
        synchronization.synchronization_config_hash = synchronization_config_hash;
        synchronization.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn synchronize_financial_data(_synchronization_id: u64) -> Vec<u8> {
        vec![]
    }
}
