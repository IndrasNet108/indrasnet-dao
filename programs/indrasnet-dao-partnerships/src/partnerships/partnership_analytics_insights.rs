//! Partnership Analytics Insights module
//!
//! Insights generation for partnerships
//!
//! On-chain: Metadata for insights
//! Off-chain: Actual insights generation, recommendations

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Insight type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InsightType {
    /// Performance insight
    Performance,
    /// Revenue insight
    Revenue,
    /// Risk insight
    Risk,
    /// Opportunity insight
    Opportunity,
}

/// Insight status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InsightStatus {
    /// Insight generated
    Generated,
    /// Insight reviewed
    Reviewed,
    /// Insight actioned
    Actioned,
}

/// Partnership analytics insight metadata (on-chain)
///
/// Stores metadata for analytics insights
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsInsightMetadata {
    /// Insight ID
    pub insight_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Insight type
    pub insight_type: InsightType,
    /// Status
    pub status: InsightStatus,
    /// Created at
    pub created_at: i64,
    /// Insight data hash
    pub insight_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for partnership analytics insights
pub mod onchain {
    use super::*;

    /// Initialize partnership analytics insight
    pub fn initialize_partnership_analytics_insight(
        insight: &mut PartnershipAnalyticsInsightMetadata,
        insight_id: u64,
        partnership_id: u64,
        insight_type: InsightType,
        insight_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(insight_id > 0, IndrasError::InvalidInput);
        
        insight.insight_id = insight_id;
        insight.partnership_id = partnership_id;
        insight.insight_type = insight_type;
        insight.status = InsightStatus::Generated;
        insight.created_at = current_time;
        insight.insight_data_hash = insight_data_hash;
        insight.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for partnership analytics insights
pub mod offchain {
    /// Generate insight
    pub fn generate_insight(_insight_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
