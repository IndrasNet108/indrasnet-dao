//! Memory - память коллективного AI
//!
//! On-chain: Хранение метаданных памяти, ссылок на данные
//! Off-chain: Фактическое хранение и обработка данных памяти

use anchor_lang::prelude::*;
use crate::collective_ai_functions::types::*;

/// Memory metadata (on-chain)
///
/// Хранит метаданные памяти:
/// - ID памяти
/// - Тип памяти
/// - Ссылка на данные (URI или hash)
/// - Размер данных
#[account]
#[derive(InitSpace)]
pub struct MemoryMetadata {
    /// Уникальный ID памяти
    pub memory_id: u64,
    /// Тип памяти
    pub memory_type: MemoryType,
    /// Ссылка на данные (URI или hash)
    #[max_len(200)]
    pub data_uri: String,
    /// Hash данных для проверки целостности
    pub data_hash: [u8; 32],
    /// Размер данных (в байтах)
    pub data_size: u64,
    /// Время создания
    pub created_at: i64,
    /// Время последнего доступа
    pub last_accessed_at: Option<i64>,
    /// Количество обращений
    pub access_count: u64,
    /// Bump seed для PDA
    pub bump: u8,
}

impl MemoryMetadata {
    /// Обновление времени последнего доступа
    pub fn update_access(&mut self, current_time: i64) {
        self.last_accessed_at = Some(current_time);
        self.access_count = self.access_count.saturating_add(1);
    }

    /// Проверка целостности данных
    pub fn verify_hash(&self, computed_hash: &[u8; 32]) -> bool {
        self.data_hash == *computed_hash
    }
}

/// On-chain функции для работы с памятью
pub mod onchain {
    use super::*;
    use crate::error::IndrasError;

    /// Инициализация метаданных памяти
    pub fn initialize_memory_metadata(
        memory: &mut MemoryMetadata,
        memory_id: u64,
        memory_type: MemoryType,
        data_uri: String,
        data_hash: [u8; 32],
        data_size: u64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(memory_id > 0, IndrasError::InvalidInput);
        require!(!data_uri.is_empty(), IndrasError::InvalidInput);
        require!(data_uri.len() <= 200, IndrasError::InvalidInput);
        require!(data_size > 0, IndrasError::InvalidInput);
        
        memory.memory_id = memory_id;
        memory.memory_type = memory_type;
        memory.data_uri = data_uri;
        memory.data_hash = data_hash;
        memory.data_size = data_size;
        memory.created_at = current_time;
        memory.last_accessed_at = None;
        memory.access_count = 0;
        memory.bump = bump;
        
        Ok(())
    }

    /// Обновление метаданных памяти
    pub fn update_memory_metadata(
        memory: &mut MemoryMetadata,
        new_data_uri: String,
        new_data_hash: [u8; 32],
        new_data_size: u64,
        current_time: i64,
    ) -> Result<()> {
        require!(!new_data_uri.is_empty(), IndrasError::InvalidInput);
        require!(new_data_uri.len() <= 200, IndrasError::InvalidInput);
        require!(new_data_size > 0, IndrasError::InvalidInput);
        
        memory.data_uri = new_data_uri;
        memory.data_hash = new_data_hash;
        memory.data_size = new_data_size;
        memory.update_access(current_time);
        
        Ok(())
    }
}

/// Off-chain функции для работы с памятью
///
/// Эти функции должны быть реализованы в off-chain сервисе
/// для фактического хранения и обработки данных памяти.
pub mod offchain {
    // Off-chain функции будут реализованы в отдельном сервисе
    
    /// Сохранение данных в память
    pub fn store_memory(_memory_id: u64, _data: &[u8]) -> Result<String, String> {
        // Реализация в off-chain сервисе
        // Сохраняет данные в БД или файловую систему
        // Возвращает URI или hash
        Ok(String::new())
    }

    /// Загрузка данных из памяти
    pub fn load_memory(_memory_id: u64) -> Result<Vec<u8>, String> {
        // Реализация в off-chain сервисе
        // Загружает данные по URI
        Ok(Vec::new())
    }

    /// Поиск в памяти
    pub fn search_memory(_query: &str) -> Vec<u64> {
        // Реализация в off-chain сервисе
        // Поиск по содержимому памяти
        vec![]
    }
}
