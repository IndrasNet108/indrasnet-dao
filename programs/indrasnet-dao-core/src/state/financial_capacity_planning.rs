//! Financial Capacity Planning module
//!
//! Financial capacity planning
//!
//! On-chain: Metadata for capacity planning
//! Off-chain: Actual planning, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Capacity type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCapacityType {
    /// Production capacity
    Production,
    /// Service capacity
    Service,
    /// Infrastructure capacity
    Infrastructure,
    /// Custom capacity
    Custom,
}

/// Planning status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCapacityPlanningStatus {
    /// Planning active
    Active,
    /// Planning paused
    Paused,
    /// Planning optimized
    Optimized,
}

/// Financial capacity planning metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCapacityPlanningMetadata {
    /// Planning ID
    pub planning_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Capacity type
    pub capacity_type: FinancialCapacityType,
    /// Status
    pub status: FinancialCapacityPlanningStatus,
    /// Created at
    pub created_at: i64,
    /// Planning config hash
    pub planning_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_capacity_planning(
        planning: &mut FinancialCapacityPlanningMetadata,
        planning_id: u64,
        entity_id: u64,
        capacity_type: FinancialCapacityType,
        planning_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(planning_id > 0, IndrasError::InvalidInput);
        planning.planning_id = planning_id;
        planning.entity_id = entity_id;
        planning.capacity_type = capacity_type;
        planning.status = FinancialCapacityPlanningStatus::Active;
        planning.created_at = current_time;
        planning.planning_config_hash = planning_config_hash;
        planning.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn plan_capacity(_planning_id: u64) -> Vec<u8> {
        vec![]
    }
}
