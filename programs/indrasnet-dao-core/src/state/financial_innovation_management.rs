//! Financial Innovation Management module
//!
//! Financial innovation management
//!
//! On-chain: Metadata for innovation
//! Off-chain: Actual innovation, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Innovation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInnovationType {
    /// Product innovation
    Product,
    /// Process innovation
    Process,
    /// Business model innovation
    BusinessModel,
    /// Custom innovation
    Custom,
}

/// Innovation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialInnovationStatus {
    /// Innovation active
    Active,
    /// Innovation paused
    Paused,
    /// Innovation implemented
    Implemented,
}

/// Financial innovation management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialInnovationManagementMetadata {
    /// Innovation ID
    pub innovation_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Innovation type
    pub innovation_type: FinancialInnovationType,
    /// Status
    pub status: FinancialInnovationStatus,
    /// Created at
    pub created_at: i64,
    /// Innovation data hash
    pub innovation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_innovation_management(
        innovation: &mut FinancialInnovationManagementMetadata,
        innovation_id: u64,
        entity_id: u64,
        innovation_type: FinancialInnovationType,
        innovation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(innovation_id > 0, IndrasError::InvalidInput);
        innovation.innovation_id = innovation_id;
        innovation.entity_id = entity_id;
        innovation.innovation_type = innovation_type;
        innovation.status = FinancialInnovationStatus::Active;
        innovation.created_at = current_time;
        innovation.innovation_data_hash = innovation_data_hash;
        innovation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_innovation(_innovation_id: u64) -> Vec<u8> {
        vec![]
    }
}
