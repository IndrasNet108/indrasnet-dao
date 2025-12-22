    // ===== VOTING INSTRUCTIONS =====
    
    /// Cast a vote on a proposal
    pub fn cast_vote(
        ctx: Context<CastVote>,
        proposal_id: u64,
        vote_choice: voting_types::VoteType,
    ) -> Result<()> {
        instructions::cast_vote_handler(ctx, proposal_id, vote_choice)
    }
    
    /// Tally votes for a proposal
    pub fn tally_votes(
        ctx: Context<TallyVotes>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::tally_votes_handler(ctx, proposal_id)
    }
    
    /// Execute a proposal
    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
        proposal_id: u64,
        execution_data: String,
    ) -> Result<()> {
        instructions::execute_proposal_handler(ctx, proposal_id, execution_data)
    }
    
    // ===== PROPOSAL EXECUTION MANAGEMENT =====
    
    /// Schedule proposal execution
    pub fn schedule_proposal_execution(
        ctx: Context<ScheduleProposalExecutionCtx>,
        execution_id: u64,
        proposal_id: u64,
        executor: Pubkey,
        execution_data: String,
    ) -> Result<()> {
        instructions::schedule_proposal_execution_handler(ctx, execution_id, proposal_id, executor, execution_data)
    }
    
    /// Update proposal execution
    pub fn update_proposal_execution(
        ctx: Context<UpdateProposalExecutionCtx>,
        execution_data: Option<String>,
        status: Option<state::proposal_execution::ExecutionStatus>,
    ) -> Result<()> {
        instructions::update_proposal_execution_handler(ctx, execution_data, status)
    }
    
    /// Cancel proposal execution
    pub fn cancel_proposal_execution(
        ctx: Context<CancelProposalExecutionCtx>,
    ) -> Result<()> {
        instructions::cancel_proposal_execution_handler(ctx)
    }
    
    // ===== VOTE DELEGATION MANAGEMENT =====
    
    /// Create vote delegation
    pub fn create_vote_delegation(
        ctx: Context<CreateVoteDelegationCtx>,
        delegate: Pubkey,
        weight: u64,
    ) -> Result<()> {
        instructions::create_vote_delegation_handler(ctx, delegate, weight)
    }
    
    /// Update vote delegation weight
    pub fn update_vote_delegation_weight(
        ctx: Context<UpdateVoteDelegationWeightCtx>,
        new_weight: u64,
    ) -> Result<()> {
        instructions::update_vote_delegation_weight_handler(ctx, new_weight)
    }
    
    /// Deactivate vote delegation
    pub fn deactivate_vote_delegation(
        ctx: Context<DeactivateVoteDelegationCtx>,
    ) -> Result<()> {
        instructions::deactivate_vote_delegation_handler(ctx)
    }
    
    /// Reactivate vote delegation
    pub fn reactivate_vote_delegation(
        ctx: Context<ReactivateVoteDelegationCtx>,
    ) -> Result<()> {
        instructions::reactivate_vote_delegation_handler(ctx)
    }
