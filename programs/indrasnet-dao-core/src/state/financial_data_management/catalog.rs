//! Financial Data Catalog module
//!
//! Financial data catalog
//!
//! On-chain: Metadata for data catalog
//! Off-chain: Actual catalog, metadata management

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Catalog type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataCatalogType {
    /// Business glossary
    BusinessGlossary,
    /// Data dictionary
    DataDictionary,
    /// Metadata catalog
    MetadataCatalog,
    /// Custom catalog
    Custom,
}

/// Catalog status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataCatalogStatus {
    /// Catalog active
    Active,
    /// Catalog paused
    Paused,
    /// Catalog disabled
    Disabled,
}

/// Financial data catalog metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataCatalogMetadata {
    /// Catalog ID
    pub catalog_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Catalog type
    pub catalog_type: FinancialDataCatalogType,
    /// Status
    pub status: FinancialDataCatalogStatus,
    /// Created at
    pub created_at: i64,
    /// Catalog config hash
    pub catalog_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_catalog(
        catalog: &mut FinancialDataCatalogMetadata,
        catalog_id: u64,
        entity_id: u64,
        catalog_type: FinancialDataCatalogType,
        catalog_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(catalog_id > 0, IndrasError::InvalidInput);
        catalog.catalog_id = catalog_id;
        catalog.entity_id = entity_id;
        catalog.catalog_type = catalog_type;
        catalog.status = FinancialDataCatalogStatus::Active;
        catalog.created_at = current_time;
        catalog.catalog_config_hash = catalog_config_hash;
        catalog.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_data_catalog(_catalog_id: u64) -> Vec<u8> {
        vec![]
    }
}
