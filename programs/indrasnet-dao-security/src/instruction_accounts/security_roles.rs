// Accounts for Security role registry instructions

use anchor_lang::prelude::*;
use crate::state::SecurityRoleRegistry;

#[derive(Accounts)]
pub struct InitializeSecurityRoleRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + SecurityRoleRegistry::INIT_SPACE,
        seeds = [b"security_roles"],
        bump
    )]
    pub role_registry: Account<'info, SecurityRoleRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSecurityRoleRegistry<'info> {
    #[account(
        mut,
        seeds = [b"security_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, SecurityRoleRegistry>,

    #[account(
        constraint = authority.key() == role_registry.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}
