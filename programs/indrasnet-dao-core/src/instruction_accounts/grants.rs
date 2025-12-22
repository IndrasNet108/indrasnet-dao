// Accounts structures for grant instructions
// NOTE: Types like DaoConfig, Idea, Grant, Treasury and anchor_lang types are already imported in lib.rs before include!()
// MeshGroup is imported via state::mesh_group::MeshGroup
// GrantReport is imported via state::grant::GrantReport

#[derive(Accounts)]
#[instruction(grant_id: u64, idea_id: u64)]
pub struct CreateGrant<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + Grant::INIT_SPACE,
        seeds = [b"grant", grant_id.to_le_bytes().as_ref()],
        bump
    )]
    pub grant: Account<'info, Grant>,
    
    /// Mesh group that works on the idea
    /// CHECK: Validated in handler - must be Active, contain idea
    /// NOTE: Using UncheckedAccount to reduce stack size (BPF limit)
    #[account(mut)]
    pub mesh_group: UncheckedAccount<'info>,
    
    /// Idea associated with the grant
    /// CHECK: Validated in handler - must match idea_id and have valid status
    /// NOTE: Using UncheckedAccount to reduce stack size (BPF limit)
    pub idea: UncheckedAccount<'info>,
    
    // NOTE: Phenomenon is NOT required when creating grant
    // Phenomena are created AFTER grant for analytics (according to updated logic)
    
    /// AI Analysis account - Check compliance with DAO norms
    /// CHECK: Validated in handler - must exist, be from Core program, and idea must be Approved
    /// Seeds: [b"ai_analysis", idea.key().as_ref()]
    /// Program: indrasnet_dao_core (AI analysis integrated in Core)
    /// NOTE: Using UncheckedAccount to avoid circular dependency, validated in handler
    pub analysis: UncheckedAccount<'info>,

    /// Core-owned AI analysis registration record (created via CPI from AI program)
    #[account(
        seeds = [b"ai_analysis_record", idea.key().as_ref()],
        bump = analysis_record.bump
    )]
    pub analysis_record: Account<'info, crate::state::ai_analysis_record::AIAnalysisRecord>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    /// System program - MUST be before optional accounts for allow-missing-optionals to work
    pub system_program: Program<'info, System>,
    
    // Track B: Optional semantic domain account (B4)
    /// CHECK: Validated in handler if provided - must exist and be from Core program
    /// NOTE: Using AccountInfo to reduce stack size (BPF limit)
    /// CRITICAL: Must be at the END of accounts list for allow-missing-optionals to work
    pub semantic_domain: Option<AccountInfo<'info>>,
    
    // Track B: Optional phenomenon account (B4)
    /// CHECK: Validated in handler if provided - must exist and contain idea in related_ideas
    /// NOTE: Using AccountInfo to reduce stack size (BPF limit)
    /// CRITICAL: Must be at the END of accounts list for allow-missing-optionals to work
    pub phenomenon: Option<AccountInfo<'info>>,
    
    /// Creator's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_CREATE_GRANT permission (unless DAO authority)
    /// NOTE: Using AccountInfo to reduce stack size (BPF limit)
    pub creator_role: Option<AccountInfo<'info>>,
    
    /// AI Service Registry (optional - for semantic domain provider verification)
    /// CHECK: If provided, validates semantic domain provider signature
    /// NOTE: Using UncheckedAccount to avoid owner check, deserialized manually in handler
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
}

#[derive(Accounts)]
pub struct ApproveGrant<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = grant.status == crate::state::grant::GrantStatus::Pending @ IndrasError::InvalidState
    )]
    pub grant: Account<'info, Grant>,
    
    /// Mesh group that will receive the grant
    /// CHECK: Validated in handler - grant.mesh_group must match mesh_group.key()
    #[account(
        mut,
        constraint = grant.mesh_group == mesh_group.key() @ IndrasError::InvalidInput
    )]
    pub mesh_group: Account<'info, MeshGroup>,
    
    #[account(
        constraint = approver.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub approver: Signer<'info>,
}

#[derive(Accounts)]
pub struct ActivateGrant<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = grant.status == crate::state::grant::GrantStatus::Approved @ IndrasError::InvalidState
    )]
    pub grant: Account<'info, Grant>,
    
    #[account(
        constraint = activator.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub activator: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteGrant<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = grant.status == crate::state::grant::GrantStatus::Active @ IndrasError::InvalidState
    )]
    pub grant: Account<'info, Grant>,
    
    #[account(
        constraint = completer.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub completer: Signer<'info>,
}

