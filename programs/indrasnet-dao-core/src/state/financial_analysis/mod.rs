//! Financial Analysis state modules
//!
//! Financial analysis for the DAO:
//! - On-chain: Metadata for various financial analyses
//! - Off-chain: Actual analysis, calculations, simulations
//!
//! Includes: profitability, liquidity, solvency, efficiency, growth, valuation, investment,
//! break_even, what_if, sensitivity

pub mod profitability;
pub mod liquidity;
pub mod solvency;
pub mod efficiency;
pub mod growth;
pub mod valuation;
pub mod investment;
pub mod break_even;
pub mod what_if;
pub mod sensitivity;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use profitability::{
    ProfitabilityAnalysisMetadata, ProfitabilityAnalysisType, ProfitabilityAnalysisStatus,
    onchain::initialize_profitability_analysis,
};
pub use liquidity::{
    LiquidityAnalysisMetadata, LiquidityAnalysisType, LiquidityAnalysisStatus,
    onchain::initialize_liquidity_analysis,
};
pub use solvency::{
    SolvencyAnalysisMetadata, SolvencyAnalysisType, SolvencyAnalysisStatus,
    onchain::initialize_solvency_analysis,
};
pub use efficiency::{
    EfficiencyAnalysisMetadata, EfficiencyAnalysisType, EfficiencyAnalysisStatus,
    onchain::initialize_efficiency_analysis,
};
pub use growth::{
    GrowthAnalysisMetadata, GrowthAnalysisType, GrowthAnalysisStatus,
    onchain::initialize_growth_analysis,
};
pub use valuation::{
    ValuationAnalysisMetadata, ValuationAnalysisType, ValuationAnalysisStatus,
    onchain::initialize_valuation_analysis,
};
pub use investment::{
    InvestmentAnalysisMetadata, InvestmentAnalysisType, InvestmentAnalysisStatus,
    onchain::initialize_investment_analysis,
};
pub use break_even::{
    FinancialBreakEvenAnalysisMetadata, FinancialBreakEvenAnalysisMethod, FinancialBreakEvenAnalysisStatus,
    onchain::initialize_financial_break_even_analysis,
};
pub use what_if::{
    FinancialWhatIfAnalysisMetadata, FinancialWhatIfAnalysisScenario, FinancialWhatIfAnalysisStatus,
    onchain::initialize_financial_what_if_analysis,
};
pub use sensitivity::{
    FinancialSensitivityAnalysisMetadata, FinancialSensitivityAnalysisType, FinancialSensitivityAnalysisStatus,
    onchain::initialize_financial_sensitivity_analysis,
};
