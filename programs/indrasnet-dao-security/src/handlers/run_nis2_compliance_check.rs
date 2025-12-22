//! Handler for running NIS2 compliance checks
//!
//! NOTE: Real NIS2 compliance checks happen off-chain (services/offchain-security-service/)
//! This handler records the results on-chain

use anchor_lang::prelude::*;
use crate::events::ComplianceCheckUpdated;
use crate::RunNis2ComplianceCheck;
use crate::security::compliance_checking::{self, ComplianceFramework, ComplianceCheckStatus};

pub fn run_nis2_compliance_check_handler(ctx: Context<RunNis2ComplianceCheck>, check_id: u64) -> Result<()> {
    // NOTE: Real NIS2 compliance checks (risk_management, incident_management, etc.)
    // happen off-chain in services/offchain-security-service/
    // This handler only records the results on-chain
    
    msg!("NIS2 compliance check initiated (off-chain processing)");

    // Create a compliance check record
    let check = &mut ctx.accounts.check;
    let authority = ctx.accounts.authority.key();
    let registry = &ctx.accounts.role_registry;
    let subject_key = ctx.accounts.subject.key();

    require!(
        registry.is_compliance_admin(&authority),
        crate::error::IndrasError::Unauthorized
    );

    let (expected_pda, _) = Pubkey::find_program_address(
        &[b"compliance", subject_key.as_ref()],
        ctx.program_id,
    );
    require_keys_eq!(check.key(), expected_pda, crate::error::IndrasError::InvalidInput);
    require_keys_eq!(
        *check.to_account_info().owner,
        *ctx.program_id,
        crate::error::IndrasError::InvalidInput
    );

    compliance_checking::create_compliance_check(
        check,
        check_id,
        subject_key,
        ComplianceFramework::Nis2,
        "NIS2 Compliance Check".to_string(),
        "Automated NIS2 compliance checks (off-chain processing).".to_string(),
        ComplianceCheckStatus::InProgress, // Will be updated by off-chain service
        [0u8; 32], // Placeholder hash - will be updated by off-chain service
        "ipfs://placeholder".to_string(),
        authority,
        ctx.bumps.check,
    )?;

    emit!(ComplianceCheckUpdated {
        subject: subject_key,
        admin: authority,
        status: ComplianceCheckStatus::InProgress,
    });

    Ok(())
}
