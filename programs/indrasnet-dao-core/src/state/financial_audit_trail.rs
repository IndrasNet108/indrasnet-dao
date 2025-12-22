//! Financial Audit Trail module
//!
//! Financial audit trail
//!
//! On-chain: Metadata for audit trail
//! Off-chain: Actual trail, logging

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Trail event type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialAuditTrailEventType {
    /// Transaction event
    Transaction,
    /// Modification event
    Modification,
    /// Access event
    Access,
    /// Custom event
    Custom,
}

/// Trail status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialAuditTrailStatus {
    /// Trail active
    Active,
    /// Trail paused
    Paused,
    /// Trail disabled
    Disabled,
}

/// Financial audit trail metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialAuditTrailMetadata {
    /// Trail ID
    pub trail_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Trail event type
    pub trail_event_type: FinancialAuditTrailEventType,
    /// Status
    pub status: FinancialAuditTrailStatus,
    /// Created at
    pub created_at: i64,
    /// Trail config hash
    pub trail_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_audit_trail(
        trail: &mut FinancialAuditTrailMetadata,
        trail_id: u64,
        entity_id: u64,
        trail_event_type: FinancialAuditTrailEventType,
        trail_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(trail_id > 0, IndrasError::InvalidInput);
        trail.trail_id = trail_id;
        trail.entity_id = entity_id;
        trail.trail_event_type = trail_event_type;
        trail.status = FinancialAuditTrailStatus::Active;
        trail.created_at = current_time;
        trail.trail_config_hash = trail_config_hash;
        trail.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn log_audit_trail(_trail_id: u64) -> Vec<u8> {
        vec![]
    }
}
