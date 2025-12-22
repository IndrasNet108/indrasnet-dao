//! Business Intelligence Partnership Analytics module
//!
//! Business intelligence analytics for partnerships
//!
//! On-chain: Metadata for BI analytics
//! Off-chain: Actual BI processing, dashboards

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// BI dashboard type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BIDashboardType {
    /// Executive dashboard
    Executive,
    /// Operational dashboard
    Operational,
    /// Analytical dashboard
    Analytical,
    /// Custom dashboard
    Custom,
}

/// BI analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BIAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics inactive
    Inactive,
}

/// BI partnership analytics metadata (on-chain)
///
/// Stores metadata for BI analytics
#[account]
#[derive(InitSpace)]
pub struct BIPartnershipAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Dashboard type
    pub dashboard_type: BIDashboardType,
    /// Status
    pub status: BIAnalyticsStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Analytics data hash
    pub analytics_data_hash: [u8; 32],
    /// Dashboard URI
    #[max_len(200)]
    pub dashboard_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for BI partnership analytics
pub mod onchain {
    use super::*;

    /// Initialize BI partnership analytics
    pub fn initialize_bi_partnership_analytics(
        analytics: &mut BIPartnershipAnalyticsMetadata,
        analytics_id: u64,
        partnership_id: u64,
        dashboard_type: BIDashboardType,
        analytics_data_hash: [u8; 32],
        dashboard_uri: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        require!(dashboard_uri.len() <= 200, IndrasError::InvalidInput);
        
        analytics.analytics_id = analytics_id;
        analytics.partnership_id = partnership_id;
        analytics.dashboard_type = dashboard_type;
        analytics.status = BIAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.updated_at = current_time;
        analytics.analytics_data_hash = analytics_data_hash;
        analytics.dashboard_uri = dashboard_uri;
        analytics.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for BI partnership analytics
pub mod offchain {
    /// Generate BI dashboard
    pub fn generate_bi_dashboard(_analytics_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
