//! Partnership Analytics Attribution module
//!
//! Partnership analytics attribution
//!
//! On-chain: Metadata for attribution
//! Off-chain: Actual attribution, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Attribution model
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipAttributionModel {
    /// First-touch attribution
    FirstTouch,
    /// Last-touch attribution
    LastTouch,
    /// Linear attribution
    Linear,
    /// Custom model
    Custom,
}

/// Attribution status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipAttributionStatus {
    /// Attribution pending
    Pending,
    /// Attribution in progress
    InProgress,
    /// Attribution completed
    Completed,
}

/// Partnership analytics attribution metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsAttributionMetadata {
    /// Attribution ID
    pub attribution_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Attribution model
    pub attribution_model: PartnershipAttributionModel,
    /// Status
    pub status: PartnershipAttributionStatus,
    /// Created at
    pub created_at: i64,
    /// Attribution data hash
    pub attribution_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_attribution(
        attribution: &mut PartnershipAnalyticsAttributionMetadata,
        attribution_id: u64,
        partnership_id: u64,
        attribution_model: PartnershipAttributionModel,
        attribution_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(attribution_id > 0, IndrasError::InvalidInput);
        attribution.attribution_id = attribution_id;
        attribution.partnership_id = partnership_id;
        attribution.attribution_model = attribution_model;
        attribution.status = PartnershipAttributionStatus::Pending;
        attribution.created_at = current_time;
        attribution.attribution_data_hash = attribution_data_hash;
        attribution.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn perform_attribution(_attribution_id: u64) -> Vec<u8> {
        vec![]
    }
}
