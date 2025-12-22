//! Financial Data Security module
//!
//! Financial data security
//!
//! On-chain: Metadata for data security
//! Off-chain: Actual security, encryption

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Security measure type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataSecurityMeasureType {
    /// Encryption
    Encryption,
    /// Access control
    AccessControl,
    /// Audit logging
    AuditLogging,
    /// Custom measure
    Custom,
}

/// Security status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialDataSecurityStatus {
    /// Security active
    Active,
    /// Security paused
    Paused,
    /// Security disabled
    Disabled,
}

/// Financial data security metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialDataSecurityMetadata {
    /// Security ID
    pub security_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Security measure type
    pub security_measure_type: FinancialDataSecurityMeasureType,
    /// Status
    pub status: FinancialDataSecurityStatus,
    /// Created at
    pub created_at: i64,
    /// Security config hash
    pub security_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_data_security(
        security: &mut FinancialDataSecurityMetadata,
        security_id: u64,
        entity_id: u64,
        security_measure_type: FinancialDataSecurityMeasureType,
        security_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(security_id > 0, IndrasError::InvalidInput);
        security.security_id = security_id;
        security.entity_id = entity_id;
        security.security_measure_type = security_measure_type;
        security.status = FinancialDataSecurityStatus::Active;
        security.created_at = current_time;
        security.security_config_hash = security_config_hash;
        security.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn secure_financial_data(_security_id: u64) -> Vec<u8> {
        vec![]
    }
}
