//! Invoice Management module
//!
//! Invoice management
//!
//! On-chain: Metadata for invoices
//! Off-chain: Actual invoice generation, delivery

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Invoice status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum InvoiceStatus {
    /// Invoice draft
    Draft,
    /// Invoice sent
    Sent,
    /// Invoice paid
    Paid,
    /// Invoice overdue
    Overdue,
    /// Invoice cancelled
    Cancelled,
}

/// Invoice metadata (on-chain)
///
/// Stores metadata for invoices
#[account]
#[derive(InitSpace)]
pub struct InvoiceMetadata {
    /// Invoice ID
    pub invoice_id: u64,
    /// Amount (in smallest unit)
    pub amount: u64,
    /// Status
    pub status: InvoiceStatus,
    /// Created at
    pub created_at: i64,
    /// Due date
    pub due_date: Option<i64>,
    /// Paid at
    pub paid_at: Option<i64>,
    /// Invoice data hash
    pub invoice_data_hash: [u8; 32],
    /// Invoice URI
    #[max_len(200)]
    pub invoice_uri: String,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for invoice management
pub mod onchain {
    use super::*;

    /// Initialize invoice
    pub fn initialize_invoice(
        invoice: &mut InvoiceMetadata,
        invoice_id: u64,
        amount: u64,
        invoice_data_hash: [u8; 32],
        invoice_uri: String,
        due_date: Option<i64>,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(invoice_id > 0, IndrasError::InvalidInput);
        require!(amount > 0, IndrasError::InvalidInput);
        require!(invoice_uri.len() <= 200, IndrasError::InvalidInput);
        
        invoice.invoice_id = invoice_id;
        invoice.amount = amount;
        invoice.status = InvoiceStatus::Draft;
        invoice.created_at = current_time;
        invoice.due_date = due_date;
        invoice.paid_at = None;
        invoice.invoice_data_hash = invoice_data_hash;
        invoice.invoice_uri = invoice_uri;
        invoice.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for invoice management
pub mod offchain {
    /// Generate invoice
    pub fn generate_invoice(_invoice_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}
