//! Financial Digital Transformation module
//!
//! Financial digital transformation
//!
//! On-chain: Metadata for digital transformation
//! Off-chain: Actual transformation, management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Transformation area
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDigitalTransformationArea {
    /// Process automation
    ProcessAutomation,
    /// Data analytics
    DataAnalytics,
    /// Customer experience
    CustomerExperience,
    /// Custom area
    Custom,
}

/// Transformation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDigitalTransformationStatus {
    /// Transformation active
    Active,
    /// Transformation paused
    Paused,
    /// Transformation completed
    Completed,
}

/// Financial digital transformation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDigitalTransformationMetadata {
    /// Transformation ID
    pub transformation_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Transformation area
    pub transformation_area: FinancialDigitalTransformationArea,
    /// Status
    pub status: FinancialDigitalTransformationStatus,
    /// Created at
    pub created_at: i64,
    /// Transformation data hash
    pub transformation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_digital_transformation(
        transformation: &mut FinancialDigitalTransformationMetadata,
        transformation_id: u64,
        entity_id: u64,
        transformation_area: FinancialDigitalTransformationArea,
        transformation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(transformation_id > 0, IndrasError::InvalidInput);
        transformation.transformation_id = transformation_id;
        transformation.entity_id = entity_id;
        transformation.transformation_area = transformation_area;
        transformation.status = FinancialDigitalTransformationStatus::Active;
        transformation.created_at = current_time;
        transformation.transformation_data_hash = transformation_data_hash;
        transformation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_digital_transformation(_transformation_id: u64) -> Vec<u8> {
        vec![]
    }
}
