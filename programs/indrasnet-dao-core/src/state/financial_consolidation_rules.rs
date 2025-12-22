//! Financial Consolidation Rules module
//!
//! Financial consolidation rules
//!
//! On-chain: Metadata for consolidation rules
//! Off-chain: Actual rules, application

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Rule type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialConsolidationRuleType {
    /// Elimination rule
    Elimination,
    /// Translation rule
    Translation,
    /// Adjustment rule
    Adjustment,
    /// Custom rule
    Custom,
}

/// Rule status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialConsolidationRuleStatus {
    /// Rule active
    Active,
    /// Rule paused
    Paused,
    /// Rule disabled
    Disabled,
}

/// Financial consolidation rules metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialConsolidationRulesMetadata {
    /// Rule ID
    pub rule_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Rule type
    pub rule_type: FinancialConsolidationRuleType,
    /// Status
    pub status: FinancialConsolidationRuleStatus,
    /// Created at
    pub created_at: i64,
    /// Rule config hash
    pub rule_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_consolidation_rules(
        rule: &mut FinancialConsolidationRulesMetadata,
        rule_id: u64,
        entity_id: u64,
        rule_type: FinancialConsolidationRuleType,
        rule_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(rule_id > 0, IndrasError::InvalidInput);
        rule.rule_id = rule_id;
        rule.entity_id = entity_id;
        rule.rule_type = rule_type;
        rule.status = FinancialConsolidationRuleStatus::Active;
        rule.created_at = current_time;
        rule.rule_config_hash = rule_config_hash;
        rule.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn apply_consolidation_rule(_rule_id: u64) -> Vec<u8> {
        vec![]
    }
}
