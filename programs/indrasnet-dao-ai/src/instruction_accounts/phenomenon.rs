// Accounts structures for phenomenon creation instructions
// NOTE: Types like DaoConfig, Idea, Grant, Phenomenon and anchor_lang types are imported in lib.rs before include!()
// Do not re-import here - they are already imported in lib.rs

/// Accounts for creating a phenomenon from ideas with grants
/// КРИТИЧНО: Феномены создаются ИИ ПОСЛЕ гранта для аналитики
#[derive(Accounts)]
#[instruction(phenomenon_id: u64)]
pub struct CreatePhenomenon<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    /// Phenomenon account - создается в Core программе через CPI
    /// NOTE: Phenomenon находится в Core программе, поэтому используем CPI
    #[account(
        init,
        payer = observer,
        space = 8 + Phenomenon::INIT_SPACE,
        seeds = [b"phenomenon", phenomenon_id.to_le_bytes().as_ref()],
        bump
    )]
    pub phenomenon: Account<'info, Phenomenon>,
    
    /// Observer (ИИ или authority) - создатель феномена
    #[account(
        mut,
        constraint = observer.key() == dao_config.authority @ indrasnet_dao_core::error::IndrasError::Unauthorized
    )]
    pub observer: Signer<'info>,
    
    /// AI Service Registry (optional - for provider whitelist check)
    /// CHECK: If provided, verifies that embedding_provider_pubkey is authorized
    /// NOTE: Using UncheckedAccount to avoid owner check, deserialized manually in handler
    pub ai_service_registry: Option<UncheckedAccount<'info>>,
    
    pub system_program: Program<'info, System>,
}

/// Accounts for adding idea to phenomenon
/// 
/// КРИТИЧНО: Идея может быть добавлена в феномен в трех случаях:
/// 1. С грантом в статусе Approved, Active или Cancelled (отказ пользователя)
/// 2. С переданными правами e.V. без гранта (автор не хочет реализовывать идею)
/// 
/// Grant опционален: если не предоставлен, проверяется наличие переданных прав e.V.
#[derive(Accounts)]
#[instruction(idea_id: u64)]
pub struct AddIdeaToPhenomenon<'info> {
    #[account(
        seeds = [b"dao_config"],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub phenomenon: Account<'info, Phenomenon>,
    
    #[account(constraint = idea.id == idea_id @ indrasnet_dao_core::error::IndrasError::InvalidInput)]
    pub idea: Account<'info, Idea>,
    
    /// Grant account - опционален
    /// Если предоставлен: должен быть Approved, Active или Cancelled (отказ пользователя)
    /// Если не предоставлен: идея должна иметь переданные права e.V. (rights_transferred_to_ev)
    /// CHECK: Validated in handler
    pub grant: Option<Account<'info, Grant>>,
    
    /// Observer (ИИ или authority) - добавляющий идею в феномен
    #[account(
        constraint = observer.key() == dao_config.authority @ indrasnet_dao_core::error::IndrasError::Unauthorized
    )]
    pub observer: Signer<'info>,
}
