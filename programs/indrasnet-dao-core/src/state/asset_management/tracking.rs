//! Asset Tracking module
//!
//! Asset tracking and monitoring
//!
//! On-chain: Metadata for asset tracking
//! Off-chain: Actual tracking, monitoring

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Tracking status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AssetTrackingStatus {
    /// Tracking active
    Active,
    /// Tracking paused
    Paused,
    /// Tracking stopped
    Stopped,
}

/// Asset tracking metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct AssetTrackingMetadata {
    /// Tracking ID
    pub tracking_id: u64,
    /// Asset ID
    pub asset_id: u64,
    /// Status
    pub status: AssetTrackingStatus,
    /// Created at
    pub created_at: i64,
    /// Tracking config hash
    pub tracking_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_asset_tracking(
        tracking: &mut AssetTrackingMetadata,
        tracking_id: u64,
        asset_id: u64,
        tracking_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(tracking_id > 0, IndrasError::InvalidInput);
        tracking.tracking_id = tracking_id;
        tracking.asset_id = asset_id;
        tracking.status = AssetTrackingStatus::Active;
        tracking.created_at = current_time;
        tracking.tracking_config_hash = tracking_config_hash;
        tracking.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn track_asset(_tracking_id: u64) -> Vec<u8> {
        vec![]
    }
}
