//! Investment Analysis module
//!
//! Investment analysis and evaluation
//!
//! On-chain: Metadata for investment analysis
//! Off-chain: Actual analysis, evaluation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InvestmentAnalysisType {
    /// Fundamental analysis
    Fundamental,
    /// Technical analysis
    Technical,
    /// Quantitative analysis
    Quantitative,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InvestmentAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Investment analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct InvestmentAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Investment ID
    pub investment_id: u64,
    /// Analysis type
    pub analysis_type: InvestmentAnalysisType,
    /// Status
    pub status: InvestmentAnalysisStatus,
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
    pub fn initialize_investment_analysis(
        analysis: &mut InvestmentAnalysisMetadata,
        analysis_id: u64,
        investment_id: u64,
        analysis_type: InvestmentAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.investment_id = investment_id;
        analysis.analysis_type = analysis_type;
        analysis.status = InvestmentAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_investment(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_investment_analysis() {
        let mut analysis = InvestmentAnalysisMetadata {
            analysis_id: 0,
            investment_id: 0,
            analysis_type: InvestmentAnalysisType::Fundamental,
            status: InvestmentAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_investment_analysis(
            &mut analysis,
            1,
            10,
            InvestmentAnalysisType::Technical,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.investment_id, 10);
        assert_eq!(analysis.analysis_type, InvestmentAnalysisType::Technical);
        assert_eq!(analysis.status, InvestmentAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_investment_analysis_invalid_id() {
        let mut analysis = InvestmentAnalysisMetadata {
            analysis_id: 0,
            investment_id: 0,
            analysis_type: InvestmentAnalysisType::Fundamental,
            status: InvestmentAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_investment_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            InvestmentAnalysisType::Technical,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_investment_analysis_all_types() {
        let types = vec![
            InvestmentAnalysisType::Fundamental,
            InvestmentAnalysisType::Technical,
            InvestmentAnalysisType::Quantitative,
            InvestmentAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = InvestmentAnalysisMetadata {
                analysis_id: 0,
                investment_id: 0,
                analysis_type: InvestmentAnalysisType::Fundamental,
                status: InvestmentAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_investment_analysis(
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
    fn test_investment_analysis_type_variants() {
        assert_eq!(InvestmentAnalysisType::Fundamental, InvestmentAnalysisType::Fundamental);
        assert_eq!(InvestmentAnalysisType::Technical, InvestmentAnalysisType::Technical);
        assert_eq!(InvestmentAnalysisType::Quantitative, InvestmentAnalysisType::Quantitative);
        assert_eq!(InvestmentAnalysisType::Custom, InvestmentAnalysisType::Custom);
    }

    #[test]
    fn test_investment_analysis_status_variants() {
        assert_eq!(InvestmentAnalysisStatus::Pending, InvestmentAnalysisStatus::Pending);
        assert_eq!(InvestmentAnalysisStatus::InProgress, InvestmentAnalysisStatus::InProgress);
        assert_eq!(InvestmentAnalysisStatus::Completed, InvestmentAnalysisStatus::Completed);
    }

    #[test]
    fn test_investment_analysis_type_all_variants_unique() {
        let variants = vec![
            InvestmentAnalysisType::Fundamental,
            InvestmentAnalysisType::Technical,
            InvestmentAnalysisType::Quantitative,
            InvestmentAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_investment_analysis_status_all_variants_unique() {
        let variants = vec![
            InvestmentAnalysisStatus::Pending,
            InvestmentAnalysisStatus::InProgress,
            InvestmentAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_investment() {
        let result = offchain::analyze_investment(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
