//! Governance state modules
//!
//! Governance management for the DAO:
//! - On-chain: Metadata, policies, committees, security board
//! - Off-chain: Advanced analytics, optimization, recommendations
//!
//! Includes: analytics, voting, participation

pub mod security_board;
pub mod quorum;
pub mod security_policies;
pub mod security_excellence;
pub mod proposal_lifecycle;
pub mod security_committees;
pub mod analytics;
pub mod voting;
pub mod participation;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use security_board::{
    SecurityBoardMemberMetadata, SecurityBoardDecisionMetadata,
    SecurityBoardMemberRole, SecurityBoardDecisionStatus,
    onchain as security_board_onchain,
};
pub use quorum::{
    QuorumMetadata, QuorumCalculationMethod,
    onchain as quorum_onchain,
};
pub use security_policies::{
    SecurityPolicyMetadata, SecurityPolicyStatus,
    onchain as security_policies_onchain,
};
pub use security_excellence::{
    SecurityExcellenceMetadata,
    onchain as security_excellence_onchain,
};
pub use proposal_lifecycle::{
    ProposalLifecycleMetadata, ProposalLifecycleStage,
    onchain as proposal_lifecycle_onchain,
};
pub use security_committees::{
    SecurityCommitteeMetadata, CommitteeMemberRole,
    onchain as security_committees_onchain,
};
pub use analytics::{
    GovernanceAnalyticsMetadata, GovernanceAnalyticsType, GovernanceAnalyticsStatus,
    onchain::initialize_governance_analytics,
};
pub use voting::{
    GovernanceVotingMetadata, GovernanceVotingType, GovernanceVotingStatus,
    onchain::initialize_governance_voting,
};
pub use participation::{
    GovernanceParticipationMetadata, GovernanceParticipationType, GovernanceParticipationStatus,
    onchain::initialize_governance_participation,
};
