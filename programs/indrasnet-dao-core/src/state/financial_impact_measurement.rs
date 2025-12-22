//! Financial Impact Measurement module
//!
//! Financial impact measurement
//!
//! On-chain: Metadata for impact measurement
//! Off-chain: Actual measurement, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Impact type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialImpactType {
    /// Social impact
    Social,
    /// Environmental impact
    Environmental,
    /// Economic impact
    Economic,
    /// Custom impact
    Custom,
}

/// Impact status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialImpactStatus {
    /// Impact measuring
    Measuring,
    /// Impact measured
    Measured,
    /// Impact reported
    Reported,
}

/// Financial impact measurement metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialImpactMeasurementMetadata {
    /// Measurement ID
    pub measurement_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Impact type
    pub impact_type: FinancialImpactType,
    /// Status
    pub status: FinancialImpactStatus,
    /// Created at
    pub created_at: i64,
    /// Measurement data hash
    pub measurement_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_impact_measurement(
        measurement: &mut FinancialImpactMeasurementMetadata,
        measurement_id: u64,
        entity_id: u64,
        impact_type: FinancialImpactType,
        measurement_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(measurement_id > 0, IndrasError::InvalidInput);
        measurement.measurement_id = measurement_id;
        measurement.entity_id = entity_id;
        measurement.impact_type = impact_type;
        measurement.status = FinancialImpactStatus::Measuring;
        measurement.created_at = current_time;
        measurement.measurement_data_hash = measurement_data_hash;
        measurement.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn measure_impact(_measurement_id: u64) -> Vec<u8> {
        vec![]
    }
}
