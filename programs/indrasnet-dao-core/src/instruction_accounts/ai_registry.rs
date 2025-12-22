/// Instruction accounts for managing the AI Service Registry

// AIServiceRegistry is already imported in lib.rs, no need to re-import here

#[derive(Accounts)]
pub struct InitializeAiServiceRegistry<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,

    #[account(
        init,
        payer = authority,
        space = 8 + AIServiceRegistry::INIT_SPACE,
        seeds = [b"ai_service_registry".as_ref()],
        bump
    )]
    pub ai_service_registry: Account<'info, AIServiceRegistry>,

    /// SECURITY: Only DAO authority can initialize AI service registry
    #[account(
        mut,
        constraint = authority.key() == dao_config.authority @ crate::error::IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddAiService<'info> {
    #[account(
        mut,
        has_one = authority @ crate::error::IndrasError::Unauthorized,
    )]
    pub ai_service_registry: Account<'info, AIServiceRegistry>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}
