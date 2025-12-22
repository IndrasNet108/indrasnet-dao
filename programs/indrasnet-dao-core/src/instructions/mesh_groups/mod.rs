//! Mesh Group instruction handlers
//!
//! Handlers for mesh group operations: create, join, remove, lifecycle management
//!
//! NOTE: This module implements mesh group functionality for MVP v1.0.0.
//!
//! IMPORTANT: All references to "Sandbox" have been removed.
//! Mesh groups can be 1-7 members (if more needed, create additional mesh group).

mod helpers;
mod create;
mod members;
mod lifecycle;
mod ideas;
mod embedding;

#[cfg(test)]
mod tests;

// Re-export all handlers
pub use create::*;
pub use members::*;
pub use lifecycle::*;
pub use ideas::*;
pub use embedding::*;
