//! Internal Audit module
//!
//! Internal audit management
//!
//! On-chain: Metadata for internal audits
//! Off-chain: Actual audit, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Audit type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InternalAuditType {
    /// Financial audit
    Financial,
    /// Operational audit
    Operational,
    /// Compliance audit
    Compliance,
    /// Custom audit
    Custom,
}

/// Audit status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InternalAuditStatus {
    /// Audit scheduled
    Scheduled,
    /// Audit in progress
    InProgress,
    /// Audit completed
    Completed,
}

/// Internal audit metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct InternalAuditMetadata {
    /// Audit ID
    pub audit_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Audit type
    pub audit_type: InternalAuditType,
    /// Status
    pub status: InternalAuditStatus,
    /// Created at
    pub created_at: i64,
    /// Audit data hash
    pub audit_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_internal_audit(
        audit: &mut InternalAuditMetadata,
        audit_id: u64,
        entity_id: u64,
        audit_type: InternalAuditType,
        audit_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(audit_id > 0, IndrasError::InvalidInput);
        audit.audit_id = audit_id;
        audit.entity_id = entity_id;
        audit.audit_type = audit_type;
        audit.status = InternalAuditStatus::Scheduled;
        audit.created_at = current_time;
        audit.audit_data_hash = audit_data_hash;
        audit.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn conduct_internal_audit(_audit_id: u64) -> Vec<u8> {
        vec![]
    }
}
