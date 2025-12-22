//! Financial Claims Management module
//!
//! Financial claims management
//!
//! On-chain: Metadata for claims
//! Off-chain: Actual claims, processing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Claim type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialClaimType {
    /// Insurance claim
    Insurance,
    /// Warranty claim
    Warranty,
    /// Refund claim
    Refund,
    /// Custom claim
    Custom,
}

/// Claim status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialClaimStatus {
    /// Claim pending
    Pending,
    /// Claim in progress
    InProgress,
    /// Claim resolved
    Resolved,
}

/// Financial claims management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialClaimsManagementMetadata {
    /// Claim ID
    pub claim_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Claim type
    pub claim_type: FinancialClaimType,
    /// Status
    pub status: FinancialClaimStatus,
    /// Created at
    pub created_at: i64,
    /// Claim data hash
    pub claim_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_claims_management(
        claim: &mut FinancialClaimsManagementMetadata,
        claim_id: u64,
        entity_id: u64,
        claim_type: FinancialClaimType,
        claim_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(claim_id > 0, IndrasError::InvalidInput);
        claim.claim_id = claim_id;
        claim.entity_id = entity_id;
        claim.claim_type = claim_type;
        claim.status = FinancialClaimStatus::Pending;
        claim.created_at = current_time;
        claim.claim_data_hash = claim_data_hash;
        claim.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn process_claim(_claim_id: u64) -> Vec<u8> {
        vec![]
    }
}
