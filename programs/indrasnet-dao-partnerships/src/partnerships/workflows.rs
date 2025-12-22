//! Workflows module
//!
//! Partnership workflow management
//!
//! On-chain: Metadata for workflows
//! Off-chain: Actual workflow execution, automation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Workflow status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum WorkflowStatus {
    /// Workflow active
    Active,
    /// Workflow inactive
    Inactive,
    /// Workflow executing
    Executing,
    /// Workflow error
    Error,
}

/// Partnership workflow metadata (on-chain)
///
/// Stores metadata for partnership workflows
#[account]
#[derive(InitSpace)]
pub struct PartnershipWorkflowMetadata {
    /// Workflow ID
    pub workflow_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Workflow name
    #[max_len(100)]
    pub name: String,
    /// Status
    pub status: WorkflowStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Workflow config hash
    pub workflow_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for workflows
pub mod onchain {
    use super::*;

    /// Initialize partnership workflow
    pub fn initialize_partnership_workflow(
        workflow: &mut PartnershipWorkflowMetadata,
        workflow_id: u64,
        partnership_id: u64,
        name: String,
        workflow_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(workflow_id > 0, IndrasError::InvalidInput);
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= 100, IndrasError::InvalidInput);
        
        workflow.workflow_id = workflow_id;
        workflow.partnership_id = partnership_id;
        workflow.name = name;
        workflow.status = WorkflowStatus::Active;
        workflow.created_at = current_time;
        workflow.updated_at = current_time;
        workflow.workflow_config_hash = workflow_config_hash;
        workflow.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for workflows
pub mod offchain {
    /// Execute workflow
    pub fn execute_workflow(_workflow_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}
