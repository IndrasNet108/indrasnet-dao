//! Financial Workflow module
//!
//! Financial workflow automation
//!
//! On-chain: Metadata for financial workflows
//! Off-chain: Actual workflow, automation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Workflow type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWorkflowType {
    /// Approval workflow
    Approval,
    /// Processing workflow
    Processing,
    /// Reporting workflow
    Reporting,
    /// Custom workflow
    Custom,
}

/// Workflow status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialWorkflowStatus {
    /// Workflow active
    Active,
    /// Workflow paused
    Paused,
    /// Workflow disabled
    Disabled,
}

/// Financial workflow metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialWorkflowMetadata {
    /// Workflow ID
    pub workflow_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Workflow type
    pub workflow_type: FinancialWorkflowType,
    /// Status
    pub status: FinancialWorkflowStatus,
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
    pub fn initialize_financial_workflow(
        workflow: &mut FinancialWorkflowMetadata,
        workflow_id: u64,
        entity_id: u64,
        workflow_type: FinancialWorkflowType,
        workflow_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(workflow_id > 0, IndrasError::InvalidInput);
        workflow.workflow_id = workflow_id;
        workflow.entity_id = entity_id;
        workflow.workflow_type = workflow_type;
        workflow.status = FinancialWorkflowStatus::Active;
        workflow.created_at = current_time;
        workflow.workflow_config_hash = workflow_config_hash;
        workflow.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_financial_workflow(_workflow_id: u64) -> Vec<u8> {
        vec![]
    }
}
