//! Financial Disclosure module
//!
//! Financial disclosure management
//!
//! On-chain: Metadata for financial disclosures
//! Off-chain: Actual disclosure, publication

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Disclosure type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDisclosureType {
    /// Public disclosure
    Public,
    /// Regulated disclosure
    Regulated,
    /// Voluntary disclosure
    Voluntary,
    /// Custom disclosure
    Custom,
}

/// Disclosure status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDisclosureStatus {
    /// Disclosure draft
    Draft,
    /// Disclosure published
    Published,
    /// Disclosure archived
    Archived,
}

/// Financial disclosure metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDisclosureMetadata {
    /// Disclosure ID
    pub disclosure_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Disclosure type
    pub disclosure_type: FinancialDisclosureType,
    /// Status
    pub status: FinancialDisclosureStatus,
    /// Created at
    pub created_at: i64,
    /// Disclosure data hash
    pub disclosure_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_disclosure(
        disclosure: &mut FinancialDisclosureMetadata,
        disclosure_id: u64,
        entity_id: u64,
        disclosure_type: FinancialDisclosureType,
        disclosure_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(disclosure_id > 0, IndrasError::InvalidInput);
        disclosure.disclosure_id = disclosure_id;
        disclosure.entity_id = entity_id;
        disclosure.disclosure_type = disclosure_type;
        disclosure.status = FinancialDisclosureStatus::Draft;
        disclosure.created_at = current_time;
        disclosure.disclosure_data_hash = disclosure_data_hash;
        disclosure.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn publish_disclosure(_disclosure_id: u64) -> Vec<u8> {
        vec![]
    }
}
