//! Test modules and fixtures
//!
//! This module provides test fixtures and helpers for testing instructions
//! with solana-program-test.

pub mod fixtures;
#[cfg(all(test, feature = "program-test"))]
pub mod pda_seed_template;
