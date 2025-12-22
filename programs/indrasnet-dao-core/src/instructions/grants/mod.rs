//! Grant instruction handlers
//!
//! Handlers for grant operations: create, approve, activate, complete, disburse
//!
//! NOTE: This module implements grant functionality for MVP v1.0.0.
//!
//! IMPORTANT: MeshGroup state type is now migrated and used in handlers.
//! Phenomenon is NOT required for grant creation (created AFTER grant for analytics).

mod create;
mod approve;
mod lifecycle;
mod disburse;

// Re-export all handlers
pub use create::*;
pub use approve::*;
pub use lifecycle::*;
pub use disburse::*;
