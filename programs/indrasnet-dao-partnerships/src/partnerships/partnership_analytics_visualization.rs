//! Partnership Analytics Visualization module
//!
//! Partnership analytics visualization
//!
//! On-chain: Metadata for visualization
//! Off-chain: Actual visualization, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Visualization type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipVisualizationType {
    /// Chart visualization
    Chart,
    /// Graph visualization
    Graph,
    /// Map visualization
    Map,
    /// Custom visualization
    Custom,
}

/// Visualization status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipVisualizationStatus {
    /// Visualization generating
    Generating,
    /// Visualization ready
    Ready,
    /// Visualization published
    Published,
}

/// Partnership analytics visualization metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsVisualizationMetadata {
    /// Visualization ID
    pub visualization_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Visualization type
    pub visualization_type: PartnershipVisualizationType,
    /// Status
    pub status: PartnershipVisualizationStatus,
    /// Created at
    pub created_at: i64,
    /// Visualization data hash
    pub visualization_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_visualization(
        visualization: &mut PartnershipAnalyticsVisualizationMetadata,
        visualization_id: u64,
        partnership_id: u64,
        visualization_type: PartnershipVisualizationType,
        visualization_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(visualization_id > 0, IndrasError::InvalidInput);
        visualization.visualization_id = visualization_id;
        visualization.partnership_id = partnership_id;
        visualization.visualization_type = visualization_type;
        visualization.status = PartnershipVisualizationStatus::Generating;
        visualization.created_at = current_time;
        visualization.visualization_data_hash = visualization_data_hash;
        visualization.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_visualization(_visualization_id: u64) -> Vec<u8> {
        vec![]
    }
}
