//! DeFi Borrowing module
//!
//! Borrowing operations
//!
//! On-chain: Metadata for borrowing
//! Off-chain: Actual borrowing, repayment

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Borrowing type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BorrowingType {
    /// Overcollateralized
    Overcollateralized,
    /// Under-collateralized
    Undercollateralized,
    /// Flash loans
    FlashLoans,
    /// Custom type
    Custom,
}

/// Borrowing status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BorrowingStatus {
    /// Borrowing active
    Active,
    /// Borrowing paused
    Paused,
    /// Borrowing repaid
    Repaid,
}

/// Borrowing metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct BorrowingMetadata {
    /// Borrowing ID
    pub borrowing_id: u64,
    /// Protocol ID
    pub protocol_id: u64,
    /// Borrowing type
    pub borrowing_type: BorrowingType,
    /// Status
    pub status: BorrowingStatus,
    /// Created at
    pub created_at: i64,
    /// Borrowing config hash
    pub borrowing_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_borrowing(
        borrowing: &mut BorrowingMetadata,
        borrowing_id: u64,
        protocol_id: u64,
        borrowing_type: BorrowingType,
        borrowing_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(borrowing_id > 0, IndrasError::InvalidInput);
        borrowing.borrowing_id = borrowing_id;
        borrowing.protocol_id = protocol_id;
        borrowing.borrowing_type = borrowing_type;
        borrowing.status = BorrowingStatus::Active;
        borrowing.created_at = current_time;
        borrowing.borrowing_config_hash = borrowing_config_hash;
        borrowing.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_borrowing(_borrowing_id: u64) -> Vec<u8> {
        vec![]
    }
}
