//! Partnership Analytics Conversion module
//!
//! Partnership analytics conversion
//!
//! On-chain: Metadata for conversion
//! Off-chain: Actual conversion, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Conversion type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipConversionType {
    /// Lead conversion
    Lead,
    /// Trial conversion
    Trial,
    /// Purchase conversion
    Purchase,
    /// Custom conversion
    Custom,
}

/// Conversion status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipConversionStatus {
    /// Conversion tracking
    Tracking,
    /// Conversion tracked
    Tracked,
    /// Conversion optimized
    Optimized,
}

/// Partnership analytics conversion metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsConversionMetadata {
    /// Conversion ID
    pub conversion_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Conversion type
    pub conversion_type: PartnershipConversionType,
    /// Status
    pub status: PartnershipConversionStatus,
    /// Created at
    pub created_at: i64,
    /// Conversion data hash
    pub conversion_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_conversion(
        conversion: &mut PartnershipAnalyticsConversionMetadata,
        conversion_id: u64,
        partnership_id: u64,
        conversion_type: PartnershipConversionType,
        conversion_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(conversion_id > 0, IndrasError::InvalidInput);
        conversion.conversion_id = conversion_id;
        conversion.partnership_id = partnership_id;
        conversion.conversion_type = conversion_type;
        conversion.status = PartnershipConversionStatus::Tracking;
        conversion.created_at = current_time;
        conversion.conversion_data_hash = conversion_data_hash;
        conversion.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn track_conversion(_conversion_id: u64) -> Vec<u8> {
        vec![]
    }
}
