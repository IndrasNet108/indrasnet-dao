// Accounts structures for AI analysis instructions
// NOTE: Types like DaoConfig, Idea, AIAnalysis and anchor_lang types are imported in lib.rs before include!()

use indrasnet_dao_core::state::rate_limit_tracker::RateLimitTracker;
use indrasnet_dao_core;

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct AnalyzeIdea<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump,
        seeds::program = indrasnet_dao_core::ID
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ indrasnet_dao_core::error::IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    #[account(
        init,
        payer = analyzer,
        space = 8 + AIAnalysis::INIT_SPACE,
        seeds = [b"ai_analysis", idea.key().as_ref()],
        bump
    )]
    pub analysis: Account<'info, AIAnalysis>,

    /// Rate limit tracker for analyzer (1 analysis per 5 minutes)
    #[account(
        init_if_needed,
        payer = analyzer,
        space = 8 + RateLimitTracker::INIT_SPACE,
        seeds = [b"rate_limit", analyzer.key().as_ref(), b"analyze_idea"],
        bump
    )]
    pub rate_limit_tracker: Account<'info, RateLimitTracker>,
    
    #[account(
        mut,
        constraint = analyzer.key() == dao_config.authority @ indrasnet_dao_core::error::IndrasError::Unauthorized
    )]
    pub analyzer: Signer<'info>,
    
    /// Security program for CPI call to check AI analysis security
    /// CHECK: Must be Security program (optional - if not provided, security check is skipped)
    pub security_program: Option<Program<'info, indrasnet_dao_security::program::IndrasnetDaoSecurity>>,
    
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

    /// Core program for CPI registration of AI analysis
    pub core_program: Program<'info, indrasnet_dao_core::program::IndrasnetDaoCore>,

    /// AI program account (self) for CPI registration
    /// CHECK: Address is validated by Anchor
    #[account(address = crate::ID)]
    pub ai_program: UncheckedAccount<'info>,

    /// Core-owned AI analysis record (initialized via CPI)
    /// CHECK: Created and validated by Core program during CPI
    #[account(mut)]
    pub analysis_record: UncheckedAccount<'info>,

    /// AI program CPI authority PDA (signer via invoke_signed)
    /// CHECK: PDA verified in Core program
    #[account(
        seeds = [b"ai_cpi_authority"],
        bump
    )]
    pub ai_cpi_authority: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct UpdateIdeaStatusFromAnalysis<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ indrasnet_dao_core::error::IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    #[account(constraint = analysis.idea_id == idea_id @ indrasnet_dao_core::error::IndrasError::InvalidInput)]
    pub analysis: Account<'info, AIAnalysis>,
    
    #[account(
        constraint = updater.key() == dao_config.authority @ indrasnet_dao_core::error::IndrasError::Unauthorized
    )]
    pub updater: Signer<'info>,
}
