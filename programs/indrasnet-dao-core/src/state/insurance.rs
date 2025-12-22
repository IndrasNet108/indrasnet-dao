//! Insurance module
//!
//! Insurance management
//!
//! On-chain: Metadata for insurance policies
//! Off-chain: Actual insurance calculations, claims processing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Insurance policy status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InsurancePolicyStatus {
    /// Policy active
    Active,
    /// Policy expired
    Expired,
    /// Policy cancelled
    Cancelled,
}

/// Insurance policy metadata (on-chain)
///
/// Stores metadata for insurance policies
#[account]
#[derive(InitSpace)]
pub struct InsurancePolicyMetadata {
    /// Policy ID
    pub policy_id: u64,
    /// Insured pubkey
    pub insured_pubkey: Pubkey,
    /// Coverage amount (in smallest unit)
    pub coverage_amount: u64,
    /// Premium amount (in smallest unit)
    pub premium_amount: u64,
    /// Status
    pub status: InsurancePolicyStatus,
    /// Created at
    pub created_at: i64,
    /// Expires at
    pub expires_at: i64,
    /// Policy data hash
    pub policy_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for insurance
pub mod onchain {
    use super::*;

    /// Initialize insurance policy
    pub fn initialize_insurance_policy(
        policy: &mut InsurancePolicyMetadata,
        policy_id: u64,
        insured_pubkey: Pubkey,
        coverage_amount: u64,
        premium_amount: u64,
        policy_data_hash: [u8; 32],
        expires_at: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(policy_id > 0, IndrasError::InvalidInput);
        require!(coverage_amount > 0, IndrasError::InvalidInput);
        require!(expires_at > current_time, IndrasError::InvalidInput);
        
        policy.policy_id = policy_id;
        policy.insured_pubkey = insured_pubkey;
        policy.coverage_amount = coverage_amount;
        policy.premium_amount = premium_amount;
        policy.status = InsurancePolicyStatus::Active;
        policy.created_at = current_time;
        policy.expires_at = expires_at;
        policy.policy_data_hash = policy_data_hash;
        policy.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for insurance
pub mod offchain {
    /// Process insurance claim
    pub fn process_claim(_policy_id: u64, _claim_amount: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
