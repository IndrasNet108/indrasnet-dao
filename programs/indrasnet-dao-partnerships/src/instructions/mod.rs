//! Instructions for the IndrasNet DAO Partnerships program

pub mod partnership;
pub mod revenue_sharing;
pub mod metrics;
pub mod role_registry;

// Re-export handlers
pub use partnership::*;
pub use revenue_sharing::*;
pub use metrics::*;
pub use role_registry::*;
