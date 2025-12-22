//! Financial Sensitivity Analysis module
//!
//! Financial sensitivity analysis
//!
//! On-chain: Metadata for sensitivity analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSensitivityAnalysisType {
    /// Variable sensitivity
    Variable,
    /// Parameter sensitivity
    Parameter,
    /// Assumption sensitivity
    Assumption,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialSensitivityAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Financial sensitivity analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialSensitivityAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis type
    pub analysis_type: FinancialSensitivityAnalysisType,
    /// Status
    pub status: FinancialSensitivityAnalysisStatus,
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
    pub fn initialize_financial_sensitivity_analysis(
        analysis: &mut FinancialSensitivityAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_type: FinancialSensitivityAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_type = analysis_type;
        analysis.status = FinancialSensitivityAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_sensitivity(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_sensitivity_analysis() {
        let mut analysis = FinancialSensitivityAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: FinancialSensitivityAnalysisType::Variable,
            status: FinancialSensitivityAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_sensitivity_analysis(
            &mut analysis,
            1,
            10,
            FinancialSensitivityAnalysisType::Parameter,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_type, FinancialSensitivityAnalysisType::Parameter);
        assert_eq!(analysis.status, FinancialSensitivityAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_financial_sensitivity_analysis_invalid_id() {
        let mut analysis = FinancialSensitivityAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: FinancialSensitivityAnalysisType::Variable,
            status: FinancialSensitivityAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_sensitivity_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            FinancialSensitivityAnalysisType::Parameter,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_sensitivity_analysis_all_types() {
        let types = vec![
            FinancialSensitivityAnalysisType::Variable,
            FinancialSensitivityAnalysisType::Parameter,
            FinancialSensitivityAnalysisType::Assumption,
            FinancialSensitivityAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = FinancialSensitivityAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_type: FinancialSensitivityAnalysisType::Variable,
                status: FinancialSensitivityAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_sensitivity_analysis(
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
    fn test_financial_sensitivity_analysis_type_variants() {
        assert_eq!(FinancialSensitivityAnalysisType::Variable, FinancialSensitivityAnalysisType::Variable);
        assert_eq!(FinancialSensitivityAnalysisType::Parameter, FinancialSensitivityAnalysisType::Parameter);
        assert_eq!(FinancialSensitivityAnalysisType::Assumption, FinancialSensitivityAnalysisType::Assumption);
        assert_eq!(FinancialSensitivityAnalysisType::Custom, FinancialSensitivityAnalysisType::Custom);
    }

    #[test]
    fn test_financial_sensitivity_analysis_status_variants() {
        assert_eq!(FinancialSensitivityAnalysisStatus::Pending, FinancialSensitivityAnalysisStatus::Pending);
        assert_eq!(FinancialSensitivityAnalysisStatus::InProgress, FinancialSensitivityAnalysisStatus::InProgress);
        assert_eq!(FinancialSensitivityAnalysisStatus::Completed, FinancialSensitivityAnalysisStatus::Completed);
    }

    #[test]
    fn test_financial_sensitivity_analysis_type_all_variants_unique() {
        let variants = vec![
            FinancialSensitivityAnalysisType::Variable,
            FinancialSensitivityAnalysisType::Parameter,
            FinancialSensitivityAnalysisType::Assumption,
            FinancialSensitivityAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_sensitivity_analysis_status_all_variants_unique() {
        let variants = vec![
            FinancialSensitivityAnalysisStatus::Pending,
            FinancialSensitivityAnalysisStatus::InProgress,
            FinancialSensitivityAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_sensitivity() {
        let result = offchain::analyze_sensitivity(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
