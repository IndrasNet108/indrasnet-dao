// Accounts structures for idea voting instructions
// NOTE: Types like DaoConfig, Idea, IdeaVote, MeshGroup and anchor_lang types are already imported in lib.rs before include!()
// NOTE: Phenomenon is NOT required for voting (created AFTER grant for analytics)

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct CastIdeaVote<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub idea: Account<'info, Idea>,
    
    /// Mesh group that contains this idea (required for voting)
    /// CHECK: Validated in handler - must contain idea
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Vote account - PDA with voter in seeds to prevent duplicate voting
    #[account(
        init,
        payer = voter,
        space = 8 + IdeaVote::INIT_SPACE,
        seeds = [b"idea_vote", idea.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, IdeaVote>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct TallyIdeaVotes<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub idea: Account<'info, Idea>,
    
    /// Governance params for quorum calculation (optional - can be initialized later)
    /// CHECK: Governance params PDA - validated by seeds
    #[account(
        seeds = [b"governance_params", dao_config.key().as_ref()],
        bump
    )]
    pub governance_params: UncheckedAccount<'info>,
    
    /// Anchor record - will be created if voting passes (to record grant decision)
    /// CHECK: Anchor record PDA - created in handler if voting passes
    #[account(
        init_if_needed,
        payer = tallyer,
        space = 8 + AnchorRecord::INIT_SPACE,
        seeds = [b"anchor_record", idea_id.to_le_bytes().as_ref()],
        bump
    )]
    pub anchor_record: Account<'info, AnchorRecord>,
    
    /// Tallyer must be authority (only authority can tally votes)
    #[account(
        mut,
        constraint = tallyer.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub tallyer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    
    // Vote accounts passed via remaining_accounts
    // Each account should be an IdeaVote account for this idea
}

/// Accounts for starting voting on an idea
/// Validates that idea is ready: Approved, in mesh group
/// NOTE: Phenomenon is NOT required (created AFTER grant for analytics)
/// Voting is about granting funds, so anchoring happens AFTER successful voting
#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct StartIdeaVoting<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub idea: Account<'info, Idea>,
    
    /// Mesh group that contains this idea
    /// CHECK: Validated in handler - must contain idea
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Phenomenon account (optional - for validation)
    /// CHECK: Validated in handler if provided
    pub phenomenon: Option<UncheckedAccount<'info>>,
    
    /// Starter must be authority (only authority can start voting)
    #[account(
        constraint = starter.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub starter: Signer<'info>,
}
