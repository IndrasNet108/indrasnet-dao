//! Financial Continuity Planning module
//!
//! Financial continuity planning
//!
//! On-chain: Metadata for continuity planning
//! Off-chain: Actual planning, implementation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Planning scope
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialContinuityPlanningScope {
    /// Business continuity
    Business,
    /// Financial continuity
    Financial,
    /// Operational continuity
    Operational,
    /// Custom scope
    Custom,
}

/// Planning status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialContinuityPlanningStatus {
    /// Planning active
    Active,
    /// Planning paused
    Paused,
    /// Planning implemented
    Implemented,
}

/// Financial continuity planning metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialContinuityPlanningMetadata {
    /// Planning ID
    pub planning_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Planning scope
    pub planning_scope: FinancialContinuityPlanningScope,
    /// Status
    pub status: FinancialContinuityPlanningStatus,
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
    pub fn initialize_financial_continuity_planning(
        planning: &mut FinancialContinuityPlanningMetadata,
        planning_id: u64,
        entity_id: u64,
        planning_scope: FinancialContinuityPlanningScope,
        planning_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(planning_id > 0, IndrasError::InvalidInput);
        planning.planning_id = planning_id;
        planning.entity_id = entity_id;
        planning.planning_scope = planning_scope;
        planning.status = FinancialContinuityPlanningStatus::Active;
        planning.created_at = current_time;
        planning.planning_config_hash = planning_config_hash;
        planning.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn implement_continuity_plan(_planning_id: u64) -> Vec<u8> {
        vec![]
    }
}
