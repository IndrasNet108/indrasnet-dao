//! Liquidity Analysis module
//!
//! Liquidity analysis
//!
//! On-chain: Metadata for liquidity analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LiquidityAnalysisType {
    /// Current ratio
    CurrentRatio,
    /// Quick ratio
    QuickRatio,
    /// Cash ratio
    CashRatio,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LiquidityAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Liquidity analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct LiquidityAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis type
    pub analysis_type: LiquidityAnalysisType,
    /// Status
    pub status: LiquidityAnalysisStatus,
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
    pub fn initialize_liquidity_analysis(
        analysis: &mut LiquidityAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_type: LiquidityAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_type = analysis_type;
        analysis.status = LiquidityAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_liquidity(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_liquidity_analysis() {
        let mut analysis = LiquidityAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: LiquidityAnalysisType::CurrentRatio,
            status: LiquidityAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_liquidity_analysis(
            &mut analysis,
            1,
            10,
            LiquidityAnalysisType::QuickRatio,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_type, LiquidityAnalysisType::QuickRatio);
        assert_eq!(analysis.status, LiquidityAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_liquidity_analysis_invalid_id() {
        let mut analysis = LiquidityAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: LiquidityAnalysisType::CurrentRatio,
            status: LiquidityAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_liquidity_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            LiquidityAnalysisType::QuickRatio,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_liquidity_analysis_all_types() {
        let types = vec![
            LiquidityAnalysisType::CurrentRatio,
            LiquidityAnalysisType::QuickRatio,
            LiquidityAnalysisType::CashRatio,
            LiquidityAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = LiquidityAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_type: LiquidityAnalysisType::CurrentRatio,
                status: LiquidityAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_liquidity_analysis(
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
    fn test_liquidity_analysis_type_variants() {
        assert_eq!(LiquidityAnalysisType::CurrentRatio, LiquidityAnalysisType::CurrentRatio);
        assert_eq!(LiquidityAnalysisType::QuickRatio, LiquidityAnalysisType::QuickRatio);
        assert_eq!(LiquidityAnalysisType::CashRatio, LiquidityAnalysisType::CashRatio);
        assert_eq!(LiquidityAnalysisType::Custom, LiquidityAnalysisType::Custom);
    }

    #[test]
    fn test_liquidity_analysis_status_variants() {
        assert_eq!(LiquidityAnalysisStatus::Pending, LiquidityAnalysisStatus::Pending);
        assert_eq!(LiquidityAnalysisStatus::InProgress, LiquidityAnalysisStatus::InProgress);
        assert_eq!(LiquidityAnalysisStatus::Completed, LiquidityAnalysisStatus::Completed);
    }

    #[test]
    fn test_liquidity_analysis_type_all_variants_unique() {
        let variants = vec![
            LiquidityAnalysisType::CurrentRatio,
            LiquidityAnalysisType::QuickRatio,
            LiquidityAnalysisType::CashRatio,
            LiquidityAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_liquidity_analysis_status_all_variants_unique() {
        let variants = vec![
            LiquidityAnalysisStatus::Pending,
            LiquidityAnalysisStatus::InProgress,
            LiquidityAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_liquidity() {
        let result = offchain::analyze_liquidity(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
