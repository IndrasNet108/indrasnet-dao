// Accounts structures for Performance Analytics instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::ai::performance_analytics::PerformanceAnalytics;

#[derive(Accounts)]
#[instruction(analytics_id: u64)]
pub struct CreatePerformanceAnalytics<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PerformanceAnalytics::INIT_SPACE,
        seeds = [b"performance_analytics", analytics_id.to_le_bytes().as_ref()],
        bump
    )]
    pub analytics: Account<'info, PerformanceAnalytics>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
