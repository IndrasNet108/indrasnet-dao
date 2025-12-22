//! Financial Diversity & Inclusion module
//!
//! Financial diversity and inclusion management
//!
//! On-chain: Metadata for diversity and inclusion
//! Off-chain: Actual D&I, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// D&I dimension
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDIDimension {
    /// Gender diversity
    Gender,
    /// Ethnic diversity
    Ethnic,
    /// Age diversity
    Age,
    /// Custom dimension
    Custom,
}

/// D&I status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDIStatus {
    /// D&I active
    Active,
    /// D&I paused
    Paused,
    /// D&I achieved
    Achieved,
}

/// Financial diversity and inclusion metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDiversityInclusionMetadata {
    /// D&I ID
    pub di_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// D&I dimension
    pub di_dimension: FinancialDIDimension,
    /// Status
    pub status: FinancialDIStatus,
    /// Created at
    pub created_at: i64,
    /// D&I config hash
    pub di_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_diversity_inclusion(
        di: &mut FinancialDiversityInclusionMetadata,
        di_id: u64,
        entity_id: u64,
        di_dimension: FinancialDIDimension,
        di_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(di_id > 0, IndrasError::InvalidInput);
        di.di_id = di_id;
        di.entity_id = entity_id;
        di.di_dimension = di_dimension;
        di.status = FinancialDIStatus::Active;
        di.created_at = current_time;
        di.di_config_hash = di_config_hash;
        di.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_diversity_inclusion(_di_id: u64) -> Vec<u8> {
        vec![]
    }
}
