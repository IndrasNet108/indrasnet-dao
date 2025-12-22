// Accounts structures for Security Enhancements instructions
// NOTE: anchor_lang types are imported in lib.rs before include!()

use crate::ai::security_enhancements::SecurityEnhancement;

#[derive(Accounts)]
#[instruction(enhancement_id: u64)]
pub struct CreateSecurityEnhancement<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + SecurityEnhancement::INIT_SPACE,
        seeds = [b"security_enhancement", enhancement_id.to_le_bytes().as_ref()],
        bump
    )]
    pub enhancement: Account<'info, SecurityEnhancement>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(enhancement_id: u64)]
pub struct UpdateSecurityEnhancementStatus<'info> {
    #[account(
        mut,
        seeds = [b"security_enhancement", enhancement_id.to_le_bytes().as_ref()],
        bump = enhancement.bump
    )]
    pub enhancement: Account<'info, SecurityEnhancement>,
    
    pub authority: Signer<'info>,
}
