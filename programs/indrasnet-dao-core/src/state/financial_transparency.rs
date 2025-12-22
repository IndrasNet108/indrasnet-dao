//! Financial Transparency module
//!
//! Financial transparency management
//!
//! On-chain: Metadata for financial transparency
//! Off-chain: Actual transparency, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Transparency level
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialTransparencyLevel {
    /// Full transparency
    Full,
    /// Partial transparency
    Partial,
    /// Minimal transparency
    Minimal,
    /// Custom level
    Custom,
}

/// Transparency status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialTransparencyStatus {
    /// Transparency active
    Active,
    /// Transparency paused
    Paused,
    /// Transparency disabled
    Disabled,
}

/// Financial transparency metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialTransparencyMetadata {
    /// Transparency ID
    pub transparency_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Transparency level
    pub transparency_level: FinancialTransparencyLevel,
    /// Status
    pub status: FinancialTransparencyStatus,
    /// Created at
    pub created_at: i64,
    /// Transparency config hash
    pub transparency_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_transparency(
        transparency: &mut FinancialTransparencyMetadata,
        transparency_id: u64,
        entity_id: u64,
        transparency_level: FinancialTransparencyLevel,
        transparency_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(transparency_id > 0, IndrasError::InvalidInput);
        transparency.transparency_id = transparency_id;
        transparency.entity_id = entity_id;
        transparency.transparency_level = transparency_level;
        transparency.status = FinancialTransparencyStatus::Active;
        transparency.created_at = current_time;
        transparency.transparency_config_hash = transparency_config_hash;
        transparency.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn ensure_transparency(_transparency_id: u64) -> Vec<u8> {
        vec![]
    }
}
