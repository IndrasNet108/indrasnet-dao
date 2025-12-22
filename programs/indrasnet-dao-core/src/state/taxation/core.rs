//! Taxation module
//!
//! Taxation management
//!
//! On-chain: Metadata for tax records
//! Off-chain: Actual tax calculations, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Tax type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxType {
    /// Income tax
    Income,
    /// Capital gains tax
    CapitalGains,
    /// Value added tax
    VAT,
    /// Custom tax
    Custom,
}

/// Tax record status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TaxRecordStatus {
    /// Record draft
    Draft,
    /// Record filed
    Filed,
    /// Record paid
    Paid,
}

/// Tax record metadata (on-chain)
///
/// Stores metadata for tax records
#[account]
#[derive(InitSpace)]
pub struct TaxRecordMetadata {
    /// Record ID
    pub record_id: u64,
    /// Tax type
    pub tax_type: TaxType,
    /// Tax amount (in smallest unit)
    pub tax_amount: u64,
    /// Status
    pub status: TaxRecordStatus,
    /// Created at
    pub created_at: i64,
    /// Filed at
    pub filed_at: Option<i64>,
    /// Tax data hash
    pub tax_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for taxation
pub mod onchain {
    use super::*;

    /// Initialize tax record
    pub fn initialize_tax_record(
        record: &mut TaxRecordMetadata,
        record_id: u64,
        tax_type: TaxType,
        tax_amount: u64,
        tax_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(record_id > 0, IndrasError::InvalidInput);
        require!(tax_amount > 0, IndrasError::InvalidInput);
        
        record.record_id = record_id;
        record.tax_type = tax_type;
        record.tax_amount = tax_amount;
        record.status = TaxRecordStatus::Draft;
        record.created_at = current_time;
        record.filed_at = None;
        record.tax_data_hash = tax_data_hash;
        record.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for taxation
pub mod offchain {
    /// Calculate tax
    pub fn calculate_tax(_record_id: u64) -> u64 {
        // Implementation in off-chain service
        0
    }
}
