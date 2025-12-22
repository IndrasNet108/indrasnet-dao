//! Invoice Processing module
//!
//! Invoice processing and automation
//!
//! On-chain: Metadata for invoice processing
//! Off-chain: Actual processing, automation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Processing status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InvoiceProcessingStatus {
    /// Processing pending
    Pending,
    /// Processing in progress
    InProgress,
    /// Processing completed
    Completed,
    /// Processing failed
    Failed,
}

/// Invoice processing metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct InvoiceProcessingMetadata {
    /// Processing ID
    pub processing_id: u64,
    /// Invoice ID
    pub invoice_id: u64,
    /// Status
    pub status: InvoiceProcessingStatus,
    /// Created at
    pub created_at: i64,
    /// Completed at
    pub completed_at: Option<i64>,
    /// Processing data hash
    pub processing_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_invoice_processing(
        processing: &mut InvoiceProcessingMetadata,
        processing_id: u64,
        invoice_id: u64,
        processing_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(processing_id > 0, IndrasError::InvalidInput);
        processing.processing_id = processing_id;
        processing.invoice_id = invoice_id;
        processing.status = InvoiceProcessingStatus::Pending;
        processing.created_at = current_time;
        processing.completed_at = None;
        processing.processing_data_hash = processing_data_hash;
        processing.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn process_invoice(_processing_id: u64) -> Vec<u8> {
        vec![]
    }
}
