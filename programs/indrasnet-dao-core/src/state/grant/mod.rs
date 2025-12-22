//! Grant Module
//!
//! This module manages grants for mesh groups working on ideas.
//! Split into submodules:
//! - types: GrantStatus, GrantCategory, GrantType, VerificationStatus enums
//! - lifecycle: Grant struct and lifecycle methods (new, approve, activate)
//! - disbursement: Disbursement methods
//! - analytics: Grant analytics and metrics
//! - evaluation: Grant evaluation and assessment
//! - tracking: Grant tracking and monitoring

use anchor_lang::prelude::*;

pub mod types;
pub mod lifecycle;
pub mod disbursement;
pub mod analytics;
pub mod evaluation;
pub mod tracking;
pub mod vote;
pub mod semantic;
pub mod report;

// Re-export types
pub use types::*;
pub use vote::{GrantVote, VoterType};
pub use semantic::{VotingLayer, CompetencyLevel, CompetencySource};

// Re-export GrantDisbursementType
pub use types::GrantDisbursementType;
pub use analytics::{
    GrantAnalyticsMetadata, GrantAnalyticsType, GrantAnalyticsStatus,
    onchain::initialize_grant_analytics,
};
pub use evaluation::{
    GrantEvaluationMetadata, GrantEvaluationCriteria, GrantEvaluationStatus,
    onchain::initialize_grant_evaluation,
};
pub use tracking::{
    GrantTrackingMetadata, GrantTrackingMetric, GrantTrackingStatus,
    onchain::initialize_grant_tracking,
};
pub use report::{GrantReport, GrantReportStatus};

/// Grant account structure
#[account]
#[derive(InitSpace)]
pub struct Grant {
    pub id: u64,
    pub idea_id: u64,                    // Associated idea
    pub mesh_group: Pubkey,              // Mesh group recipient
    pub category: GrantCategory,
    pub status: GrantStatus,
    pub base_amount: u64,                // Base amount
    pub reputation_bonus: u64,           // Reputation bonus
    pub total_amount: u64,               // Total amount
    pub disbursed_amount: u64,           // Disbursed amount
    pub grant_type: GrantType,           // Grant type by development stage
    pub disbursement_type: GrantDisbursementType, // Disbursement type (Urgent/Escrow/Standard)
    pub milestone_id: Option<u64>,       // Associated milestone (if any)
    pub verification_status: VerificationStatus, // Milestone verification status
    // NOTE: When grant is approved, author MUST transfer commercialization right to e.V.
    // Author remains copyright owner (does not transfer)
    // e.V. receives right to transfer Idea to commercial enterprise
    // e.V. also stores author's copyright (as custodian)
    pub commercialization_right_transferred: bool, // Commercialization right transferred to e.V.
    pub created_at: i64,
    pub approved_at: Option<i64>,
    pub completed_at: Option<i64>,
    // Grant voting fields
    pub voting_end: i64,              // Voting period end (created_at + 7 days)
    pub total_votes: u64,              // Total vote count
    pub total_yes_weight: u64,         // Total "Yes" vote weight
    pub total_no_weight: u64,          // Total "No" vote weight
    pub total_abstain_weight: u64,     // Total "Abstain" vote weight
    pub quorum_reached: bool,          // Quorum reached
    // Semantic domain and voting layer fields
    #[max_len(100)]
    pub semantic_domain: Option<String>, // Semantic domain ID (optional)
    pub semantic_domain_account: Option<Pubkey>, // Semantic domain account (optional)
    pub semantic_distance: Option<f32>, // Semantic distance to related ideas (optional)
    pub phenomenon_membership: Option<Pubkey>, // Phenomenon this grant belongs to (optional)
    pub grant_level: u8,                // Grant level (1=author, 2=author+expert, 3=all)
    pub voting_layer: VotingLayer,      // Voting layer configuration
    // Grant report fields (for escrow and standard grants)
    pub final_report_submitted: bool,   // Final report submitted
    pub final_report_approved: bool,   // Final report approved
    pub final_report_submitted_at: Option<i64>, // Final report submission time
    pub final_report_approved_at: Option<i64>,  // Final report approval time
    pub escrow_account: Option<Pubkey>, // Escrow account (if disbursement_type == Escrow)
    pub bump: u8,
}
