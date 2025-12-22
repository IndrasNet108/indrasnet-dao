//! Event definitions for the IndrasNet DAO Security program

use anchor_lang::prelude::*;
use crate::types::RiskLevel;

/// Event: Security check request
///
/// Emitted when Security module receives request to check transaction/proposal
/// Off-chain ML service listens to this event and performs analysis
///
/// Used for:
/// - Real-time threat detection via ML models
/// - Anomaly detection in system
/// - Integration with Solana via CPI
/// - Monitoring and logging through Security module
#[event]
pub struct SecurityCheckRequested {
    /// Transaction/proposal hash for identification
    pub transaction_hash: [u8; 32],
    /// Transaction type (proposal, transfer, etc.)
    pub transaction_type: String,
    /// Transaction initiator
    pub caller: Pubkey,
    /// Timestamp
    pub timestamp: i64,
    /// Data for analysis (hash or first N bytes)
    pub data_hash: [u8; 32],
    /// Additional data (title, description for proposals)
    pub metadata: String,
}

/// Event: Security check completed
///
/// Emitted when ML analysis is complete and results are ready
///
/// Contains results from:
/// - Real-time threat detection ML model
/// - Anomaly detection ML model
/// - Integrated via CPI and logged through Security module
#[event]
pub struct SecurityCheckCompleted {
    /// Transaction/proposal hash
    pub transaction_hash: [u8; 32],
    /// Risk level (if any)
    pub risk_level: Option<RiskLevel>,
    /// Threat detection score (0-100) from ML model
    pub threat_score: u8,
    /// Anomaly detection score (0-100) from ML model
    pub anomaly_score: u8,
    /// Threat details
    pub threat_details: String,
    /// Recommendation
    pub recommendation: String,
    /// ML analysis complete
    pub ml_analysis_complete: bool,
    /// Timestamp
    pub timestamp: i64,
}

/// Event: Security check status updated
///
/// Emitted when check status is updated (e.g., after DAO voting)
#[event]
pub struct SecurityCheckStatusUpdated {
    /// Transaction/proposal hash
    pub transaction_hash: [u8; 32],
    /// New status
    pub status: String,
    /// Timestamp
    pub timestamp: i64,
}

/// Event: Compliance check updated
#[event]
pub struct ComplianceCheckUpdated {
    /// Subject of compliance record
    pub subject: Pubkey,
    /// Admin who performed update
    pub admin: Pubkey,
    /// New status
    pub status: crate::security::compliance_checking::ComplianceCheckStatus,
}

/// Event: Security analytics report created
#[event]
pub struct SecurityAnalyticsReportCreated {
    /// Idea ID (analytics scope)
    pub idea_id: u64,
    /// Report ID
    pub report_id: u64,
    /// Admin who created report
    pub admin: Pubkey,
    /// Report type
    pub report_type: crate::security::security_analytics::AnalyticsReportType,
}
