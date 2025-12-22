//! Partnership Analytics Reporting module
//!
//! Partnership analytics reporting
//!
//! On-chain: Metadata for reporting
//! Off-chain: Actual reporting, generation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Report format
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipReportFormat {
    /// PDF format
    PDF,
    /// Excel format
    Excel,
    /// JSON format
    JSON,
    /// Custom format
    Custom,
}

/// Report status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipReportStatus {
    /// Report generating
    Generating,
    /// Report ready
    Ready,
    /// Report delivered
    Delivered,
}

/// Partnership analytics reporting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsReportingMetadata {
    /// Report ID
    pub report_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Report format
    pub report_format: PartnershipReportFormat,
    /// Status
    pub status: PartnershipReportStatus,
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
    pub fn initialize_partnership_analytics_reporting(
        report: &mut PartnershipAnalyticsReportingMetadata,
        report_id: u64,
        partnership_id: u64,
        report_format: PartnershipReportFormat,
        report_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(report_id > 0, IndrasError::InvalidInput);
        report.report_id = report_id;
        report.partnership_id = partnership_id;
        report.report_format = report_format;
        report.status = PartnershipReportStatus::Generating;
        report.created_at = current_time;
        report.report_data_hash = report_data_hash;
        report.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_report(_report_id: u64) -> Vec<u8> {
        vec![]
    }
}
