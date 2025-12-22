    // ===== PROPOSAL INSTRUCTIONS =====
    
    /// Create a new proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_id: u64,
        title: String,
        description: String,
        proposal_type: String,
        voting_duration: Option<i64>,
    ) -> Result<()> {
        instructions::create_proposal_handler(ctx, proposal_id, title, description, proposal_type, voting_duration)
    }
    
    /// Activate proposal (move from Draft to Active)
    pub fn activate_proposal(
        ctx: Context<ActivateProposal>,
        proposal_id: u64,
        min_quorum: u64,
        total_members: u64,
    ) -> Result<()> {
        instructions::activate_proposal_handler(ctx, proposal_id, min_quorum, total_members)
    }
    
    /// Pass proposal (move from Active to Passed)
    pub fn pass_proposal(
        ctx: Context<PassProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::pass_proposal_handler(ctx, proposal_id)
    }
    
    /// Reject proposal (move from Active to Rejected)
    pub fn reject_proposal(
        ctx: Context<RejectProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::reject_proposal_handler(ctx, proposal_id)
    }
    
    /// Cancel proposal (move from Draft or Active to Cancelled)
    pub fn cancel_proposal(
        ctx: Context<CancelProposal>,
        proposal_id: u64,
        reason: String,
    ) -> Result<()> {
        instructions::cancel_proposal_handler(ctx, proposal_id, reason)
    }
    
    /// Archive proposal (move from Executed, Rejected, or Cancelled to Archived)
    pub fn archive_proposal(
        ctx: Context<ArchiveProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::archive_proposal_handler(ctx, proposal_id)
    }
