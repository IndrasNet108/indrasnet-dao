//! Financial Data Backup module
//!
//! Financial data backup and recovery
//!
//! On-chain: Metadata for data backup
//! Off-chain: Actual backup, recovery

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Backup type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataBackupType {
    /// Full backup
    Full,
    /// Incremental backup
    Incremental,
    /// Differential backup
    Differential,
    /// Custom backup
    Custom,
}

/// Backup status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataBackupStatus {
    /// Backup active
    Active,
    /// Backup paused
    Paused,
    /// Backup disabled
    Disabled,
}

/// Financial data backup metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataBackupMetadata {
    /// Backup ID
    pub backup_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Backup type
    pub backup_type: FinancialDataBackupType,
    /// Status
    pub status: FinancialDataBackupStatus,
    /// Created at
    pub created_at: i64,
    /// Backup config hash
    pub backup_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_backup(
        backup: &mut FinancialDataBackupMetadata,
        backup_id: u64,
        entity_id: u64,
        backup_type: FinancialDataBackupType,
        backup_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(backup_id > 0, IndrasError::InvalidInput);
        backup.backup_id = backup_id;
        backup.entity_id = entity_id;
        backup.backup_type = backup_type;
        backup.status = FinancialDataBackupStatus::Active;
        backup.created_at = current_time;
        backup.backup_config_hash = backup_config_hash;
        backup.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn backup_financial_data(_backup_id: u64) -> Vec<u8> {
        vec![]
    }
}
