//! Documents module
//!
//! Partnership document management
//!
//! On-chain: Metadata for documents
//! Off-chain: Actual document storage, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Document type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipDocumentType {
    /// Agreement document
    Agreement,
    /// Contract document
    Contract,
    /// Proposal document
    Proposal,
    /// Report document
    Report,
}

/// Document status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DocumentStatus {
    /// Document draft
    Draft,
    /// Document active
    Active,
    /// Document archived
    Archived,
}

/// Partnership document metadata (on-chain)
///
/// Stores metadata for partnership documents
#[account]
#[derive(InitSpace)]
pub struct PartnershipDocumentMetadata {
    /// Document ID
    pub document_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Document type
    pub document_type: PartnershipDocumentType,
    /// Status
    pub status: DocumentStatus,
    /// Created at
    pub created_at: i64,
    /// Document data hash
    pub document_data_hash: [u8; 32],
    /// Document URI
    #[max_len(200)]
    pub document_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for documents
pub mod onchain {
    use super::*;

    /// Initialize partnership document
    pub fn initialize_partnership_document(
        document: &mut PartnershipDocumentMetadata,
        document_id: u64,
        partnership_id: u64,
        document_type: PartnershipDocumentType,
        document_data_hash: [u8; 32],
        document_uri: String,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(document_id > 0, IndrasError::InvalidInput);
        require!(document_uri.len() <= 200, IndrasError::InvalidInput);
        
        document.document_id = document_id;
        document.partnership_id = partnership_id;
        document.document_type = document_type;
        document.status = DocumentStatus::Draft;
        document.created_at = current_time;
        document.document_data_hash = document_data_hash;
        document.document_uri = document_uri;
        document.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for documents
pub mod offchain {
    /// Store document
    pub fn store_document(_document_id: u64, _document_data: &[u8]) -> String {
        // Implementation in off-chain service
        String::new()
    }
}
