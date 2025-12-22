//! Financial Stakeholder Management module
//!
//! Financial stakeholder management
//!
//! On-chain: Metadata for stakeholders
//! Off-chain: Actual stakeholders, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Stakeholder type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialStakeholderType {
    /// Shareholders
    Shareholders,
    /// Employees
    Employees,
    /// Customers
    Customers,
    /// Suppliers
    Suppliers,
}

/// Stakeholder status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialStakeholderStatus {
    /// Stakeholder active
    Active,
    /// Stakeholder paused
    Paused,
    /// Stakeholder engaged
    Engaged,
}

/// Financial stakeholder management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialStakeholderManagementMetadata {
    /// Stakeholder ID
    pub stakeholder_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Stakeholder type
    pub stakeholder_type: FinancialStakeholderType,
    /// Status
    pub status: FinancialStakeholderStatus,
    /// Created at
    pub created_at: i64,
    /// Stakeholder data hash
    pub stakeholder_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_stakeholder_management(
        stakeholder: &mut FinancialStakeholderManagementMetadata,
        stakeholder_id: u64,
        entity_id: u64,
        stakeholder_type: FinancialStakeholderType,
        stakeholder_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(stakeholder_id > 0, IndrasError::InvalidInput);
        stakeholder.stakeholder_id = stakeholder_id;
        stakeholder.entity_id = entity_id;
        stakeholder.stakeholder_type = stakeholder_type;
        stakeholder.status = FinancialStakeholderStatus::Active;
        stakeholder.created_at = current_time;
        stakeholder.stakeholder_data_hash = stakeholder_data_hash;
        stakeholder.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_stakeholder(_stakeholder_id: u64) -> Vec<u8> {
        vec![]
    }
}
