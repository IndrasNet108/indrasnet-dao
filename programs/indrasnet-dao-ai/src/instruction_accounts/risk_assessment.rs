// Accounts structures for Risk Assessment instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::ai::risk_assessment::RiskAssessment;

#[derive(Accounts)]
#[instruction(assessment_id: u64)]
pub struct CreateRiskAssessment<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RiskAssessment::INIT_SPACE,
        seeds = [b"risk_assessment", assessment_id.to_le_bytes().as_ref()],
        bump
    )]
    pub assessment: Account<'info, RiskAssessment>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
