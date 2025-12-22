// Accounts structures for Partnership instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::state::PartnershipMetadata;
use crate::state::PartnershipConfigAccount;

#[derive(Accounts)]
#[instruction(partnership_id: u64)]
pub struct CreatePartnership<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PartnershipMetadata::INIT_SPACE,
        seeds = [b"partnership", partnership_id.to_le_bytes().as_ref()],
        bump
    )]
    pub partnership: Account<'info, PartnershipMetadata>,
    
    /// CHECK: Partner address
    pub partner: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"partnership_config"],
        bump = config.bump
    )]
    pub config: Account<'info, PartnershipConfigAccount>,

    #[account(
        seeds = [b"partnership_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, crate::state::PartnershipRoleRegistry>,
    
    #[account(
        mut,
        constraint = authority.key() == config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePartnershipConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PartnershipConfigAccount::INIT_SPACE,
        seeds = [b"partnership_config"],
        bump
    )]
    pub partnership_config: Account<'info, PartnershipConfigAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(partnership_id: u64)]
pub struct UpdatePartnership<'info> {
    #[account(
        mut,
        seeds = [b"partnership", partnership_id.to_le_bytes().as_ref()],
        bump = partnership.bump
    )]
    pub partnership: Account<'info, PartnershipMetadata>,
    
    #[account(
        seeds = [b"partnership_config"],
        bump = config.bump
    )]
    pub config: Account<'info, PartnershipConfigAccount>,

    #[account(
        seeds = [b"partnership_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, crate::state::PartnershipRoleRegistry>,
    
    #[account(
        constraint = authority.key() == config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}
