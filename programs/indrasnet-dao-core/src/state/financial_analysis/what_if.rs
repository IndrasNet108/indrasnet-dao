//! Financial What-If Analysis module
//!
//! Financial what-if analysis
//!
//! On-chain: Metadata for what-if analysis
//! Off-chain: Actual analysis, simulation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis scenario
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWhatIfAnalysisScenario {
    /// Revenue change
    RevenueChange,
    /// Cost change
    CostChange,
    /// Market change
    MarketChange,
    /// Custom scenario
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWhatIfAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Financial what-if analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialWhatIfAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis scenario
    pub analysis_scenario: FinancialWhatIfAnalysisScenario,
    /// Status
    pub status: FinancialWhatIfAnalysisStatus,
    /// Created at
    pub created_at: i64,
    /// Analysis data hash
    pub analysis_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_what_if_analysis(
        analysis: &mut FinancialWhatIfAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_scenario: FinancialWhatIfAnalysisScenario,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_scenario = analysis_scenario;
        analysis.status = FinancialWhatIfAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_what_if(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_what_if_analysis() {
        let mut analysis = FinancialWhatIfAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_scenario: FinancialWhatIfAnalysisScenario::RevenueChange,
            status: FinancialWhatIfAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_what_if_analysis(
            &mut analysis,
            1,
            10,
            FinancialWhatIfAnalysisScenario::CostChange,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_scenario, FinancialWhatIfAnalysisScenario::CostChange);
        assert_eq!(analysis.status, FinancialWhatIfAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_financial_what_if_analysis_invalid_id() {
        let mut analysis = FinancialWhatIfAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_scenario: FinancialWhatIfAnalysisScenario::RevenueChange,
            status: FinancialWhatIfAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_what_if_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            FinancialWhatIfAnalysisScenario::CostChange,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_what_if_analysis_all_scenarios() {
        let scenarios = vec![
            FinancialWhatIfAnalysisScenario::RevenueChange,
            FinancialWhatIfAnalysisScenario::CostChange,
            FinancialWhatIfAnalysisScenario::MarketChange,
            FinancialWhatIfAnalysisScenario::Custom,
        ];

        for scenario in scenarios {
            let mut analysis = FinancialWhatIfAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_scenario: FinancialWhatIfAnalysisScenario::RevenueChange,
                status: FinancialWhatIfAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_what_if_analysis(
                &mut analysis,
                1,
                10,
                scenario,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(analysis.analysis_scenario, scenario);
        }
    }

    #[test]
    fn test_financial_what_if_analysis_scenario_variants() {
        assert_eq!(FinancialWhatIfAnalysisScenario::RevenueChange, FinancialWhatIfAnalysisScenario::RevenueChange);
        assert_eq!(FinancialWhatIfAnalysisScenario::CostChange, FinancialWhatIfAnalysisScenario::CostChange);
        assert_eq!(FinancialWhatIfAnalysisScenario::MarketChange, FinancialWhatIfAnalysisScenario::MarketChange);
        assert_eq!(FinancialWhatIfAnalysisScenario::Custom, FinancialWhatIfAnalysisScenario::Custom);
    }

    #[test]
    fn test_financial_what_if_analysis_status_variants() {
        assert_eq!(FinancialWhatIfAnalysisStatus::Pending, FinancialWhatIfAnalysisStatus::Pending);
        assert_eq!(FinancialWhatIfAnalysisStatus::InProgress, FinancialWhatIfAnalysisStatus::InProgress);
        assert_eq!(FinancialWhatIfAnalysisStatus::Completed, FinancialWhatIfAnalysisStatus::Completed);
    }

    #[test]
    fn test_financial_what_if_analysis_scenario_all_variants_unique() {
        let variants = vec![
            FinancialWhatIfAnalysisScenario::RevenueChange,
            FinancialWhatIfAnalysisScenario::CostChange,
            FinancialWhatIfAnalysisScenario::MarketChange,
            FinancialWhatIfAnalysisScenario::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_what_if_analysis_status_all_variants_unique() {
        let variants = vec![
            FinancialWhatIfAnalysisStatus::Pending,
            FinancialWhatIfAnalysisStatus::InProgress,
            FinancialWhatIfAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_what_if() {
        let result = offchain::analyze_what_if(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
