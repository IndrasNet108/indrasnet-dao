//! Handlers for the IndrasNet DAO Security program
//!
//! Handlers process instructions for Security modules
//!
//! Architecture:
//! - On-chain: Validation, state management, basic checks, фиксация результатов
//! - Off-chain: Actual processing, monitoring, analysis (в отдельном сервисе services/offchain-security-service/)

pub mod security_analytics;
pub mod compliance_checking;
pub mod run_nis2_compliance_check;
pub mod security_check;
pub mod security_roles;

// Re-exports
pub use security_analytics::*;
pub use compliance_checking::*;
pub use run_nis2_compliance_check::*;
pub use security_check::*;
pub use security_roles::*;
