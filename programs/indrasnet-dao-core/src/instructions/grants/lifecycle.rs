//! Grant lifecycle management handlers

use anchor_lang::prelude::*;
use crate::state::grant::GrantStatus;

/// Activate grant
///
/// This handler activates an approved grant, making it ready for disbursement.
pub fn activate_grant_handler(
    ctx: Context<crate::ActivateGrant>,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    
    grant.activate()?;
    
    msg!("Grant {} activated by {}", grant.id, ctx.accounts.activator.key());
    
    Ok(())
}

/// Complete grant
///
/// This handler marks a grant as completed.
/// Complete grant
///
/// This handler marks a grant as completed.
pub fn complete_grant_handler(
    ctx: Context<crate::CompleteGrant>,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    
    grant.status = GrantStatus::Completed;
    grant.completed_at = Some(Clock::get()?.unix_timestamp);
    
    msg!("Grant {} completed by {}", grant.id, ctx.accounts.completer.key());
    
    Ok(())
}
