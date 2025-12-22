//! Financial Approval Workflow module
//!
//! Financial approval workflow
//!
//! On-chain: Metadata for approval workflow
//! Off-chain: Actual workflow, approval process

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Approval level
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialApprovalLevel {
    /// Single approval
    Single,
    /// Multi-level approval
    MultiLevel,
    /// Delegated approval
    Delegated,
    /// Custom approval
    Custom,
}

/// Approval status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialApprovalWorkflowStatus {
    /// Workflow active
    Active,
    /// Workflow paused
    Paused,
    /// Workflow disabled
    Disabled,
}

/// Financial approval workflow metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialApprovalWorkflowMetadata {
    /// Workflow ID
    pub workflow_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Approval level
    pub approval_level: FinancialApprovalLevel,
    /// Status
    pub status: FinancialApprovalWorkflowStatus,
    /// Created at
    pub created_at: i64,
    /// Workflow config hash
    pub workflow_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_approval_workflow(
        workflow: &mut FinancialApprovalWorkflowMetadata,
        workflow_id: u64,
        entity_id: u64,
        approval_level: FinancialApprovalLevel,
        workflow_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(workflow_id > 0, IndrasError::InvalidInput);
        workflow.workflow_id = workflow_id;
        workflow.entity_id = entity_id;
        workflow.approval_level = approval_level;
        workflow.status = FinancialApprovalWorkflowStatus::Active;
        workflow.created_at = current_time;
        workflow.workflow_config_hash = workflow_config_hash;
        workflow.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_approval_workflow(_workflow_id: u64) -> Vec<u8> {
        vec![]
    }
}
