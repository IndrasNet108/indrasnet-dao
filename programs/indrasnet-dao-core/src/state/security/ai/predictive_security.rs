//! Predictive Security module
//!
//! Predictive security analysis and threat prediction
//!
//! On-chain: Metadata for predictions, threat forecasts
//! Off-chain: Actual ML-based prediction, threat analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Threat prediction confidence
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PredictionConfidence {
    /// Low confidence
    Low,
    /// Medium confidence
    Medium,
    /// High confidence
    High,
}

/// Predictive threat metadata (on-chain)
///
/// Stores metadata for predicted threats
#[account]
#[derive(InitSpace)]
pub struct PredictiveThreatMetadata {
    /// Threat ID
    pub threat_id: u64,
    /// Predicted threat level
    pub predicted_level: u8, // 0-100
    /// Confidence
    pub confidence: PredictionConfidence,
    /// Created at
    pub created_at: i64,
    /// Predicted occurrence time
    pub predicted_occurrence_time: Option<i64>,
    /// Threat data hash
    pub threat_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for predictive security
pub mod onchain {
    use super::*;

    /// Initialize predictive threat
    pub fn initialize_predictive_threat(
        threat: &mut PredictiveThreatMetadata,
        threat_id: u64,
        predicted_level: u8,
        confidence: PredictionConfidence,
        threat_data_hash: [u8; 32],
        predicted_occurrence_time: Option<i64>,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(threat_id > 0, IndrasError::InvalidInput);
        require!(predicted_level <= 100, IndrasError::InvalidInput);
        
        threat.threat_id = threat_id;
        threat.predicted_level = predicted_level;
        threat.confidence = confidence;
        threat.created_at = current_time;
        threat.predicted_occurrence_time = predicted_occurrence_time;
        threat.threat_data_hash = threat_data_hash;
        threat.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for predictive security
pub mod offchain {
    /// Predict threats
    pub fn predict_threats() -> Vec<u64> {
        // Implementation in off-chain service
        // Uses ML models to predict threats
        vec![]
    }
}
