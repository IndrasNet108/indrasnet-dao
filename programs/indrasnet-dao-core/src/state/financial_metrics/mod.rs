//! Financial Metrics state modules
//!
//! Financial metrics and dashboard for the DAO:
//! - On-chain: Metadata for ratios, benchmarking, KPIs, dashboard
//! - Off-chain: Actual calculation, benchmarking, tracking, visualization
//!
//! Includes: ratios, benchmarking, kpis, dashboard

pub mod ratios;
pub mod benchmarking;
pub mod kpis;
pub mod dashboard;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use ratios::{
    FinancialRatiosMetadata, FinancialRatioType, FinancialRatioStatus,
    onchain::initialize_financial_ratios,
};
pub use benchmarking::{
    FinancialBenchmarkingMetadata, FinancialBenchmarkType, FinancialBenchmarkStatus,
    onchain::initialize_financial_benchmarking,
};
pub use kpis::{
    FinancialKPIsMetadata, FinancialKPICategory, FinancialKPIStatus,
    onchain::initialize_financial_kpis,
};
pub use dashboard::{
    FinancialDashboardMetadata, FinancialDashboardType, FinancialDashboardStatus,
    onchain::initialize_financial_dashboard,
};
