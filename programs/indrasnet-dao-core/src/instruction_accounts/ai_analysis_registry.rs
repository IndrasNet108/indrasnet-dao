// Accounts structures for AI analysis registration
// NOTE: Types like DaoConfig, Idea, AIAnalysisRecord and anchor_lang types are imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct RegisterAiAnalysis<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,

    #[account(constraint = idea.id == idea_id @ crate::error::IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,

    /// CHECK: AI analysis account (owned by AI program, verified in handler)
    pub analysis: UncheckedAccount<'info>,

    #[account(
        init,
        payer = analyzer,
        space = 8 + AIAnalysisRecord::INIT_SPACE,
        seeds = [b"ai_analysis_record", idea.key().as_ref()],
        bump
    )]
    pub analysis_record: Account<'info, AIAnalysisRecord>,

    /// CHECK: AI program account (verified in handler)
    pub ai_program: UncheckedAccount<'info>,

    /// CPI authority PDA derived by AI program (must be signer)
    /// CHECK: Address is validated in handler
    pub ai_cpi_authority: UncheckedAccount<'info>,

    /// Analyzer pays rent for the record (must be signer in outer tx)
    #[account(mut)]
    pub analyzer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
