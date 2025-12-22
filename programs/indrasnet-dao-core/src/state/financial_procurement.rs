//! Financial Procurement module
//!
//! Financial procurement management
//!
//! On-chain: Metadata for procurement
//! Off-chain: Actual procurement, process management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Procurement type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialProcurementType {
    /// Direct procurement
    Direct,
    /// Competitive bidding
    CompetitiveBidding,
    /// Framework agreement
    FrameworkAgreement,
    /// Custom type
    Custom,
}

/// Procurement status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialProcurementStatus {
    /// Procurement pending
    Pending,
    /// Procurement in progress
    InProgress,
    /// Procurement completed
    Completed,
}

/// Financial procurement metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialProcurementMetadata {
    /// Procurement ID
    pub procurement_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Procurement type
    pub procurement_type: FinancialProcurementType,
    /// Status
    pub status: FinancialProcurementStatus,
    /// Created at
    pub created_at: i64,
    /// Procurement data hash
    pub procurement_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_procurement(
        procurement: &mut FinancialProcurementMetadata,
        procurement_id: u64,
        entity_id: u64,
        procurement_type: FinancialProcurementType,
        procurement_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(procurement_id > 0, IndrasError::InvalidInput);
        procurement.procurement_id = procurement_id;
        procurement.entity_id = entity_id;
        procurement.procurement_type = procurement_type;
        procurement.status = FinancialProcurementStatus::Pending;
        procurement.created_at = current_time;
        procurement.procurement_data_hash = procurement_data_hash;
        procurement.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_procurement(_procurement_id: u64) -> Vec<u8> {
        vec![]
    }
}
