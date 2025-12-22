// Accounts structures for mesh group instructions
// NOTE: Types like DaoConfig, Idea, MeshGroup and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(mesh_group_id: u64)]
pub struct CreateMeshGroup<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + MeshGroup::INIT_SPACE,
        seeds = [b"mesh_group", mesh_group_id.to_le_bytes().as_ref()],
        bump
    )]
    pub mesh_group: Account<'info, MeshGroup>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    
    /// CHECK: Optional idea account - validated in handler if provided
    /// CRITICAL: If idea is provided, it must pass AI analysis and be Approved
    /// NOTE: Using Account because idea.status and idea.id fields are accessed
    pub idea: Option<Account<'info, Idea>>,
    
    /// CHECK: Optional AI Analysis account - validated in handler if provided
    /// CRITICAL: If AI analysis is provided, it must confirm that idea can enter mesh group
    pub ai_analysis: Option<UncheckedAccount<'info>>,

    /// Core-owned AI analysis registration record (required when idea is provided)
    /// CHECK: Validated in handler if provided
    pub ai_analysis_record: Option<Account<'info, crate::state::ai_analysis_record::AIAnalysisRecord>>,
    
    /// Creator's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_MANAGE_MESH_GROUPS permission (unless DAO authority)
    pub creator_role: Option<Account<'info, crate::state::member::MemberRole>>,
    
    /// AI Service Registry (optional - for embedding provider verification)
    /// CHECK: Validated in handler if embedding provided
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    /// Rate limit tracker (optional - for SEC-INV-9: 1 group per week per creator)
    /// PDA: [b"rate_limit", creator.key().as_ref(), b"create_mesh_group"]
    /// CHECK: If provided, validates rate limit; if not provided, rate limit is skipped
    #[account(
        init_if_needed,
        payer = creator,
        space = 8 + RateLimitTracker::INIT_SPACE,
        seeds = [b"rate_limit", creator.key().as_ref(), b"create_mesh_group"],
        bump
    )]
    pub rate_limit_tracker: Option<Account<'info, RateLimitTracker>>,
}

#[derive(Accounts)]
#[instruction(mesh_group_id: u64)]
pub struct UpdateMeshGroupEmbedding<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut, constraint = mesh_group.id == mesh_group_id @ IndrasError::InvalidInput)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Updater must be mesh group leader, creator, or DAO authority
    /// SECURITY: Only authorized users can update embeddings
    #[account(
        constraint = updater.key() == mesh_group.leader || 
                     updater.key() == mesh_group.created_by || 
                     updater.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub updater: Signer<'info>,
    
    /// AI Service Registry (optional - for provider verification)
    /// CHECK: Validated in handler
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinMeshGroup<'info> {
    #[account(
        mut,
        constraint = mesh_group.status == crate::state::mesh_group::GroupStatus::Forming || 
                     mesh_group.status == crate::state::mesh_group::GroupStatus::Active @ IndrasError::InvalidState
    )]
    pub mesh_group: Account<'info, MeshGroup>,
    
    pub member: Signer<'info>,
    
    #[account(
        constraint = approver.key() == mesh_group.leader || approver.key() == mesh_group.created_by @ IndrasError::Unauthorized
    )]
    pub approver: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveMeshGroupMember<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        constraint = mesh_group.status == crate::state::mesh_group::GroupStatus::Forming || 
                     mesh_group.status == crate::state::mesh_group::GroupStatus::Active @ IndrasError::InvalidState
    )]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// CHECK: Member account to remove - validated in handler by checking membership
    pub member_to_remove: UncheckedAccount<'info>,
    
    pub remover: Signer<'info>,
    
    /// Remover's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_MANAGE_MESH_GROUPS permission (unless leader/creator/authority)
    pub remover_role: Option<Account<'info, crate::state::member::MemberRole>>,
}

#[derive(Accounts)]
pub struct ManageMeshGroup<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    pub manager: Signer<'info>,
    
    /// Manager's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_MANAGE_MESH_GROUPS permission (unless leader/creator/authority)
    pub manager_role: Option<Account<'info, crate::state::member::MemberRole>>,
}

