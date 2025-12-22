//! Financial Merger & Acquisition module
//!
//! Financial M&A
//!
//! On-chain: Metadata for M&A
//! Off-chain: Actual M&A, transaction management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Transaction type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialMATransactionType {
    /// Merger
    Merger,
    /// Acquisition
    Acquisition,
    /// Divestiture
    Divestiture,
    /// Custom transaction
    Custom,
}

/// Transaction status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialMATransactionStatus {
    /// Transaction pending
    Pending,
    /// Transaction in progress
    InProgress,
    /// Transaction completed
    Completed,
}

/// Financial M&A metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialMergerAcquisitionMetadata {
    /// Transaction ID
    pub transaction_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Transaction type
    pub transaction_type: FinancialMATransactionType,
    /// Status
    pub status: FinancialMATransactionStatus,
    /// Created at
    pub created_at: i64,
    /// Transaction data hash
    pub transaction_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_merger_acquisition(
        transaction: &mut FinancialMergerAcquisitionMetadata,
        transaction_id: u64,
        entity_id: u64,
        transaction_type: FinancialMATransactionType,
        transaction_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(transaction_id > 0, IndrasError::InvalidInput);
        transaction.transaction_id = transaction_id;
        transaction.entity_id = entity_id;
        transaction.transaction_type = transaction_type;
        transaction.status = FinancialMATransactionStatus::Pending;
        transaction.created_at = current_time;
        transaction.transaction_data_hash = transaction_data_hash;
        transaction.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_ma_transaction(_transaction_id: u64) -> Vec<u8> {
        vec![]
    }
}
