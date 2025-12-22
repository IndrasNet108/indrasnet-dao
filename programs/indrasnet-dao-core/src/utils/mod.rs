//! Utility functions for the IndrasNet DAO Core program

pub mod account_helpers;
pub mod canonical_hash;
pub mod dbscan_validation;
pub mod ed25519_verify;
pub mod phenomenon_deserialize;
pub mod role_helpers;

// Real runtime tests for utils modules
#[cfg(all(test, feature = "program-test"))]
mod canonical_hash_program_test;
#[cfg(all(test, feature = "program-test"))]
mod dbscan_validation_program_test;
#[cfg(all(test, feature = "program-test"))]
mod ed25519_verify_program_test;
#[cfg(all(test, feature = "program-test"))]
mod phenomenon_deserialize_program_test;

// Re-export commonly used functions
pub use ed25519_verify::verify_ed25519_signature;
pub use canonical_hash::{
    compute_canonical_embedding_hash,
    compute_canonical_distance_bundle_hash,
};
pub use dbscan_validation::{
    validate_dbscan_reachability,
    validate_no_noise_points,
};
pub use phenomenon_deserialize::{
    is_idea_in_phenomenon,
    get_phenomenon_status,
};
pub use role_helpers::assert_role;
