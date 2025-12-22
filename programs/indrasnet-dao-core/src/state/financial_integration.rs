//! Financial Integration module
//!
//! Financial system integration
//!
//! On-chain: Metadata for financial integrations
//! Off-chain: Actual integration, API connections

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Integration type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIntegrationType {
    /// Banking integration
    Banking,
    /// Payment gateway integration
    PaymentGateway,
    /// Accounting system integration
    AccountingSystem,
    /// Custom integration
    Custom,
}

/// Integration status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialIntegrationStatus {
    /// Integration active
    Active,
    /// Integration paused
    Paused,
    /// Integration disabled
    Disabled,
}

/// Financial integration metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialIntegrationMetadata {
    /// Integration ID
    pub integration_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Integration type
    pub integration_type: FinancialIntegrationType,
    /// Status
    pub status: FinancialIntegrationStatus,
    /// Created at
    pub created_at: i64,
    /// Integration config hash
    pub integration_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_integration(
        integration: &mut FinancialIntegrationMetadata,
        integration_id: u64,
        entity_id: u64,
        integration_type: FinancialIntegrationType,
        integration_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(integration_id > 0, IndrasError::InvalidInput);
        integration.integration_id = integration_id;
        integration.entity_id = entity_id;
        integration.integration_type = integration_type;
        integration.status = FinancialIntegrationStatus::Active;
        integration.created_at = current_time;
        integration.integration_config_hash = integration_config_hash;
        integration.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn integrate_financial_system(_integration_id: u64) -> Vec<u8> {
        vec![]
    }
}
