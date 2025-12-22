//! Budgeting module
//!
//! Budget management (including analysis, forecasting, and control)
//!
//! On-chain: Metadata for budgets, analysis, forecasting, and control
//! Off-chain: Actual budget calculations, tracking, analysis, forecasting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Budget status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BudgetStatus {
    /// Budget draft
    Draft,
    /// Budget approved
    Approved,
    /// Budget active
    Active,
    /// Budget closed
    Closed,
}

/// Budget analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BudgetAnalysisType {
    /// Variance analysis
    Variance,
    /// Trend analysis
    Trend,
    /// Comparative analysis
    Comparative,
    /// Custom analysis
    Custom,
}

/// Budget forecasting method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BudgetForecastingMethod {
    /// Time series
    TimeSeries,
    /// Regression
    Regression,
    /// Machine learning
    MachineLearning,
    /// Custom method
    Custom,
}

/// Budget control type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BudgetControlType {
    /// Hard limit
    HardLimit,
    /// Soft limit
    SoftLimit,
    /// Warning threshold
    WarningThreshold,
    /// Custom control
    Custom,
}

/// Budget metadata (on-chain)
///
/// Stores metadata for budgets
#[account]
#[derive(InitSpace)]
pub struct BudgetMetadata {
    /// Budget ID
    pub budget_id: u64,
    /// Budget amount (in smallest unit)
    pub budget_amount: u64,
    /// Spent amount (in smallest unit)
    pub spent_amount: u64,
    /// Status
    pub status: BudgetStatus,
    /// Created at
    pub created_at: i64,
    /// Period start
    pub period_start: i64,
    /// Period end
    pub period_end: i64,
    /// Budget data hash
    pub budget_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Budget analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct BudgetAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Budget ID
    pub budget_id: u64,
    /// Analysis type
    pub analysis_type: BudgetAnalysisType,
    /// Status (Pending, InProgress, Completed)
    pub status: u8, // Simplified status enum
    /// Created at
    pub created_at: i64,
    /// Analysis data hash
    pub analysis_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Budget forecasting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct BudgetForecastingMetadata {
    /// Forecasting ID
    pub forecasting_id: u64,
    /// Budget ID
    pub budget_id: u64,
    /// Forecasting method
    pub forecasting_method: BudgetForecastingMethod,
    /// Status (Pending, InProgress, Completed)
    pub status: u8, // Simplified status enum
    /// Created at
    pub created_at: i64,
    /// Forecasting data hash
    pub forecasting_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Budget control metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct BudgetControlMetadata {
    /// Control ID
    pub control_id: u64,
    /// Budget ID
    pub budget_id: u64,
    /// Control type
    pub control_type: BudgetControlType,
    /// Status (Active, Paused, Disabled)
    pub status: u8, // Simplified status enum
    /// Created at
    pub created_at: i64,
    /// Control config hash
    pub control_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for budgeting
pub mod onchain {
    use super::*;

    /// Initialize budget
    pub fn initialize_budget(
        budget: &mut BudgetMetadata,
        budget_id: u64,
        budget_amount: u64,
        budget_data_hash: [u8; 32],
        period_start: i64,
        period_end: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(budget_id > 0, IndrasError::InvalidInput);
        require!(budget_amount > 0, IndrasError::InvalidInput);
        require!(period_end > period_start, IndrasError::InvalidInput);
        
        budget.budget_id = budget_id;
        budget.budget_amount = budget_amount;
        budget.spent_amount = 0;
        budget.status = BudgetStatus::Draft;
        budget.created_at = current_time;
        budget.period_start = period_start;
        budget.period_end = period_end;
        budget.budget_data_hash = budget_data_hash;
        budget.bump = bump;
        
        Ok(())
    }

    /// Initialize budget analysis
    pub fn initialize_budget_analysis(
        analysis: &mut BudgetAnalysisMetadata,
        analysis_id: u64,
        budget_id: u64,
        analysis_type: BudgetAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.budget_id = budget_id;
        analysis.analysis_type = analysis_type;
        analysis.status = 0; // Pending
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }

    /// Initialize budget forecasting
    pub fn initialize_budget_forecasting(
        forecasting: &mut BudgetForecastingMetadata,
        forecasting_id: u64,
        budget_id: u64,
        forecasting_method: BudgetForecastingMethod,
        forecasting_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(forecasting_id > 0, IndrasError::InvalidInput);
        forecasting.forecasting_id = forecasting_id;
        forecasting.budget_id = budget_id;
        forecasting.forecasting_method = forecasting_method;
        forecasting.status = 0; // Pending
        forecasting.created_at = current_time;
        forecasting.forecasting_data_hash = forecasting_data_hash;
        forecasting.bump = bump;
        Ok(())
    }

    /// Initialize budget control
    pub fn initialize_budget_control(
        control: &mut BudgetControlMetadata,
        control_id: u64,
        budget_id: u64,
        control_type: BudgetControlType,
        control_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(control_id > 0, IndrasError::InvalidInput);
        control.control_id = control_id;
        control.budget_id = budget_id;
        control.control_type = control_type;
        control.status = 0; // Active
        control.created_at = current_time;
        control.control_config_hash = control_config_hash;
        control.bump = bump;
        Ok(())
    }
}

/// Off-chain functions for budgeting
pub mod offchain {
    /// Calculate budget utilization
    pub fn calculate_budget_utilization(_budget_id: u64) -> u8 {
        // Implementation in off-chain service
        0
    }

    /// Analyze budget
    pub fn analyze_budget(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }

    /// Forecast budget
    pub fn forecast_budget(_forecasting_id: u64) -> Vec<u8> {
        vec![]
    }

    /// Enforce budget control
    pub fn enforce_budget_control(_control_id: u64) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_budget() {
        let mut budget = BudgetMetadata {
            budget_id: 0,
            budget_amount: 0,
            spent_amount: 999,
            status: BudgetStatus::Closed,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            budget_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget(
            &mut budget,
            1,
            10000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(budget.budget_id, 1);
        assert_eq!(budget.budget_amount, 10000);
        assert_eq!(budget.spent_amount, 0);
        assert_eq!(budget.status, BudgetStatus::Draft);
        assert_eq!(budget.created_at, 1500);
        assert_eq!(budget.period_start, 1000);
        assert_eq!(budget.period_end, 2000);
        assert_eq!(budget.budget_data_hash, [1u8; 32]);
        assert_eq!(budget.bump, 255);
    }

    #[test]
    fn test_initialize_budget_invalid_id() {
        let mut budget = BudgetMetadata {
            budget_id: 0,
            budget_amount: 0,
            spent_amount: 0,
            status: BudgetStatus::Draft,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            budget_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget(
            &mut budget,
            0, // Invalid: must be > 0
            10000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_budget_invalid_amount() {
        let mut budget = BudgetMetadata {
            budget_id: 0,
            budget_amount: 0,
            spent_amount: 0,
            status: BudgetStatus::Draft,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            budget_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget(
            &mut budget,
            1,
            0, // Invalid: must be > 0
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_budget_invalid_period() {
        let mut budget = BudgetMetadata {
            budget_id: 0,
            budget_amount: 0,
            spent_amount: 0,
            status: BudgetStatus::Draft,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            budget_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget(
            &mut budget,
            1,
            10000,
            [1u8; 32],
            2000,
            1000, // Invalid: end <= start
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_budget_always_draft_on_init() {
        let mut budget = BudgetMetadata {
            budget_id: 0,
            budget_amount: 0,
            spent_amount: 0,
            status: BudgetStatus::Closed, // Will be reset
            created_at: 0,
            period_start: 0,
            period_end: 0,
            budget_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget(
            &mut budget,
            1,
            10000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(budget.status, BudgetStatus::Draft);
        assert_eq!(budget.spent_amount, 0);
    }

    #[test]
    fn test_initialize_budget_analysis() {
        let mut analysis = BudgetAnalysisMetadata {
            analysis_id: 0,
            budget_id: 0,
            analysis_type: BudgetAnalysisType::Variance,
            status: 99,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget_analysis(
            &mut analysis,
            1,
            10,
            BudgetAnalysisType::Trend,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.budget_id, 10);
        assert_eq!(analysis.analysis_type, BudgetAnalysisType::Trend);
        assert_eq!(analysis.status, 0); // Pending
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_budget_forecasting() {
        let mut forecasting = BudgetForecastingMetadata {
            forecasting_id: 0,
            budget_id: 0,
            forecasting_method: BudgetForecastingMethod::TimeSeries,
            status: 99,
            created_at: 0,
            forecasting_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget_forecasting(
            &mut forecasting,
            1,
            10,
            BudgetForecastingMethod::MachineLearning,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(forecasting.forecasting_id, 1);
        assert_eq!(forecasting.budget_id, 10);
        assert_eq!(forecasting.forecasting_method, BudgetForecastingMethod::MachineLearning);
        assert_eq!(forecasting.status, 0); // Pending
        assert_eq!(forecasting.created_at, 1000);
        assert_eq!(forecasting.forecasting_data_hash, [1u8; 32]);
        assert_eq!(forecasting.bump, 255);
    }

    #[test]
    fn test_initialize_budget_control() {
        let mut control = BudgetControlMetadata {
            control_id: 0,
            budget_id: 0,
            control_type: BudgetControlType::HardLimit,
            status: 99,
            created_at: 0,
            control_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_budget_control(
            &mut control,
            1,
            10,
            BudgetControlType::SoftLimit,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(control.control_id, 1);
        assert_eq!(control.budget_id, 10);
        assert_eq!(control.control_type, BudgetControlType::SoftLimit);
        assert_eq!(control.status, 0); // Active
        assert_eq!(control.created_at, 1000);
        assert_eq!(control.control_config_hash, [1u8; 32]);
        assert_eq!(control.bump, 255);
    }

    #[test]
    fn test_budget_status_variants() {
        assert_eq!(BudgetStatus::Draft, BudgetStatus::Draft);
        assert_eq!(BudgetStatus::Approved, BudgetStatus::Approved);
        assert_eq!(BudgetStatus::Active, BudgetStatus::Active);
        assert_eq!(BudgetStatus::Closed, BudgetStatus::Closed);
    }

    #[test]
    fn test_budget_analysis_type_variants() {
        assert_eq!(BudgetAnalysisType::Variance, BudgetAnalysisType::Variance);
        assert_eq!(BudgetAnalysisType::Trend, BudgetAnalysisType::Trend);
        assert_eq!(BudgetAnalysisType::Comparative, BudgetAnalysisType::Comparative);
        assert_eq!(BudgetAnalysisType::Custom, BudgetAnalysisType::Custom);
    }

    #[test]
    fn test_budget_forecasting_method_variants() {
        assert_eq!(BudgetForecastingMethod::TimeSeries, BudgetForecastingMethod::TimeSeries);
        assert_eq!(BudgetForecastingMethod::Regression, BudgetForecastingMethod::Regression);
        assert_eq!(BudgetForecastingMethod::MachineLearning, BudgetForecastingMethod::MachineLearning);
        assert_eq!(BudgetForecastingMethod::Custom, BudgetForecastingMethod::Custom);
    }

    #[test]
    fn test_budget_control_type_variants() {
        assert_eq!(BudgetControlType::HardLimit, BudgetControlType::HardLimit);
        assert_eq!(BudgetControlType::SoftLimit, BudgetControlType::SoftLimit);
        assert_eq!(BudgetControlType::WarningThreshold, BudgetControlType::WarningThreshold);
        assert_eq!(BudgetControlType::Custom, BudgetControlType::Custom);
    }

    #[test]
    fn test_budget_status_all_variants_unique() {
        let variants = vec![
            BudgetStatus::Draft,
            BudgetStatus::Approved,
            BudgetStatus::Active,
            BudgetStatus::Closed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_budget_analysis_type_all_variants_unique() {
        let variants = vec![
            BudgetAnalysisType::Variance,
            BudgetAnalysisType::Trend,
            BudgetAnalysisType::Comparative,
            BudgetAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_budget_forecasting_method_all_variants_unique() {
        let variants = vec![
            BudgetForecastingMethod::TimeSeries,
            BudgetForecastingMethod::Regression,
            BudgetForecastingMethod::MachineLearning,
            BudgetForecastingMethod::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_budget_control_type_all_variants_unique() {
        let variants = vec![
            BudgetControlType::HardLimit,
            BudgetControlType::SoftLimit,
            BudgetControlType::WarningThreshold,
            BudgetControlType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_initialize_budget_all_analysis_types() {
        let analysis_types = vec![
            BudgetAnalysisType::Variance,
            BudgetAnalysisType::Trend,
            BudgetAnalysisType::Comparative,
            BudgetAnalysisType::Custom,
        ];

        for analysis_type in analysis_types {
            let mut analysis = BudgetAnalysisMetadata {
                analysis_id: 0,
                budget_id: 0,
                analysis_type: BudgetAnalysisType::Variance,
                status: 0,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_budget_analysis(
                &mut analysis,
                1,
                10,
                analysis_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(analysis.analysis_type, analysis_type);
        }
    }

    #[test]
    fn test_initialize_budget_all_forecasting_methods() {
        let methods = vec![
            BudgetForecastingMethod::TimeSeries,
            BudgetForecastingMethod::Regression,
            BudgetForecastingMethod::MachineLearning,
            BudgetForecastingMethod::Custom,
        ];

        for method in methods {
            let mut forecasting = BudgetForecastingMetadata {
                forecasting_id: 0,
                budget_id: 0,
                forecasting_method: BudgetForecastingMethod::TimeSeries,
                status: 0,
                created_at: 0,
                forecasting_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_budget_forecasting(
                &mut forecasting,
                1,
                10,
                method,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(forecasting.forecasting_method, method);
        }
    }

    #[test]
    fn test_initialize_budget_all_control_types() {
        let control_types = vec![
            BudgetControlType::HardLimit,
            BudgetControlType::SoftLimit,
            BudgetControlType::WarningThreshold,
            BudgetControlType::Custom,
        ];

        for control_type in control_types {
            let mut control = BudgetControlMetadata {
                control_id: 0,
                budget_id: 0,
                control_type: BudgetControlType::HardLimit,
                status: 0,
                created_at: 0,
                control_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_budget_control(
                &mut control,
                1,
                10,
                control_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(control.control_type, control_type);
        }
    }

    #[test]
    fn test_budget_metadata_all_fields() {
        let budget = BudgetMetadata {
            budget_id: 123,
            budget_amount: 50000,
            spent_amount: 10000,
            status: BudgetStatus::Active,
            created_at: 2000,
            period_start: 1000,
            period_end: 3000,
            budget_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(budget.budget_id, 123);
        assert_eq!(budget.budget_amount, 50000);
        assert_eq!(budget.spent_amount, 10000);
        assert_eq!(budget.status, BudgetStatus::Active);
        assert_eq!(budget.created_at, 2000);
        assert_eq!(budget.period_start, 1000);
        assert_eq!(budget.period_end, 3000);
        assert_eq!(budget.budget_data_hash, [42u8; 32]);
        assert_eq!(budget.bump, 128);
    }

    #[test]
    fn test_offchain_calculate_budget_utilization() {
        let result = offchain::calculate_budget_utilization(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_offchain_analyze_budget() {
        let result = offchain::analyze_budget(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_forecast_budget() {
        let result = offchain::forecast_budget(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_enforce_budget_control() {
        let result = offchain::enforce_budget_control(1);
        assert_eq!(result, false);
    }
}
