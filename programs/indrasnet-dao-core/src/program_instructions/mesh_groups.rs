    // ===== MESH GROUP INSTRUCTIONS =====
    
    /// Create mesh group
    ///
    /// According to updated logic:
    /// - AI checks idea for compliance with DAO norms
    /// - AI confirms mesh group creation
    /// - Mesh group is created after AI confirmation
    ///
    /// Mesh group can be 1-7 people (if more needed, additional mesh group is created).
    pub fn create_mesh_group(
        ctx: Context<CreateMeshGroup>,
        mesh_group_id: u64,
        name: String,
        description: String,
        group_type: crate::state::mesh_group::GroupType,
    ) -> Result<()> {
        instructions::create_mesh_group_handler(
            ctx,
            mesh_group_id,
            name,
            description,
            group_type,
            None, // embedding_hash
            None, // embedding_signature
            None, // embedding_provider
            None, // embedding_model
            None, // embedding_model_version
            None, // embedding_provider_pubkey
        )
    }
    
    // ===== MESH GROUP GOVERNANCE INSTRUCTIONS =====
    
    /// Add member to mesh group
    ///
    /// Track A: Only Owner (leader) can add members.
    pub fn add_member_to_mesh_group(
        ctx: Context<AddMemberToMeshGroup>,
        member_pubkey: Pubkey,
        role: crate::state::mesh_group::GroupRole,
    ) -> Result<()> {
        instructions::mesh_groups_governance::add_member_to_mesh_group_handler(
            ctx, member_pubkey, role
        )
    }
    
    /// Remove member from mesh group
    ///
    /// Track A: Only Owner (leader) can remove members.
    pub fn remove_member_from_mesh_group(
        ctx: Context<RemoveMemberFromMeshGroup>,
        member_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::mesh_groups_governance::remove_member_from_mesh_group_handler(
            ctx, member_pubkey
        )
    }
    
    /// Transfer leadership to another member
    ///
    /// Track A: Only current Owner can transfer leadership.
    pub fn transfer_mesh_group_leadership(
        ctx: Context<TransferMeshGroupLeadership>,
        new_leader: Pubkey,
    ) -> Result<()> {
        instructions::mesh_groups_governance::transfer_leadership_handler(ctx, new_leader)
    }
    
    /// Update operating protocol
    ///
    /// Track A: Only Owner can update protocol parameters.
    pub fn update_mesh_group_protocol(
        ctx: Context<UpdateMeshGroupProtocol>,
        meeting_frequency: crate::state::mesh_group::MeetingFrequency,
        decision_quorum: u8,
        contribution_threshold: u32,
        inactivity_timeout_days: u16,
    ) -> Result<()> {
        instructions::mesh_groups_governance::update_mesh_group_protocol_handler(
            ctx, meeting_frequency, decision_quorum, contribution_threshold, inactivity_timeout_days
        )
    }
    
    /// Check inactivity and pause if needed
    ///
    /// Can be called by anyone. Uses protocol.inactivity_timeout_days.
    pub fn check_mesh_group_inactivity(
        ctx: Context<CheckMeshGroupInactivity>,
    ) -> Result<()> {
        instructions::mesh_groups_governance::check_mesh_group_inactivity_handler(ctx)
    }
    
    /// Join mesh group
    pub fn join_mesh_group(
        ctx: Context<JoinMeshGroup>,
        role: crate::state::mesh_group::GroupRole,
    ) -> Result<()> {
        instructions::join_mesh_group_handler(ctx, role)
    }
    
    /// Remove member from mesh group
    pub fn remove_mesh_group_member(
        ctx: Context<RemoveMeshGroupMember>,
    ) -> Result<()> {
        instructions::remove_mesh_group_member_handler(ctx)
    }
    
    /// Start mesh group (Forming -> Active)
    pub fn start_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::start_mesh_group_handler(ctx)
    }
    
    /// Pause mesh group (Active -> Paused)
    pub fn pause_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::pause_mesh_group_handler(ctx)
    }
    
    /// Resume mesh group (Paused -> Active)
    pub fn resume_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::resume_mesh_group_handler(ctx)
    }
    
    /// Complete mesh group (Active -> Completed)
    pub fn complete_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::complete_mesh_group_handler(ctx)
    }
    
    /// Close mesh group
    pub fn close_mesh_group(
        ctx: Context<CloseMeshGroup>,
    ) -> Result<()> {
        instructions::close_mesh_group_handler(ctx)
    }
    
    /// Disband mesh group (any status -> Disbanded)
    pub fn disband_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::disband_mesh_group_handler(ctx)
    }
    
    /// Add idea to mesh group
    ///
    /// CRITICAL: Only Approved ideas can be added to mesh groups.
    /// This ensures AI has checked and approved idea first.
    pub fn add_idea_to_mesh_group(
        ctx: Context<LinkIdeaToMeshGroup>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::add_idea_to_mesh_group_handler(ctx, idea_id)
    }
    
    /// Remove idea from mesh group
    pub fn remove_idea_from_mesh_group(
        ctx: Context<LinkIdeaToMeshGroup>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::remove_idea_from_mesh_group_handler(ctx, idea_id)
    }
    
    /// Anchor idea in blockchain within mesh group
    pub fn anchor_idea_in_mesh_group(
        ctx: Context<AnchorIdeaInMeshGroup>,
        idea_id: u64,
        content_hash: [u8; 32],
    ) -> Result<()> {
        instructions::anchor_idea_in_mesh_group_handler(ctx, idea_id, content_hash)
    }
    
    /// Add contribution to mesh group
    pub fn add_contribution(
        ctx: Context<AddContribution>,
    ) -> Result<()> {
        instructions::add_contribution_handler(ctx)
    }
    
    /// Update mesh group development stage
    pub fn update_mesh_group_stage(
        ctx: Context<ManageMeshGroup>,
        new_stage: crate::state::mesh_group::DevelopmentStage,
    ) -> Result<()> {
        instructions::update_mesh_group_stage_handler(ctx, new_stage)
    }
    
    /// Create supporting mesh group when main group is full (7 members)
    pub fn create_supporting_mesh_group(
        ctx: Context<CreateSupportingMeshGroup>,
        supporting_group_id: u64,
        name: String,
        description: String,
    ) -> Result<()> {
        instructions::create_supporting_mesh_group_handler(ctx, supporting_group_id, name, description)
    }
