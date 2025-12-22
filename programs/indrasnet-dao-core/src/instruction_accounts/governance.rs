// Accounts structures for governance instructions
// NOTE: Types like DaoConfig, Quorum, GovernanceParams and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
pub struct InitializeDao<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + DaoConfig::INIT_SPACE,
        seeds = [b"dao_config"],
        bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MigrateDaoConfig<'info> {
    #[account(
        mut,
        seeds = [b"dao_config"],
        bump
    )]
    pub dao_config: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManageQuorum<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = manager,
        space = 8 + Quorum::INIT_SPACE,
        seeds = [b"quorum", dao_config.key().as_ref()],
        bump
    )]
    pub quorum: Account<'info, Quorum>,
    
    #[account(
        mut,
        constraint = manager.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub manager: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeGovernanceParams<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + GovernanceParams::INIT_SPACE,
        seeds = [b"governance_params", dao_config.key().as_ref()],
        bump
    )]
    pub governance_params: Account<'info, GovernanceParams>,
    
    #[account(
        mut,
        constraint = authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGovernanceParams<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"governance_params", dao_config.key().as_ref()],
        bump = governance_params.bump
    )]
    pub governance_params: Account<'info, GovernanceParams>,
    
    #[account(
        constraint = authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(analytics_id: u64)]
pub struct InitializeGovernanceAnalytics<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + GovernanceAnalyticsMetadata::INIT_SPACE,
        seeds = [b"governance_analytics", dao_config.key().as_ref(), &analytics_id.to_le_bytes()],
        bump
    )]
    pub analytics: Account<'info, GovernanceAnalyticsMetadata>,
    
    #[account(
        mut,
        constraint = authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(participation_id: u64)]
pub struct InitializeGovernanceParticipation<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + GovernanceParticipationMetadata::INIT_SPACE,
        seeds = [b"governance_participation", dao_config.key().as_ref(), &participation_id.to_le_bytes()],
        bump
    )]
    pub participation: Account<'info, GovernanceParticipationMetadata>,
    
    #[account(
        mut,
        constraint = authority.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(voting_id: u64)]
pub struct InitializeGovernanceVoting<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + GovernanceVotingMetadata::INIT_SPACE,
        seeds = [b"governance_voting", proposal.key().as_ref(), &voting_id.to_le_bytes()],
        bump
    )]
    pub voting: Account<'info, GovernanceVotingMetadata>,
    
    #[account(
        mut,
        constraint = authority.key() == dao_config.authority || authority.key() == proposal.author @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
