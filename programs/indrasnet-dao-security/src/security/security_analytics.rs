//! Security Analytics Module
//! 
//! Comprehensive security analytics and reporting system.
//! Integrates with SIEM, EDR, and other security modules for comprehensive analysis.
//! 
//! Hybrid model: Off-chain analytics and analysis, on-chain metadata storage
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Only analytics metadata, reports, and summary statistics
//! - OFF-CHAIN: All actual analytics processing, data aggregation, and detailed reports
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/security_analytics/
//! Combined from: part_1.rs, part_2.rs, part_3.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Analytics report type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AnalyticsReportType {
    /// Threat analysis report
    ThreatAnalysis,
    /// Compliance report
    Compliance,
    /// Performance report
    Performance,
    /// Security posture report
    SecurityPosture,
    /// Incident summary report
    IncidentSummary,
}

impl Space for AnalyticsReportType {
    const INIT_SPACE: usize = 1;
}

/// Analytics report
#[account]
#[derive(InitSpace)]
pub struct SecurityAnalyticsReport {
    /// Report ID
    pub report_id: u64,
    /// Idea ID (PDA seed)
    pub idea_id: u64,
    /// Report type
    pub report_type: AnalyticsReportType,
    /// Report title
    #[max_len(200)]
    pub title: String,
    /// Report metadata URI (IPFS or similar)
    #[max_len(500)]
    pub metadata_uri: String,
    /// Report hash (for verification)
    pub report_hash: [u8; 32],
    /// Summary statistics (JSON string, max 2000 chars)
    #[max_len(2000)]
    pub summary: String,
    /// Report generation timestamp
    pub generated_at: i64,
    /// Report generator (analytics system)
    pub generator: Pubkey,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Analytics metric
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct SecurityMetric {
    /// Metric name (max 100 chars)
    pub name: String,
    /// Metric value
    pub value: u64,
    /// Metric timestamp
    pub timestamp: i64,
}

impl Space for SecurityMetric {
    const INIT_SPACE: usize = 
        4 + 100 + // name: String (max 100 chars)
        8 + // value: u64
        8; // timestamp: i64
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Create security analytics report
pub fn create_security_analytics_report(
    report: &mut SecurityAnalyticsReport,
    report_id: u64,
    idea_id: u64,
    report_type: AnalyticsReportType,
    title: String,
    metadata_uri: String,
    report_hash: [u8; 32],
    summary: String,
    generator: Pubkey,
    bump: u8,
) -> Result<()> {
    require!(!title.is_empty(), IndrasError::InvalidInput);
    require!(title.len() <= 200, IndrasError::StringTooLong);
    require!(metadata_uri.len() <= 500, IndrasError::StringTooLong);
    require!(summary.len() <= 2000, IndrasError::StringTooLong);
    
    report.report_id = report_id;
    report.idea_id = idea_id;
    report.report_type = report_type;
    report.title = title;
    report.metadata_uri = metadata_uri;
    report.report_hash = report_hash;
    report.summary = summary;
    report.generated_at = Clock::get()?.unix_timestamp;
    report.generator = generator;
    report.bump = bump;
    
    msg!("Security analytics report {} created (type: {:?})", report_id, report_type);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_security_analytics_report() {
        let mut report = SecurityAnalyticsReport {
            report_id: 0,
            idea_id: 0,
            report_type: AnalyticsReportType::ThreatAnalysis,
            title: String::new(),
            metadata_uri: String::new(),
            report_hash: [0u8; 32],
            summary: String::new(),
            generated_at: 0,
            generator: Pubkey::default(),
            bump: 0,
        };

        let generator = Pubkey::from([1u8; 32]);
        let report_hash = [1u8; 32];

        let result = create_security_analytics_report(
            &mut report,
            1,
            42,
            AnalyticsReportType::ThreatAnalysis,
            "Test Report".to_string(),
            "ipfs://test".to_string(),
            report_hash,
            "Summary".to_string(),
            generator,
            254,
        );

        assert!(result.is_ok());
        assert_eq!(report.report_id, 1);
        assert_eq!(report.idea_id, 42);
        assert_eq!(report.title, "Test Report");
    }
}
