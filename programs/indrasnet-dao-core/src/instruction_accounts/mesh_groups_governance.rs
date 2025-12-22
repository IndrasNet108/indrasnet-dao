// Mesh Group Governance instruction accounts
//
// Account structures for mesh group governance operations

#[derive(Accounts)]
pub struct AddMemberToMeshGroup<'info> {
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// SECURITY: Caller must be group leader (Owner)
    #[account(
        constraint = caller.key() == mesh_group.leader @ crate::error::IndrasError::Unauthorized
    )]
    pub caller: Signer<'info>,
    
    /// SEC-INV-15: Member history for cooldown check (optional for MVP)
    /// PDA: [b"member_history", mesh_group.key()]
    pub member_history: Option<UncheckedAccount<'info>>,
    
    /// Member account (optional - for reputation check)
    /// PDA: [b"member", member_pubkey.as_ref()]
    /// CHECK: If provided, validates member reputation; if not provided, reputation check is skipped
    /// NOTE: member_pubkey is passed as instruction parameter
    pub member_account: Option<Account<'info, crate::state::member::Member>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveMemberFromMeshGroup<'info> {
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// SECURITY: Caller must be group leader (Owner)
    #[account(
        constraint = caller.key() == mesh_group.leader @ crate::error::IndrasError::Unauthorized
    )]
    pub caller: Signer<'info>,
    
    /// SEC-INV-15: Member history for cooldown tracking (optional for MVP)
    /// PDA: [b"member_history", mesh_group.key()]
    #[account(mut)]
    pub member_history: Option<Account<'info, crate::state::mesh_group::GroupMemberHistory>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferMeshGroupLeadership<'info> {
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// SECURITY: Caller must be current leader
    #[account(
        constraint = caller.key() == mesh_group.leader @ crate::error::IndrasError::Unauthorized
    )]
    pub caller: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMeshGroupProtocol<'info> {
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Caller must be group leader
    pub caller: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckMeshGroupInactivity<'info> {
    #[account(mut)]
    pub mesh_group: Account<'info, MeshGroup>,
    
    /// Anyone can trigger inactivity check (no signer required, but included for consistency)
    pub checker: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
