//! Partnership Analytics Alerts module
//!
//! Partnership analytics alerts
//!
//! On-chain: Metadata for alerts
//! Off-chain: Actual alerts, monitoring

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Alert type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipAlertType {
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
pub enum PartnershipAlertStatus {
    /// Alert active
    Active,
    /// Alert triggered
    Triggered,
    /// Alert resolved
    Resolved,
}

/// Partnership analytics alerts metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsAlertsMetadata {
    /// Alert ID
    pub alert_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Alert type
    pub alert_type: PartnershipAlertType,
    /// Status
    pub status: PartnershipAlertStatus,
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
    pub fn initialize_partnership_analytics_alerts(
        alert: &mut PartnershipAnalyticsAlertsMetadata,
        alert_id: u64,
        partnership_id: u64,
        alert_type: PartnershipAlertType,
        alert_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(alert_id > 0, IndrasError::InvalidInput);
        alert.alert_id = alert_id;
        alert.partnership_id = partnership_id;
        alert.alert_type = alert_type;
        alert.status = PartnershipAlertStatus::Active;
        alert.created_at = current_time;
        alert.alert_config_hash = alert_config_hash;
        alert.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn monitor_alerts(_alert_id: u64) -> Vec<u8> {
        vec![]
    }
}
