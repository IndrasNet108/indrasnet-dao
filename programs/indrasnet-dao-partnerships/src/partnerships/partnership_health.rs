//! Partnership Health module
//!
//! Partnership health monitoring
//!
//! On-chain: Metadata for health metrics
//! Off-chain: Actual health calculations, alerts

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Health status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum HealthStatus {
    /// Health healthy
    Healthy,
    /// Health warning
    Warning,
    /// Health critical
    Critical,
}

/// Partnership health metadata (on-chain)
///
/// Stores metadata for partnership health
#[account]
#[derive(InitSpace)]
pub struct PartnershipHealthMetadata {
    /// Health ID
    pub health_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Health score (0-100)
    pub health_score: u8,
    /// Status
    pub status: HealthStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Health data hash
    pub health_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for partnership health
pub mod onchain {
    use super::*;

    /// Initialize partnership health
    pub fn initialize_partnership_health(
        health: &mut PartnershipHealthMetadata,
        health_id: u64,
        partnership_id: u64,
        health_score: u8,
        health_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(health_id > 0, IndrasError::InvalidInput);
        require!(health_score <= 100, IndrasError::InvalidInput);
        
        health.health_id = health_id;
        health.partnership_id = partnership_id;
        health.health_score = health_score;
        health.status = if health_score >= 80 {
            HealthStatus::Healthy
        } else if health_score >= 50 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };
        health.created_at = current_time;
        health.updated_at = current_time;
        health.health_data_hash = health_data_hash;
        health.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for partnership health
pub mod offchain {
    /// Calculate health score
    pub fn calculate_health_score(_partnership_id: u64) -> u8 {
        // Implementation in off-chain service
        0
    }
}
