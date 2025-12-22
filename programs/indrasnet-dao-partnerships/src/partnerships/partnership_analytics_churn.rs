//! Partnership Analytics Churn module
//!
//! Partnership analytics churn analysis
//!
//! On-chain: Metadata for churn
//! Off-chain: Actual churn, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Churn type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipChurnType {
    /// Customer churn
    Customer,
    /// Revenue churn
    Revenue,
    /// Engagement churn
    Engagement,
    /// Custom churn
    Custom,
}

/// Churn status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipChurnStatus {
    /// Churn analyzing
    Analyzing,
    /// Churn analyzed
    Analyzed,
    /// Churn mitigated
    Mitigated,
}

/// Partnership analytics churn metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsChurnMetadata {
    /// Churn ID
    pub churn_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Churn type
    pub churn_type: PartnershipChurnType,
    /// Status
    pub status: PartnershipChurnStatus,
    /// Created at
    pub created_at: i64,
    /// Churn data hash
    pub churn_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_churn(
        churn: &mut PartnershipAnalyticsChurnMetadata,
        churn_id: u64,
        partnership_id: u64,
        churn_type: PartnershipChurnType,
        churn_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(churn_id > 0, IndrasError::InvalidInput);
        churn.churn_id = churn_id;
        churn.partnership_id = partnership_id;
        churn.churn_type = churn_type;
        churn.status = PartnershipChurnStatus::Analyzing;
        churn.created_at = current_time;
        churn.churn_data_hash = churn_data_hash;
        churn.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_churn(_churn_id: u64) -> Vec<u8> {
        vec![]
    }
}
