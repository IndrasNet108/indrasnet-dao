//! Financial Data Enrichment module
//!
//! Financial data enrichment
//!
//! On-chain: Metadata for data enrichment
//! Off-chain: Actual enrichment, enhancement

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Enrichment type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataEnrichmentType {
    /// External data enrichment
    ExternalData,
    /// Calculated fields enrichment
    CalculatedFields,
    /// Reference data enrichment
    ReferenceData,
    /// Custom enrichment
    Custom,
}

/// Enrichment status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataEnrichmentStatus {
    /// Enrichment active
    Active,
    /// Enrichment paused
    Paused,
    /// Enrichment disabled
    Disabled,
}

/// Financial data enrichment metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataEnrichmentMetadata {
    /// Enrichment ID
    pub enrichment_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Enrichment type
    pub enrichment_type: FinancialDataEnrichmentType,
    /// Status
    pub status: FinancialDataEnrichmentStatus,
    /// Created at
    pub created_at: i64,
    /// Enrichment config hash
    pub enrichment_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_enrichment(
        enrichment: &mut FinancialDataEnrichmentMetadata,
        enrichment_id: u64,
        entity_id: u64,
        enrichment_type: FinancialDataEnrichmentType,
        enrichment_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(enrichment_id > 0, IndrasError::InvalidInput);
        enrichment.enrichment_id = enrichment_id;
        enrichment.entity_id = entity_id;
        enrichment.enrichment_type = enrichment_type;
        enrichment.status = FinancialDataEnrichmentStatus::Active;
        enrichment.created_at = current_time;
        enrichment.enrichment_config_hash = enrichment_config_hash;
        enrichment.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn enrich_financial_data(_enrichment_id: u64) -> Vec<u8> {
        vec![]
    }
}
