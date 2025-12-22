//! Partnership Analytics ROI module
//!
//! Partnership analytics ROI calculation
//!
//! On-chain: Metadata for ROI
//! Off-chain: Actual calculation, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// ROI type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipROIType {
    /// Financial ROI
    Financial,
    /// Strategic ROI
    Strategic,
    /// Operational ROI
    Operational,
    /// Custom ROI
    Custom,
}

/// ROI status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipROIStatus {
    /// ROI calculating
    Calculating,
    /// ROI calculated
    Calculated,
    /// ROI optimized
    Optimized,
}

/// Partnership analytics ROI metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsROIMetadata {
    /// ROI ID
    pub roi_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// ROI type
    pub roi_type: PartnershipROIType,
    /// Status
    pub status: PartnershipROIStatus,
    /// Created at
    pub created_at: i64,
    /// ROI data hash
    pub roi_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_roi(
        roi: &mut PartnershipAnalyticsROIMetadata,
        roi_id: u64,
        partnership_id: u64,
        roi_type: PartnershipROIType,
        roi_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(roi_id > 0, IndrasError::InvalidInput);
        roi.roi_id = roi_id;
        roi.partnership_id = partnership_id;
        roi.roi_type = roi_type;
        roi.status = PartnershipROIStatus::Calculating;
        roi.created_at = current_time;
        roi.roi_data_hash = roi_data_hash;
        roi.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn calculate_roi(_roi_id: u64) -> Vec<u8> {
        vec![]
    }
}
