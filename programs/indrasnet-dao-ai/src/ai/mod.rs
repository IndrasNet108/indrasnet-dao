//! AI modules for the IndrasNet DAO AI program
//! 
//! NOTE: Только ончейн фиксация результатов анализа
//! Оффчейн логика (model_serving, data_*, embeddings, etc.) работает как отдельные сервисы

pub mod ethics_analyzer;
pub mod offchain_analysis;
pub mod idea_analyzer;
pub mod self_healing;
pub mod risk_assessment;
pub mod performance_analytics;
pub mod security_enhancements;
pub mod semantic_risk;

pub use ethics_analyzer::*;
pub use offchain_analysis::*;
pub use idea_analyzer::*;
pub use self_healing::*;
pub use risk_assessment::*;
pub use performance_analytics::*;
pub use security_enhancements::*;
// NOTE: semantic_risk используется в instructions, но не экспортируется здесь
