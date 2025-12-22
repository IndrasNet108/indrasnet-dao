//! Financial Planning state modules
//!
//! Financial planning for the DAO:
//! - On-chain: Metadata for financial plans, strategic planning, scenario planning
//! - Off-chain: Actual planning, strategy development, scenario analysis
//!
//! Includes: core, strategic, scenario

pub mod core;
pub mod strategic;
pub mod scenario;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use core::{
    FinancialPlanMetadata, FinancialPlanStatus, FinancialPlanningType,
    onchain::initialize_financial_plan,
    onchain::initialize_advanced_financial_planning,
};
pub use strategic::{
    FinancialStrategicPlanningMetadata, FinancialStrategicPlanningHorizon, FinancialStrategicPlanningStatus,
    onchain::initialize_financial_strategic_planning,
};
pub use scenario::{
    FinancialScenarioPlanningMetadata, FinancialScenarioType, FinancialScenarioPlanningStatus,
    onchain::initialize_financial_scenario_planning,
};
