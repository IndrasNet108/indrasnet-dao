//! Mesh Group module
//! 
//! Module for managing mesh groups in DAO.
//!
//! NOTE: NO "Sandbox" - only mesh groups exist
//! Mesh group can have 1-7 members (if more needed, create additional mesh group up to 7 members and so on)
//!
//! Includes analytics, collaboration, and governance functionality

pub mod types;
#[allow(clippy::module_inception)]
pub mod mesh_group;
pub mod analytics;
pub mod collaboration;
pub mod governance;
pub mod protocol;
pub mod permissions;
pub mod member_history;

// Re-export commonly used types
pub use types::{
    GroupType, GroupStatus, GroupRole, DevelopmentStage,
    GroupMember, Milestone, MilestoneStatus, MeshGroupParams
};
pub use mesh_group::MeshGroup;
pub use analytics::{
    MeshGroupAnalyticsMetadata, MeshGroupAnalyticsType, MeshGroupAnalyticsStatus,
    onchain::initialize_mesh_group_analytics,
};
pub use collaboration::{
    MeshGroupCollaborationMetadata, MeshGroupCollaborationType, MeshGroupCollaborationStatus,
    onchain::initialize_mesh_group_collaboration,
};
pub use governance::{
    MeshGroupGovernanceMetadata, MeshGroupGovernanceModel, MeshGroupGovernanceStatus,
    onchain::initialize_mesh_group_governance,
};
pub use protocol::{OperatingProtocol, MeetingFrequency};
pub use permissions::MeshGroupPermission;
pub use member_history::{GroupMemberHistory, MemberHistoryEntry, MemberLeaveReason};
