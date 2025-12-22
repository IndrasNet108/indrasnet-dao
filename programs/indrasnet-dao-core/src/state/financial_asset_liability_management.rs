//! Financial Asset-Liability Management module
//!
//! Financial asset-liability management
//!
//! On-chain: Metadata for ALM
//! Off-chain: Actual management, optimization

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// ALM strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialALMStrategy {
    /// Duration matching
    DurationMatching,
    /// Gap management
    GapManagement,
    /// Immunization
    Immunization,
    /// Custom strategy
    Custom,
}

/// Management status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialALMStatus {
    /// Management active
    Active,
    /// Management paused
    Paused,
    /// Management optimized
    Optimized,
}

/// Financial asset-liability management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialAssetLiabilityManagementMetadata {
    /// Management ID
    pub management_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// ALM strategy
    pub alm_strategy: FinancialALMStrategy,
    /// Status
    pub status: FinancialALMStatus,
    /// Created at
    pub created_at: i64,
    /// Management config hash
    pub management_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_asset_liability_management(
        management: &mut FinancialAssetLiabilityManagementMetadata,
        management_id: u64,
        entity_id: u64,
        alm_strategy: FinancialALMStrategy,
        management_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(management_id > 0, IndrasError::InvalidInput);
        management.management_id = management_id;
        management.entity_id = entity_id;
        management.alm_strategy = alm_strategy;
        management.status = FinancialALMStatus::Active;
        management.created_at = current_time;
        management.management_config_hash = management_config_hash;
        management.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_alm(_management_id: u64) -> Vec<u8> {
        vec![]
    }
}
