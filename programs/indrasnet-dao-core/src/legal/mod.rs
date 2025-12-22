//! Legal modules
//!
//! Legal compliance and documentation management
//!
//! On-chain: Metadata for legal documents, compliance status
//! Off-chain: Actual document management, compliance checking

pub mod compliance;
pub mod documents;
pub mod frameworks;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use compliance::{
    ComplianceMetadata, ComplianceStatus, ComplianceType,
    onchain as compliance_onchain,
};
pub use documents::{
    LegalDocumentMetadata, DocumentType, DocumentStatus,
    onchain as documents_onchain,
};
pub use frameworks::{
    LegalFrameworkMetadata, FrameworkType,
    onchain as frameworks_onchain,
};
