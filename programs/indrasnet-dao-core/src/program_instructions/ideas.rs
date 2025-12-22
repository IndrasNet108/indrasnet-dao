    // ===== IDEA INSTRUCTIONS =====
    
    /// Create a new idea
    pub fn create_idea(
        ctx: Context<CreateIdea>,
        idea_id: u64,
        title: String,
        description: String,
    ) -> Result<()> {
        instructions::create_idea_handler(
            ctx,
            idea_id,
            title,
            description,
            None, // embedding_hash
            None, // embedding_signature
            None, // embedding_provider
            None, // embedding_model
            None, // embedding_model_version
            None, // embedding_provider_pubkey
        )
    }
    
    /// Complete idea (move from InProgress to Completed)
    pub fn complete_idea(
        ctx: Context<CompleteIdea>,
        idea_id: u64,
        completion_report: String,
    ) -> Result<()> {
        instructions::complete_idea_handler(ctx, idea_id, completion_report)
    }
    
    /// Archive idea (move from Completed/Executed/Rejected to Archived)
    pub fn archive_idea(
        ctx: Context<ArchiveIdea>,
        idea_id: u64,
        reason: String,
    ) -> Result<()> {
        instructions::archive_idea_handler(ctx, idea_id, reason)
    }
    
    /// Resubmit idea (move from Rejected to Resubmitted)
    pub fn resubmit_idea(
        ctx: Context<ResubmitIdea>,
        idea_id: u64,
        updated_title: Option<String>,
        updated_description: Option<String>,
    ) -> Result<()> {
        instructions::resubmit_idea_handler(ctx, idea_id, updated_title, updated_description)
    }
    
    /// Execute idea (move from Completed to Executed)
    pub fn execute_idea(
        ctx: Context<ExecuteIdea>,
        idea_id: u64,
        execution_data: String,
    ) -> Result<()> {
        instructions::execute_idea_handler(ctx, idea_id, execution_data)
    }
    
    /// Transfer rights to e.V. without grant (voluntary transfer)
    pub fn transfer_rights_to_ev(
        ctx: Context<TransferRightsToEv>,
        can_modify: bool,
        can_distribute: bool,
        can_reproduce: bool,
        can_develop: bool,
        can_sublicense: bool,
        can_gift: bool,
        can_bequeath: bool,
    ) -> Result<()> {
        instructions::transfer_rights_to_ev_handler(
            ctx,
            can_modify,
            can_distribute,
            can_reproduce,
            can_develop,
            can_sublicense,
            can_gift,
            can_bequeath,
        )
    }
