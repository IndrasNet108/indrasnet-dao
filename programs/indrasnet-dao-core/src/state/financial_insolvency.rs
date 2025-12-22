//! Financial Insolvency module
//!
//! Financial insolvency management
//!
//! On-chain: Metadata for insolvency
//! Off-chain: Actual insolvency, process management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Insolvency type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInsolvencyType {
    /// Cash flow insolvency
    CashFlow,
    /// Balance sheet insolvency
    BalanceSheet,
    /// Technical insolvency
    Technical,
    /// Custom type
    Custom,
}

/// Insolvency status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInsolvencyStatus {
    /// Insolvency pending
    Pending,
    /// Insolvency in progress
    InProgress,
    /// Insolvency resolved
    Resolved,
}

/// Financial insolvency metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialInsolvencyMetadata {
    /// Insolvency ID
    pub insolvency_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Insolvency type
    pub insolvency_type: FinancialInsolvencyType,
    /// Status
    pub status: FinancialInsolvencyStatus,
    /// Created at
    pub created_at: i64,
    /// Insolvency data hash
    pub insolvency_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_insolvency(
        insolvency: &mut FinancialInsolvencyMetadata,
        insolvency_id: u64,
        entity_id: u64,
        insolvency_type: FinancialInsolvencyType,
        insolvency_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(insolvency_id > 0, IndrasError::InvalidInput);
        insolvency.insolvency_id = insolvency_id;
        insolvency.entity_id = entity_id;
        insolvency.insolvency_type = insolvency_type;
        insolvency.status = FinancialInsolvencyStatus::Pending;
        insolvency.created_at = current_time;
        insolvency.insolvency_data_hash = insolvency_data_hash;
        insolvency.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_insolvency(_insolvency_id: u64) -> Vec<u8> {
        vec![]
    }
}
