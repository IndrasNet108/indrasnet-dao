//! Collective AI Functions for IndrasNet DAO
//!
//! Реализует концепцию "Сети Индры" - коллективный AI, где каждый узел (neuron)
//! отражает и усиливает всю систему.
//!
//! Архитектура:
//! - On-chain: Хранение метаданных, состояний, базовые проверки
//! - Off-chain: Фактическая координация AI агентов, консенсус, сложные вычисления

pub mod types;
pub mod consciousness;
pub mod memory;
pub mod neuron;
pub mod synapse;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use types::{
    NeuronStatus, SynapseType, MemoryType, ConsciousnessLevel, ConsensusStatus
};
pub use consciousness::{
    ConsciousnessMetadata,
    onchain as consciousness_onchain,
};
pub use memory::{
    MemoryMetadata,
    onchain as memory_onchain,
};
pub use neuron::{
    NeuronMetadata,
    onchain as neuron_onchain,
};
pub use synapse::{
    SynapseMetadata,
    onchain as synapse_onchain,
};
