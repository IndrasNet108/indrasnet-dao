//! Treasury state modules
//!
//! Treasury management for the DAO:
//! - On-chain: Metadata, balances, capabilities, basic operations
//! - Off-chain: Advanced analytics, reporting, optimization
//!
//! Includes: operations, analytics, risk management, allocation, advanced management

pub mod types;
pub mod events;
pub mod manager;
pub mod advanced_management;
pub mod optimized_operations;
pub mod reporting;
pub mod operations;
pub mod analytics;
pub mod risk;
pub mod allocation;

// Real runtime tests for treasury modules
#[cfg(all(test, feature = "program-test"))]
mod manager_program_test;
#[cfg(all(test, feature = "program-test"))]
mod analytics_program_test;
#[cfg(all(test, feature = "program-test"))]
mod risk_program_test;
#[cfg(all(test, feature = "program-test"))]
mod allocation_program_test;
#[cfg(all(test, feature = "program-test"))]
mod optimized_operations_program_test;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use types::{
    TreasuryOperationType, TreasuryTransactionStatus, TreasuryBalanceType
};
pub use events::{
    TreasuryDepositEvent, TreasuryWithdrawalEvent, TreasuryTransferEvent,
    CapabilityGrantEvent, CapabilityRevokeEvent, TreasuryBalanceUpdateEvent
};
pub use manager::{Treasury, onchain as manager_onchain};
pub use advanced_management::{
    TreasuryAllocationMetadata, TreasuryStrategyMetadata, TreasuryStrategyType,
    onchain as advanced_management_onchain,
};
pub use optimized_operations::{
    TreasuryBatchOperationMetadata, BatchOperationStatus,
    onchain as optimized_operations_onchain,
};
pub use reporting::{
    TreasuryReportMetadata, TreasuryReportType,
    onchain as reporting_onchain,
};
pub use operations::{
    TreasuryOperationsMetadata, TreasuryOperationStatus,
    onchain::initialize_treasury_operations,
};
// TreasuryOperationType is already exported from types
pub use analytics::{
    TreasuryAnalyticsMetadata, TreasuryAnalyticsType, TreasuryAnalyticsStatus,
    onchain::initialize_treasury_analytics,
};
pub use risk::{
    TreasuryRiskManagementMetadata, TreasuryRiskType, TreasuryRiskStatus,
    onchain::initialize_treasury_risk_management,
};
pub use allocation::{
    TreasuryAllocationStrategyMetadata, TreasuryAllocationStrategy, TreasuryAllocationStatus,
    onchain::initialize_treasury_allocation,
};
