//! Financial Regulatory Reporting module
//!
//! Financial regulatory reporting
//!
//! On-chain: Metadata for regulatory reporting
//! Off-chain: Actual reporting, submission

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Regulatory requirement
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRegulatoryRequirement {
    /// SEC reporting
    SEC,
    /// IRS reporting
    IRS,
    /// State reporting
    State,
    /// Custom requirement
    Custom,
}

/// Reporting status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialRegulatoryReportStatus {
    /// Report draft
    Draft,
    /// Report submitted
    Submitted,
    /// Report approved
    Approved,
}

/// Financial regulatory reporting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialRegulatoryReportingMetadata {
    /// Report ID
    pub report_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Regulatory requirement
    pub regulatory_requirement: FinancialRegulatoryRequirement,
    /// Status
    pub status: FinancialRegulatoryReportStatus,
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
    pub fn initialize_financial_regulatory_reporting(
        report: &mut FinancialRegulatoryReportingMetadata,
        report_id: u64,
        entity_id: u64,
        regulatory_requirement: FinancialRegulatoryRequirement,
        report_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        report.report_id = report_id;
        report.entity_id = entity_id;
        report.regulatory_requirement = regulatory_requirement;
        report.status = FinancialRegulatoryReportStatus::Draft;
        report.created_at = current_time;
        report.report_data_hash = report_data_hash;
        report.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn submit_regulatory_report(_report_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_regulatory_reporting() {
        let mut report = FinancialRegulatoryReportingMetadata {
            report_id: 0,
            entity_id: 0,
            regulatory_requirement: FinancialRegulatoryRequirement::SEC,
            status: FinancialRegulatoryReportStatus::Approved,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_regulatory_reporting(
            &mut report,
            1,
            10,
            FinancialRegulatoryRequirement::IRS,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.entity_id, 10);
        assert_eq!(report.regulatory_requirement, FinancialRegulatoryRequirement::IRS);
        assert_eq!(report.status, FinancialRegulatoryReportStatus::Draft);
        assert_eq!(report.created_at, 1000);
        assert_eq!(report.report_data_hash, [1u8; 32]);
        assert_eq!(report.bump, 255);
    }

    #[test]
    fn test_initialize_financial_regulatory_reporting_invalid_id() {
        let mut report = FinancialRegulatoryReportingMetadata {
            report_id: 0,
            entity_id: 0,
            regulatory_requirement: FinancialRegulatoryRequirement::SEC,
            status: FinancialRegulatoryReportStatus::Draft,
            created_at: 0,
            report_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_regulatory_reporting(
            &mut report,
            0, // Invalid: must be > 0
            10,
            FinancialRegulatoryRequirement::IRS,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_regulatory_reporting_all_requirements() {
        let requirements = vec![
            FinancialRegulatoryRequirement::SEC,
            FinancialRegulatoryRequirement::IRS,
            FinancialRegulatoryRequirement::State,
            FinancialRegulatoryRequirement::Custom,
        ];

        for requirement in requirements {
            let mut report = FinancialRegulatoryReportingMetadata {
                report_id: 0,
                entity_id: 0,
                regulatory_requirement: FinancialRegulatoryRequirement::SEC,
                status: FinancialRegulatoryReportStatus::Draft,
                created_at: 0,
                report_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_regulatory_reporting(
                &mut report,
                1,
                10,
                requirement,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(report.regulatory_requirement, requirement);
        }
    }

    #[test]
    fn test_financial_regulatory_requirement_variants() {
        assert_eq!(FinancialRegulatoryRequirement::SEC, FinancialRegulatoryRequirement::SEC);
        assert_eq!(FinancialRegulatoryRequirement::IRS, FinancialRegulatoryRequirement::IRS);
        assert_eq!(FinancialRegulatoryRequirement::State, FinancialRegulatoryRequirement::State);
        assert_eq!(FinancialRegulatoryRequirement::Custom, FinancialRegulatoryRequirement::Custom);
    }

    #[test]
    fn test_financial_regulatory_report_status_variants() {
        assert_eq!(FinancialRegulatoryReportStatus::Draft, FinancialRegulatoryReportStatus::Draft);
        assert_eq!(FinancialRegulatoryReportStatus::Submitted, FinancialRegulatoryReportStatus::Submitted);
        assert_eq!(FinancialRegulatoryReportStatus::Approved, FinancialRegulatoryReportStatus::Approved);
    }

    #[test]
    fn test_financial_regulatory_requirement_all_variants_unique() {
        let variants = vec![
            FinancialRegulatoryRequirement::SEC,
            FinancialRegulatoryRequirement::IRS,
            FinancialRegulatoryRequirement::State,
            FinancialRegulatoryRequirement::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_regulatory_report_status_all_variants_unique() {
        let variants = vec![
            FinancialRegulatoryReportStatus::Draft,
            FinancialRegulatoryReportStatus::Submitted,
            FinancialRegulatoryReportStatus::Approved,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_submit_regulatory_report() {
        let result = offchain::submit_regulatory_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
