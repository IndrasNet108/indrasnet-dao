    // ===== GRANT INSTRUCTIONS =====
    
    /// Create grant request
    pub fn create_grant<'info>(
        ctx: Context<'_, 'info, '_, 'info, CreateGrant<'info>>,
        grant_id: u64,
        idea_id: u64,
        category: state::grant::GrantCategory,
        grant_type: state::grant::GrantType,
        base_amount: u64,
        reputation_bonus: u64,
        milestone_id: Option<u64>,
    ) -> Result<()> {
        instructions::create_grant_handler(
            ctx,
            grant_id,
            idea_id,
            category,
            grant_type,
            base_amount,
            reputation_bonus,
            milestone_id,
            None, // semantic_domain_account
            None, // semantic_distance
            None, // phenomenon_membership
        )
    }
    
    /// Approve grant
    pub fn approve_grant(
        ctx: Context<ApproveGrant>,
    ) -> Result<()> {
        instructions::approve_grant_handler(ctx)
    }
    
    /// Activate grant
    pub fn activate_grant(
        ctx: Context<ActivateGrant>,
    ) -> Result<()> {
        instructions::activate_grant_handler(ctx)
    }
    
    /// Complete grant (move from Active to Completed)
    pub fn complete_grant(
        ctx: Context<CompleteGrant>,
    ) -> Result<()> {
        instructions::complete_grant_handler(ctx)
    }
    
    /// Disburse grant funds to recipient
    pub fn disburse_grant(
        ctx: Context<DisburseGrant>,
        amount: u64,
    ) -> Result<()> {
        instructions::disburse_grant_handler(ctx, amount)
    }
    
    // ===== GRANT VOTING INSTRUCTIONS =====
    
    /// Cast vote on grant
    ///
    /// Votes on a grant request with semantic filtering and competency-based weights.
    pub fn cast_grant_vote(
        ctx: Context<CastGrantVote>,
        grant_id: u64,
        vote_choice: voting_types::VoteType,
        voter_type: crate::state::grant::VoterType,
        competency_multiplier: Option<u64>,
    ) -> Result<()> {
        instructions::grants_voting::cast_grant_vote_handler(
            ctx, grant_id, vote_choice, voter_type, competency_multiplier
        )
    }
    
    /// Tally votes for grant
    ///
    /// Tallies votes and updates grant status based on three-layer voting thresholds.
    pub fn tally_grant_votes(
        ctx: Context<TallyGrantVotes>,
        grant_id: u64,
    ) -> Result<()> {
        instructions::grants_voting::tally_grant_votes_handler(ctx, grant_id)
    }
