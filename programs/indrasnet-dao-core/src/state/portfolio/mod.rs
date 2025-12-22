//! Portfolio state modules
//!
//! Portfolio management for the DAO:
//! - On-chain: Metadata for portfolios, optimization, risk management, diversification
//! - Off-chain: Actual portfolio management, optimization, risk analysis, diversification
//!
//! Includes: core, optimization, risk_management, diversification

pub mod core;
pub mod optimization;
pub mod risk_management;
pub mod diversification;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use core::{
    PortfolioMetadata, PortfolioStatus,
    onchain::initialize_portfolio,
};
pub use optimization::{
    PortfolioOptimizationMetadata, PortfolioOptimizationMethod, PortfolioOptimizationStatus,
    onchain::initialize_portfolio_optimization,
};
pub use risk_management::{
    PortfolioRiskManagementMetadata, PortfolioRiskManagementStrategy, PortfolioRiskManagementStatus,
    onchain::initialize_portfolio_risk_management,
};
pub use diversification::{
    PortfolioDiversificationMetadata, PortfolioDiversificationStrategy, PortfolioDiversificationStatus,
    onchain::initialize_portfolio_diversification,
};
