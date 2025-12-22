//! Financial Workforce Planning module
//!
//! Financial workforce planning
//!
//! On-chain: Metadata for workforce planning
//! Off-chain: Actual planning, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Planning approach
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWorkforcePlanningApproach {
    /// Strategic planning
    Strategic,
    /// Operational planning
    Operational,
    /// Tactical planning
    Tactical,
    /// Custom approach
    Custom,
}

/// Planning status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWorkforcePlanningStatus {
    /// Planning active
    Active,
    /// Planning paused
    Paused,
    /// Planning optimized
    Optimized,
}

/// Financial workforce planning metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialWorkforcePlanningMetadata {
    /// Planning ID
    pub planning_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Planning approach
    pub planning_approach: FinancialWorkforcePlanningApproach,
    /// Status
    pub status: FinancialWorkforcePlanningStatus,
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
    pub fn initialize_financial_workforce_planning(
        planning: &mut FinancialWorkforcePlanningMetadata,
        planning_id: u64,
        entity_id: u64,
        planning_approach: FinancialWorkforcePlanningApproach,
        planning_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(planning_id > 0, IndrasError::InvalidInput);
        planning.planning_id = planning_id;
        planning.entity_id = entity_id;
        planning.planning_approach = planning_approach;
        planning.status = FinancialWorkforcePlanningStatus::Active;
        planning.created_at = current_time;
        planning.planning_config_hash = planning_config_hash;
        planning.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn plan_workforce(_planning_id: u64) -> Vec<u8> {
        vec![]
    }
}
