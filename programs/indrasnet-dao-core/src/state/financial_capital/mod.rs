//! Financial Capital Management state modules
//!
//! Financial capital management for the DAO:
//! - On-chain: Metadata for capital management, structure, working capital, cash, debt, equity, investment
//! - Off-chain: Actual management, optimization
//!
//! Includes: core, structure, working_capital, cash_management, debt_management, equity_management, investment_management

pub mod core;
pub mod structure;
pub mod working_capital;
pub mod cash_management;
pub mod debt_management;
pub mod equity_management;
pub mod investment_management;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use core::{
    CapitalManagementMetadata, CapitalManagementStrategy, CapitalManagementStatus,
    onchain::initialize_capital_management,
};
pub use structure::{
    FinancialCapitalStructureMetadata, FinancialCapitalComponent, FinancialCapitalStructureStatus,
    onchain::initialize_financial_capital_structure,
};
pub use working_capital::{
    FinancialWorkingCapitalMetadata, FinancialWorkingCapitalComponent, FinancialWorkingCapitalStatus,
    onchain::initialize_financial_working_capital,
};
pub use cash_management::{
    FinancialCashManagementMetadata, FinancialCashManagementStrategy, FinancialCashManagementStatus,
    onchain::initialize_financial_cash_management,
};
pub use debt_management::{
    FinancialDebtManagementMetadata, FinancialDebtType, FinancialDebtManagementStatus,
    onchain::initialize_financial_debt_management,
};
pub use equity_management::{
    FinancialEquityManagementMetadata, FinancialEquityType, FinancialEquityManagementStatus,
    onchain::initialize_financial_equity_management,
};
pub use investment_management::{
    FinancialInvestmentManagementMetadata, FinancialInvestmentStrategy, FinancialInvestmentManagementStatus,
    onchain::initialize_financial_investment_management,
};
