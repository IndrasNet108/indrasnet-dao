//! Financial Data Transformation module
//!
//! Financial data transformation
//!
//! On-chain: Metadata for data transformation
//! Off-chain: Actual transformation, processing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Transformation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataTransformationType {
    /// Format transformation
    Format,
    /// Structure transformation
    Structure,
    /// Value transformation
    Value,
    /// Custom transformation
    Custom,
}

/// Transformation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataTransformationStatus {
    /// Transformation pending
    Pending,
    /// Transformation in progress
    InProgress,
    /// Transformation completed
    Completed,
}

/// Financial data transformation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataTransformationMetadata {
    /// Transformation ID
    pub transformation_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Transformation type
    pub transformation_type: FinancialDataTransformationType,
    /// Status
    pub status: FinancialDataTransformationStatus,
    /// Created at
    pub created_at: i64,
    /// Transformation config hash
    pub transformation_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_transformation(
        transformation: &mut FinancialDataTransformationMetadata,
        transformation_id: u64,
        entity_id: u64,
        transformation_type: FinancialDataTransformationType,
        transformation_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(transformation_id > 0, IndrasError::InvalidInput);
        transformation.transformation_id = transformation_id;
        transformation.entity_id = entity_id;
        transformation.transformation_type = transformation_type;
        transformation.status = FinancialDataTransformationStatus::Pending;
        transformation.created_at = current_time;
        transformation.transformation_config_hash = transformation_config_hash;
        transformation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn transform_financial_data(_transformation_id: u64) -> Vec<u8> {
        vec![]
    }
}
