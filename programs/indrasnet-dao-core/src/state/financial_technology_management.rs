//! Financial Technology Management module
//!
//! Financial technology management
//!
//! On-chain: Metadata for technology
//! Off-chain: Actual technology, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Technology type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialTechnologyType {
    /// IT infrastructure
    ITInfrastructure,
    /// Software systems
    SoftwareSystems,
    /// Hardware systems
    HardwareSystems,
    /// Custom technology
    Custom,
}

/// Technology status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialTechnologyStatus {
    /// Technology active
    Active,
    /// Technology deprecated
    Deprecated,
    /// Technology upgraded
    Upgraded,
}

/// Financial technology management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialTechnologyManagementMetadata {
    /// Technology ID
    pub technology_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Technology type
    pub technology_type: FinancialTechnologyType,
    /// Status
    pub status: FinancialTechnologyStatus,
    /// Created at
    pub created_at: i64,
    /// Technology config hash
    pub technology_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_technology_management(
        technology: &mut FinancialTechnologyManagementMetadata,
        technology_id: u64,
        entity_id: u64,
        technology_type: FinancialTechnologyType,
        technology_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(technology_id > 0, IndrasError::InvalidInput);
        technology.technology_id = technology_id;
        technology.entity_id = entity_id;
        technology.technology_type = technology_type;
        technology.status = FinancialTechnologyStatus::Active;
        technology.created_at = current_time;
        technology.technology_config_hash = technology_config_hash;
        technology.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_technology(_technology_id: u64) -> Vec<u8> {
        vec![]
    }
}
