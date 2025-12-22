//! Accounting state modules
//!
//! Accounting management for the DAO:
//! - On-chain: Metadata for accounting entries, periods, reconciliation
//! - Off-chain: Actual accounting calculations, period management, reconciliation
//!
//! Includes: core, period, reconciliation

pub mod core;
pub mod period;
pub mod reconciliation;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use core::{
    AccountingEntryMetadata, AccountingEntryType,
    onchain::initialize_accounting_entry,
};
pub use period::{
    AccountingPeriodMetadata, AccountingPeriodType, AccountingPeriodStatus,
    onchain::initialize_accounting_period,
};
pub use reconciliation::{
    AccountingReconciliationMetadata, AccountingReconciliationStatus,
    onchain::initialize_accounting_reconciliation,
};
