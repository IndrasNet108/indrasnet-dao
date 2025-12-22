//! Idea instruction handlers
//!
//! Handlers for idea operations: create, complete, archive, resubmit, execute
//!
//! NOTE: This module implements idea functionality for MVP v1.0.0.
//! Following the proper migration process: Read → Understand → Analyze → Implement

mod helpers;
mod create;
mod lifecycle;
mod transfer;
mod close;
mod embedding;

#[cfg(test)]
mod tests;

// Re-export all handlers
pub use create::*;
pub use lifecycle::*;
pub use transfer::*;
pub use close::*;
pub use embedding::*;
