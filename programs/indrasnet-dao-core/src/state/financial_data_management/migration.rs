//! Financial Data Migration module
//!
//! Financial data migration
//!
//! On-chain: Metadata for data migration
//! Off-chain: Actual migration, transformation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Migration type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataMigrationType {
    /// System migration
    System,
    /// Format migration
    Format,
    /// Platform migration
    Platform,
    /// Custom migration
    Custom,
}

/// Migration status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataMigrationStatus {
    /// Migration scheduled
    Scheduled,
    /// Migration in progress
    InProgress,
    /// Migration completed
    Completed,
    /// Migration failed
    Failed,
}

/// Financial data migration metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataMigrationMetadata {
    /// Migration ID
    pub migration_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Migration type
    pub migration_type: FinancialDataMigrationType,
    /// Status
    pub status: FinancialDataMigrationStatus,
    /// Created at
    pub created_at: i64,
    /// Migration config hash
    pub migration_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_migration(
        migration: &mut FinancialDataMigrationMetadata,
        migration_id: u64,
        entity_id: u64,
        migration_type: FinancialDataMigrationType,
        migration_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(migration_id > 0, IndrasError::InvalidInput);
        migration.migration_id = migration_id;
        migration.entity_id = entity_id;
        migration.migration_type = migration_type;
        migration.status = FinancialDataMigrationStatus::Scheduled;
        migration.created_at = current_time;
        migration.migration_config_hash = migration_config_hash;
        migration.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn migrate_financial_data(_migration_id: u64) -> Vec<u8> {
        vec![]
    }
}
