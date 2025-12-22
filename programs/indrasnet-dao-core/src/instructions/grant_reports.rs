//! Grant Report instruction handlers
//!
//! Handlers for grant report operations: submit, approve, reject

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::grant::{GrantReportStatus, GrantStatus};

// Import accounts structs - they are defined in instruction_accounts/grants.rs
// and included in lib.rs, so they are available via crate::
// Note: These are used in Context<> types, so they must be imported
use crate::SubmitGrantReport;

/// Submit grant report
///
/// Allows mesh group to submit a final report for a grant.
/// Required for Escrow and Standard grants before closing accounts.
///
/// # Security
/// - Only mesh group members can submit reports
/// - Grant must be Active or Completed
/// - Report can only be submitted once
///
/// # Compute Units
/// Recommended: 30,000 CU
pub fn submit_grant_report_handler(
    ctx: Context<SubmitGrantReport>,
    grant_id: u64,
    report_content: String,
    attachments_hash: Option<String>,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    let report = &mut ctx.accounts.report;
    let submitter = ctx.accounts.submitter.key();
    let current_time = Clock::get()?.unix_timestamp;
    
    // SECURITY: Validate grant ID matches
    require!(
        grant.id == grant_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate grant requires report
    require!(
        grant.disbursement_type.requires_report(),
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate grant is in valid state
    require!(
        grant.status == GrantStatus::Active || grant.status == GrantStatus::Completed,
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate submitter is mesh group member or DAO authority
    // Deserialize mesh group to check membership
    let mesh_group_data = ctx.accounts.mesh_group.data.borrow();
    require!(!mesh_group_data.is_empty(), IndrasError::InvalidInput);
    let mesh_group: crate::state::MeshGroup = borsh::BorshDeserialize::try_from_slice(&mesh_group_data[8..])?;
    require!(
        ctx.accounts.mesh_group.key() == grant.mesh_group,
        IndrasError::InvalidInput
    );
    require!(
        mesh_group.is_member(submitter) || ctx.accounts.dao_config.authority == submitter,
        IndrasError::Unauthorized
    );
    
    // SECURITY: Validate report not already submitted
    require!(
        !grant.final_report_submitted,
        IndrasError::InvalidState
    );
    
    // Validate report content
    require!(!report_content.is_empty(), IndrasError::InvalidInput);
    require!(report_content.len() <= 2000, IndrasError::StringTooLong);
    
    if let Some(ref hash) = attachments_hash {
        require!(hash.len() <= 500, IndrasError::StringTooLong);
    }
    
    // Update grant
    grant.final_report_submitted = true;
    grant.final_report_submitted_at = Some(current_time);
    
    // Update report
    report.grant_id = grant_id;
    report.mesh_group = grant.mesh_group;
    report.status = GrantReportStatus::Submitted;
    report.report_content = report_content;
    report.attachments_hash = attachments_hash;
    report.submitted_at = Some(current_time);
    
    msg!(
        "Grant {} report submitted by {} (mesh group: {})",
        grant_id,
        submitter,
        grant.mesh_group
    );
    
    Ok(())
}

/// Approve grant report
///
/// Allows DAO authority to approve a submitted grant report.
/// After approval, accounts can be closed (if all grants have approved reports).
///
/// # Security
/// - Only DAO authority can approve reports
/// - Report must be in Submitted status
///
/// # Compute Units
/// Recommended: 20,000 CU
pub fn approve_grant_report_handler(
    ctx: Context<crate::ApproveGrantReport>,
    grant_id: u64,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    let report = &mut ctx.accounts.report;
    let approver = ctx.accounts.approver.key();
    let dao_config = &ctx.accounts.dao_config;
    let current_time = Clock::get()?.unix_timestamp;
    
    // SECURITY: Validate grant ID matches
    require!(
        grant.id == grant_id && report.grant_id == grant_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate approver is DAO authority
    require!(
        approver == dao_config.authority,
        IndrasError::Unauthorized
    );
    
    // SECURITY: Validate report is submitted
    require!(
        report.status == GrantReportStatus::Submitted,
        IndrasError::InvalidState
    );
    
    // Update grant
    grant.final_report_approved = true;
    grant.final_report_approved_at = Some(current_time);
    
    // Update report
    report.status = GrantReportStatus::Approved;
    report.approved_at = Some(current_time);
    report.approved_by = Some(approver);
    
    msg!(
        "Grant {} report approved by {}",
        grant_id,
        approver
    );
    
    Ok(())
}

/// Reject grant report
///
/// Allows DAO authority to reject a submitted grant report.
/// Mesh group can resubmit after fixing issues.
///
/// # Security
/// - Only DAO authority can reject reports
/// - Report must be in Submitted status
///
/// # Compute Units
/// Recommended: 20,000 CU
pub fn reject_grant_report_handler(
    ctx: Context<crate::RejectGrantReport>,
    grant_id: u64,
    reason: String,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    let report = &mut ctx.accounts.report;
    let rejector = ctx.accounts.rejector.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // SECURITY: Validate grant ID matches
    require!(
        grant.id == grant_id && report.grant_id == grant_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate rejector is DAO authority
    require!(
        rejector == dao_config.authority,
        IndrasError::Unauthorized
    );
    
    // SECURITY: Validate report is submitted
    require!(
        report.status == GrantReportStatus::Submitted,
        IndrasError::InvalidState
    );
    
    // Validate reason
    require!(!reason.is_empty(), IndrasError::InvalidInput);
    require!(reason.len() <= 500, IndrasError::StringTooLong);
    
    // Reset grant report status (allows resubmission)
    grant.final_report_submitted = false;
    grant.final_report_submitted_at = None;
    
    // Update report
    report.status = GrantReportStatus::Rejected;
    report.rejection_reason = Some(reason.clone());
    
    msg!(
        "Grant {} report rejected by {} (reason: {})",
        grant_id,
        rejector,
        reason
    );
    
    Ok(())
}

#[cfg(test)]
#[allow(unused_imports, unused_variables)]
mod tests {
    use crate::state::grant::GrantDisbursementType;
    use anchor_lang::prelude::Pubkey;
    
    #[test]
    fn test_grant_disbursement_type_requires_report() {
        assert!(!GrantDisbursementType::Urgent.requires_report());
        assert!(GrantDisbursementType::Escrow.requires_report());
        assert!(GrantDisbursementType::Standard.requires_report());
    }
    
    #[test]
    fn test_grant_disbursement_type_requires_escrow() {
        assert!(!GrantDisbursementType::Urgent.requires_escrow());
        assert!(GrantDisbursementType::Escrow.requires_escrow());
        assert!(!GrantDisbursementType::Standard.requires_escrow());
    }
    
    #[test]
    fn test_grant_disbursement_type_can_close_without_report() {
        assert!(GrantDisbursementType::Urgent.can_close_without_report());
        assert!(!GrantDisbursementType::Escrow.can_close_without_report());
        assert!(!GrantDisbursementType::Standard.can_close_without_report());
    }
}
