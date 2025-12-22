// Accounts structures for Partnership role registry instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::state::PartnershipRoleRegistry;

#[derive(Accounts)]
pub struct InitializePartnershipRoleRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PartnershipRoleRegistry::INIT_SPACE,
        seeds = [b"partnership_roles"],
        bump
    )]
    pub role_registry: Account<'info, PartnershipRoleRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePartnershipRoleRegistry<'info> {
    #[account(
        mut,
        seeds = [b"partnership_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, PartnershipRoleRegistry>,

    #[account(
        constraint = authority.key() == role_registry.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}
