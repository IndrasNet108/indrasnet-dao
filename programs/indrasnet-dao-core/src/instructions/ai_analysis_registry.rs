//! AI analysis registration handlers
//!
//! Registers AI-program-owned analysis in Core via CPI guard.

use anchor_lang::prelude::*;
use crate::constants::ai_program_id;
use crate::error::IndrasError;

pub fn register_ai_analysis_handler(
    ctx: Context<crate::RegisterAiAnalysis>,
    idea_id: u64,
) -> Result<()> {
    let ai_program_key = ctx.accounts.ai_program.key();
    let expected_ai_program = ai_program_id();

    require!(
        ai_program_key == expected_ai_program,
        IndrasError::InvalidProgram
    );

    let (expected_ai_cpi_authority, _) =
        Pubkey::find_program_address(&[b"ai_cpi_authority"], &ai_program_key);
    require!(
        ctx.accounts.ai_cpi_authority.key() == expected_ai_cpi_authority,
        IndrasError::InvalidProgram
    );
    require!(
        ctx.accounts.ai_cpi_authority.is_signer,
        IndrasError::Unauthorized
    );

    let (expected_analysis, _) = Pubkey::find_program_address(
        &[b"ai_analysis", ctx.accounts.idea.key().as_ref()],
        &ai_program_key,
    );
    require!(
        ctx.accounts.analysis.key() == expected_analysis,
        IndrasError::InvalidProgram
    );
    require!(
        ctx.accounts.analysis.owner == &ai_program_key,
        IndrasError::InvalidProgram
    );

    let record = &mut ctx.accounts.analysis_record;
    record.idea_id = idea_id;
    record.analysis = ctx.accounts.analysis.key();
    record.ai_program = ai_program_key;
    record.registered_at = Clock::get()?.unix_timestamp;
    record.bump = ctx.bumps.analysis_record;

    msg!(
        "AI analysis registered for idea {} by AI program {}",
        idea_id,
        ai_program_key
    );

    Ok(())
}
