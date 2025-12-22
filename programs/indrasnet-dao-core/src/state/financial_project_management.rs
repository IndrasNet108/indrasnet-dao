//! Financial Project Management module
//!
//! Financial project management
//!
//! On-chain: Metadata for projects
//! Off-chain: Actual projects, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Project type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialProjectType {
    /// Capital project
    Capital,
    /// Operational project
    Operational,
    /// Strategic project
    Strategic,
    /// Custom project
    Custom,
}

/// Project status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialProjectStatus {
    /// Project pending
    Pending,
    /// Project in progress
    InProgress,
    /// Project completed
    Completed,
}

/// Financial project management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialProjectManagementMetadata {
    /// Project ID
    pub project_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Project type
    pub project_type: FinancialProjectType,
    /// Status
    pub status: FinancialProjectStatus,
    /// Created at
    pub created_at: i64,
    /// Project data hash
    pub project_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_project_management(
        project: &mut FinancialProjectManagementMetadata,
        project_id: u64,
        entity_id: u64,
        project_type: FinancialProjectType,
        project_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(project_id > 0, IndrasError::InvalidInput);
        project.project_id = project_id;
        project.entity_id = entity_id;
        project.project_type = project_type;
        project.status = FinancialProjectStatus::Pending;
        project.created_at = current_time;
        project.project_data_hash = project_data_hash;
        project.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_project(_project_id: u64) -> Vec<u8> {
        vec![]
    }
}
