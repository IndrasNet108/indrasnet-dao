//! Type definitions for the IndrasNet DAO Security program

use anchor_lang::prelude::*;

/// Security check result for proposals
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SecurityCheckResult {
    pub is_safe: bool,
    pub requires_dao_vote: bool,
    pub risk_flag: Option<RiskFlag>,
}

/// Risk flag for security checks
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RiskFlag {
    pub risk_level: RiskLevel,
    pub description: String,
}

/// Risk level for security checks
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}
