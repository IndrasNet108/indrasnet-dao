    // ===== IDEA VOTING INSTRUCTIONS =====
    
    /// Cast a vote on an idea
    pub fn cast_idea_vote(
        ctx: Context<CastIdeaVote>,
        idea_id: u64,
        vote_type: voting_types::VoteType,
        weight: u64,
    ) -> Result<()> {
        instructions::cast_idea_vote_handler(ctx, idea_id, vote_type, weight)
    }
    
    /// Tally votes for an idea
    pub fn tally_idea_votes(
        ctx: Context<TallyIdeaVotes>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::tally_idea_votes_handler(ctx, idea_id)
    }
    
    /// Start voting on an idea
    pub fn start_idea_voting(
        ctx: Context<StartIdeaVoting>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::start_idea_voting_handler(ctx, idea_id)
    }
