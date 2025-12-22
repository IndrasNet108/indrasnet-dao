// Accounts structures for AI analysis instructions
// NOTE: Types like DaoConfig, Idea, AIAnalysis and anchor_lang types are imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct AnalyzeIdea<'info> {
    /// DaoConfig account
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ crate::error::IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    #[account(
        init,
        payer = analyzer,
        space = 8 + 400, // Increased size for new fields (nonce: 8, expires_at: 9, buffer)
        seeds = [b"ai_analysis", idea.key().as_ref()], // Do NOT change seeds for backward compatibility
        bump
    )]
    pub analysis: Account<'info, AIAnalysis>,
    
    /// Analyzer - must be DAO authority
    /// SECURITY: Only DAO authority can submit AI analysis results
    #[account(
        mut,
        constraint = analyzer.key() == dao_config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub analyzer: Signer<'info>,
    
    /// System program - MUST be before optional accounts for allow-missing-optionals to work
    pub system_program: Program<'info, System>,
    
    /// AI Service Registry (optional for MVP)
    /// If provided, verifies that analyzer_pubkey is an authorized AI service
    /// NOTE: For MVP, this is optional - if not provided, only DAO authority can submit
    /// NOTE: Using UncheckedAccount to avoid lifetime issues in Anchor 0.32.1
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    /// Model Registry (optional for MVP)
    /// If provided, verifies that model_id:model_version is valid
    /// NOTE: For MVP, this is optional - if not provided, any model version is accepted
    /// NOTE: Using UncheckedAccount to avoid lifetime issues in Anchor 0.32.1
    pub model_registry: Option<UncheckedAccount<'info>>,
    
    /// Rate limit tracker (optional - for SEC-INV-8: 1 AI analysis per 5 minutes per analyzer)
    /// PDA: [b"rate_limit", analyzer.key().as_ref(), b"analyze_idea"]
    /// CHECK: If provided, validates rate limit; if not provided, rate limit is skipped
    /// NOTE: Uses analyzer account key (must match analyzer_pubkey parameter)
    #[account(
        init_if_needed,
        payer = analyzer,
        space = 8 + RateLimitTracker::INIT_SPACE,
        seeds = [b"rate_limit", analyzer.key().as_ref(), b"analyze_idea"],
        bump
    )]
    pub rate_limit_tracker: Option<Account<'info, RateLimitTracker>>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct UpdateIdeaStatusFromAnalysis<'info> {
    /// DaoConfig account
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ crate::error::IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    #[account(constraint = analysis.idea_id == idea_id @ crate::error::IndrasError::InvalidInput)]
    pub analysis: Account<'info, AIAnalysis>,
    
    #[account(
        constraint = updater.key() == dao_config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub updater: Signer<'info>,
}
