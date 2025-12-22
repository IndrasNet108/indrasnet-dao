//! Expense Management module
//!
//! Expense management (including analysis and forecasting)
//!
//! On-chain: Metadata for expenses, analysis, and forecasting
//! Off-chain: Actual expense tracking, approval workflows, analysis, forecasting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Expense category
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ExpenseCategory {
    /// Operating expense
    Operating,
    /// Capital expense
    Capital,
    /// Travel expense
    Travel,
    /// Custom expense
    Custom,
}

/// Expense status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ExpenseStatus {
    /// Expense pending
    Pending,
    /// Expense approved
    Approved,
    /// Expense rejected
    Rejected,
    /// Expense paid
    Paid,
}

/// Expense analysis type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ExpenseAnalysisType {
    /// Category analysis
    Category,
    /// Trend analysis
    Trend,
    /// Variance analysis
    Variance,
    /// Custom analysis
    Custom,
}

/// Expense forecasting method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ExpenseForecastingMethod {
    /// Time series
    TimeSeries,
    /// Regression
    Regression,
    /// Machine learning
    MachineLearning,
    /// Custom method
    Custom,
}

/// Expense metadata (on-chain)
///
/// Stores metadata for expenses
#[account]
#[derive(InitSpace)]
pub struct ExpenseMetadata {
    /// Expense ID
    pub expense_id: u64,
    /// Expense category
    pub expense_category: ExpenseCategory,
    /// Amount (in smallest unit)
    pub amount: u64,
    /// Status
    pub status: ExpenseStatus,
    /// Created at
    pub created_at: i64,
    /// Paid at
    pub paid_at: Option<i64>,
    /// Expense data hash
    pub expense_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Expense analysis metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct ExpenseAnalysisMetadata {
    /// Analysis ID
    pub analysis_id: u64,
    /// Period ID
    pub period_id: u64,
    /// Analysis type
    pub analysis_type: ExpenseAnalysisType,
    /// Status (Pending, InProgress, Completed)
    pub status: u8,
    /// Created at
    pub created_at: i64,
    /// Analysis data hash
    pub analysis_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Expense forecasting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct ExpenseForecastingMetadata {
    /// Forecasting ID
    pub forecasting_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Forecasting method
    pub forecasting_method: ExpenseForecastingMethod,
    /// Status (Pending, InProgress, Completed)
    pub status: u8,
    /// Created at
    pub created_at: i64,
    /// Forecasting data hash
    pub forecasting_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for expense management
pub mod onchain {
    use super::*;

    /// Initialize expense
    pub fn initialize_expense(
        expense: &mut ExpenseMetadata,
        expense_id: u64,
        expense_category: ExpenseCategory,
        amount: u64,
        expense_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(expense_id > 0, IndrasError::InvalidInput);
        require!(amount > 0, IndrasError::InvalidInput);
        
        expense.expense_id = expense_id;
        expense.expense_category = expense_category;
        expense.amount = amount;
        expense.status = ExpenseStatus::Pending;
        expense.created_at = current_time;
        expense.paid_at = None;
        expense.expense_data_hash = expense_data_hash;
        expense.bump = bump;
        
        Ok(())
    }

    /// Initialize expense analysis
    pub fn initialize_expense_analysis(
        analysis: &mut ExpenseAnalysisMetadata,
        analysis_id: u64,
        period_id: u64,
        analysis_type: ExpenseAnalysisType,
        analysis_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analysis_id > 0, IndrasError::InvalidInput);
        analysis.analysis_id = analysis_id;
        analysis.period_id = period_id;
        analysis.analysis_type = analysis_type;
        analysis.status = 0; // Pending
        analysis.created_at = current_time;
        analysis.analysis_data_hash = analysis_data_hash;
        analysis.bump = bump;
        Ok(())
    }

    /// Initialize expense forecasting
    pub fn initialize_expense_forecasting(
        forecasting: &mut ExpenseForecastingMetadata,
        forecasting_id: u64,
        entity_id: u64,
        forecasting_method: ExpenseForecastingMethod,
        forecasting_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(forecasting_id > 0, IndrasError::InvalidInput);
        forecasting.forecasting_id = forecasting_id;
        forecasting.entity_id = entity_id;
        forecasting.forecasting_method = forecasting_method;
        forecasting.status = 0; // Pending
        forecasting.created_at = current_time;
        forecasting.forecasting_data_hash = forecasting_data_hash;
        forecasting.bump = bump;
        Ok(())
    }
}

/// Off-chain functions for expense management
pub mod offchain {
    /// Process expense
    pub fn process_expense(_expense_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }

    /// Analyze expenses
    pub fn analyze_expenses(_analysis_id: u64) -> Vec<u8> {
        vec![]
    }

    /// Forecast expenses
    pub fn forecast_expenses(_forecasting_id: u64) -> Vec<u8> {
        vec![]
    }
}
