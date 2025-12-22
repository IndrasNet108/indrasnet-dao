//! Financial Resource Management module
//!
//! Financial resource management
//!
//! On-chain: Metadata for resources
//! Off-chain: Actual resources, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Resource type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialResourceType {
    /// Human resources
    Human,
    /// Financial resources
    Financial,
    /// Physical resources
    Physical,
    /// Custom resource
    Custom,
}

/// Resource status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialResourceStatus {
    /// Resource active
    Active,
    /// Resource allocated
    Allocated,
    /// Resource optimized
    Optimized,
}

/// Financial resource management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialResourceManagementMetadata {
    /// Resource ID
    pub resource_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Resource type
    pub resource_type: FinancialResourceType,
    /// Status
    pub status: FinancialResourceStatus,
    /// Created at
    pub created_at: i64,
    /// Resource config hash
    pub resource_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_resource_management(
        resource: &mut FinancialResourceManagementMetadata,
        resource_id: u64,
        entity_id: u64,
        resource_type: FinancialResourceType,
        resource_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(resource_id > 0, IndrasError::InvalidInput);
        resource.resource_id = resource_id;
        resource.entity_id = entity_id;
        resource.resource_type = resource_type;
        resource.status = FinancialResourceStatus::Active;
        resource.created_at = current_time;
        resource.resource_config_hash = resource_config_hash;
        resource.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_resource(_resource_id: u64) -> Vec<u8> {
        vec![]
    }
}
