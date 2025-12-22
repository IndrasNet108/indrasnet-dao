//! Security Analytics handlers
//!
//! Handlers for Security Analytics instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Actual analytics processing (in separate service)

use anchor_lang::prelude::*;
use crate::events::SecurityAnalyticsReportCreated;
use crate::security::security_analytics::*;
use crate::instruction_accounts::security_handlers::*;

/// Create security analytics report
///
/// Creates a security analytics report with metadata
///
/// # Compute Units
/// Recommended: 40,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~30,000 CU
pub fn create_security_analytics_report_handler(
    ctx: Context<CreateSecurityAnalyticsReport>,
    report_id: u64,
    idea_id: u64,
    report_type: AnalyticsReportType,
    title: String,
    metadata_uri: String,
    report_hash: [u8; 32],
    summary: String,
) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let generator = ctx.accounts.authority.key();
    let registry = &ctx.accounts.role_registry;

    require!(
        registry.is_analytics_admin(&generator),
        crate::error::IndrasError::Unauthorized
    );

    let (expected_pda, _) = Pubkey::find_program_address(
        &[b"analytics", idea_id.to_le_bytes().as_ref()],
        ctx.program_id,
    );
    require_keys_eq!(report.key(), expected_pda, crate::error::IndrasError::InvalidInput);
    require_keys_eq!(
        *report.to_account_info().owner,
        *ctx.program_id,
        crate::error::IndrasError::InvalidInput
    );
    
    create_security_analytics_report(
        report,
        report_id,
        idea_id,
        report_type,
        title,
        metadata_uri,
        report_hash,
        summary,
        generator,
        ctx.bumps.report,
    )?;

    emit!(SecurityAnalyticsReportCreated {
        idea_id,
        report_id,
        admin: generator,
        report_type,
    });

    Ok(())
}
