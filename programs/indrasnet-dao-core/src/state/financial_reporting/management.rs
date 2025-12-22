//! Financial Management Reporting module
//!
//! Financial management reporting
//!
//! On-chain: Metadata for management reporting
//! Off-chain: Actual reporting, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Report type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialManagementReportType {
    /// Executive report
    Executive,
    /// Operational report
    Operational,
    /// Analytical report
    Analytical,
    /// Custom report
    Custom,
}

/// Report status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialManagementReportStatus {
    /// Report generating
    Generating,
    /// Report ready
    Ready,
    /// Report published
    Published,
}

/// Financial management reporting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialManagementReportingMetadata {
    /// Report ID
    pub report_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Report type
    pub report_type: FinancialManagementReportType,
    /// Status
    pub status: FinancialManagementReportStatus,
    /// Created at
    pub created_at: i64,
    /// Report period start
    pub report_period_start: i64,
    /// Report period end
    pub report_period_end: i64,
    /// Report data hash
    pub report_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_management_reporting(
        report: &mut FinancialManagementReportingMetadata,
        report_id: u64,
        entity_id: u64,
        report_type: FinancialManagementReportType,
        report_data_hash: [u8; 32],
        report_period_start: i64,
        report_period_end: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        require!(report_period_end > report_period_start, IndrasError::InvalidInput);
        report.report_id = report_id;
        report.entity_id = entity_id;
        report.report_type = report_type;
        report.status = FinancialManagementReportStatus::Generating;
        report.created_at = current_time;
        report.report_period_start = report_period_start;
        report.report_period_end = report_period_end;
        report.report_data_hash = report_data_hash;
        report.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_management_report(_report_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_management_reporting() {
        let mut report = FinancialManagementReportingMetadata {
            report_id: 0,
            entity_id: 0,
            report_type: FinancialManagementReportType::Executive,
            status: FinancialManagementReportStatus::Published,
            created_at: 0,
            report_period_start: 0,
            report_period_end: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_management_reporting(
            &mut report,
            1,
            10,
            FinancialManagementReportType::Operational,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.entity_id, 10);
        assert_eq!(report.report_type, FinancialManagementReportType::Operational);
        assert_eq!(report.status, FinancialManagementReportStatus::Generating);
        assert_eq!(report.created_at, 1500);
        assert_eq!(report.report_period_start, 1000);
        assert_eq!(report.report_period_end, 2000);
        assert_eq!(report.report_data_hash, [1u8; 32]);
        assert_eq!(report.bump, 255);
    }

    #[test]
    fn test_initialize_financial_management_reporting_invalid_id() {
        let mut report = FinancialManagementReportingMetadata {
            report_id: 0,
            entity_id: 0,
            report_type: FinancialManagementReportType::Executive,
            status: FinancialManagementReportStatus::Generating,
            created_at: 0,
            report_period_start: 0,
            report_period_end: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_management_reporting(
            &mut report,
            0, // Invalid: must be > 0
            10,
            FinancialManagementReportType::Operational,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_management_reporting_invalid_period() {
        let mut report = FinancialManagementReportingMetadata {
            report_id: 0,
            entity_id: 0,
            report_type: FinancialManagementReportType::Executive,
            status: FinancialManagementReportStatus::Generating,
            created_at: 0,
            report_period_start: 0,
            report_period_end: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_management_reporting(
            &mut report,
            1,
            10,
            FinancialManagementReportType::Operational,
            [1u8; 32],
            2000,
            1000, // Invalid: end <= start
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_management_reporting_all_types() {
        let types = vec![
            FinancialManagementReportType::Executive,
            FinancialManagementReportType::Operational,
            FinancialManagementReportType::Analytical,
            FinancialManagementReportType::Custom,
        ];

        for report_type in types {
            let mut report = FinancialManagementReportingMetadata {
                report_id: 0,
                entity_id: 0,
                report_type: FinancialManagementReportType::Executive,
                status: FinancialManagementReportStatus::Generating,
                created_at: 0,
                report_period_start: 0,
                report_period_end: 0,
                report_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_management_reporting(
                &mut report,
                1,
                10,
                report_type,
                [0u8; 32],
                1000,
                2000,
                1500,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.report_type, report_type);
        }
    }

    #[test]
    fn test_financial_management_report_type_variants() {
        assert_eq!(FinancialManagementReportType::Executive, FinancialManagementReportType::Executive);
        assert_eq!(FinancialManagementReportType::Operational, FinancialManagementReportType::Operational);
        assert_eq!(FinancialManagementReportType::Analytical, FinancialManagementReportType::Analytical);
        assert_eq!(FinancialManagementReportType::Custom, FinancialManagementReportType::Custom);
    }

    #[test]
    fn test_financial_management_report_status_variants() {
        assert_eq!(FinancialManagementReportStatus::Generating, FinancialManagementReportStatus::Generating);
        assert_eq!(FinancialManagementReportStatus::Ready, FinancialManagementReportStatus::Ready);
        assert_eq!(FinancialManagementReportStatus::Published, FinancialManagementReportStatus::Published);
    }

    #[test]
    fn test_financial_management_report_type_all_variants_unique() {
        let variants = vec![
            FinancialManagementReportType::Executive,
            FinancialManagementReportType::Operational,
            FinancialManagementReportType::Analytical,
            FinancialManagementReportType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_management_report_status_all_variants_unique() {
        let variants = vec![
            FinancialManagementReportStatus::Generating,
            FinancialManagementReportStatus::Ready,
            FinancialManagementReportStatus::Published,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_generate_management_report() {
        let result = offchain::generate_management_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
