//! Financial Alerts module
//!
//! Financial alerts and notifications
//!
//! On-chain: Metadata for financial alerts
//! Off-chain: Actual alerts, notifications

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Alert type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialAlertType {
    /// Threshold alert
    Threshold,
    /// Anomaly alert
    Anomaly,
    /// Trend alert
    Trend,
    /// Custom alert
    Custom,
}

/// Alert status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialAlertStatus {
    /// Alert active
    Active,
    /// Alert triggered
    Triggered,
    /// Alert disabled
    Disabled,
}

/// Financial alerts metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialAlertsMetadata {
    /// Alert ID
    pub alert_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Alert type
    pub alert_type: FinancialAlertType,
    /// Status
    pub status: FinancialAlertStatus,
    /// Created at
    pub created_at: i64,
    /// Alert config hash
    pub alert_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_alerts(
        alert: &mut FinancialAlertsMetadata,
        alert_id: u64,
        entity_id: u64,
        alert_type: FinancialAlertType,
        alert_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(alert_id > 0, IndrasError::InvalidInput);
        alert.alert_id = alert_id;
        alert.entity_id = entity_id;
        alert.alert_type = alert_type;
        alert.status = FinancialAlertStatus::Active;
        alert.created_at = current_time;
        alert.alert_config_hash = alert_config_hash;
        alert.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn monitor_financial_alerts(_alert_id: u64) -> Vec<u8> {
        vec![]
    }
}
