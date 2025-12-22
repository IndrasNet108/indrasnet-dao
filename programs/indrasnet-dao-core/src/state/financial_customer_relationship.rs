//! Financial Customer Relationship module
//!
//! Financial customer relationship management
//!
//! On-chain: Metadata for customer relationships
//! Off-chain: Actual relationships, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Relationship type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCustomerRelationshipType {
    /// B2B relationship
    B2B,
    /// B2C relationship
    B2C,
    /// B2G relationship
    B2G,
    /// Custom relationship
    Custom,
}

/// Relationship status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCustomerRelationshipStatus {
    /// Relationship active
    Active,
    /// Relationship paused
    Paused,
    /// Relationship optimized
    Optimized,
}

/// Financial customer relationship metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialCustomerRelationshipMetadata {
    /// Relationship ID
    pub relationship_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Relationship type
    pub relationship_type: FinancialCustomerRelationshipType,
    /// Status
    pub status: FinancialCustomerRelationshipStatus,
    /// Created at
    pub created_at: i64,
    /// Relationship data hash
    pub relationship_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_customer_relationship(
        relationship: &mut FinancialCustomerRelationshipMetadata,
        relationship_id: u64,
        entity_id: u64,
        relationship_type: FinancialCustomerRelationshipType,
        relationship_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(relationship_id > 0, IndrasError::InvalidInput);
        relationship.relationship_id = relationship_id;
        relationship.entity_id = entity_id;
        relationship.relationship_type = relationship_type;
        relationship.status = FinancialCustomerRelationshipStatus::Active;
        relationship.created_at = current_time;
        relationship.relationship_data_hash = relationship_data_hash;
        relationship.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_customer_relationship(_relationship_id: u64) -> Vec<u8> {
        vec![]
    }
}
