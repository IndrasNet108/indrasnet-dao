// Accounts structures for Revenue Sharing instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::partnerships::revenue_sharing::{RevenueShareConfig, RevenueDistribution};

#[derive(Accounts)]
#[instruction(config_id: u64, partnership_id: u64)]
pub struct CreateRevenueShareConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RevenueShareConfig::INIT_SPACE,
        seeds = [b"revenue_config", config_id.to_le_bytes().as_ref()],
        bump
    )]
    pub config: Account<'info, RevenueShareConfig>,

    #[account(
        seeds = [b"partnership_config"],
        bump = partnership_config.bump
    )]
    pub partnership_config: Account<'info, PartnershipConfigAccount>,

    #[account(
        seeds = [b"partnership_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, crate::state::PartnershipRoleRegistry>,

    #[account(
        seeds = [b"partnership", partnership_id.to_le_bytes().as_ref()],
        bump = partnership.bump,
        constraint = partnership.partnership_id == partnership_id @ crate::error::IndrasError::InvalidInput
    )]
    pub partnership: Account<'info, crate::state::PartnershipMetadata>,
    
    #[account(
        mut,
        constraint = authority.key() == partnership_config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(distribution_id: u64, partnership_id: u64)]
pub struct CreateRevenueDistribution<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RevenueDistribution::INIT_SPACE,
        seeds = [b"revenue_distribution", distribution_id.to_le_bytes().as_ref()],
        bump
    )]
    pub distribution: Account<'info, RevenueDistribution>,

    #[account(
        seeds = [b"partnership_config"],
        bump = partnership_config.bump
    )]
    pub partnership_config: Account<'info, PartnershipConfigAccount>,

    #[account(
        seeds = [b"partnership_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, crate::state::PartnershipRoleRegistry>,

    #[account(
        seeds = [b"partnership", partnership_id.to_le_bytes().as_ref()],
        bump = partnership.bump,
        constraint = partnership.partnership_id == partnership_id @ crate::error::IndrasError::InvalidInput
    )]
    pub partnership: Account<'info, crate::state::PartnershipMetadata>,
    
    #[account(
        mut,
        constraint = authority.key() == partnership_config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(distribution_id: u64, partnership_id: u64)]
pub struct DepositPartnershipRevenue<'info> {
    #[account(
        mut,
        seeds = [b"revenue_distribution", distribution_id.to_le_bytes().as_ref()],
        bump = distribution.bump
    )]
    pub distribution: Account<'info, RevenueDistribution>,
    
    #[account(
        seeds = [b"partnership_config"],
        bump = partnership_config.bump
    )]
    pub partnership_config: Account<'info, PartnershipConfigAccount>,

    #[account(
        seeds = [b"partnership_roles"],
        bump = role_registry.bump
    )]
    pub role_registry: Account<'info, crate::state::PartnershipRoleRegistry>,

    #[account(
        seeds = [b"partnership", partnership_id.to_le_bytes().as_ref()],
        bump = partnership.bump,
        constraint = partnership.partnership_id == partnership_id @ crate::error::IndrasError::InvalidInput
    )]
    pub partnership: Account<'info, crate::state::PartnershipMetadata>,

    #[account(
        mut,
        constraint = authority.key() == partnership_config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}
