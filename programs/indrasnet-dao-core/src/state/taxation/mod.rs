//! Taxation state modules
//!
//! Taxation management for the DAO:
//! - On-chain: Metadata for tax records, compliance, planning, optimization
//! - Off-chain: Actual tax calculations, compliance, planning, optimization
//!
//! Includes: core, compliance, planning, optimization

pub mod core;
pub mod compliance;
pub mod planning;
pub mod optimization;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use core::{
    TaxRecordMetadata, TaxType, TaxRecordStatus,
    onchain::initialize_tax_record,
};
pub use compliance::{
    TaxComplianceMetadata, TaxComplianceRequirement, TaxComplianceStatus,
    onchain::initialize_tax_compliance,
};
pub use planning::{
    TaxPlanningMetadata, TaxPlanningStrategy, TaxPlanningStatus,
    onchain::initialize_tax_planning,
};
pub use optimization::{
    TaxOptimizationMetadata, TaxOptimizationStrategy, TaxOptimizationStatus,
    onchain::initialize_tax_optimization,
};
