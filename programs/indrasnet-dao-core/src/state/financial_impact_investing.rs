//! Financial Impact Investing module
//!
//! Financial impact investing
//!
//! On-chain: Metadata for impact investing
//! Off-chain: Actual investing, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Impact investment strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialImpactInvestmentStrategy {
    /// Thematic investing
    Thematic,
    /// ESG integration
    ESGIntegration,
    /// Impact-first
    ImpactFirst,
    /// Custom strategy
    Custom,
}

/// Investment status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialImpactInvestmentStatus {
    /// Investment active
    Active,
    /// Investment paused
    Paused,
    /// Investment completed
    Completed,
}

/// Financial impact investing metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialImpactInvestingMetadata {
    /// Investment ID
    pub investment_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Impact investment strategy
    pub impact_investment_strategy: FinancialImpactInvestmentStrategy,
    /// Status
    pub status: FinancialImpactInvestmentStatus,
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
    pub fn initialize_financial_impact_investing(
        investment: &mut FinancialImpactInvestingMetadata,
        investment_id: u64,
        entity_id: u64,
        impact_investment_strategy: FinancialImpactInvestmentStrategy,
        investment_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(investment_id > 0, IndrasError::InvalidInput);
        investment.investment_id = investment_id;
        investment.entity_id = entity_id;
        investment.impact_investment_strategy = impact_investment_strategy;
        investment.status = FinancialImpactInvestmentStatus::Active;
        investment.created_at = current_time;
        investment.investment_data_hash = investment_data_hash;
        investment.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_impact_investment(_investment_id: u64) -> Vec<u8> {
        vec![]
    }
}
