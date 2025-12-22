//! Synapse - связь между узлами в коллективном AI
//!
//! On-chain: Хранение метаданных связей, весов, типов
//! Off-chain: Фактическая передача данных между узлами

use anchor_lang::prelude::*;
use crate::collective_ai_functions::types::*;

/// Synapse metadata (on-chain)
///
/// Хранит метаданные связи между узлами:
/// - ID связи
/// - Тип связи
/// - Вес связи (для усиления сигнала)
/// - Статус связи
#[account]
#[derive(InitSpace)]
pub struct SynapseMetadata {
    /// Уникальный ID связи
    pub synapse_id: u64,
    /// ID исходного нейрона (откуда идет сигнал)
    pub source_neuron_id: u64,
    /// ID целевого нейрона (куда идет сигнал)
    pub target_neuron_id: u64,
    /// Тип связи
    pub synapse_type: SynapseType,
    /// Вес связи (0-100, для усиления сигнала)
    pub weight: u8,
    /// Статус связи (активна/неактивна)
    pub is_active: bool,
    /// Время создания
    pub created_at: i64,
    /// Время последнего использования
    pub last_used_at: Option<i64>,
    /// Bump seed для PDA
    pub bump: u8,
}

impl SynapseMetadata {
    /// Проверка, активна ли связь
    pub fn is_active_synapse(&self) -> bool {
        self.is_active
    }

    /// Обновление веса связи
    pub fn update_weight(&mut self, new_weight: u8) -> Result<()> {
        require!(new_weight <= 100, crate::error::IndrasError::InvalidInput);
        self.weight = new_weight;
        Ok(())
    }

    /// Активация связи
    pub fn activate(&mut self, current_time: i64) {
        self.is_active = true;
        self.last_used_at = Some(current_time);
    }

    /// Деактивация связи
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

/// On-chain функции для работы со связями
pub mod onchain {
    use super::*;
    use crate::error::IndrasError;

    /// Инициализация метаданных связи
    pub fn initialize_synapse_metadata(
        synapse: &mut SynapseMetadata,
        synapse_id: u64,
        source_neuron_id: u64,
        target_neuron_id: u64,
        synapse_type: SynapseType,
        weight: u8,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(synapse_id > 0, IndrasError::InvalidInput);
        require!(source_neuron_id != target_neuron_id, IndrasError::InvalidInput);
        require!(weight <= 100, IndrasError::InvalidInput);
        
        synapse.synapse_id = synapse_id;
        synapse.source_neuron_id = source_neuron_id;
        synapse.target_neuron_id = target_neuron_id;
        synapse.synapse_type = synapse_type;
        synapse.weight = weight;
        synapse.is_active = true;
        synapse.created_at = current_time;
        synapse.last_used_at = Some(current_time);
        synapse.bump = bump;
        
        Ok(())
    }

    /// Обновление веса связи (для обучения сети)
    pub fn update_synapse_weight(
        synapse: &mut SynapseMetadata,
        new_weight: u8,
    ) -> Result<()> {
        synapse.update_weight(new_weight)
    }
}

/// Off-chain функции для работы со связями
///
/// Эти функции должны быть реализованы в off-chain сервисе
/// для фактической передачи данных между узлами.
pub mod offchain {
    // Off-chain функции будут реализованы в отдельном сервисе
    
    /// Передача данных через связь
    /// 
    /// Фактическая передача данных от одного нейрона к другому:
    /// - Применение веса связи
    /// - Обработка типа связи
    /// - Логирование передачи
    pub fn transmit_data(
        _synapse_id: u64,
        _data: &[u8],
        _weight: u8,
    ) -> Vec<u8> {
        // Реализация в off-chain сервисе
        // Применяет вес связи к данным
        vec![]
    }
}
