//! Proposal Execution Module
//!
//! This module manages proposal execution lifecycle.
//! Split into submodules:
//! - types: ExecutionStatus enum
//! - execution: ProposalExecution struct and lifecycle methods

use anchor_lang::prelude::*;

pub mod types;
pub mod execution;

// Re-export types
pub use types::ExecutionStatus;

/// Proposal execution account
#[account]
#[derive(Debug, PartialEq, InitSpace)]
pub struct ProposalExecution {
    pub id: u64,
    pub proposal_id: u64,
    pub executor: Pubkey,
    pub executed_at: i64,
    /// SECURITY: Timestamp when proposal was passed (for execution delay check)
    pub passed_at: Option<i64>,
    /// SECURITY: Minimum timestamp when execution can occur (timelock)
    pub execution_allowed_at: Option<i64>,
    /// SECURITY: Compliance proof (cryptographic proof of security compliance)
    pub compliance_proof: Option<crate::state::security::ComplianceProof>,
    #[max_len(1000)]
    pub execution_data: String,
    pub status: ExecutionStatus,
    pub bump: u8,
}
