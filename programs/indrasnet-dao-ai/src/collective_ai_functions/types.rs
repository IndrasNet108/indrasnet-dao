//! Types for Collective AI Functions
//!
//! Определяет типы данных для коллективного AI:
//! - Neuron: узел сети (AI агент)
//! - Synapse: связь между узлами
//! - Memory: память системы
//! - Consciousness: состояние сознания системы

use anchor_lang::prelude::*;

/// Neuron status - статус узла в коллективном AI
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum NeuronStatus {
    /// Активен и готов к работе
    Active,
    /// Ожидает активации
    Pending,
    /// Неактивен
    Inactive,
    /// Ошибка в работе
    Error,
}

/// Synapse type - тип связи между узлами
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum SynapseType {
    /// Прямая связь (один к одному)
    Direct,
    /// Множественная связь (один ко многим)
    Broadcast,
    /// Обратная связь
    Feedback,
}

/// Memory type - тип памяти
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MemoryType {
    /// Краткосрочная память (рабочая память)
    ShortTerm,
    /// Долгосрочная память (архив)
    LongTerm,
    /// Эпизодическая память (события)
    Episodic,
}

/// Consciousness level - уровень сознания системы
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ConsciousnessLevel {
    /// Низкий уровень - базовые реакции
    Low,
    /// Средний уровень - адаптация
    Medium,
    /// Высокий уровень - самосознание
    High,
}

/// Consensus status - статус консенсуса
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ConsensusStatus {
    /// Консенсус достигнут
    Reached,
    /// В процессе достижения консенсуса
    InProgress,
    /// Консенсус не достигнут
    Failed,
}
