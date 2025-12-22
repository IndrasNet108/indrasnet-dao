// Accounts structures for voting instructions
// NOTE: Types like DaoConfig, Proposal, IdeaVote, ProposalExecution, VoteDelegation and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = voter,
        space = 8 + IdeaVote::INIT_SPACE,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, IdeaVote>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TallyVotes<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    /// SECURITY: Only proposal author or DAO authority can tally votes
    #[account(
        constraint = author.key() == proposal.author || 
                     author.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub author: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ExecuteProposal<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = executor,
        space = 8 + ProposalExecution::INIT_SPACE,
        seeds = [b"proposal_execution", dao_config.key().as_ref(), &proposal_id.to_le_bytes()],
        bump
    )]
    pub proposal_execution: Account<'info, ProposalExecution>,
    
    /// SECURITY: Only DAO authority can execute proposals
    #[account(
        mut,
        constraint = executor.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub executor: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// ===== PROPOSAL EXECUTION MANAGEMENT ACCOUNTS =====

#[derive(Accounts)]
#[instruction(execution_id: u64)]
pub struct ScheduleProposalExecutionCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = scheduler,
        space = 8 + ProposalExecution::INIT_SPACE,
        seeds = [b"proposal_execution", dao_config.key().as_ref(), &execution_id.to_le_bytes()],
        bump
    )]
    pub proposal_execution: Account<'info, ProposalExecution>,
    
    /// SECURITY: Only proposal author or DAO authority can schedule execution
    #[account(
        mut,
        constraint = scheduler.key() == proposal.author || 
                     scheduler.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub scheduler: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProposalExecutionCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"proposal_execution", dao_config.key().as_ref(), &proposal_execution.id.to_le_bytes()],
        bump = proposal_execution.bump,
        constraint = updater.key() == proposal_execution.executor || updater.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub proposal_execution: Account<'info, ProposalExecution>,
    
    #[account(
        constraint = updater.key() == proposal_execution.executor || updater.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub updater: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelProposalExecutionCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"proposal_execution", dao_config.key().as_ref(), &proposal_execution.id.to_le_bytes()],
        bump = proposal_execution.bump,
        constraint = proposal_execution.status == crate::state::proposal_execution::ExecutionStatus::Pending || 
                     proposal_execution.status == crate::state::proposal_execution::ExecutionStatus::InProgress @ IndrasError::InvalidState
    )]
    pub proposal_execution: Account<'info, ProposalExecution>,
    
    #[account(
        constraint = canceller.key() == proposal_execution.executor || canceller.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub canceller: Signer<'info>,
}

// ===== VOTE DELEGATION MANAGEMENT ACCOUNTS =====

#[derive(Accounts)]
pub struct CreateVoteDelegationCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = delegator,
        space = 8 + VoteDelegation::INIT_SPACE,
        seeds = [b"vote_delegation", dao_config.key().as_ref(), delegator.key().as_ref(), delegate.key().as_ref()],
        bump
    )]
    pub vote_delegation: Account<'info, VoteDelegation>,
    
    #[account(mut)]
    pub delegator: Signer<'info>,
    
    /// CHECK: Delegate is validated in handler
    pub delegate: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateVoteDelegationWeightCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"vote_delegation", dao_config.key().as_ref(), delegator.key().as_ref(), vote_delegation.delegate.as_ref()],
        bump = vote_delegation.bump,
        constraint = delegator.key() == vote_delegation.delegator @ IndrasError::Unauthorized,
        constraint = vote_delegation.is_active @ IndrasError::InvalidState
    )]
    pub vote_delegation: Account<'info, VoteDelegation>,
    
    #[account(
        constraint = delegator.key() == vote_delegation.delegator @ IndrasError::Unauthorized
    )]
    pub delegator: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetVoteDelegationExpirationCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"vote_delegation", dao_config.key().as_ref(), vote_delegation.delegator.as_ref(), vote_delegation.delegate.as_ref()],
        bump = vote_delegation.bump
    )]
    pub vote_delegation: Account<'info, VoteDelegation>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CheckAndAutoDeactivateDelegationCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub vote_delegation: Account<'info, VoteDelegation>,
    
    /// Anyone can check and auto-deactivate expired delegations
    pub checker: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeactivateVoteDelegationCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"vote_delegation", dao_config.key().as_ref(), delegator.key().as_ref(), vote_delegation.delegate.as_ref()],
        bump = vote_delegation.bump,
        constraint = delegator.key() == vote_delegation.delegator @ IndrasError::Unauthorized,
        constraint = vote_delegation.is_active @ IndrasError::InvalidState
    )]
    pub vote_delegation: Account<'info, VoteDelegation>,
    
    #[account(
        constraint = delegator.key() == vote_delegation.delegator @ IndrasError::Unauthorized
    )]
    pub delegator: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReactivateVoteDelegationCtx<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"vote_delegation", dao_config.key().as_ref(), delegator.key().as_ref(), vote_delegation.delegate.as_ref()],
        bump = vote_delegation.bump,
        constraint = delegator.key() == vote_delegation.delegator @ IndrasError::Unauthorized,
        constraint = !vote_delegation.is_active @ IndrasError::InvalidState
    )]
    pub vote_delegation: Account<'info, VoteDelegation>,
    
    #[account(
        constraint = delegator.key() == vote_delegation.delegator @ IndrasError::Unauthorized
    )]
    pub delegator: Signer<'info>,
}
