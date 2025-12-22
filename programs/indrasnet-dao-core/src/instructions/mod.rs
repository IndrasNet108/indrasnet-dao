//! Instruction handlers module
//!
//! This module contains all instruction handlers organized by category.
//! Handlers are split into separate modules for better organization and maintainability.

pub mod governance;
pub mod governance_analytics;
pub mod ideas;
pub mod idea_proposal;
pub mod proposal;
pub mod proposal_template;
pub mod treasury_proposal;
pub mod voting;
pub mod treasury;
pub mod grants;
pub mod grants_voting;
// IDEA VOTING: Enabled for MVP
pub mod idea_voting;
// REMOVED FOR MVP: Can be deferred
// pub mod commercial_enterprise;
pub mod mesh_groups;
pub mod mesh_groups_governance;
// REMOVED FOR MVP: Can be deferred
// pub mod role_management;
// NOTE: ai_analysis and phenomenon moved to AI program (indrasnet-dao-ai) for modular architecture
// pub mod ai_analysis;
// pub mod phenomenon;
pub mod ai_analysis_registry;
// Track B: Semantic Distance module
pub mod semantic_distance;

// Testing-only module
#[cfg(feature = "test-bpf")]
pub mod testing;

// AI Registry Management module
pub mod ai_registry_management;

// Expert Registry module
pub mod expert_registry;

// Member Management module
pub mod member_management;

// Grant Reports module
pub mod grant_reports;

// Re-export all handlers for easy access
pub use governance::*;
pub use governance_analytics::*;
pub use ideas::*;
pub use idea_proposal::*;
pub use proposal::*;
pub use proposal_template::*;
pub use treasury_proposal::*;
pub use voting::*;
pub use treasury::*;
pub use grants::*;
// REMOVED FOR MVP: Can be deferred
// pub use grant_voting::*;
// IDEA VOTING: Enabled for MVP
pub use idea_voting::*;
// REMOVED FOR MVP: Can be deferred
// pub use commercial_enterprise::*;
pub use mesh_groups::*;
// REMOVED FOR MVP: Can be deferred
// pub use role_management::*;
// NOTE: ai_analysis and phenomenon moved to AI program (indrasnet-dao-ai) for modular architecture
// pub use ai_analysis::*;
// pub use phenomenon::*;
pub use ai_analysis_registry::*;
// Track B: Semantic Distance handlers
pub use semantic_distance::*;

#[cfg(feature = "test-bpf")]
pub use testing::*;

pub use ai_registry_management::*;

// Expert Registry handlers
pub use expert_registry::*;

// Member Management handlers
pub use member_management::*;

// Grant Reports handlers
pub use grant_reports::*;
