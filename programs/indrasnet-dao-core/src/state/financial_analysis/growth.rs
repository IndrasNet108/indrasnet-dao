//! Growth Analysis module
//!
//! Growth analysis
//!
//! On-chain: Metadata for growth analysis
//! Off-chain: Actual analysis, calculation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrowthAnalysisType {
    /// Revenue growth
    RevenueGrowth,
    /// Profit growth
    ProfitGrowth,
    /// Asset growth
    AssetGrowth,
    /// Custom analysis
    Custom,
}

/// Analysis status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum GrowthAnalysisStatus {
    /// Analysis pending
    Pending,
    /// Analysis in progress
    InProgress,
    /// Analysis completed
    Completed,
}

/// Growth analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct GrowthAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Analysis type
    pub analysis_type: GrowthAnalysisType,
    /// Status
    pub status: GrowthAnalysisStatus,
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
    pub fn initialize_growth_analysis(
        analysis: &mut GrowthAnalysisMetadata,
        analysis_id: u64,
        entity_id: u64,
        analysis_type: GrowthAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.entity_id = entity_id;
        analysis.analysis_type = analysis_type;
        analysis.status = GrowthAnalysisStatus::Pending;
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn analyze_growth(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_growth_analysis() {
        let mut analysis = GrowthAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: GrowthAnalysisType::RevenueGrowth,
            status: GrowthAnalysisStatus::Completed,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_growth_analysis(
            &mut analysis,
            1,
            10,
            GrowthAnalysisType::ProfitGrowth,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analysis.analysis_id, 1);
        assert_eq!(analysis.entity_id, 10);
        assert_eq!(analysis.analysis_type, GrowthAnalysisType::ProfitGrowth);
        assert_eq!(analysis.status, GrowthAnalysisStatus::Pending);
        assert_eq!(analysis.created_at, 1000);
        assert_eq!(analysis.analysis_data_hash, [1u8; 32]);
        assert_eq!(analysis.bump, 255);
    }

    #[test]
    fn test_initialize_growth_analysis_invalid_id() {
        let mut analysis = GrowthAnalysisMetadata {
            analysis_id: 0,
            entity_id: 0,
            analysis_type: GrowthAnalysisType::RevenueGrowth,
            status: GrowthAnalysisStatus::Pending,
            created_at: 0,
            analysis_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_growth_analysis(
            &mut analysis,
            0, // Invalid: must be > 0
            10,
            GrowthAnalysisType::ProfitGrowth,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_growth_analysis_all_types() {
        let types = vec![
            GrowthAnalysisType::RevenueGrowth,
            GrowthAnalysisType::ProfitGrowth,
            GrowthAnalysisType::AssetGrowth,
            GrowthAnalysisType::Custom,
        ];

        for analysis_type in types {
            let mut analysis = GrowthAnalysisMetadata {
                analysis_id: 0,
                entity_id: 0,
                analysis_type: GrowthAnalysisType::RevenueGrowth,
                status: GrowthAnalysisStatus::Pending,
                created_at: 0,
                analysis_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_growth_analysis(
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
    fn test_growth_analysis_type_variants() {
        assert_eq!(GrowthAnalysisType::RevenueGrowth, GrowthAnalysisType::RevenueGrowth);
        assert_eq!(GrowthAnalysisType::ProfitGrowth, GrowthAnalysisType::ProfitGrowth);
        assert_eq!(GrowthAnalysisType::AssetGrowth, GrowthAnalysisType::AssetGrowth);
        assert_eq!(GrowthAnalysisType::Custom, GrowthAnalysisType::Custom);
    }

    #[test]
    fn test_growth_analysis_status_variants() {
        assert_eq!(GrowthAnalysisStatus::Pending, GrowthAnalysisStatus::Pending);
        assert_eq!(GrowthAnalysisStatus::InProgress, GrowthAnalysisStatus::InProgress);
        assert_eq!(GrowthAnalysisStatus::Completed, GrowthAnalysisStatus::Completed);
    }

    #[test]
    fn test_growth_analysis_type_all_variants_unique() {
        let variants = vec![
            GrowthAnalysisType::RevenueGrowth,
            GrowthAnalysisType::ProfitGrowth,
            GrowthAnalysisType::AssetGrowth,
            GrowthAnalysisType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_growth_analysis_status_all_variants_unique() {
        let variants = vec![
            GrowthAnalysisStatus::Pending,
            GrowthAnalysisStatus::InProgress,
            GrowthAnalysisStatus::Completed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_analyze_growth() {
        let result = offchain::analyze_growth(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
