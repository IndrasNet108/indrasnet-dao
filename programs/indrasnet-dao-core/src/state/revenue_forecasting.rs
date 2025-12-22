//! Revenue Forecasting module
//!
//! Revenue forecasting and prediction
//!
//! On-chain: Metadata for revenue forecasting
//! Off-chain: Actual forecasting, prediction

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Forecasting method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum RevenueForecastingMethod {
    /// Time series
    TimeSeries,
    /// Regression
    Regression,
    /// Machine learning
    MachineLearning,
    /// Custom method
    Custom,
}

/// Forecasting status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum RevenueForecastingStatus {
    /// Forecasting pending
    Pending,
    /// Forecasting in progress
    InProgress,
    /// Forecasting completed
    Completed,
}

/// Revenue forecasting metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct RevenueForecastingMetadata {
    /// Forecasting ID
    pub forecasting_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Forecasting method
    pub forecasting_method: RevenueForecastingMethod,
    /// Status
    pub status: RevenueForecastingStatus,
    /// Created at
    pub created_at: i64,
    /// Forecasting data hash
    pub forecasting_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_revenue_forecasting(
        forecasting: &mut RevenueForecastingMetadata,
        forecasting_id: u64,
        entity_id: u64,
        forecasting_method: RevenueForecastingMethod,
        forecasting_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(forecasting_id > 0, IndrasError::InvalidInput);
        forecasting.forecasting_id = forecasting_id;
        forecasting.entity_id = entity_id;
        forecasting.forecasting_method = forecasting_method;
        forecasting.status = RevenueForecastingStatus::Pending;
        forecasting.created_at = current_time;
        forecasting.forecasting_data_hash = forecasting_data_hash;
        forecasting.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn forecast_revenue(_forecasting_id: u64) -> Vec<u8> {
        vec![]
    }
}
