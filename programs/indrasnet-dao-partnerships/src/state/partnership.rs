//! Partnership state structure
//!
//! Main account structure for storing partnership data on-chain

use anchor_lang::prelude::*;
use crate::partnerships::types::*;

/// Partnership metadata (on-chain)
///
/// Stores partnership metadata and state on-chain.
/// Detailed partnership documents, analytics, reporting happen off-chain.
#[account]
#[derive(InitSpace)]
pub struct PartnershipMetadata {
    /// Unique partnership ID
    pub partnership_id: u64,
    /// Partner organization name
    #[max_len(200)]
    pub partner_name: String,
    /// Type of partnership
    pub partnership_type: PartnershipType,
    /// Partnership tier
    pub tier: PartnershipTier,
    /// Partnership description
    #[max_len(1000)]
    pub description: String,
    /// Partnership terms
    pub terms: PartnershipTerms,
    /// Current status
    pub status: PartnershipStatus,
    /// Partner address
    pub partner_address: Pubkey,
    /// Creator/authority
    pub creator: Pubkey,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Version number for updates
    pub version: u64,
    /// Partnership metadata URI (IPFS or similar)
    #[max_len(500)]
    pub metadata_uri: String,
    /// Bump seed for PDA
    pub bump: u8,
}

impl PartnershipMetadata {
    /// Update partnership status
    pub fn update_status(&mut self, new_status: PartnershipStatus, current_time: i64) {
        self.status = new_status;
        self.updated_at = current_time;
        self.version = self.version.saturating_add(1);
    }

    /// Update partnership terms
    pub fn update_terms(&mut self, new_terms: PartnershipTerms, current_time: i64) {
        self.terms = new_terms;
        self.updated_at = current_time;
        self.version = self.version.saturating_add(1);
    }
}
