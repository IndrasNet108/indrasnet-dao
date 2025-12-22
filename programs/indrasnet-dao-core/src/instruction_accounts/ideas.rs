// Accounts structures for idea instructions
// NOTE: Types like DaoConfig, Idea, IdeaExecution, AnchorRecord and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct CreateIdea<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = author,
        space = 8 + Idea::INIT_SPACE,
        seeds = [b"idea", idea_id.to_le_bytes().as_ref()],
        bump
    )]
    pub idea: Account<'info, Idea>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    
    /// Author's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_CREATE_IDEA permission (unless DAO authority)
    /// NOTE: Using Account instead of UncheckedAccount because has_permission() method is used
    pub author_role: Option<Account<'info, crate::state::member::MemberRole>>,
    
    /// AI Service Registry (optional - for embedding provider verification)
    /// CHECK: Validated in handler if embedding provided
    /// NOTE: Using UncheckedAccount to avoid owner check, deserialized manually in handler
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    /// Embedding deduplication account (optional - for anti-duplication)
    /// CHECK: Validated in handler if embedding provided
    /// NOTE: Using UncheckedAccount to avoid owner check, deserialized manually in handler
    pub embedding_deduplication: Option<UncheckedAccount<'info>>,
    
    /// Rate limit tracker (optional - for SEC-INV-8: 1 idea per day per author)
    /// PDA: [b"rate_limit", author.key().as_ref(), b"create_idea"]
    /// CHECK: If provided, validates rate limit; if not provided, rate limit is skipped
    #[account(
        init_if_needed,
        payer = author,
        space = 8 + RateLimitTracker::INIT_SPACE,
        seeds = [b"rate_limit", author.key().as_ref(), b"create_idea"],
        bump
    )]
    pub rate_limit_tracker: Option<Account<'info, RateLimitTracker>>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct CompleteIdea<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    /// Mesh group that works on this idea (optional, but recommended)
    /// CHECK: Validated in handler - must contain idea and be completed
    /// NOTE: Using AccountInfo since MeshGroup state is not yet migrated
    #[account(mut)]
    pub mesh_group: Option<AccountInfo<'info>>,
    
    #[account(
        constraint = completer.key() == dao_config.authority || 
                     completer.key() == idea.author @ IndrasError::Unauthorized
    )]
    pub completer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct ArchiveIdea<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    #[account(
        constraint = archiver.key() == dao_config.authority || 
                     archiver.key() == idea.author @ IndrasError::Unauthorized
    )]
    pub archiver: Signer<'info>,
}

/// Close Idea account and return rent
///
/// NOTE: Grant report validation is performed OFF-CHAIN.
/// Off-chain service must validate all grants before calling this instruction.
/// On-chain we only check authorization - minimize transactions.
#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct CloseIdea<'info> {
    /// Idea account to close
    /// CHECK: Account will be closed and rent returned to destination
    #[account(
        mut,
        close = destination,
        constraint = idea.id == idea_id @ crate::error::IndrasError::InvalidInput
    )]
    pub idea: Account<'info, Idea>,
    
    /// Destination account to receive rent
    /// CHECK: Must be signer to receive rent
    #[account(mut)]
    pub destination: Signer<'info>,
    
    /// Closer (must be author or DAO authority)
    /// SECURITY: Must be signer and authorized
    #[account(
        constraint = closer.key() == idea.author || 
                     closer.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub closer: Signer<'info>,
    
    /// DAO config for validation
    /// CHECK: Read-only, used for validation
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    /// System program for account closure
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct ResubmitIdea<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    #[account(
        constraint = resubmitter.key() == dao_config.authority || 
                     resubmitter.key() == idea.author @ IndrasError::Unauthorized
    )]
    pub resubmitter: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteIdea<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub idea: Account<'info, Idea>,
    
    #[account(
        init,
        payer = executor,
        space = 8 + IdeaExecution::INIT_SPACE,
        seeds = [b"idea_execution", idea.key().as_ref()],
        bump
    )]
    pub idea_execution: Account<'info, IdeaExecution>,
    
    /// Executor must be authority or idea author
    #[account(
        mut,
        constraint = executor.key() == dao_config.authority || 
                     executor.key() == idea.author @ IndrasError::Unauthorized
    )]
    pub executor: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct TransferRightsToEv<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    /// AnchorRecord to verify idea was anchored (authorship fixed)
    #[account(
        constraint = anchor_record.idea_id == idea.id @ IndrasError::InvalidInput,
        constraint = anchor_record.anchorer == idea.author @ IndrasError::Unauthorized
    )]
    pub anchor_record: Account<'info, AnchorRecord>,
    
    /// Only author can transfer rights
    #[account(
        constraint = author.key() == idea.author @ IndrasError::Unauthorized
    )]
    pub author: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct UpdateIdeaEmbedding<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = idea.id == idea_id @ IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    /// Updater must be idea author or DAO authority
    /// SECURITY: Only authorized users can update embeddings
    #[account(
        constraint = updater.key() == idea.author || 
                     updater.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub updater: Signer<'info>,
    
    /// AI Service Registry (optional - for provider verification)
    /// CHECK: Validated in handler
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    /// Embedding deduplication account (optional - for anti-duplication)
    /// CHECK: Validated in handler
    pub embedding_deduplication: Option<UncheckedAccount<'info>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idea_id: u64, proposal_id: u64)]
pub struct ConvertIdeaToProposal<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        seeds = [b"idea", idea_id.to_le_bytes().as_ref()],
        bump = idea.bump
    )]
    pub idea: Account<'info, Idea>,
    
    #[account(
        init,
        payer = author,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    /// Author's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_PROPOSE permission (unless DAO authority)
    pub author_role: Option<Account<'info, crate::state::member::MemberRole>>,
    
    pub system_program: Program<'info, System>,
}
