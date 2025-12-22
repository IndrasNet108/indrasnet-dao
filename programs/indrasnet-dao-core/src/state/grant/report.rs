//! Grant Report Module
//!
//! This module manages grant reports for escrow and standard grants.
//! Reports are required before closing accounts for certain grant types.

use anchor_lang::prelude::*;

/// Grant Report Status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrantReportStatus {
    NotSubmitted,    // Report not submitted
    Submitted,       // Report submitted, awaiting approval
    Approved,        // Report approved
    Rejected,        // Report rejected
}

impl Space for GrantReportStatus {
    const INIT_SPACE: usize = 1;
}

/// Grant Report account structure
/// Created for each grant requiring a report
#[account]
#[derive(InitSpace)]
pub struct GrantReport {
    pub grant_id: u64,                  // Associated grant ID
    pub mesh_group: Pubkey,            // Mesh group that submitted report
    pub status: GrantReportStatus,     // Report status
    #[max_len(2000)]
    pub report_content: String,        // Report content
    #[max_len(500)]
    pub attachments_hash: Option<String>, // Attachments hash (IPFS or other storage)
    pub submitted_at: Option<i64>,      // Submission time
    pub approved_at: Option<i64>,       // Approval time
    pub approved_by: Option<Pubkey>,    // Who approved the report
    #[max_len(500)]
    pub rejection_reason: Option<String>, // Rejection reason (if rejected)
    pub bump: u8,
}

impl GrantReport {
    /// Checks if account can be closed based on report
    pub fn can_close_account(&self) -> bool {
        self.status == GrantReportStatus::Approved
    }
    
    /// Checks if report is submitted
    pub fn is_submitted(&self) -> bool {
        matches!(
            self.status,
            GrantReportStatus::Submitted | 
            GrantReportStatus::Approved | 
            GrantReportStatus::Rejected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_report_status_variants() {
        assert_eq!(GrantReportStatus::NotSubmitted, GrantReportStatus::NotSubmitted);
        assert_eq!(GrantReportStatus::Submitted, GrantReportStatus::Submitted);
        assert_eq!(GrantReportStatus::Approved, GrantReportStatus::Approved);
        assert_eq!(GrantReportStatus::Rejected, GrantReportStatus::Rejected);
    }

    #[test]
    fn test_can_close_account() {
        let mut report = GrantReport {
            grant_id: 1,
            mesh_group: Pubkey::new_unique(),
            status: GrantReportStatus::NotSubmitted,
            report_content: String::new(),
            attachments_hash: None,
            submitted_at: None,
            approved_at: None,
            approved_by: None,
            rejection_reason: None,
            bump: 0,
        };
        
        assert!(!report.can_close_account());
        
        report.status = GrantReportStatus::Submitted;
        assert!(!report.can_close_account());
        
        report.status = GrantReportStatus::Approved;
        assert!(report.can_close_account());
        
        report.status = GrantReportStatus::Rejected;
        assert!(!report.can_close_account());
    }

    #[test]
    fn test_is_submitted() {
        let mut report = GrantReport {
            grant_id: 1,
            mesh_group: Pubkey::new_unique(),
            status: GrantReportStatus::NotSubmitted,
            report_content: String::new(),
            attachments_hash: None,
            submitted_at: None,
            approved_at: None,
            approved_by: None,
            rejection_reason: None,
            bump: 0,
        };
        
        assert!(!report.is_submitted());
        
        report.status = GrantReportStatus::Submitted;
        assert!(report.is_submitted());
        
        report.status = GrantReportStatus::Approved;
        assert!(report.is_submitted());
        
        report.status = GrantReportStatus::Rejected;
        assert!(report.is_submitted());
    }
}
