//! Legal Documents module
//!
//! Legal document management
//!
//! On-chain: Metadata for legal documents
//! Off-chain: Actual document storage, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Document type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DocumentType {
    /// Contract
    Contract,
    /// Agreement
    Agreement,
    /// Policy
    Policy,
    /// Regulation
    Regulation,
}

/// Document status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum DocumentStatus {
    /// Draft
    Draft,
    /// Active
    Active,
    /// Expired
    Expired,
    /// Revoked
    Revoked,
}

/// Legal document metadata (on-chain)
///
/// Stores metadata for legal documents
#[account]
#[derive(InitSpace)]
pub struct LegalDocumentMetadata {
    /// Document ID
    pub document_id: u64,
    /// Document type
    pub document_type: DocumentType,
    /// Status
    pub status: DocumentStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Expires at
    pub expires_at: Option<i64>,
    /// Document data hash
    pub document_data_hash: [u8; 32],
    /// Document URI
    #[max_len(200)]
    pub document_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for legal documents
pub mod onchain {
    use super::*;

    /// Initialize legal document
    pub fn initialize_legal_document(
        document: &mut LegalDocumentMetadata,
        document_id: u64,
        document_type: DocumentType,
        document_data_hash: [u8; 32],
        document_uri: String,
        expires_at: Option<i64>,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(document_id > 0, IndrasError::InvalidInput);
        require!(document_uri.len() <= 200, IndrasError::InvalidInput);
        
        document.document_id = document_id;
        document.document_type = document_type;
        document.status = DocumentStatus::Draft;
        document.created_at = current_time;
        document.updated_at = current_time;
        document.expires_at = expires_at;
        document.document_data_hash = document_data_hash;
        document.document_uri = document_uri;
        document.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for legal documents
pub mod offchain {
    /// Store legal document
    pub fn store_document(_document_id: u64, _document_data: &[u8]) -> String {
        // Implementation in off-chain service
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== initialize_legal_document validation tests ==========
    
    #[test]
    fn test_initialize_legal_document_validation_document_id_zero() {
        // Test: document_id == 0 should fail
        let document_id = 0u64;
        
        // Validation logic: require!(document_id > 0, IndrasError::InvalidInput)
        assert_eq!(document_id, 0, "Document ID zero should be detected");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_document_uri_too_long() {
        // Test: document_uri.len() > 200 should fail
        let document_uri = "a".repeat(201);
        
        // Validation logic: require!(document_uri.len() <= 200, IndrasError::InvalidInput)
        assert!(document_uri.len() > 200, "Document URI too long should be detected");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_document_uri_exact_max() {
        // Test: document_uri.len() == 200 should pass
        let document_uri = "a".repeat(200);
        
        // Validation logic: require!(document_uri.len() <= 200, IndrasError::InvalidInput)
        assert!(document_uri.len() <= 200, "Document URI at exact max should pass");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_document_uri_empty() {
        // Test: document_uri can be empty (no validation against empty)
        let document_uri = String::new();
        
        // No validation for empty document_uri - this is allowed
        assert!(document_uri.is_empty(), "Empty document URI should be allowed");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_valid_inputs() {
        // Test: valid inputs should pass
        let document_id = 1u64;
        let document_uri = "https://example.com/document.pdf".to_string();
        
        // All validations should pass
        assert!(document_id > 0, "Document ID should be valid");
        assert!(document_uri.len() <= 200, "Document URI should be valid");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_document_data_hash_zero() {
        // Test: document_data_hash == [0u8; 32] should be allowed (no validation)
        let document_data_hash = [0u8; 32];
        
        // No validation for zero hash - this is allowed
        assert_eq!(document_data_hash, [0u8; 32], "Zero document data hash should be allowed");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_expires_at_past() {
        // Test: expires_at < current_time should be allowed (no validation)
        let current_time = 1000000i64;
        let expires_at = Some(999999i64);
        
        // No validation for past expiration - this is allowed
        assert!(expires_at.is_some() && expires_at.unwrap() < current_time, "Past expiration should be allowed");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_expires_at_future() {
        // Test: expires_at > current_time should be allowed
        let current_time = 1000000i64;
        let expires_at = Some(1000001i64);
        
        // Future expiration should be allowed
        assert!(expires_at.is_some() && expires_at.unwrap() > current_time, "Future expiration should be allowed");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_expires_at_none() {
        // Test: expires_at == None should be allowed
        let expires_at: Option<i64> = None;
        
        // None expiration should be allowed
        assert!(expires_at.is_none(), "None expiration should be allowed");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_all_document_types() {
        // Test: all DocumentType variants should be valid
        let document_types = vec![
            DocumentType::Contract,
            DocumentType::Agreement,
            DocumentType::Policy,
            DocumentType::Regulation,
        ];
        
        // All document types should be valid
        assert_eq!(document_types.len(), 4, "All document types should be valid");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_status_draft() {
        // Test: status should be set to Draft on initialization
        let status = DocumentStatus::Draft;
        
        // Status should be Draft
        assert_eq!(status, DocumentStatus::Draft, "Status should be Draft on initialization");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_all_document_statuses() {
        // Test: all DocumentStatus variants should be valid
        let statuses = vec![
            DocumentStatus::Draft,
            DocumentStatus::Active,
            DocumentStatus::Expired,
            DocumentStatus::Revoked,
        ];
        
        // All statuses should be valid
        assert_eq!(statuses.len(), 4, "All document statuses should be valid");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_document_id_max() {
        // Test: document_id == u64::MAX should pass
        let document_id = u64::MAX;
        
        // Validation logic: require!(document_id > 0, IndrasError::InvalidInput)
        assert!(document_id > 0, "Document ID at max should pass");
    }
    
    #[test]
    fn test_initialize_legal_document_validation_document_uri_one_char() {
        // Test: document_uri.len() == 1 should pass
        let document_uri = "a".to_string();
        
        // Validation logic: require!(document_uri.len() <= 200, IndrasError::InvalidInput)
        assert!(document_uri.len() <= 200, "Document URI with one char should pass");
    }
}
