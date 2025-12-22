//! Payment state modules
//!
//! Payment and invoice management for the DAO:
//! - On-chain: Metadata for payments, reconciliation, invoices, invoice processing
//! - Off-chain: Actual payment processing, reconciliation, invoice management, processing
//!
//! Includes: processing, reconciliation, invoice_management, invoice_processing

pub mod processing;
pub mod reconciliation;
pub mod invoice_management;
pub mod invoice_processing;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use processing::{
    PaymentMetadata, PaymentMethod, PaymentStatus,
    onchain::initialize_payment,
};
pub use reconciliation::{
    PaymentReconciliationMetadata, PaymentReconciliationStatus,
    onchain::initialize_payment_reconciliation,
};
pub use invoice_management::{
    InvoiceMetadata, InvoiceStatus,
    onchain::initialize_invoice,
};
pub use invoice_processing::{
    InvoiceProcessingMetadata, InvoiceProcessingStatus,
    onchain::initialize_invoice_processing,
};
