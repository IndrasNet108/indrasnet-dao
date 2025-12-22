//! Financial Inventory Management module
//!
//! Financial inventory management
//!
//! On-chain: Metadata for inventory
//! Off-chain: Actual inventory, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Inventory type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInventoryType {
    /// Raw materials
    RawMaterials,
    /// Work in progress
    WorkInProgress,
    /// Finished goods
    FinishedGoods,
    /// Custom type
    Custom,
}

/// Inventory status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInventoryStatus {
    /// Inventory active
    Active,
    /// Inventory paused
    Paused,
    /// Inventory optimized
    Optimized,
}

/// Financial inventory management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialInventoryManagementMetadata {
    /// Inventory ID
    pub inventory_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Inventory type
    pub inventory_type: FinancialInventoryType,
    /// Status
    pub status: FinancialInventoryStatus,
    /// Created at
    pub created_at: i64,
    /// Inventory config hash
    pub inventory_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_inventory_management(
        inventory: &mut FinancialInventoryManagementMetadata,
        inventory_id: u64,
        entity_id: u64,
        inventory_type: FinancialInventoryType,
        inventory_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(inventory_id > 0, IndrasError::InvalidInput);
        inventory.inventory_id = inventory_id;
        inventory.entity_id = entity_id;
        inventory.inventory_type = inventory_type;
        inventory.status = FinancialInventoryStatus::Active;
        inventory.created_at = current_time;
        inventory.inventory_config_hash = inventory_config_hash;
        inventory.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_inventory(_inventory_id: u64) -> Vec<u8> {
        vec![]
    }
}
