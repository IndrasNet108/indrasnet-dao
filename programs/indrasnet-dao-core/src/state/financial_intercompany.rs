//! Financial Intercompany module
//!
//! Financial intercompany transactions
//!
//! On-chain: Metadata for intercompany transactions
//! Off-chain: Actual transactions, reconciliation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Transaction type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIntercompanyTransactionType {
    /// Loan
    Loan,
    /// Sale
    Sale,
    /// Service
    Service,
    /// Custom transaction
    Custom,
}

/// Transaction status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIntercompanyTransactionStatus {
    /// Transaction pending
    Pending,
    /// Transaction completed
    Completed,
    /// Transaction reconciled
    Reconciled,
}

/// Financial intercompany metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialIntercompanyMetadata {
    /// Transaction ID
    pub transaction_id: u64,
    /// From entity ID
    pub from_entity_id: u64,
    /// To entity ID
    pub to_entity_id: u64,
    /// Transaction type
    pub transaction_type: FinancialIntercompanyTransactionType,
    /// Status
    pub status: FinancialIntercompanyTransactionStatus,
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
    pub fn initialize_financial_intercompany(
        transaction: &mut FinancialIntercompanyMetadata,
        transaction_id: u64,
        from_entity_id: u64,
        to_entity_id: u64,
        transaction_type: FinancialIntercompanyTransactionType,
        transaction_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(transaction_id > 0, IndrasError::InvalidInput);
        require!(from_entity_id != to_entity_id, IndrasError::InvalidInput);
        transaction.transaction_id = transaction_id;
        transaction.from_entity_id = from_entity_id;
        transaction.to_entity_id = to_entity_id;
        transaction.transaction_type = transaction_type;
        transaction.status = FinancialIntercompanyTransactionStatus::Pending;
        transaction.created_at = current_time;
        transaction.transaction_data_hash = transaction_data_hash;
        transaction.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn process_intercompany_transaction(_transaction_id: u64) -> Vec<u8> {
        vec![]
    }
}
