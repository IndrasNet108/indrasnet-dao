//! Financial Waste Management module
//!
//! Financial waste management
//!
//! On-chain: Metadata for waste management
//! Off-chain: Actual waste, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Waste type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWasteType {
    /// Solid waste
    Solid,
    /// Liquid waste
    Liquid,
    /// Hazardous waste
    Hazardous,
    /// Custom waste
    Custom,
}

/// Waste status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWasteStatus {
    /// Waste active
    Active,
    /// Waste paused
    Paused,
    /// Waste minimized
    Minimized,
}

/// Financial waste management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialWasteManagementMetadata {
    /// Waste ID
    pub waste_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Waste type
    pub waste_type: FinancialWasteType,
    /// Status
    pub status: FinancialWasteStatus,
    /// Created at
    pub created_at: i64,
    /// Waste config hash
    pub waste_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_waste_management(
        waste: &mut FinancialWasteManagementMetadata,
        waste_id: u64,
        entity_id: u64,
        waste_type: FinancialWasteType,
        waste_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(waste_id > 0, IndrasError::InvalidInput);
        waste.waste_id = waste_id;
        waste.entity_id = entity_id;
        waste.waste_type = waste_type;
        waste.status = FinancialWasteStatus::Active;
        waste.created_at = current_time;
        waste.waste_config_hash = waste_config_hash;
        waste.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_waste(_waste_id: u64) -> Vec<u8> {
        vec![]
    }
}
