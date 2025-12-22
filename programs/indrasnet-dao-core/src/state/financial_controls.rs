//! Financial Controls module
//!
//! Financial controls and safeguards
//!
//! On-chain: Metadata for financial controls
//! Off-chain: Actual controls, monitoring

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Control type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialControlType {
    /// Preventive control
    Preventive,
    /// Detective control
    Detective,
    /// Corrective control
    Corrective,
    /// Custom control
    Custom,
}

/// Control status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialControlStatus {
    /// Control active
    Active,
    /// Control paused
    Paused,
    /// Control disabled
    Disabled,
}

/// Financial controls metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialControlsMetadata {
    /// Control ID
    pub control_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Control type
    pub control_type: FinancialControlType,
    /// Status
    pub status: FinancialControlStatus,
    /// Created at
    pub created_at: i64,
    /// Control config hash
    pub control_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_controls(
        control: &mut FinancialControlsMetadata,
        control_id: u64,
        entity_id: u64,
        control_type: FinancialControlType,
        control_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(control_id > 0, IndrasError::InvalidInput);
        control.control_id = control_id;
        control.entity_id = entity_id;
        control.control_type = control_type;
        control.status = FinancialControlStatus::Active;
        control.created_at = current_time;
        control.control_config_hash = control_config_hash;
        control.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn enforce_financial_controls(_control_id: u64) -> bool {
        false
    }
}
