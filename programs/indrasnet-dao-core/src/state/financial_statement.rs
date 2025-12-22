//! Financial Statement module
//!
//! Financial statement generation
//!
//! On-chain: Metadata for financial statements
//! Off-chain: Actual generation, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Statement type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialStatementType {
    /// Balance sheet
    BalanceSheet,
    /// Income statement
    IncomeStatement,
    /// Cash flow statement
    CashFlowStatement,
    /// Custom statement
    Custom,
}

/// Statement status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialStatementStatus {
    /// Statement draft
    Draft,
    /// Statement final
    Final,
    /// Statement published
    Published,
}

/// Financial statement metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialStatementMetadata {
    /// Statement ID
    pub statement_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Statement type
    pub statement_type: FinancialStatementType,
    /// Status
    pub status: FinancialStatementStatus,
    /// Created at
    pub created_at: i64,
    /// Statement period start
    pub statement_period_start: i64,
    /// Statement period end
    pub statement_period_end: i64,
    /// Statement data hash
    pub statement_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_statement(
        statement: &mut FinancialStatementMetadata,
        statement_id: u64,
        entity_id: u64,
        statement_type: FinancialStatementType,
        statement_data_hash: [u8; 32],
        statement_period_start: i64,
        statement_period_end: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(statement_id > 0, IndrasError::InvalidInput);
        require!(statement_period_end > statement_period_start, IndrasError::InvalidInput);
        statement.statement_id = statement_id;
        statement.entity_id = entity_id;
        statement.statement_type = statement_type;
        statement.status = FinancialStatementStatus::Draft;
        statement.created_at = current_time;
        statement.statement_period_start = statement_period_start;
        statement.statement_period_end = statement_period_end;
        statement.statement_data_hash = statement_data_hash;
        statement.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_financial_statement(_statement_id: u64) -> Vec<u8> {
        vec![]
    }
}
