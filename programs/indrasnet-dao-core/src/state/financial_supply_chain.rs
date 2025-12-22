//! Financial Supply Chain module
//!
//! Financial supply chain management
//!
//! On-chain: Metadata for supply chain
//! Off-chain: Actual supply chain, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Supply chain stage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSupplyChainStage {
    /// Planning
    Planning,
    /// Sourcing
    Sourcing,
    /// Manufacturing
    Manufacturing,
    /// Distribution
    Distribution,
}

/// Supply chain status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSupplyChainStatus {
    /// Supply chain active
    Active,
    /// Supply chain paused
    Paused,
    /// Supply chain optimized
    Optimized,
}

/// Financial supply chain metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialSupplyChainMetadata {
    /// Supply chain ID
    pub supply_chain_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Supply chain stage
    pub supply_chain_stage: FinancialSupplyChainStage,
    /// Status
    pub status: FinancialSupplyChainStatus,
    /// Created at
    pub created_at: i64,
    /// Supply chain config hash
    pub supply_chain_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_supply_chain(
        supply_chain: &mut FinancialSupplyChainMetadata,
        supply_chain_id: u64,
        entity_id: u64,
        supply_chain_stage: FinancialSupplyChainStage,
        supply_chain_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(supply_chain_id > 0, IndrasError::InvalidInput);
        supply_chain.supply_chain_id = supply_chain_id;
        supply_chain.entity_id = entity_id;
        supply_chain.supply_chain_stage = supply_chain_stage;
        supply_chain.status = FinancialSupplyChainStatus::Active;
        supply_chain.created_at = current_time;
        supply_chain.supply_chain_config_hash = supply_chain_config_hash;
        supply_chain.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_supply_chain(_supply_chain_id: u64) -> Vec<u8> {
        vec![]
    }
}
