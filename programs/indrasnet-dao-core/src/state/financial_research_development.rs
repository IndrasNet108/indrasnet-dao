//! Financial Research & Development module
//!
//! Financial R&D management
//!
//! On-chain: Metadata for R&D
//! Off-chain: Actual R&D, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// R&D type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRDType {
    /// Basic research
    BasicResearch,
    /// Applied research
    AppliedResearch,
    /// Development
    Development,
    /// Custom type
    Custom,
}

/// R&D status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRDStatus {
    /// R&D active
    Active,
    /// R&D paused
    Paused,
    /// R&D completed
    Completed,
}

/// Financial R&D metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialResearchDevelopmentMetadata {
    /// R&D ID
    pub rd_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// R&D type
    pub rd_type: FinancialRDType,
    /// Status
    pub status: FinancialRDStatus,
    /// Created at
    pub created_at: i64,
    /// R&D data hash
    pub rd_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_research_development(
        rd: &mut FinancialResearchDevelopmentMetadata,
        rd_id: u64,
        entity_id: u64,
        rd_type: FinancialRDType,
        rd_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(rd_id > 0, IndrasError::InvalidInput);
        rd.rd_id = rd_id;
        rd.entity_id = entity_id;
        rd.rd_type = rd_type;
        rd.status = FinancialRDStatus::Active;
        rd.created_at = current_time;
        rd.rd_data_hash = rd_data_hash;
        rd.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_rd(_rd_id: u64) -> Vec<u8> {
        vec![]
    }
}
