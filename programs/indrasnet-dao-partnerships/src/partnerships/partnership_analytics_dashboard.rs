//! Partnership Analytics Dashboard module
//!
//! Partnership analytics dashboard
//!
//! On-chain: Metadata for dashboard
//! Off-chain: Actual dashboard, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Dashboard type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipDashboardType {
    /// Executive dashboard
    Executive,
    /// Operational dashboard
    Operational,
    /// Analytical dashboard
    Analytical,
    /// Custom dashboard
    Custom,
}

/// Dashboard status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipDashboardStatus {
    /// Dashboard generating
    Generating,
    /// Dashboard ready
    Ready,
    /// Dashboard published
    Published,
}

/// Partnership analytics dashboard metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsDashboardMetadata {
    /// Dashboard ID
    pub dashboard_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Dashboard type
    pub dashboard_type: PartnershipDashboardType,
    /// Status
    pub status: PartnershipDashboardStatus,
    /// Created at
    pub created_at: i64,
    /// Dashboard data hash
    pub dashboard_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_dashboard(
        dashboard: &mut PartnershipAnalyticsDashboardMetadata,
        dashboard_id: u64,
        partnership_id: u64,
        dashboard_type: PartnershipDashboardType,
        dashboard_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(dashboard_id > 0, IndrasError::InvalidInput);
        dashboard.dashboard_id = dashboard_id;
        dashboard.partnership_id = partnership_id;
        dashboard.dashboard_type = dashboard_type;
        dashboard.status = PartnershipDashboardStatus::Generating;
        dashboard.created_at = current_time;
        dashboard.dashboard_data_hash = dashboard_data_hash;
        dashboard.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_dashboard(_dashboard_id: u64) -> Vec<u8> {
        vec![]
    }
}
