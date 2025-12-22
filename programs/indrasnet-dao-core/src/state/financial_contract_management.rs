//! Financial Contract Management module
//!
//! Financial contract management
//!
//! On-chain: Metadata for contracts
//! Off-chain: Actual contracts, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Contract type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialContractType {
    /// Service contract
    Service,
    /// Supply contract
    Supply,
    /// Lease contract
    Lease,
    /// Custom contract
    Custom,
}

/// Contract status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialContractStatus {
    /// Contract active
    Active,
    /// Contract expired
    Expired,
    /// Contract terminated
    Terminated,
}

/// Financial contract management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialContractManagementMetadata {
    /// Contract ID
    pub contract_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Contract type
    pub contract_type: FinancialContractType,
    /// Status
    pub status: FinancialContractStatus,
    /// Created at
    pub created_at: i64,
    /// Contract data hash
    pub contract_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_contract_management(
        contract: &mut FinancialContractManagementMetadata,
        contract_id: u64,
        entity_id: u64,
        contract_type: FinancialContractType,
        contract_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(contract_id > 0, IndrasError::InvalidInput);
        contract.contract_id = contract_id;
        contract.entity_id = entity_id;
        contract.contract_type = contract_type;
        contract.status = FinancialContractStatus::Active;
        contract.created_at = current_time;
        contract.contract_data_hash = contract_data_hash;
        contract.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_contract(_contract_id: u64) -> Vec<u8> {
        vec![]
    }
}
