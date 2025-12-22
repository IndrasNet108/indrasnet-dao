//! Financial Sustainability module
//!
//! Financial sustainability management
//!
//! On-chain: Metadata for sustainability
//! Off-chain: Actual sustainability, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Sustainability dimension
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSustainabilityDimension {
    /// Environmental
    Environmental,
    /// Social
    Social,
    /// Governance
    Governance,
    /// Economic
    Economic,
}

/// Sustainability status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSustainabilityStatus {
    /// Sustainability active
    Active,
    /// Sustainability paused
    Paused,
    /// Sustainability certified
    Certified,
}

/// Financial sustainability metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialSustainabilityMetadata {
    /// Sustainability ID
    pub sustainability_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Sustainability dimension
    pub sustainability_dimension: FinancialSustainabilityDimension,
    /// Status
    pub status: FinancialSustainabilityStatus,
    /// Created at
    pub created_at: i64,
    /// Sustainability config hash
    pub sustainability_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_sustainability(
        sustainability: &mut FinancialSustainabilityMetadata,
        sustainability_id: u64,
        entity_id: u64,
        sustainability_dimension: FinancialSustainabilityDimension,
        sustainability_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(sustainability_id > 0, IndrasError::InvalidInput);
        sustainability.sustainability_id = sustainability_id;
        sustainability.entity_id = entity_id;
        sustainability.sustainability_dimension = sustainability_dimension;
        sustainability.status = FinancialSustainabilityStatus::Active;
        sustainability.created_at = current_time;
        sustainability.sustainability_config_hash = sustainability_config_hash;
        sustainability.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_sustainability(_sustainability_id: u64) -> Vec<u8> {
        vec![]
    }
}
