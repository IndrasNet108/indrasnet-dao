//! Neuron - узел в коллективном AI
//!
//! On-chain: Хранение метаданных нейрона, статуса, связей
//! Off-chain: Фактическая логика работы нейрона, обработка данных

use anchor_lang::prelude::*;
use crate::collective_ai_functions::types::*;

/// Neuron metadata (on-chain)
///
/// Хранит метаданные узла в коллективном AI:
/// - ID и статус
/// - Связи с другими узлами (synapses)
/// - Метаданные для координации
#[account]
#[derive(InitSpace)]
pub struct NeuronMetadata {
    /// Уникальный ID нейрона
    pub neuron_id: u64,
    /// Статус нейрона
    pub status: NeuronStatus,
    /// Владелец/создатель нейрона
    pub owner: Pubkey,
    /// Количество активных связей (synapses)
    pub synapse_count: u32,
    /// Время создания
    pub created_at: i64,
    /// Время последнего обновления
    pub updated_at: i64,
    /// Bump seed для PDA
    pub bump: u8,
}

impl NeuronMetadata {
    /// Проверка, активен ли нейрон
    pub fn is_active(&self) -> bool {
        self.status == NeuronStatus::Active
    }

    /// Обновление статуса
    pub fn update_status(&mut self, new_status: NeuronStatus, current_time: i64) {
        self.status = new_status;
        self.updated_at = current_time;
    }

    /// Увеличение счетчика связей
    pub fn increment_synapse_count(&mut self) {
        self.synapse_count = self.synapse_count.saturating_add(1);
    }

    /// Уменьшение счетчика связей
    pub fn decrement_synapse_count(&mut self) {
        self.synapse_count = self.synapse_count.saturating_sub(1);
    }
}

/// On-chain функции для работы с нейронами
pub mod onchain {
    use super::*;
    use crate::error::IndrasError;

    /// Инициализация метаданных нейрона
    pub fn initialize_neuron_metadata(
        neuron: &mut NeuronMetadata,
        neuron_id: u64,
        owner: Pubkey,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(neuron_id > 0, IndrasError::InvalidInput);
        
        neuron.neuron_id = neuron_id;
        neuron.status = NeuronStatus::Pending;
        neuron.owner = owner;
        neuron.synapse_count = 0;
        neuron.created_at = current_time;
        neuron.updated_at = current_time;
        neuron.bump = bump;
        
        Ok(())
    }

    /// Активация нейрона
    pub fn activate_neuron(
        neuron: &mut NeuronMetadata,
        current_time: i64,
    ) -> Result<()> {
        require!(
            neuron.status == NeuronStatus::Pending,
            IndrasError::InvalidState
        );
        
        neuron.update_status(NeuronStatus::Active, current_time);
        Ok(())
    }

    /// Деактивация нейрона
    pub fn deactivate_neuron(
        neuron: &mut NeuronMetadata,
        current_time: i64,
    ) -> Result<()> {
        require!(
            neuron.is_active(),
            IndrasError::InvalidState
        );
        
        neuron.update_status(NeuronStatus::Inactive, current_time);
        Ok(())
    }
}

/// Off-chain функции для работы с нейронами
///
/// Эти функции должны быть реализованы в off-chain сервисе
/// для фактической координации AI агентов.
pub mod offchain {
    // Off-chain функции будут реализованы в отдельном сервисе
    // Например: offchain_services/collective_ai_coordinator/
    
    /// Координация работы нейрона
    /// 
    /// Фактическая логика работы AI агента:
    /// - Обработка входных данных
    /// - Генерация выходных данных
    /// - Координация с другими нейронами
    pub fn coordinate_neuron(_neuron_id: u64) {
        // Реализация в off-chain сервисе
        // Использует Gemini API или другие AI сервисы
    }

    /// Обработка данных нейроном
    pub fn process_neuron_data(_neuron_id: u64, _data: &[u8]) -> Vec<u8> {
        // Реализация в off-chain сервисе
        vec![]
    }
}
