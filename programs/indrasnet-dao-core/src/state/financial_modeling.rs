//! Financial Modeling module
//!
//! Financial modeling and simulation
//!
//! On-chain: Metadata for financial models
//! Off-chain: Actual modeling, simulation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Model type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialModelType {
    /// DCF model
    DCF,
    /// Monte Carlo simulation
    MonteCarlo,
    /// Scenario analysis
    ScenarioAnalysis,
    /// Custom model
    Custom,
}

/// Model status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialModelStatus {
    /// Model draft
    Draft,
    /// Model active
    Active,
    /// Model archived
    Archived,
}

/// Financial modeling metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialModelingMetadata {
    /// Model ID
    pub model_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Model type
    pub model_type: FinancialModelType,
    /// Status
    pub status: FinancialModelStatus,
    /// Created at
    pub created_at: i64,
    /// Model config hash
    pub model_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_modeling(
        model: &mut FinancialModelingMetadata,
        model_id: u64,
        entity_id: u64,
        model_type: FinancialModelType,
        model_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(model_id > 0, IndrasError::InvalidInput);
        model.model_id = model_id;
        model.entity_id = entity_id;
        model.model_type = model_type;
        model.status = FinancialModelStatus::Draft;
        model.created_at = current_time;
        model.model_config_hash = model_config_hash;
        model.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn run_financial_model(_model_id: u64) -> Vec<u8> {
        vec![]
    }
}
