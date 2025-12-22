// Accounts structures for proposal instructions
// NOTE: Types like DaoConfig, Proposal, IndrasError and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    /// Author's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_PROPOSE permission (unless DAO authority)
    pub author_role: Option<Account<'info, crate::state::member::MemberRole>>,
    
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ActivateProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        constraint = activator.key() == proposal.author @ IndrasError::Unauthorized
    )]
    pub activator: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct PassProposal<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    /// SECURITY: Only DAO authority can pass proposals
    #[account(
        constraint = authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct RejectProposal<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    /// SECURITY: Only DAO authority can reject proposals
    #[account(
        constraint = authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CancelProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        constraint = canceller.key() == proposal.author @ IndrasError::Unauthorized
    )]
    pub canceller: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ArchiveProposal<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    /// SECURITY: Only proposal author or DAO authority can archive
    #[account(
        constraint = archiver.key() == proposal.author || 
                     archiver.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub archiver: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct SetProposalExpiration<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    /// SECURITY: Only proposal author or DAO authority can set expiration
    #[account(
        mut,
        constraint = authority.key() == proposal.author || 
                     authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CheckAndAutoArchiveProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    /// Anyone can check and auto-archive expired proposals
    pub checker: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct AutoTransitionProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    /// Anyone can trigger auto-transition (it's idempotent and safe)
    pub trigger: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64, amendment_id: u64)]
pub struct AmendProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = author,
        space = 8 + ProposalAmendment::INIT_SPACE,
        seeds = [b"proposal_amendment", proposal.key().as_ref(), &amendment_id.to_le_bytes()],
        bump
    )]
    pub amendment: Account<'info, ProposalAmendment>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CreateTreasuryProposal<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = author,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        seeds = [b"treasury", dao_config.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    /// Author's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_PROPOSE permission (unless DAO authority)
    pub author_role: Option<Account<'info, crate::state::member::MemberRole>>,
    
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ExecuteTreasuryProposal<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        mut,
        seeds = [b"treasury", dao_config.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub executor: Signer<'info>,
    
    /// Capability account (optional - required for GrantCapability/RevokeCapability operations)
    /// PDA: [b"capability", capability_grantee.as_ref(), treasury.key().as_ref()]
    /// CHECK: If provided, validates capability operations
    /// NOTE: Seeds computed in handler from operation.capability_grantee
    pub capability: Option<UncheckedAccount<'info>>,
    
    pub system_program: Program<'info, System>,
}
