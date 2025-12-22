//! DeFi Analytics module
//!
//! DeFi analytics and metrics
//!
//! On-chain: Metadata for analytics
//! Off-chain: Actual analytics, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DeFiAnalyticsType {
    /// Volume analytics
    Volume,
    /// Yield analytics
    Yield,
    /// Risk analytics
    Risk,
    /// Custom analytics
    Custom,
}

/// Analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DeFiAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// DeFi analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct DeFiAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Protocol ID
    pub protocol_id: u64,
    /// Analytics type
    pub analytics_type: DeFiAnalyticsType,
    /// Status
    pub status: DeFiAnalyticsStatus,
    /// Created at
    pub created_at: i64,
    /// Analytics config hash
    pub analytics_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_defi_analytics(
        analytics: &mut DeFiAnalyticsMetadata,
        analytics_id: u64,
        protocol_id: u64,
        analytics_type: DeFiAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.protocol_id = protocol_id;
        analytics.analytics_type = analytics_type;
        analytics.status = DeFiAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_defi_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }
}
