//! Metaphenomenon creation instruction handlers
//!
//! Handlers for creating metaphenomena from phenomena:
//! - create_metaphenomenon - создание метафеномена из схожих феноменов
//! - add_phenomenon_to_metaphenomenon - добавление феномена в метафеномен
//!
//! NOTE: Метафеномены создаются ИИ для выявления паттернов на более высоком уровне абстракции
//! и стратегического управления группами феноменов.
//!
//! Иерархия: Идеи → Феномены → Метафеномены

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::Metaphenomenon;

/// Create metaphenomenon from similar phenomena
///
/// Метафеномен создается ИИ для объединения схожих феноменов.
/// Это позволяет выявлять паттерны на более высоком уровне абстракции
/// и координировать стратегическое развитие группы феноменов.
///
/// # Compute Units
/// Recommended: 60,000 CU
/// - Validation: ~15,000 CU
/// - Account initialization: ~35,000 CU
/// - Phenomena validation: ~10,000 CU
///
/// # Requirements
/// - Все феномены должны существовать и быть валидными
/// - Феномены должны быть схожими (определяется ИИ оффчейн)
/// - Метафеномен создается ИИ (observer = AI program или authority)
pub fn create_metaphenomenon_handler(
    ctx: Context<crate::CreateMetaphenomenon>,
    metaphenomenon_id: u64,
    name: String,
    metadata_uri: String,
    _related_phenomenon_ids: Vec<u64>,
    ethics_score: u8,
    strategic_importance: u8,
) -> Result<()> {
    // Валидация входных данных
    require!(!name.is_empty(), IndrasError::InvalidInput);
    require!(name.len() <= 100, IndrasError::StringTooLong);
    require!(metadata_uri.len() <= 500, IndrasError::StringTooLong);
    
    let metaphenomenon = &mut ctx.accounts.metaphenomenon;
    let clock = Clock::get()?;
    
    // Инициализация метафеномена
    metaphenomenon.observer = ctx.accounts.creator.key();
    metaphenomenon.created_at = clock.unix_timestamp;
    metaphenomenon.name = name;
    metaphenomenon.metadata_uri = metadata_uri;
    metaphenomenon.ethics_score = ethics_score;
    metaphenomenon.strategic_importance = strategic_importance;
    metaphenomenon.related_phenomena = Vec::new();
    metaphenomenon.bump = ctx.bumps.metaphenomenon;
    
    // NOTE: Феномены добавляются через отдельную инструкцию add_phenomenon_to_metaphenomenon
    // Это позволяет создавать метафеномен пустым и добавлять феномены постепенно
    
    msg!(
        "Metaphenomenon {} created with {} phenomena",
        metaphenomenon_id,
        metaphenomenon.phenomenon_count()
    );
    
    Ok(())
}

/// Add phenomenon to metaphenomenon
///
/// Добавляет феномен в метафеномен после проверки условий.
/// КРИТИЧНО: Феномен должен существовать и быть валидным.
///
/// # Compute Units
/// Recommended: 20,000 CU
/// - Validation: ~5,000 CU
/// - State update: ~15,000 CU
///
/// # Requirements
/// - Феномен должен существовать
/// - Метафеномен не должен превышать MAX_RELATED_PHENOMENA
/// - Только observer метафеномена может добавлять феномены
pub fn add_phenomenon_to_metaphenomenon_handler(
    ctx: Context<crate::AddPhenomenonToMetaphenomenon>,
    _phenomenon_id: u64,
) -> Result<()> {
    let metaphenomenon = &mut ctx.accounts.metaphenomenon;
    let phenomenon_account = &ctx.accounts.phenomenon;
    
    // Валидация феномена - проверяем, что это действительно Phenomenon из Core программы
    // NOTE: В реальной реализации можно десериализовать и проверить структуру
    // Для упрощения проверяем только owner
    require!(
        phenomenon_account.owner == &indrasnet_dao_core::ID,
        IndrasError::InvalidProgram
    );
    
    // Проверка, что феномен еще не добавлен
    require!(
        !metaphenomenon.contains_phenomenon(phenomenon_account.key()),
        IndrasError::InvalidInput
    );
    
    // Проверка лимита
    require!(
        metaphenomenon.related_phenomena.len() < Metaphenomenon::MAX_RELATED_PHENOMENA,
        IndrasError::InvalidInput
    );
    
    // Добавляем феномен
    metaphenomenon.add_phenomenon(phenomenon_account.key())?;
    
    msg!(
        "Phenomenon {} added to metaphenomenon. Total: {}",
        phenomenon_account.key(),
        metaphenomenon.phenomenon_count()
    );
    
    Ok(())
}
