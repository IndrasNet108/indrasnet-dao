//! Financial Agility module
//!
//! Financial agility management
//!
//! On-chain: Metadata for agility
//! Off-chain: Actual agility, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Agility dimension
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialAgilityDimension {
    /// Strategic agility
    Strategic,
    /// Operational agility
    Operational,
    /// Market agility
    Market,
    /// Custom dimension
    Custom,
}

/// Agility status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialAgilityStatus {
    /// Agility active
    Active,
    /// Agility paused
    Paused,
    /// Agility achieved
    Achieved,
}

/// Financial agility metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialAgilityMetadata {
    /// Agility ID
    pub agility_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Agility dimension
    pub agility_dimension: FinancialAgilityDimension,
    /// Status
    pub status: FinancialAgilityStatus,
    /// Created at
    pub created_at: i64,
    /// Agility config hash
    pub agility_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_agility(
        agility: &mut FinancialAgilityMetadata,
        agility_id: u64,
        entity_id: u64,
        agility_dimension: FinancialAgilityDimension,
        agility_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(agility_id > 0, IndrasError::InvalidInput);
        agility.agility_id = agility_id;
        agility.entity_id = entity_id;
        agility.agility_dimension = agility_dimension;
        agility.status = FinancialAgilityStatus::Active;
        agility.created_at = current_time;
        agility.agility_config_hash = agility_config_hash;
        agility.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_agility(_agility_id: u64) -> Vec<u8> {
        vec![]
    }
}
