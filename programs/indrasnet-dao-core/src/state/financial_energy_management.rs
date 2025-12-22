//! Financial Energy Management module
//!
//! Financial energy management
//!
//! On-chain: Metadata for energy management
//! Off-chain: Actual energy, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Energy source
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialEnergySource {
    /// Renewable energy
    Renewable,
    /// Non-renewable energy
    NonRenewable,
    /// Mixed energy
    Mixed,
    /// Custom source
    Custom,
}

/// Energy status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialEnergyStatus {
    /// Energy active
    Active,
    /// Energy paused
    Paused,
    /// Energy optimized
    Optimized,
}

/// Financial energy management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialEnergyManagementMetadata {
    /// Energy ID
    pub energy_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Energy source
    pub energy_source: FinancialEnergySource,
    /// Status
    pub status: FinancialEnergyStatus,
    /// Created at
    pub created_at: i64,
    /// Energy config hash
    pub energy_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_energy_management(
        energy: &mut FinancialEnergyManagementMetadata,
        energy_id: u64,
        entity_id: u64,
        energy_source: FinancialEnergySource,
        energy_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(energy_id > 0, IndrasError::InvalidInput);
        energy.energy_id = energy_id;
        energy.entity_id = entity_id;
        energy.energy_source = energy_source;
        energy.status = FinancialEnergyStatus::Active;
        energy.created_at = current_time;
        energy.energy_config_hash = energy_config_hash;
        energy.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_energy(_energy_id: u64) -> Vec<u8> {
        vec![]
    }
}
