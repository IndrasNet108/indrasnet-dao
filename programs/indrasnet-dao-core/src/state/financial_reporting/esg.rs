//! Financial ESG Reporting module
//!
//! Financial ESG reporting
//!
//! On-chain: Metadata for ESG reporting
//! Off-chain: Actual reporting, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// ESG dimension
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialESGDimension {
    /// Environmental
    Environmental,
    /// Social
    Social,
    /// Governance
    Governance,
    /// Integrated
    Integrated,
}

/// ESG report status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialESGReportStatus {
    /// Report generating
    Generating,
    /// Report ready
    Ready,
    /// Report published
    Published,
}

/// Financial ESG reporting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialESGReportingMetadata {
    /// Report ID
    pub report_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// ESG dimension
    pub esg_dimension: FinancialESGDimension,
    /// Status
    pub status: FinancialESGReportStatus,
    /// Created at
    pub created_at: i64,
    /// Report data hash
    pub report_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_esg_reporting(
        report: &mut FinancialESGReportingMetadata,
        report_id: u64,
        entity_id: u64,
        esg_dimension: FinancialESGDimension,
        report_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        report.report_id = report_id;
        report.entity_id = entity_id;
        report.esg_dimension = esg_dimension;
        report.status = FinancialESGReportStatus::Generating;
        report.created_at = current_time;
        report.report_data_hash = report_data_hash;
        report.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_esg_report(_report_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_esg_reporting() {
        let mut report = FinancialESGReportingMetadata {
            report_id: 0,
            entity_id: 0,
            esg_dimension: FinancialESGDimension::Environmental,
            status: FinancialESGReportStatus::Published,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_esg_reporting(
            &mut report,
            1,
            10,
            FinancialESGDimension::Social,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.entity_id, 10);
        assert_eq!(report.esg_dimension, FinancialESGDimension::Social);
        assert_eq!(report.status, FinancialESGReportStatus::Generating);
        assert_eq!(report.created_at, 1000);
        assert_eq!(report.report_data_hash, [1u8; 32]);
        assert_eq!(report.bump, 255);
    }

    #[test]
    fn test_initialize_financial_esg_reporting_invalid_id() {
        let mut report = FinancialESGReportingMetadata {
            report_id: 0,
            entity_id: 0,
            esg_dimension: FinancialESGDimension::Environmental,
            status: FinancialESGReportStatus::Generating,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_esg_reporting(
            &mut report,
            0, // Invalid: must be > 0
            10,
            FinancialESGDimension::Social,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_esg_reporting_all_dimensions() {
        let dimensions = vec![
            FinancialESGDimension::Environmental,
            FinancialESGDimension::Social,
            FinancialESGDimension::Governance,
            FinancialESGDimension::Integrated,
        ];

        for dimension in dimensions {
            let mut report = FinancialESGReportingMetadata {
                report_id: 0,
                entity_id: 0,
                esg_dimension: FinancialESGDimension::Environmental,
                status: FinancialESGReportStatus::Generating,
                created_at: 0,
                report_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_esg_reporting(
                &mut report,
                1,
                10,
                dimension,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.esg_dimension, dimension);
        }
    }

    #[test]
    fn test_financial_esg_dimension_variants() {
        assert_eq!(FinancialESGDimension::Environmental, FinancialESGDimension::Environmental);
        assert_eq!(FinancialESGDimension::Social, FinancialESGDimension::Social);
        assert_eq!(FinancialESGDimension::Governance, FinancialESGDimension::Governance);
        assert_eq!(FinancialESGDimension::Integrated, FinancialESGDimension::Integrated);
    }

    #[test]
    fn test_financial_esg_report_status_variants() {
        assert_eq!(FinancialESGReportStatus::Generating, FinancialESGReportStatus::Generating);
        assert_eq!(FinancialESGReportStatus::Ready, FinancialESGReportStatus::Ready);
        assert_eq!(FinancialESGReportStatus::Published, FinancialESGReportStatus::Published);
    }

    #[test]
    fn test_financial_esg_dimension_all_variants_unique() {
        let variants = vec![
            FinancialESGDimension::Environmental,
            FinancialESGDimension::Social,
            FinancialESGDimension::Governance,
            FinancialESGDimension::Integrated,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_esg_report_status_all_variants_unique() {
        let variants = vec![
            FinancialESGReportStatus::Generating,
            FinancialESGReportStatus::Ready,
            FinancialESGReportStatus::Published,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_generate_esg_report() {
        let result = offchain::generate_esg_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
