//! Transaction Monitoring module
//!
//! Transaction monitoring and alerts
//!
//! On-chain: Metadata for transaction monitoring
//! Off-chain: Actual monitoring, alerting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Monitoring type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TransactionMonitoringType {
    /// Real-time monitoring
    RealTime,
    /// Batch monitoring
    Batch,
    /// Event-based monitoring
    EventBased,
    /// Custom type
    Custom,
}

/// Monitoring status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TransactionMonitoringStatus {
    /// Monitoring active
    Active,
    /// Monitoring paused
    Paused,
    /// Monitoring disabled
    Disabled,
}

/// Transaction monitoring metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct TransactionMonitoringMetadata {
    /// Monitoring ID
    pub monitoring_id: u64,
    /// Account ID
    pub account_id: u64,
    /// Monitoring type
    pub monitoring_type: TransactionMonitoringType,
    /// Status
    pub status: TransactionMonitoringStatus,
    /// Created at
    pub created_at: i64,
    /// Monitoring config hash
    pub monitoring_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_transaction_monitoring(
        monitoring: &mut TransactionMonitoringMetadata,
        monitoring_id: u64,
        account_id: u64,
        monitoring_type: TransactionMonitoringType,
        monitoring_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(monitoring_id > 0, IndrasError::InvalidInput);
        monitoring.monitoring_id = monitoring_id;
        monitoring.account_id = account_id;
        monitoring.monitoring_type = monitoring_type;
        monitoring.status = TransactionMonitoringStatus::Active;
        monitoring.created_at = current_time;
        monitoring.monitoring_config_hash = monitoring_config_hash;
        monitoring.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn monitor_transactions(_monitoring_id: u64) -> Vec<u8> {
        vec![]
    }
}
