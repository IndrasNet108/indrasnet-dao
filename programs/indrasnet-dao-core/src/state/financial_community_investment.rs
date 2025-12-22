//! Financial Community Investment module
//!
//! Financial community investment
//!
//! On-chain: Metadata for community investment
//! Off-chain: Actual investment, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Investment focus
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCommunityInvestmentFocus {
    /// Education
    Education,
    /// Healthcare
    Healthcare,
    /// Economic development
    EconomicDevelopment,
    /// Custom focus
    Custom,
}

/// Investment status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCommunityInvestmentStatus {
    /// Investment active
    Active,
    /// Investment paused
    Paused,
    /// Investment completed
    Completed,
}

/// Financial community investment metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCommunityInvestmentMetadata {
    /// Investment ID
    pub investment_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Investment focus
    pub investment_focus: FinancialCommunityInvestmentFocus,
    /// Status
    pub status: FinancialCommunityInvestmentStatus,
    /// Created at
    pub created_at: i64,
    /// Investment data hash
    pub investment_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_community_investment(
        investment: &mut FinancialCommunityInvestmentMetadata,
        investment_id: u64,
        entity_id: u64,
        investment_focus: FinancialCommunityInvestmentFocus,
        investment_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(investment_id > 0, IndrasError::InvalidInput);
        investment.investment_id = investment_id;
        investment.entity_id = entity_id;
        investment.investment_focus = investment_focus;
        investment.status = FinancialCommunityInvestmentStatus::Active;
        investment.created_at = current_time;
        investment.investment_data_hash = investment_data_hash;
        investment.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_community_investment(_investment_id: u64) -> Vec<u8> {
        vec![]
    }
}
