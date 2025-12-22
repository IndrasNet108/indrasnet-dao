//! Financial Crisis Management module
//!
//! Financial crisis management
//!
//! On-chain: Metadata for crisis management
//! Off-chain: Actual crisis management, response

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Crisis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCrisisType {
    /// Liquidity crisis
    Liquidity,
    /// Solvency crisis
    Solvency,
    /// Market crisis
    Market,
    /// Custom crisis
    Custom,
}

/// Crisis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCrisisStatus {
    /// Crisis pending
    Pending,
    /// Crisis active
    Active,
    /// Crisis resolved
    Resolved,
}

/// Financial crisis management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCrisisManagementMetadata {
    /// Crisis ID
    pub crisis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Crisis type
    pub crisis_type: FinancialCrisisType,
    /// Status
    pub status: FinancialCrisisStatus,
    /// Created at
    pub created_at: i64,
    /// Crisis data hash
    pub crisis_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_crisis_management(
        crisis: &mut FinancialCrisisManagementMetadata,
        crisis_id: u64,
        entity_id: u64,
        crisis_type: FinancialCrisisType,
        crisis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(crisis_id > 0, IndrasError::InvalidInput);
        crisis.crisis_id = crisis_id;
        crisis.entity_id = entity_id;
        crisis.crisis_type = crisis_type;
        crisis.status = FinancialCrisisStatus::Pending;
        crisis.created_at = current_time;
        crisis.crisis_data_hash = crisis_data_hash;
        crisis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_crisis(_crisis_id: u64) -> Vec<u8> {
        vec![]
    }
}
