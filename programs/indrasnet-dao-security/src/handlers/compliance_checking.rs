//! Compliance Checking handlers
//!
//! Handlers for Compliance Checking instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Actual compliance checking (in separate service)

use anchor_lang::prelude::*;
use crate::events::ComplianceCheckUpdated;
use crate::security::compliance_checking::*;
use crate::instruction_accounts::security_handlers::*;

/// Create compliance check
///
/// Creates a compliance check record for a specific framework
///
/// # Compute Units
/// Recommended: 40,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~30,000 CU
pub fn create_compliance_check_handler(
    ctx: Context<CreateComplianceCheck>,
    check_id: u64,
    framework: ComplianceFramework,
    name: String,
    description: String,
    status: ComplianceCheckStatus,
    result_hash: [u8; 32],
    metadata_uri: String,
) -> Result<()> {
    let check = &mut ctx.accounts.check;
    let checker = ctx.accounts.authority.key();
    let registry = &ctx.accounts.role_registry;
    let subject_key = ctx.accounts.subject.key();

    require!(
        registry.is_compliance_admin(&checker),
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
    
    create_compliance_check(
        check,
        check_id,
        subject_key,
        framework,
        name,
        description,
        status,
        result_hash,
        metadata_uri,
        checker,
        ctx.bumps.check,
    )?;

    emit!(ComplianceCheckUpdated {
        subject: subject_key,
        admin: checker,
        status,
    });

    Ok(())
}

/// Update compliance check status
///
/// Updates the status of a compliance check
///
/// # Compute Units
/// Recommended: 20,000 CU
/// - Validation: ~5,000 CU
/// - State update: ~15,000 CU
pub fn update_compliance_check_status_handler(
    ctx: Context<UpdateComplianceCheckStatus>,
    status: ComplianceCheckStatus,
    result_hash: Option<[u8; 32]>,
) -> Result<()> {
    let check = &mut ctx.accounts.check;
    let updater = ctx.accounts.authority.key();
    let registry = &ctx.accounts.role_registry;
    let subject_key = ctx.accounts.subject.key();

    require!(
        registry.is_compliance_admin(&updater),
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
    
    update_compliance_check_status(check, status, result_hash)?;

    emit!(ComplianceCheckUpdated {
        subject: subject_key,
        admin: updater,
        status,
    });

    Ok(())
}
