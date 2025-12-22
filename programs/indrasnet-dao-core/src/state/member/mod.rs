//! Member module
//! 
//! Provides member management functionality:
//! - types: Member and MemberRegistry structures
//! - lifecycle: Member lifecycle methods (new, leave, suspend, activate, ban)
//! - actions: Member action methods (add_contribution, cast_vote, create_idea, create_proposal)
//! - permissions: Member permission check methods
//! - registry: MemberRegistry management methods
//! - analytics: Member analytics and metrics
//! - contribution: Member contribution tracking
//! - reputation: Member reputation system

pub mod types;
pub mod lifecycle;
pub mod actions;
pub mod permissions;
pub mod role;
pub mod registry;
pub mod analytics;
pub mod contribution;
pub mod reputation;

// Real runtime tests for member modules
#[cfg(all(test, feature = "program-test"))]
mod actions_program_test;
#[cfg(all(test, feature = "program-test"))]
mod analytics_program_test;
#[cfg(all(test, feature = "program-test"))]
mod contribution_program_test;
#[cfg(all(test, feature = "program-test"))]
mod reputation_program_test;
#[cfg(all(test, feature = "program-test"))]
mod registry_program_test;

// Re-export types
pub use types::{Member, MemberRegistry};
// Re-export MemberRole struct for use in other programs
pub use role::MemberRole;
pub use role::role_permissions;
pub use analytics::{
    MemberAnalyticsMetadata, MemberAnalyticsType, MemberAnalyticsStatus,
    onchain::initialize_member_analytics,
};
pub use contribution::{
    MemberContributionMetadata, MemberContributionType, MemberContributionStatus,
    onchain::initialize_member_contribution,
};
pub use reputation::{
    MemberReputationMetadata, MemberReputationFactor, MemberReputationStatus,
    onchain::initialize_member_reputation,
};
