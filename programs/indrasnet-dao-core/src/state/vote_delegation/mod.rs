//! Vote Delegation State
//!
//! This module defines the state structures for vote delegation functionality.
//! Split into submodules:
//! - lifecycle: delegation lifecycle methods (new, update_weight, deactivate, reactivate)

use anchor_lang::prelude::*;

pub mod lifecycle;

/// Vote delegation account
#[account]
#[derive(InitSpace)]
pub struct VoteDelegation {
    pub delegator: Pubkey,        // Who is delegating
    pub delegate: Pubkey,         // Who receives the delegation
    pub weight: u64,              // Amount of voting power delegated
    pub created_at: i64,          // When delegation was created
    pub updated_at: i64,          // When delegation was last updated
    pub is_active: bool,          // Whether delegation is active
    /// Expiration timestamp - delegation will be auto-deactivated after this time
    /// None means delegation never expires
    pub expires_at: Option<i64>,
    pub bump: u8,                 // PDA bump
}
