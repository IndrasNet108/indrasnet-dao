//! Reviews module
//!
//! Partnership reviews and ratings
//!
//! On-chain: Metadata for reviews, ratings
//! Off-chain: Actual review processing, aggregation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Review status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ReviewStatus {
    /// Review pending
    Pending,
    /// Review published
    Published,
    /// Review rejected
    Rejected,
}

/// Partnership review metadata (on-chain)
///
/// Stores metadata for partnership reviews
#[account]
#[derive(InitSpace)]
pub struct PartnershipReviewMetadata {
    /// Review ID
    pub review_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Reviewer pubkey
    pub reviewer_pubkey: Pubkey,
    /// Rating (1-5)
    pub rating: u8,
    /// Status
    pub status: ReviewStatus,
    /// Created at
    pub created_at: i64,
    /// Review data hash
    pub review_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for reviews
pub mod onchain {
    use super::*;

    /// Initialize partnership review
    pub fn initialize_partnership_review(
        review: &mut PartnershipReviewMetadata,
        review_id: u64,
        partnership_id: u64,
        reviewer_pubkey: Pubkey,
        rating: u8,
        review_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(review_id > 0, IndrasError::InvalidInput);
        require!(rating >= 1 && rating <= 5, IndrasError::InvalidInput);
        
        review.review_id = review_id;
        review.partnership_id = partnership_id;
        review.reviewer_pubkey = reviewer_pubkey;
        review.rating = rating;
        review.status = ReviewStatus::Pending;
        review.created_at = current_time;
        review.review_data_hash = review_data_hash;
        review.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for reviews
pub mod offchain {
    /// Process review
    pub fn process_review(_review_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
