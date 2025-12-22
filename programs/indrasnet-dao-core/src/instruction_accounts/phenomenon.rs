// Accounts structures for phenomenon instructions (Track B)
// NOTE: Phenomenon type is already imported in lib.rs before include!()

/// Accounts for creating a phenomenon
#[derive(Accounts)]
#[instruction(phenomenon_id: u64)]
pub struct CreatePhenomenon<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        init,
        payer = observer,
        space = 8 + Phenomenon::INIT_SPACE,
        seeds = [b"phenomenon", phenomenon_id.to_le_bytes().as_ref()],
        bump
    )]
    pub phenomenon: Account<'info, Phenomenon>,
    
    /// SECURITY: Only DAO authority can create phenomena (AI-driven feature)
    #[account(
        mut,
        constraint = observer.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub observer: Signer<'info>,
    
    /// AI Service Registry (optional - for provider whitelist check)
    /// CHECK: If provided, verifies that embedding_provider_pubkey is authorized
    /// NOTE: Using UncheckedAccount to avoid owner check, deserialized manually in handler
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    pub system_program: Program<'info, System>,
}

// NOTE: CreatePhenomenonFrom1Idea temporarily disabled
// Lifetime error E0597 persists even with minimal structure (1 Account, no Option)
// This confirms the issue is in Anchor 0.32.1 macro, not in our code
// TODO: Use existing create_phenomenon instruction with related_ideas parameter instead
/*
/// Accounts for creating a phenomenon from 1 idea
#[derive(Accounts)]
#[instruction(phenomenon_id: u64)]
pub struct CreatePhenomenonFrom1Idea<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Phenomenon::INIT_SPACE,
        seeds = [b"phenomenon", phenomenon_id.to_le_bytes().as_ref()],
        bump
    )]
    pub phenomenon: Account<'info, Phenomenon>,
    
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, crate::state::DaoConfig>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub idea1: Account<'info, crate::state::Idea>,
    
    pub system_program: Program<'info, System>,
}
*/
