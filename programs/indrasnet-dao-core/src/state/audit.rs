//! Audit module
//!
//! Financial audit management
//!
//! On-chain: Metadata for audits
//! Off-chain: Actual audit execution, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Audit type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AuditType {
    /// Internal audit
    Internal,
    /// External audit
    External,
    /// Regulatory audit
    Regulatory,
}

/// Audit status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AuditStatus {
    /// Audit scheduled
    Scheduled,
    /// Audit in progress
    InProgress,
    /// Audit completed
    Completed,
}

/// Audit metadata (on-chain)
///
/// Stores metadata for audits
#[account]
#[derive(InitSpace)]
pub struct AuditMetadata {
    /// Audit ID
    pub audit_id: u64,
    /// Audit type
    pub audit_type: AuditType,
    /// Status
    pub status: AuditStatus,
    /// Created at
    pub created_at: i64,
    /// Completed at
    pub completed_at: Option<i64>,
    /// Audit data hash
    pub audit_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for audit
pub mod onchain {
    use super::*;

    /// Initialize audit
    pub fn initialize_audit(
        audit: &mut AuditMetadata,
        audit_id: u64,
        audit_type: AuditType,
        audit_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(audit_id > 0, IndrasError::InvalidInput);
        
        audit.audit_id = audit_id;
        audit.audit_type = audit_type;
        audit.status = AuditStatus::Scheduled;
        audit.created_at = current_time;
        audit.completed_at = None;
        audit.audit_data_hash = audit_data_hash;
        audit.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for audit
pub mod offchain {
    /// Execute audit
    pub fn execute_audit(_audit_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
