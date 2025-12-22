// Accounts structures for metaphenomenon instructions
// NOTE: Types like DaoConfig, Metaphenomenon and anchor_lang types are already imported in lib.rs before include!()
// Do not re-import here - they are already imported in lib.rs

#[derive(Accounts)]
#[instruction(metaphenomenon_id: u64)]
pub struct CreateMetaphenomenon<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, indrasnet_dao_core::state::DaoConfig>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + Metaphenomenon::INIT_SPACE,
        seeds = [b"metaphenomenon", metaphenomenon_id.to_le_bytes().as_ref()],
        bump
    )]
    pub metaphenomenon: Account<'info, Metaphenomenon>,
    
    #[account(
        mut,
        constraint = creator.key() == dao_config.authority @ IndrasError::Unauthorized
    )]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(phenomenon_id: u64)]
pub struct AddPhenomenonToMetaphenomenon<'info> {
    #[account(
        mut,
        constraint = metaphenomenon.related_phenomena.len() < Metaphenomenon::MAX_RELATED_PHENOMENA @ IndrasError::InvalidInput
    )]
    pub metaphenomenon: Account<'info, Metaphenomenon>,
    
    /// Phenomenon to add to metaphenomenon
    /// CHECK: Validated in handler - must exist and be from Core program
    /// NOTE: Using UncheckedAccount to avoid circular dependency, validated in handler
    /// Program: indrasnet_dao_core
    pub phenomenon: UncheckedAccount<'info>,
    
    #[account(
        mut,
        constraint = authority.key() == metaphenomenon.observer @ IndrasError::Unauthorized
    )]
    pub authority: Signer<'info>,
}
