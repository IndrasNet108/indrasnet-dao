//! Idea rights transfer handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::idea::TransferredRights;

pub fn transfer_rights_to_ev_handler(
    ctx: Context<crate::TransferRightsToEv>,
    can_modify: bool,
    can_distribute: bool,
    can_reproduce: bool,
    can_develop: bool,
    can_sublicense: bool,
    can_gift: bool,
    can_bequeath: bool,
) -> Result<()> {
    let idea = &mut ctx.accounts.idea;
    let anchor_record = &ctx.accounts.anchor_record;
    
    // Check 1: Only author can transfer rights
    require!(
        ctx.accounts.author.key() == idea.author,
        IndrasError::Unauthorized
    );
    
    // Check 2: Idea must be anchored (authorship established)
    require!(
        anchor_record.idea_id == idea.id,
        IndrasError::InvalidInput
    );
    require!(
        anchor_record.anchorer == idea.author,
        IndrasError::Unauthorized
    );
    
    // Check 3: Rights not yet transferred (or can be updated)
    // For simplicity, allow updating rights
    
    // Check 4: At least one right must be transferred
    require!(
        can_modify || can_distribute || can_reproduce || can_develop || 
        can_sublicense || can_gift || can_bequeath,
        IndrasError::InvalidInput
    );
    
    // Create transferred rights structure
    let transferred_rights = TransferredRights {
        can_modify,
        can_distribute,
        can_reproduce,
        can_develop,
        can_sublicense,
        can_gift,
        can_bequeath,
        transferred_at: Clock::get()?.unix_timestamp,
        transferred_by: idea.author,
    };
    
    // Transfer rights
    idea.rights_transferred_to_ev = Some(transferred_rights);
    
    msg!("Rights transferred to e.V. by author {} for idea {} (modify: {}, distribute: {}, reproduce: {}, develop: {}, sublicense: {}, gift: {}, bequeath: {})",
         idea.author,
         idea.id,
         can_modify,
         can_distribute,
         can_reproduce,
         can_develop,
         can_sublicense,
         can_gift,
         can_bequeath);
    
    Ok(())
}
