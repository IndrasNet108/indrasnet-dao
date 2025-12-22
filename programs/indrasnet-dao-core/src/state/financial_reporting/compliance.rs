//! Financial Compliance Reporting module
//!
//! Financial compliance reporting
//!
//! On-chain: Metadata for compliance reporting
//! Off-chain: Actual reporting, submission

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Compliance framework
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialComplianceFramework {
    /// GAAP
    GAAP,
    /// IFRS
    IFRS,
    /// SOX
    SOX,
    /// Custom framework
    Custom,
}

/// Report status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialComplianceReportStatus {
    /// Report draft
    Draft,
    /// Report submitted
    Submitted,
    /// Report approved
    Approved,
}

/// Financial compliance reporting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialComplianceReportingMetadata {
    /// Report ID
    pub report_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Compliance framework
    pub compliance_framework: FinancialComplianceFramework,
    /// Status
    pub status: FinancialComplianceReportStatus,
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
    pub fn initialize_financial_compliance_reporting(
        report: &mut FinancialComplianceReportingMetadata,
        report_id: u64,
        entity_id: u64,
        compliance_framework: FinancialComplianceFramework,
        report_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        report.report_id = report_id;
        report.entity_id = entity_id;
        report.compliance_framework = compliance_framework;
        report.status = FinancialComplianceReportStatus::Draft;
        report.created_at = current_time;
        report.report_data_hash = report_data_hash;
        report.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn submit_compliance_report(_report_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_compliance_reporting() {
        let mut report = FinancialComplianceReportingMetadata {
            report_id: 0,
            entity_id: 0,
            compliance_framework: FinancialComplianceFramework::GAAP,
            status: FinancialComplianceReportStatus::Approved,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_compliance_reporting(
            &mut report,
            1,
            10,
            FinancialComplianceFramework::IFRS,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.entity_id, 10);
        assert_eq!(report.compliance_framework, FinancialComplianceFramework::IFRS);
        assert_eq!(report.status, FinancialComplianceReportStatus::Draft);
        assert_eq!(report.created_at, 1000);
        assert_eq!(report.report_data_hash, [1u8; 32]);
        assert_eq!(report.bump, 255);
    }

    #[test]
    fn test_initialize_financial_compliance_reporting_invalid_id() {
        let mut report = FinancialComplianceReportingMetadata {
            report_id: 0,
            entity_id: 0,
            compliance_framework: FinancialComplianceFramework::GAAP,
            status: FinancialComplianceReportStatus::Draft,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_compliance_reporting(
            &mut report,
            0, // Invalid: must be > 0
            10,
            FinancialComplianceFramework::IFRS,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_compliance_reporting_all_frameworks() {
        let frameworks = vec![
            FinancialComplianceFramework::GAAP,
            FinancialComplianceFramework::IFRS,
            FinancialComplianceFramework::SOX,
            FinancialComplianceFramework::Custom,
        ];

        for framework in frameworks {
            let mut report = FinancialComplianceReportingMetadata {
                report_id: 0,
                entity_id: 0,
                compliance_framework: FinancialComplianceFramework::GAAP,
                status: FinancialComplianceReportStatus::Draft,
                created_at: 0,
                report_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_compliance_reporting(
                &mut report,
                1,
                10,
                framework,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.compliance_framework, framework);
        }
    }

    #[test]
    fn test_financial_compliance_framework_variants() {
        assert_eq!(FinancialComplianceFramework::GAAP, FinancialComplianceFramework::GAAP);
        assert_eq!(FinancialComplianceFramework::IFRS, FinancialComplianceFramework::IFRS);
        assert_eq!(FinancialComplianceFramework::SOX, FinancialComplianceFramework::SOX);
        assert_eq!(FinancialComplianceFramework::Custom, FinancialComplianceFramework::Custom);
    }

    #[test]
    fn test_financial_compliance_report_status_variants() {
        assert_eq!(FinancialComplianceReportStatus::Draft, FinancialComplianceReportStatus::Draft);
        assert_eq!(FinancialComplianceReportStatus::Submitted, FinancialComplianceReportStatus::Submitted);
        assert_eq!(FinancialComplianceReportStatus::Approved, FinancialComplianceReportStatus::Approved);
    }

    #[test]
    fn test_financial_compliance_framework_all_variants_unique() {
        let variants = vec![
            FinancialComplianceFramework::GAAP,
            FinancialComplianceFramework::IFRS,
            FinancialComplianceFramework::SOX,
            FinancialComplianceFramework::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_compliance_report_status_all_variants_unique() {
        let variants = vec![
            FinancialComplianceReportStatus::Draft,
            FinancialComplianceReportStatus::Submitted,
            FinancialComplianceReportStatus::Approved,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_submit_compliance_report() {
        let result = offchain::submit_compliance_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
