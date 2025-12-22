//! Partnership Analytics Lifetime Value module
//!
//! Partnership analytics lifetime value
//!
//! On-chain: Metadata for lifetime value
//! Off-chain: Actual calculation, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Value type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipLifetimeValueType {
    /// Customer lifetime value
    Customer,
    /// Partnership lifetime value
    Partnership,
    /// Revenue lifetime value
    Revenue,
    /// Custom value
    Custom,
}

/// Value status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipLifetimeValueStatus {
    /// Value calculating
    Calculating,
    /// Value calculated
    Calculated,
    /// Value optimized
    Optimized,
}

/// Partnership analytics lifetime value metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsLifetimeValueMetadata {
    /// Value ID
    pub value_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Value type
    pub value_type: PartnershipLifetimeValueType,
    /// Status
    pub status: PartnershipLifetimeValueStatus,
    /// Created at
    pub created_at: i64,
    /// Value data hash
    pub value_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_lifetime_value(
        value: &mut PartnershipAnalyticsLifetimeValueMetadata,
        value_id: u64,
        partnership_id: u64,
        value_type: PartnershipLifetimeValueType,
        value_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(value_id > 0, IndrasError::InvalidInput);
        value.value_id = value_id;
        value.partnership_id = partnership_id;
        value.value_type = value_type;
        value.status = PartnershipLifetimeValueStatus::Calculating;
        value.created_at = current_time;
        value.value_data_hash = value_data_hash;
        value.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn calculate_lifetime_value(_value_id: u64) -> Vec<u8> {
        vec![]
    }
}
