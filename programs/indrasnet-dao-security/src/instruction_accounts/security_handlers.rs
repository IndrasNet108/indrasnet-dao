//! Account structures for Security handlers
//!
//! This file contains account structures for Security program instructions
//!
//! NOTE: IoT security, IR team, SSO, network monitoring, cloud monitoring accounts
//! moved to off-chain service (services/offchain-security-service/)

use anchor_lang::prelude::*;
use crate::state::SecurityRoleRegistry;

// ===== SECURITY ANALYTICS ACCOUNTS =====

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct CreateSecurityAnalyticsReport<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + crate::security::security_analytics::SecurityAnalyticsReport::INIT_SPACE,
        seeds = [b"analytics", idea_id.to_le_bytes().as_ref()],
        bump
    )]
    pub report: Account<'info, crate::security::security_analytics::SecurityAnalyticsReport>,

    #[account(
        seeds = [b"security_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, SecurityRoleRegistry>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// ===== COMPLIANCE CHECKING ACCOUNTS =====

#[derive(Accounts)]
#[instruction(check_id: u64)]
pub struct CreateComplianceCheck<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + crate::security::compliance_checking::ComplianceCheck::INIT_SPACE,
        seeds = [b"compliance", subject.key().as_ref()],
        bump
    )]
    pub check: Account<'info, crate::security::compliance_checking::ComplianceCheck>,

    /// CHECK: Subject of compliance record (key used for PDA seeds)
    pub subject: AccountInfo<'info>,

    #[account(
        seeds = [b"security_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, SecurityRoleRegistry>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateComplianceCheckStatus<'info> {
    #[account(
        mut,
        seeds = [b"compliance", subject.key().as_ref()],
        bump = check.bump
    )]
    pub check: Account<'info, crate::security::compliance_checking::ComplianceCheck>,

    /// CHECK: Subject of compliance record (key used for PDA seeds)
    pub subject: AccountInfo<'info>,

    #[account(
        seeds = [b"security_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, SecurityRoleRegistry>,
    
    pub authority: Signer<'info>,
}


// ===== SECURITY CHECK (CPI) ACCOUNTS =====

#[derive(Accounts)]
pub struct CheckProposalSecurity<'info> {
    /// Caller (from DAO program via CPI)
    pub caller: Signer<'info>,
    
    /// AI program (optional - for AI-enhanced security checks)
    /// CHECK: If provided, must be AI program
    pub ai_program: Option<AccountInfo<'info>>,
    
    pub system_program: Program<'info, System>,
}

// NOTE: UpdateSecurityCheckResult removed - off-chain service uses create_security_analytics_report
