//! Approvals module
//!
//! Partnership approval workflow management
//!
//! On-chain: Metadata for approvals, approval status
//! Off-chain: Actual approval workflow execution

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Approval status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ApprovalStatus {
    /// Approval pending
    Pending,
    /// Approval approved
    Approved,
    /// Approval rejected
    Rejected,
    /// Approval cancelled
    Cancelled,
}

/// Partnership approval metadata (on-chain)
///
/// Stores metadata for partnership approvals
#[account]
#[derive(InitSpace)]
pub struct PartnershipApprovalMetadata {
    /// Approval ID
    pub approval_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Approver pubkey
    pub approver_pubkey: Pubkey,
    /// Status
    pub status: ApprovalStatus,
    /// Created at
    pub created_at: i64,
    /// Decided at
    pub decided_at: Option<i64>,
    /// Approval data hash
    pub approval_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for approvals
pub mod onchain {
    use super::*;

    /// Initialize partnership approval
    pub fn initialize_partnership_approval(
        approval: &mut PartnershipApprovalMetadata,
        approval_id: u64,
        partnership_id: u64,
        approver_pubkey: Pubkey,
        approval_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(approval_id > 0, IndrasError::InvalidInput);
        
        approval.approval_id = approval_id;
        approval.partnership_id = partnership_id;
        approval.approver_pubkey = approver_pubkey;
        approval.status = ApprovalStatus::Pending;
        approval.created_at = current_time;
        approval.decided_at = None;
        approval.approval_data_hash = approval_data_hash;
        approval.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for approvals
pub mod offchain {
    /// Process approval
    pub fn process_approval(_approval_id: u64, _approved: bool) -> bool {
        // Implementation in off-chain service
        false
    }
}
