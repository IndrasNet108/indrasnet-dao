//! Dashboards module
//!
//! Partnership dashboards and visualization
//!
//! On-chain: Metadata for dashboards
//! Off-chain: Actual dashboard generation, visualization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Dashboard type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DashboardType {
    /// Overview dashboard
    Overview,
    /// Performance dashboard
    Performance,
    /// Revenue dashboard
    Revenue,
    /// Custom dashboard
    Custom,
}

/// Partnership dashboard metadata (on-chain)
///
/// Stores metadata for partnership dashboards
#[account]
#[derive(InitSpace)]
pub struct PartnershipDashboardMetadata {
    /// Dashboard ID
    pub dashboard_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Dashboard type
    pub dashboard_type: DashboardType,
    /// Created at
    pub created_at: i64,
    /// Dashboard data hash
    pub dashboard_data_hash: [u8; 32],
    /// Dashboard URI
    #[max_len(200)]
    pub dashboard_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for dashboards
pub mod onchain {
    use super::*;

    /// Initialize partnership dashboard
    pub fn initialize_partnership_dashboard(
        dashboard: &mut PartnershipDashboardMetadata,
        dashboard_id: u64,
        partnership_id: u64,
        dashboard_type: DashboardType,
        dashboard_data_hash: [u8; 32],
        dashboard_uri: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(dashboard_id > 0, IndrasError::InvalidInput);
        require!(dashboard_uri.len() <= 200, IndrasError::InvalidInput);
        
        dashboard.dashboard_id = dashboard_id;
        dashboard.partnership_id = partnership_id;
        dashboard.dashboard_type = dashboard_type;
        dashboard.created_at = current_time;
        dashboard.dashboard_data_hash = dashboard_data_hash;
        dashboard.dashboard_uri = dashboard_uri;
        dashboard.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for dashboards
pub mod offchain {
    /// Generate dashboard
    pub fn generate_dashboard(_dashboard_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
