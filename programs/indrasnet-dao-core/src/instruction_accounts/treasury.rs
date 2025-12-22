// Accounts structures for treasury instructions
// NOTE: Types like DaoConfig, Treasury, Capability and anchor_lang types are already imported in lib.rs before include!()

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = initializer,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury", dao_config.key().as_ref()],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// SECURITY: Only DAO authority can initialize treasury
    #[account(
        mut,
        constraint = initializer.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub initializer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositToTreasury<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"treasury", dao_config.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub depositor: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawTreasuryWithCapability<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [b"treasury", dao_config.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// Capability account - Check rights to withdraw funds
    /// CHECK: Validated in handler - must be valid, not expired, and grantee must match withdrawer
    #[account(
        constraint = capability.grantee == withdrawer.key() @ IndrasError::Unauthorized,
        constraint = capability.expires_at > 0 @ IndrasError::CapabilityExpired,
        constraint = Clock::get()?.unix_timestamp < capability.expires_at @ IndrasError::CapabilityExpired
    )]
    pub capability: Account<'info, Capability>,
    
    #[account(mut)]
    pub withdrawer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(grantee: Pubkey)]
pub struct GrantCapability<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = granter,
        space = 8 + Capability::INIT_SPACE,
        seeds = [b"capability", grantee.as_ref(), granter.key().as_ref()],
        bump
    )]
    pub capability: Account<'info, Capability>,
    
    #[account(mut)]
    pub granter: Signer<'info>,
    
    /// Granter's role (optional - for permission check)
    /// CHECK: If provided, must have CAN_MANAGE_TREASURY permission (unless DAO authority)
    pub granter_role: Option<Account<'info, crate::state::member::MemberRole>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeCapability<'info> {
    #[account(
        mut,
        constraint = capability.granter == revoker.key() @ IndrasError::Unauthorized
    )]
    pub capability: Account<'info, Capability>,
    
    pub revoker: Signer<'info>,
}
