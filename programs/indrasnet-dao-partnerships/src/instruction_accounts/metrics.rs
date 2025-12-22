// Accounts structures for Metrics Tracking instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::partnerships::metrics::EnhancedPartnershipMetrics;

#[derive(Accounts)]
#[instruction(metrics_id: u64, partnership_id: u64)]
pub struct TrackPartnershipMetrics<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + EnhancedPartnershipMetrics::INIT_SPACE,
        seeds = [b"partnership_metrics", metrics_id.to_le_bytes().as_ref()],
        bump
    )]
    pub metrics: Account<'info, EnhancedPartnershipMetrics>,

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
