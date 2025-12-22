//! Grant approval handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::grant::GrantStatus;

/// Approve grant
///
/// This handler approves a grant request and adds it to the mesh group.
/// 
/// NOTE: When grant is approved, author MUST transfer commercialization rights to e.V.
/// Grant is funding, in exchange for which author transfers to e.V. the right to transfer Idea to commercial enterprise.
/// Author remains copyright owner (does not transfer).
/// e.V. receives right to decide when and how to transfer idea to commercialization.
/// e.V. also stores author's copyright (as custodian).
/// User becomes e.V. member when joining DAO and pays membership fees.
/// 
/// # Security
/// - Validates grant is in Pending status
/// - Validates approver has authority
/// - Ensures grant is added to mesh group safely
/// 
/// NOTE: MeshGroup.add_grant() will be called when MeshGroup state is migrated.
pub fn approve_grant_handler(
    ctx: Context<crate::ApproveGrant>,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    
    // SECURITY: Validate grant is in correct state for approval
    require!(
        grant.status == GrantStatus::Pending,
        IndrasError::InvalidState
    );
    
    grant.approve()?;
    
    // CRITICAL: When grant is approved, author MUST transfer commercialization rights to e.V.
    // This is a condition for receiving grant: in exchange for funding, author transfers to e.V. the right to transfer Idea to commercial enterprise
    // Author remains copyright owner (does not transfer)
    // e.V. receives right to decide when and how to transfer idea to commercialization
    // e.V. also stores author's copyright (as custodian)
    grant.commercialization_right_transferred = true;
    
    // Add grant to mesh group after approval
    let mesh_group = &mut ctx.accounts.mesh_group;
    mesh_group.add_grant(grant.id)?;
    
    msg!("Grant {} approved by {} - author transfers commercialization right to e.V., author retains IP rights, e.V. is custodian",
         grant.id,
         ctx.accounts.approver.key());
    
    Ok(())
}