#[derive(Accounts)]
pub struct CloseMeshGroup<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    pub closer: Signer<'info>,
    
    /// Closer's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_MANAGE_MESH_GROUPS permission (unless leader/creator/authority)
    pub closer_role: Option<Account<'info, crate::state::member::MemberRole>>,
}

/// Close Mesh Group account and return rent
///
/// NOTE: Grant report validation is performed OFF-CHAIN.
/// Off-chain service must validate all grants before calling this instruction.
/// On-chain we only check authorization - minimize transactions.
#[derive(Accounts)]
#[instruction(mesh_group_id: u64)]
pub struct CloseMeshGroupAccount<'info> {
    /// Mesh Group account to close
    /// CHECK: Account will be closed and rent returned to destination
    #[account(
        mut,
        close = destination,
        constraint = mesh_group.id == mesh_group_id @ crate::error::IndrasError::InvalidInput
    )]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Destination account to receive rent
    /// CHECK: Must be signer to receive rent
    #[account(mut)]
    pub destination: Signer<'info>,
    
    /// Closer (must be leader, creator, or DAO authority)
    /// SECURITY: Must be signer and authorized
    #[account(
        constraint = closer.key() == mesh_group.leader || 
                     closer.key() == mesh_group.created_by || 
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
pub struct LinkIdeaToMeshGroup<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    #[account(mut)]
    pub idea: Account<'info, Idea>,
    
    /// CHECK: AI Analysis account - validated in handler
    /// CRITICAL: Idea must pass AI analysis before adding to mesh group
    /// Using UncheckedAccount to avoid circular dependency between programs
    pub ai_analysis: Option<UncheckedAccount<'info>>,

    /// Core-owned AI analysis registration record (required for AI validation)
    /// CHECK: Validated in handler
    pub ai_analysis_record: Option<Account<'info, crate::state::ai_analysis_record::AIAnalysisRecord>>,
    
    #[account(
        constraint = linker.key() == mesh_group.leader || 
                     linker.key() == mesh_group.created_by || 
                     linker.key() == idea.author || 
                     linker.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub linker: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddContribution<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Member who made the contribution
    /// CHECK: Validated in handler - must be a member of the group
    pub contributor: Signer<'info>,
}

/// Accounts for anchoring idea in blockchain within mesh group
/// CRITICAL: Anchoring idea to blockchain happens in mesh group after final approval
#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct AnchorIdeaInMeshGroup<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    #[account(mut, constraint = idea.id == idea_id @ IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    /// AnchorRecord for anchoring idea in blockchain
    #[account(
        init,
        payer = anchorer,
        space = 8 + AnchorRecord::INIT_SPACE,
        seeds = [b"anchor_record", idea_id.to_le_bytes().as_ref()],
        bump
    )]
    pub anchor_record: Account<'info, AnchorRecord>,
    
    /// Only mesh group leader or idea author can anchor idea
    #[account(
        mut,
        constraint = (anchorer.key() == mesh_group.leader || 
                      anchorer.key() == mesh_group.created_by || 
                      anchorer.key() == idea.author || 
                      anchorer.key() == dao_config.authority) @ IndrasError::Unauthorized
    )]
    pub anchorer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Accounts for creating a supporting mesh group when main group is full (7 members)
/// Supporting groups work on the same ideas as the main group
/// Mesh group can have maximum 7 members. If more needed, additional mesh group is created.
#[derive(Accounts)]
#[instruction(supporting_group_id: u64)]
pub struct CreateSupportingMeshGroup<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    /// Main mesh group (must be full to create supporting group)
    #[account(mut)]
    pub main_group: Account<'info, MeshGroup>,
    
    /// New supporting mesh group
    #[account(
        init,
        payer = creator,
        space = 8 + MeshGroup::INIT_SPACE,
        seeds = [b"mesh_group", supporting_group_id.to_le_bytes().as_ref()],
        bump
    )]
    pub supporting_group: Account<'info, MeshGroup>,
    
    #[account(
        mut,
        constraint = creator.key() == dao_config.authority || 
                     creator.key() == main_group.leader || 
                     creator.key() == main_group.created_by @ IndrasError::Unauthorized
    )]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
