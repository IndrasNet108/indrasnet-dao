//! Financial Disaster Recovery module
//!
//! Financial disaster recovery
//!
//! On-chain: Metadata for disaster recovery
//! Off-chain: Actual recovery, implementation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Recovery type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDisasterRecoveryType {
    /// Data recovery
    Data,
    /// System recovery
    System,
    /// Business recovery
    Business,
    /// Custom recovery
    Custom,
}

/// Recovery status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDisasterRecoveryStatus {
    /// Recovery active
    Active,
    /// Recovery paused
    Paused,
    /// Recovery completed
    Completed,
}

/// Financial disaster recovery metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDisasterRecoveryMetadata {
    /// Recovery ID
    pub recovery_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Recovery type
    pub recovery_type: FinancialDisasterRecoveryType,
    /// Status
    pub status: FinancialDisasterRecoveryStatus,
    /// Created at
    pub created_at: i64,
    /// Recovery config hash
    pub recovery_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_disaster_recovery(
        recovery: &mut FinancialDisasterRecoveryMetadata,
        recovery_id: u64,
        entity_id: u64,
        recovery_type: FinancialDisasterRecoveryType,
        recovery_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(recovery_id > 0, IndrasError::InvalidInput);
        recovery.recovery_id = recovery_id;
        recovery.entity_id = entity_id;
        recovery.recovery_type = recovery_type;
        recovery.status = FinancialDisasterRecoveryStatus::Active;
        recovery.created_at = current_time;
        recovery.recovery_config_hash = recovery_config_hash;
        recovery.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn implement_disaster_recovery(_recovery_id: u64) -> Vec<u8> {
        vec![]
    }
}
