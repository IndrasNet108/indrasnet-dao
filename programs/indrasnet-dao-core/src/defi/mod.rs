//! DeFi modules
//!
//! DeFi functionality for the DAO:
//! - On-chain: Metadata, basic operations, state management
//! - Off-chain: Actual DEX interactions, oracle data, analytics

pub mod dex_cpi;
pub mod liquidity_dashboard;
pub mod oracles;
pub mod risk_alerts;
pub mod yield_farming;
pub mod lending;
pub mod borrowing;
pub mod amm;
pub mod liquidity_provision;
pub mod arbitrage;
pub mod flash_loans;
pub mod defi_analytics;
pub mod defi_risk_management;
pub mod defi_governance;
pub mod defi_compliance;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use dex_cpi::{
    DEXOperationMetadata, DEXOperationType, DEXOperationStatus,
    onchain as dex_cpi_onchain,
};
pub use liquidity_dashboard::{
    LiquidityPoolMetadata, LiquidityMetric,
    onchain as liquidity_dashboard_onchain,
};
pub use oracles::{
    OracleDataMetadata, OracleSource, OracleDataStatus,
    onchain as oracles_onchain,
};
pub use risk_alerts::{
    RiskAlertMetadata, RiskAlertLevel, RiskAlertStatus,
    onchain as risk_alerts_onchain,
};
pub use yield_farming::{
    YieldFarmingMetadata, YieldFarmingStrategy, YieldFarmingStatus,
    onchain as yield_farming_onchain,
};
pub use lending::{
    LendingMetadata, LendingType, LendingStatus,
    onchain as lending_onchain,
};
pub use borrowing::{
    BorrowingMetadata, BorrowingType, BorrowingStatus,
    onchain as borrowing_onchain,
};
pub use amm::{
    AMMMetadata, AMMType, AMMStatus,
    onchain as amm_onchain,
};
pub use liquidity_provision::{
    LiquidityProvisionMetadata, LiquidityProvisionType, LiquidityProvisionStatus,
    onchain as liquidity_provision_onchain,
};
pub use arbitrage::{
    ArbitrageMetadata, ArbitrageType, ArbitrageStatus,
    onchain as arbitrage_onchain,
};
pub use flash_loans::{
    FlashLoanMetadata, FlashLoanType, FlashLoanStatus,
    onchain as flash_loans_onchain,
};
pub use defi_analytics::{
    DeFiAnalyticsMetadata, DeFiAnalyticsType, DeFiAnalyticsStatus,
    onchain as defi_analytics_onchain,
};
pub use defi_risk_management::{
    DeFiRiskManagementMetadata, DeFiRiskType, DeFiRiskStatus,
    onchain as defi_risk_management_onchain,
};
pub use defi_governance::{
    DeFiGovernanceMetadata, DeFiGovernanceType, DeFiGovernanceStatus,
    onchain as defi_governance_onchain,
};
pub use defi_compliance::{
    DeFiComplianceMetadata, DeFiComplianceStandard, DeFiComplianceStatus,
    onchain as defi_compliance_onchain,
};