#[derive(Accounts)]
pub struct DisburseGrant<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub grant: Account<'info, Grant>,
    
    /// Mesh group that will receive the disbursement
    /// CHECK: Validated in handler - grant.mesh_group must match mesh_group.key()
    #[account(
        mut,
        constraint = grant.mesh_group == mesh_group.key() @ IndrasError::InvalidInput
    )]
    pub mesh_group: Account<'info, MeshGroup>,
    
    #[account(
        mut,
        seeds = [b"treasury", dao_config.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// CHECK: Recipient account - validated by disburser authority
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,
    
    #[account(
        constraint = disburser.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub disburser: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// ===== GRANT VOTING ACCOUNTS =====

#[derive(Accounts)]
#[instruction(grant_id: u64)]
pub struct CastGrantVote<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = grant.status == crate::state::grant::GrantStatus::Pending @ IndrasError::InvalidState,
        constraint = Clock::get()?.unix_timestamp <= grant.voting_end @ IndrasError::InvalidState
    )]
    pub grant: Account<'info, Grant>,
    
    /// Mesh group that is requesting the grant
    /// CHECK: Validated in handler - must contain grant's idea
    /// Using UncheckedAccount to reduce stack size (BPF limit)
    #[account(mut)]
    pub mesh_group: UncheckedAccount<'info>,
    
    /// Idea associated with the grant
    /// Using UncheckedAccount to reduce stack size (BPF limit)
    pub idea: UncheckedAccount<'info>,
    
    /// Vote account - PDA with voter in seeds to prevent duplicate voting
    #[account(
        init,
        payer = voter,
        space = 8 + crate::state::grant::GrantVote::INIT_SPACE,
        seeds = [b"grant_vote", grant.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, crate::state::grant::GrantVote>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    /// Expert entry (optional - required if voter_type is Expert)
    /// CHECK: Validated in handler - must be valid expert for grant's semantic domain
    /// Using UncheckedAccount to reduce stack size
    /// NOTE: Domain index validation is done off-chain or via expert_entry PDA seeds
    pub expert_entry: Option<UncheckedAccount<'info>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(grant_id: u64)]
pub struct TallyGrantVotes<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = grant.status == crate::state::grant::GrantStatus::Pending @ IndrasError::InvalidState
    )]
    pub grant: Account<'info, Grant>,
    
    /// Mesh group that is requesting the grant
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Idea associated with the grant
    pub idea: Account<'info, Idea>,
    
    /// Authority who can tally votes (DAO authority only for security)
    /// SECURITY: Only DAO authority can tally votes to prevent manipulation
    #[account(
        constraint = tally_authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub tally_authority: Signer<'info>,
}

// ===== GRANT REPORT ACCOUNTS =====

/// Submit grant report
#[derive(Accounts)]
#[instruction(grant_id: u64)]
pub struct SubmitGrantReport<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub grant: Account<'info, Grant>,
    
    /// Mesh group that received the grant (for membership validation)
    /// CHECK: Validated in handler - submitter must be member or authority
    #[account(mut)]
    pub mesh_group: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = submitter,
        space = 8 + GrantReport::INIT_SPACE,
        seeds = [b"grant_report", grant_id.to_le_bytes().as_ref()],
        bump
    )]
    pub report: Account<'info, GrantReport>,
    
    /// SECURITY: Submitter must be mesh group member or DAO authority
    /// Note: Full membership check is done in handler (requires deserialization)
    #[account(mut)]
    pub submitter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Approve grant report
#[derive(Accounts)]
#[instruction(grant_id: u64)]
pub struct ApproveGrantReport<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub grant: Account<'info, Grant>,
    
    #[account(mut)]
    pub report: Account<'info, GrantReport>,
    
    #[account(
        constraint = approver.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub approver: Signer<'info>,
}

/// Reject grant report
#[derive(Accounts)]
#[instruction(grant_id: u64)]
pub struct RejectGrantReport<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub grant: Account<'info, Grant>,
    
    #[account(mut)]
    pub report: Account<'info, GrantReport>,
    
    #[account(
        constraint = rejector.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub rejector: Signer<'info>,
}
