//! Accounts structures for Core program instructions
//! 
//! All Accounts structures must be in this module for Anchor 0.32.1 compatibility

pub mod ideas;
pub mod proposal;
pub mod voting;
pub mod governance;
// Track B: Phenomenon accounts
pub mod phenomenon;
// Track B: Semantic Distance accounts
pub mod semantic_distance;
// Expert Registry accounts
pub mod expert_registry;

// Re-export all Accounts structures
pub use ideas::*;
pub use proposal::*;
pub use voting::*;
pub use governance::*;
// Track B: Phenomenon accounts
pub use phenomenon::*;
// Track B: Semantic Distance accounts
pub use semantic_distance::*;
// Expert Registry accounts
pub use expert_registry::*;
