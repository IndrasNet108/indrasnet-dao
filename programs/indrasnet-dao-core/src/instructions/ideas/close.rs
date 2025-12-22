//! Idea account closure handlers

use anchor_lang::prelude::*;
use crate::error::IndrasError;

pub fn close_idea_handler(
    ctx: Context<crate::CloseIdea>,
    idea_id: u64,
) -> Result<()> {
    let idea = &ctx.accounts.idea;
    let closer = ctx.accounts.closer.key();
    let destination = ctx.accounts.destination.key();
    let dao_config = &ctx.accounts.dao_config;
    
    // SECURITY: Validate idea ID matches
    require!(
        idea.id == idea_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate closer is authorized (author or DAO authority)
    require!(
        idea.author == closer || dao_config.authority == closer,
        IndrasError::Unauthorized
    );
    
    // NOTE: Grant report validation is performed OFF-CHAIN
    // On-chain we only check flags in grants (if passed in accounts)
    // Minimize transactions - do not search all grants on-chain
    // Off-chain service must validate all grants and ensure reports are approved
    // before calling close_idea
    
    msg!(
        "Idea {} closed by {}, rent will be returned to {}",
        idea_id,
        closer,
        destination
    );
    
    // Anchor's `close = destination` will automatically:
    // 1. Transfer all lamports from idea account to destination
    // 2. Set account data to zero
    // 3. Mark account as closed
    
    Ok(())
}
