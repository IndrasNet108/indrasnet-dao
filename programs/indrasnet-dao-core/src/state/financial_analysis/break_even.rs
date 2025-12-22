//! Financial Break-Even Analysis module
//!
//! Financial break-even analysis
//!
//! On-chain: Metadata for break-even analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBreakEvenAnalysisMethod {
    /// Contribution margin method
    ContributionMargin,
    /// Equation method
    Equation,
    /// Graphical method
    Graphical,
    /// Custom method
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBreakEvenAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Financial break-even analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialBreakEvenAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis method
    pub analysis_method: FinancialBreakEvenAnalysisMethod,
    /// Status
    pub status: FinancialBreakEvenAnalysisStatus,
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
    pub fn initialize_financial_break_even_analysis(
        analysis: &mut FinancialBreakEvenAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_method: FinancialBreakEvenAnalysisMethod,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_method = analysis_method;
        analysis.status = FinancialBreakEvenAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_break_even(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_break_even_analysis() {
        let mut analysis = FinancialBreakEvenAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_method: FinancialBreakEvenAnalysisMethod::ContributionMargin,
            status: FinancialBreakEvenAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_break_even_analysis(
            &mut analysis,
            1,
            10,
            FinancialBreakEvenAnalysisMethod::Equation,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_method, FinancialBreakEvenAnalysisMethod::Equation);
        assert_eq!(analysis.status, FinancialBreakEvenAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_financial_break_even_analysis_invalid_id() {
        let mut analysis = FinancialBreakEvenAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_method: FinancialBreakEvenAnalysisMethod::ContributionMargin,
            status: FinancialBreakEvenAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_break_even_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            FinancialBreakEvenAnalysisMethod::Equation,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_break_even_analysis_all_methods() {
        let methods = vec![
            FinancialBreakEvenAnalysisMethod::ContributionMargin,
            FinancialBreakEvenAnalysisMethod::Equation,
            FinancialBreakEvenAnalysisMethod::Graphical,
            FinancialBreakEvenAnalysisMethod::Custom,
        ];

        for method in methods {
            let mut analysis = FinancialBreakEvenAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_method: FinancialBreakEvenAnalysisMethod::ContributionMargin,
                status: FinancialBreakEvenAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_break_even_analysis(
                &mut analysis,
                1,
                10,
                method,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(analysis.analysis_method, method);
        }
    }

    #[test]
    fn test_financial_break_even_analysis_method_variants() {
        assert_eq!(FinancialBreakEvenAnalysisMethod::ContributionMargin, FinancialBreakEvenAnalysisMethod::ContributionMargin);
        assert_eq!(FinancialBreakEvenAnalysisMethod::Equation, FinancialBreakEvenAnalysisMethod::Equation);
        assert_eq!(FinancialBreakEvenAnalysisMethod::Graphical, FinancialBreakEvenAnalysisMethod::Graphical);
        assert_eq!(FinancialBreakEvenAnalysisMethod::Custom, FinancialBreakEvenAnalysisMethod::Custom);
    }

    #[test]
    fn test_financial_break_even_analysis_status_variants() {
        assert_eq!(FinancialBreakEvenAnalysisStatus::Pending, FinancialBreakEvenAnalysisStatus::Pending);
        assert_eq!(FinancialBreakEvenAnalysisStatus::InProgress, FinancialBreakEvenAnalysisStatus::InProgress);
        assert_eq!(FinancialBreakEvenAnalysisStatus::Completed, FinancialBreakEvenAnalysisStatus::Completed);
    }

    #[test]
    fn test_financial_break_even_analysis_method_all_variants_unique() {
        let variants = vec![
            FinancialBreakEvenAnalysisMethod::ContributionMargin,
            FinancialBreakEvenAnalysisMethod::Equation,
            FinancialBreakEvenAnalysisMethod::Graphical,
            FinancialBreakEvenAnalysisMethod::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_break_even_analysis_status_all_variants_unique() {
        let variants = vec![
            FinancialBreakEvenAnalysisStatus::Pending,
            FinancialBreakEvenAnalysisStatus::InProgress,
            FinancialBreakEvenAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_break_even() {
        let result = offchain::analyze_break_even(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
