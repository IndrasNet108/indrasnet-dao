//! Tax Planning module
//!
//! Tax planning and strategy
//!
//! On-chain: Metadata for tax planning
//! Off-chain: Actual planning, strategy development

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Planning strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxPlanningStrategy {
    /// Tax deferral
    TaxDeferral,
    /// Tax reduction
    TaxReduction,
    /// Tax avoidance
    TaxAvoidance,
    /// Custom strategy
    Custom,
}

/// Planning status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxPlanningStatus {
    /// Planning active
    Active,
    /// Planning paused
    Paused,
    /// Planning disabled
    Disabled,
}

/// Tax planning metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct TaxPlanningMetadata {
    /// Planning ID
    pub planning_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Planning strategy
    pub planning_strategy: TaxPlanningStrategy,
    /// Status
    pub status: TaxPlanningStatus,
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
    pub fn initialize_tax_planning(
        planning: &mut TaxPlanningMetadata,
        planning_id: u64,
        entity_id: u64,
        planning_strategy: TaxPlanningStrategy,
        planning_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(planning_id > 0, IndrasError::InvalidInput);
        planning.planning_id = planning_id;
        planning.entity_id = entity_id;
        planning.planning_strategy = planning_strategy;
        planning.status = TaxPlanningStatus::Active;
        planning.created_at = current_time;
        planning.planning_config_hash = planning_config_hash;
        planning.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn develop_tax_plan(_planning_id: u64) -> Vec<u8> {
        vec![]
    }
}
