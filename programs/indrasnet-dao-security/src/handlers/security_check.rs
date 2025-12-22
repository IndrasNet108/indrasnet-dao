//! Security check handler for proposal validation

use anchor_lang::prelude::*;
use crate::instruction_accounts::security_handlers::CheckProposalSecurity;
use crate::types::{SecurityCheckResult, RiskFlag, RiskLevel};

/// Check proposal security (CPI callable from DAO)
///
/// NOTE: Real security analysis happens off-chain (services/offchain-security-service/)
/// This handler provides basic on-chain validation and records results
pub fn check_proposal_security_cpi_handler(
    _ctx: Context<CheckProposalSecurity>,
    proposal_title: String,
    proposal_description: String,
) -> Result<SecurityCheckResult> {
    // Basic validation
    require!(!proposal_title.is_empty(), crate::error::IndrasError::InvalidInput);
    require!(!proposal_description.is_empty(), crate::error::IndrasError::InvalidInput);
    
    // NOTE: Real security analysis (ML, threat detection, etc.) happens off-chain
    // This handler only provides basic validation and records results
    
    // For MVP: Basic check - if title/description are too short, flag as requiring vote
    let requires_vote = proposal_title.len() < 10 || proposal_description.len() < 50;
    
    let result = SecurityCheckResult {
        is_safe: !requires_vote,
        requires_dao_vote: requires_vote,
        risk_flag: if requires_vote {
            Some(RiskFlag {
                risk_level: RiskLevel::Medium,
                description: "Proposal requires DAO vote for security review".to_string(),
            })
        } else {
            None
        },
    };
    
    msg!("Security check completed: safe={}, requires_vote={}", result.is_safe, result.requires_dao_vote);
    
    Ok(result)
}
