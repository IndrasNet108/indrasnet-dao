//! Financial Reporting state modules
//!
//! Financial reporting management for the DAO:
//! - On-chain: Metadata for financial reports
//! - Off-chain: Actual report generation, analysis
//!
//! Includes: core, esg, compliance, risk, regulatory, management, segment

pub mod core;
pub mod esg;
pub mod compliance;
pub mod risk;
pub mod regulatory;
pub mod management;
pub mod segment;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use core::{
    FinancialReportMetadata, FinancialReportType, FinancialReportStatus,
    onchain::initialize_financial_report,
};
pub use esg::{
    FinancialESGReportingMetadata, FinancialESGDimension, FinancialESGReportStatus,
    onchain::initialize_financial_esg_reporting,
};
pub use compliance::{
    FinancialComplianceReportingMetadata, FinancialComplianceFramework, FinancialComplianceReportStatus,
    onchain::initialize_financial_compliance_reporting,
};
pub use risk::{
    FinancialRiskReportingMetadata, FinancialRiskReportType, FinancialRiskReportStatus,
    onchain::initialize_financial_risk_reporting,
};
pub use regulatory::{
    FinancialRegulatoryReportingMetadata, FinancialRegulatoryRequirement, FinancialRegulatoryReportStatus,
    onchain::initialize_financial_regulatory_reporting,
};
pub use management::{
    FinancialManagementReportingMetadata, FinancialManagementReportType, FinancialManagementReportStatus,
    onchain::initialize_financial_management_reporting,
};
pub use segment::{
    FinancialSegmentReportingMetadata, FinancialSegmentType, FinancialSegmentReportingStatus,
    onchain::initialize_financial_segment_reporting,
};
