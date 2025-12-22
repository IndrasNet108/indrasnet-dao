//! Loans module
//!
//! Loan management
//!
//! On-chain: Metadata for loans
//! Off-chain: Actual loan calculations, repayments

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Loan status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LoanStatus {
    /// Loan active
    Active,
    /// Loan repaid
    Repaid,
    /// Loan defaulted
    Defaulted,
}

/// Loan metadata (on-chain)
///
/// Stores metadata for loans
#[account]
#[derive(InitSpace)]
pub struct LoanMetadata {
    /// Loan ID
    pub loan_id: u64,
    /// Borrower pubkey
    pub borrower_pubkey: Pubkey,
    /// Lender pubkey
    pub lender_pubkey: Pubkey,
    /// Principal amount (in smallest unit)
    pub principal: u64,
    /// Interest rate (scaled)
    pub interest_rate: u64,
    /// Status
    pub status: LoanStatus,
    /// Created at
    pub created_at: i64,
    /// Maturity date
    pub maturity_date: i64,
    /// Loan data hash
    pub loan_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for loans
pub mod onchain {
    use super::*;

    /// Initialize loan
    pub fn initialize_loan(
        loan: &mut LoanMetadata,
        loan_id: u64,
        borrower_pubkey: Pubkey,
        lender_pubkey: Pubkey,
        principal: u64,
        interest_rate: u64,
        loan_data_hash: [u8; 32],
        maturity_date: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(loan_id > 0, IndrasError::InvalidInput);
        require!(principal > 0, IndrasError::InvalidInput);
        require!(maturity_date > current_time, IndrasError::InvalidInput);
        
        loan.loan_id = loan_id;
        loan.borrower_pubkey = borrower_pubkey;
        loan.lender_pubkey = lender_pubkey;
        loan.principal = principal;
        loan.interest_rate = interest_rate;
        loan.status = LoanStatus::Active;
        loan.created_at = current_time;
        loan.maturity_date = maturity_date;
        loan.loan_data_hash = loan_data_hash;
        loan.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for loans
pub mod offchain {
    /// Calculate loan payment
    pub fn calculate_loan_payment(_loan_id: u64, _current_time: i64) -> u64 {
        // Implementation in off-chain service
        0
    }
}
