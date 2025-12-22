/*
 * Copyright (c) 2024 Captain Light. All rights reserved.
 * IndrasNet DAO Core - Treasury, Governance, Voting, Grants, Mesh Groups
 * Created by: Captain Light
 * Contact: info@indrasnet.ee
 * GitLab: https://gitlab.com/IndrasNet/indrasnet-dao-v3
 */

#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
// UncheckedAccount is used in instruction_accounts for optional accounts to avoid owner checks

// ===== PROGRAM ID =====
declare_id!("8aVykRb29z7tHpMVXyGAvgxoyRi8AZeK3vSSsR7NS4Mr");

// ===== MODULE DECLARATIONS =====
pub mod error;
pub mod constants;
pub mod types;
pub mod events;
pub mod state;
pub mod utils;
pub mod voting_types;
pub mod voting_functions;
// pub mod defi; // REMOVED FOR MVP: Move to off-chain service
// pub mod legal; // REMOVED FOR MVP: Move to off-chain service
pub mod instructions;

// Test fixtures and helpers (only compiled in test mode)
#[cfg(test)]
pub mod tests;

// ===== RE-EXPORTS FOR CRATE-WIDE USE =====
pub use crate::error::IndrasError;

// ===== TYPES FOR ACCOUNTS STRUCTURES =====
// Import types that will be used in Accounts structs (must be before include!)
// These imports make types available to included Accounts structs
use crate::state::{DaoConfig, Idea, Proposal, ProposalExecution, VoteDelegation, ExecutionStatus};
use crate::state::idea_vote::IdeaVote;
use crate::state::proposal::{ProposalAmendment, ProposalTemplate};
// Types used in instruction_accounts via include!()
use crate::state::Grant;
use crate::state::MeshGroup;
use crate::state::grant::GrantReport;
use crate::state::GovernanceParams;
use crate::state::ai_analysis_record::AIAnalysisRecord;
// NOTE: Phenomenon and AIAnalysis moved to AI program (indrasnet-dao-ai) for modular architecture
// use crate::state::Phenomenon;
// use crate::state::ai_analysis::AIAnalysis;
use crate::state::ai_service_registry::AIServiceRegistry;
use crate::state::rate_limit_tracker::RateLimitTracker;
// NOTE: VerifiedDistance moved to AI program (indrasnet-dao-ai) with create_phenomenon
// #[allow(unused_imports)]
// use crate::state::grant::semantic::VerifiedDistance;
use crate::state::governance::{
    analytics::GovernanceAnalyticsMetadata,
    participation::GovernanceParticipationMetadata,
    voting::GovernanceVotingMetadata,
};
// Security program types for CPI (one-way dependency: Core → Security)
// NOTE: Temporarily disabled - Security in exclude, causes build issues
// use indrasnet_dao_security;


// ===== ACCOUNT TYPES USED IN ACCOUNTS STRUCTS =====
// These account types are defined here (as in v3) since they're simple account types

/// Treasury account structure
#[account]
#[derive(InitSpace)]
pub struct Treasury {
    #[max_len(100)]
    pub name: String,
    pub balance: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

/// Capability account structure for treasury operations
#[account]
#[derive(InitSpace)]
pub struct Capability {
    pub grantee: Pubkey,
    pub granter: Pubkey,
    #[max_len(50)]
    pub capability_type: String,
    pub expires_at: i64,
    pub bump: u8,
}

/// IdeaExecution account structure
#[account]
#[derive(Debug, PartialEq, InitSpace)]
pub struct IdeaExecution {
    pub id: u64,
    pub idea_id: u64,
    pub executor: Pubkey,
    pub executed_at: i64,
    #[max_len(1000)]
    pub execution_data: String,
    pub status: ExecutionStatus,
    pub bump: u8,
}

/// AnchorRecord account structure for recording grant decisions after successful voting
#[account]
#[derive(InitSpace)]
pub struct AnchorRecord {
    pub idea_id: u64,
    #[max_len(500)]
    pub anchor_data: String,
    pub anchored_at: i64,
    pub anchorer: Pubkey,
    pub bump: u8,
}

/// Quorum account structure for managing voting quorum requirements
#[account]
#[derive(InitSpace)]
pub struct Quorum {
    pub id: u64,
    pub required_votes: u64,
    pub quorum_threshold: u64,
    pub update_timestamp: i64,
    pub bump: u8,
}

// ===== ACCOUNTS STRUCTURES (must be in crate root for Anchor 0.32.1 macro) =====
// Include Accounts structures inline for Anchor macro compatibility
// (Anchor 0.32.1 #[program] macro requires Accounts structs in crate root)
include!("instruction_accounts/governance.rs");
include!("instruction_accounts/ideas.rs");
include!("instruction_accounts/proposal.rs");
include!("instruction_accounts/proposal_template.rs");
include!("instruction_accounts/voting.rs");
include!("instruction_accounts/treasury.rs");
include!("instruction_accounts/grants.rs");
include!("instruction_accounts/ai_analysis_registry.rs");
// IDEA VOTING: Enabled for MVP
include!("instruction_accounts/idea_voting.rs");
// REMOVED FOR MVP: Can be deferred
// include!("instruction_accounts/commercial_enterprise.rs");
include!("instruction_accounts/mesh_groups.rs");
include!("instruction_accounts/mesh_groups_governance.rs");
// REMOVED FOR MVP: Can be deferred
// include!("instruction_accounts/role_management.rs");
// NOTE: ai_analysis and phenomenon accounts moved to AI program (indrasnet-dao-ai) for modular architecture
// include!("instruction_accounts/ai_analysis.rs");
// include!("instruction_accounts/phenomenon.rs");
// Track B: Semantic Distance accounts
include!("instruction_accounts/semantic_distance.rs");
// AI Registry accounts
include!("instruction_accounts/ai_registry.rs");
// Expert Registry accounts
include!("instruction_accounts/expert_registry.rs");
// Member Management accounts
include!("instruction_accounts/member_management.rs");
// Grant Reports accounts (included in grants.rs)
// Testing-only accounts
#[cfg(feature = "test-bpf")]
include!("instruction_accounts/testing.rs");

// ===== PROGRAM MODULE =====
// NOTE: For MVP - instructions must be directly in the module for IDL generation to work
// Anchor 0.32.1 #[program] macro cannot process instructions from include!() files
// This is a known limitation - we inline the instructions here
#[program]
pub mod indrasnet_dao_core {
    use super::*;

    // MVP: Instructions must be directly in the module (not via include!()) for IDL generation
    // Anchor 0.32.1 #[program] macro cannot process include!() files for IDL
    // Workaround: Copy instruction function signatures directly here
    
    // ===== GOVERNANCE INSTRUCTIONS =====
    
    /// Initialize DAO
    pub fn initialize_dao(
        ctx: Context<InitializeDao>,
        name: String,
        description: String,
    ) -> Result<()> {
        instructions::initialize_dao_handler(ctx, name, description)
    }

    /// Migrate DaoConfig from v1 to v2 (adds schema_version + dev_mode)
    pub fn migrate_dao_config_v1_to_v2(
        ctx: Context<MigrateDaoConfig>,
    ) -> Result<()> {
        instructions::migrate_dao_config_v1_to_v2_handler(ctx)
    }
    
    /// Manage quorum for voting
    pub fn manage_quorum(
        ctx: Context<ManageQuorum>,
        quorum_id: u64,
        required_votes: u64,
        quorum_threshold: u64,
    ) -> Result<()> {
        instructions::manage_quorum_handler(ctx, quorum_id, required_votes, quorum_threshold)
    }
    
    /// Initialize governance parameters
    pub fn initialize_governance_params(
        ctx: Context<InitializeGovernanceParams>,
        quorum_percentage: u8,
        vote_duration_hours: u64,
        delegate_weight_percentage: u8,
        early_quorum_enabled: bool,
    ) -> Result<()> {
        instructions::initialize_governance_params_handler(ctx, quorum_percentage, vote_duration_hours, delegate_weight_percentage, early_quorum_enabled)
    }
    
    /// Update governance parameters
    pub fn update_governance_params(
        ctx: Context<UpdateGovernanceParams>,
        quorum_percentage: Option<u8>,
        vote_duration_hours: Option<u64>,
        delegate_weight_percentage: Option<u8>,
        early_quorum_enabled: Option<bool>,
    ) -> Result<()> {
        instructions::update_governance_params_handler(
            ctx,
            quorum_percentage,
            vote_duration_hours,
            delegate_weight_percentage,
            early_quorum_enabled,
        )
    }
    
    // ===== GOVERNANCE ANALYTICS INSTRUCTIONS =====
    
    /// Initialize governance analytics metadata
    pub fn initialize_governance_analytics(
        ctx: Context<InitializeGovernanceAnalytics>,
        analytics_id: u64,
        governance_id: u64,
        analytics_type: state::governance::analytics::GovernanceAnalyticsType,
        analytics_config_hash: [u8; 32],
    ) -> Result<()> {
        instructions::initialize_governance_analytics_handler(
            ctx,
            analytics_id,
            governance_id,
            analytics_type,
            analytics_config_hash,
        )
    }
    
    /// Initialize governance participation metadata
    pub fn initialize_governance_participation(
        ctx: Context<InitializeGovernanceParticipation>,
        participation_id: u64,
        member_id: u64,
        participation_type: state::governance::participation::GovernanceParticipationType,
        participation_config_hash: [u8; 32],
    ) -> Result<()> {
        instructions::initialize_governance_participation_handler(
            ctx,
            participation_id,
            member_id,
            participation_type,
            participation_config_hash,
        )
    }
    
    /// Initialize governance voting metadata
    pub fn initialize_governance_voting(
        ctx: Context<InitializeGovernanceVoting>,
        voting_id: u64,
        proposal_id: u64,
        voting_type: state::governance::voting::GovernanceVotingType,
        voting_data_hash: [u8; 32],
    ) -> Result<()> {
        instructions::initialize_governance_voting_handler(
            ctx,
            voting_id,
            proposal_id,
            voting_type,
            voting_data_hash,
        )
    }
    
    // ===== PROPOSAL INSTRUCTIONS =====
    
    /// Create a new proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_id: u64,
        title: String,
        description: String,
        proposal_type: String,
        voting_duration: Option<i64>,
    ) -> Result<()> {
        instructions::create_proposal_handler(ctx, proposal_id, title, description, proposal_type, voting_duration)
    }
    
    /// Activate proposal (move from Draft to Active)
    pub fn activate_proposal(
        ctx: Context<ActivateProposal>,
        proposal_id: u64,
        min_quorum: u64,
        total_members: u64,
    ) -> Result<()> {
        instructions::activate_proposal_handler(ctx, proposal_id, min_quorum, total_members)
    }
    
    /// Pass proposal (move from Active to Passed)
    pub fn pass_proposal(
        ctx: Context<PassProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::pass_proposal_handler(ctx, proposal_id)
    }
    
    /// Reject proposal (move from Active to Rejected)
    pub fn reject_proposal(
        ctx: Context<RejectProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::reject_proposal_handler(ctx, proposal_id)
    }
    
    /// Cancel proposal (move from Draft or Active to Cancelled)
    pub fn cancel_proposal(
        ctx: Context<CancelProposal>,
        proposal_id: u64,
        reason: String,
    ) -> Result<()> {
        instructions::cancel_proposal_handler(ctx, proposal_id, reason)
    }
    
    /// Archive proposal (move from Executed, Rejected, or Cancelled to Archived)
    pub fn archive_proposal(
        ctx: Context<ArchiveProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::archive_proposal_handler(ctx, proposal_id)
    }
    
    /// Set expiration time for proposal
    pub fn set_proposal_expiration(
        ctx: Context<SetProposalExpiration>,
        proposal_id: u64,
        expires_at: Option<i64>,
    ) -> Result<()> {
        instructions::set_proposal_expiration_handler(ctx, proposal_id, expires_at)
    }
    
    /// Check and auto-archive expired proposal
    pub fn check_and_auto_archive_proposal(
        ctx: Context<CheckAndAutoArchiveProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::check_and_auto_archive_proposal_handler(ctx, proposal_id)
    }
    
    /// Automatically transition proposal after voting period ends
    pub fn auto_transition_proposal(
        ctx: Context<AutoTransitionProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::auto_transition_proposal_handler(ctx, proposal_id)
    }
    
    /// Create an amendment to a proposal
    pub fn amend_proposal(
        ctx: Context<AmendProposal>,
        proposal_id: u64,
        amendment_id: u64,
        content: String,
    ) -> Result<()> {
        instructions::amend_proposal_handler(ctx, proposal_id, amendment_id, content)
    }
    
    // ===== PROPOSAL TEMPLATE INSTRUCTIONS =====
    
    /// Create a new proposal template
    ///
    /// Creates a template without fields. Fields can be added later via add_template_field.
    pub fn create_proposal_template(
        ctx: Context<CreateProposalTemplate>,
        template_id: u64,
        name: String,
        description: String,
        proposal_type: String,
    ) -> Result<()> {
        instructions::create_proposal_template_handler(ctx, template_id, name, description, proposal_type)
    }
    
    /// Add a field to a proposal template
    pub fn add_template_field(
        ctx: Context<AddTemplateField>,
        field_name: String,
        field_description: String,
        field_type: state::proposal::TemplateFieldType,
        required: bool,
    ) -> Result<()> {
        instructions::add_template_field_handler(ctx, field_name, field_description, field_type, required)
    }
    
    /// Update proposal template
    ///
    /// Note: Pass empty string to skip updating that field. Fields cannot be updated after creation.
    pub fn update_proposal_template(
        ctx: Context<UpdateProposalTemplate>,
        name: String,
        description: String,
    ) -> Result<()> {
        instructions::update_proposal_template_handler(ctx, name, description)
    }
    
    /// Deactivate proposal template
    pub fn deactivate_proposal_template(
        ctx: Context<ManageProposalTemplate>,
    ) -> Result<()> {
        instructions::deactivate_proposal_template_handler(ctx)
    }
    
    /// Activate proposal template
    pub fn activate_proposal_template(
        ctx: Context<ManageProposalTemplate>,
    ) -> Result<()> {
        instructions::activate_proposal_template_handler(ctx)
    }
    
    /// Create proposal from template
    pub fn create_proposal_from_template(
        ctx: Context<CreateProposalFromTemplate>,
        proposal_id: u64,
        template_id: u64,
        values: Vec<String>,
        voting_duration: Option<i64>,
    ) -> Result<()> {
        instructions::create_proposal_from_template_handler(ctx, proposal_id, template_id, values, voting_duration)
    }
    
    // ===== TREASURY PROPOSAL INSTRUCTIONS =====
    
    /// Create a Treasury proposal
    pub fn create_treasury_proposal(
        ctx: Context<CreateTreasuryProposal>,
        proposal_id: u64,
        title: String,
        description: String,
        treasury_operation: state::proposal::treasury::TreasuryOperationData,
        voting_duration: Option<i64>,
    ) -> Result<()> {
        instructions::create_treasury_proposal_handler(ctx, proposal_id, title, description, treasury_operation, voting_duration)
    }
    
    /// Execute a Treasury proposal
    pub fn execute_treasury_proposal(
        ctx: Context<ExecuteTreasuryProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::execute_treasury_proposal_handler(ctx, proposal_id)
    }
    
    // ===== IDEA TO PROPOSAL CONVERSION (RARE CASE) =====
    
    /// Convert an Idea to a Proposal
    ///
    /// This is a rare case - normally Ideas are handled by Mesh Groups, while Proposals
    /// are for general DAO governance. This instruction allows converting an Idea into
    /// a Proposal for special governance voting scenarios.
    pub fn convert_idea_to_proposal(
        ctx: Context<ConvertIdeaToProposal>,
        idea_id: u64,
        proposal_id: u64,
        title: Option<String>,
        description: Option<String>,
        voting_duration: Option<i64>,
    ) -> Result<()> {
        instructions::convert_idea_to_proposal_handler(ctx, idea_id, proposal_id, title, description, voting_duration)
    }
    
    // ===== PROPOSAL VOTING INSTRUCTIONS =====
    
    /// Cast a vote on a proposal
    pub fn cast_vote(
        ctx: Context<CastVote>,
        proposal_id: u64,
        vote_choice: voting_types::VoteType,
    ) -> Result<()> {
        instructions::cast_vote_handler(ctx, proposal_id, vote_choice)
    }
    
    /// Tally votes for a proposal
    pub fn tally_votes(
        ctx: Context<TallyVotes>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::tally_votes_handler(ctx, proposal_id)
    }
    
    /// Execute a proposal
    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
        proposal_id: u64,
        execution_data: String,
    ) -> Result<()> {
        instructions::execute_proposal_handler(ctx, proposal_id, execution_data)
    }
    
    // ===== PROPOSAL EXECUTION MANAGEMENT INSTRUCTIONS =====
    
    /// Schedule proposal execution
    pub fn schedule_proposal_execution(
        ctx: Context<ScheduleProposalExecutionCtx>,
        execution_id: u64,
        proposal_id: u64,
        executor: Pubkey,
        execution_data: String,
    ) -> Result<()> {
        instructions::schedule_proposal_execution_handler(ctx, execution_id, proposal_id, executor, execution_data)
    }
    
    /// Update proposal execution
    pub fn update_proposal_execution(
        ctx: Context<UpdateProposalExecutionCtx>,
        execution_data: Option<String>,
        status: Option<state::proposal_execution::ExecutionStatus>,
    ) -> Result<()> {
        instructions::update_proposal_execution_handler(ctx, execution_data, status)
    }
    
    /// Cancel proposal execution
    pub fn cancel_proposal_execution(
        ctx: Context<CancelProposalExecutionCtx>,
    ) -> Result<()> {
        instructions::cancel_proposal_execution_handler(ctx)
    }
    
    // ===== VOTE DELEGATION MANAGEMENT INSTRUCTIONS =====
    
    /// Create vote delegation
    pub fn create_vote_delegation(
        ctx: Context<CreateVoteDelegationCtx>,
        delegate: Pubkey,
        weight: u64,
        expires_at: Option<i64>,
    ) -> Result<()> {
        instructions::create_vote_delegation_handler(ctx, delegate, weight, expires_at)
    }
    
    /// Update vote delegation weight
    pub fn update_vote_delegation_weight(
        ctx: Context<UpdateVoteDelegationWeightCtx>,
        new_weight: u64,
    ) -> Result<()> {
        instructions::update_vote_delegation_weight_handler(ctx, new_weight)
    }
    
    /// Deactivate vote delegation
    pub fn deactivate_vote_delegation(
        ctx: Context<DeactivateVoteDelegationCtx>,
    ) -> Result<()> {
        instructions::deactivate_vote_delegation_handler(ctx)
    }
    
    /// Reactivate vote delegation
    pub fn reactivate_vote_delegation(
        ctx: Context<ReactivateVoteDelegationCtx>,
    ) -> Result<()> {
        instructions::reactivate_vote_delegation_handler(ctx)
    }
    
    /// Set expiration time for vote delegation
    pub fn set_vote_delegation_expiration(
        ctx: Context<SetVoteDelegationExpirationCtx>,
        expires_at: Option<i64>,
    ) -> Result<()> {
        instructions::set_vote_delegation_expiration_handler(ctx, expires_at)
    }
    
    /// Check and auto-deactivate expired vote delegation
    pub fn check_and_auto_deactivate_delegation(
        ctx: Context<CheckAndAutoDeactivateDelegationCtx>,
    ) -> Result<()> {
        instructions::check_and_auto_deactivate_delegation_handler(ctx)
    }
    
    // ===== TREASURY INSTRUCTIONS =====
    
    /// Initialize treasury
    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>,
        treasury_name: String,
    ) -> Result<()> {
        instructions::initialize_treasury_handler(ctx, treasury_name)
    }
    
    /// Deposit to treasury
    pub fn deposit_to_treasury(
        ctx: Context<DepositToTreasury>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_to_treasury_handler(ctx, amount)
    }
    
    /// Withdraw from treasury with capability
    pub fn withdraw_treasury_with_capability(
        ctx: Context<WithdrawTreasuryWithCapability>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw_treasury_with_capability_handler(ctx, amount)
    }
    
    /// Grant capability for treasury operations
    pub fn grant_capability(
        ctx: Context<GrantCapability>,
        grantee: Pubkey,
        capability_type: String,
        expires_at: i64,
    ) -> Result<()> {
        instructions::grant_capability_handler(ctx, grantee, capability_type, expires_at)
    }
    
    /// Revoke capability
    pub fn revoke_capability(
        ctx: Context<RevokeCapability>,
    ) -> Result<()> {
        instructions::revoke_capability_handler(ctx)
    }
    
    // ===== IDEA INSTRUCTIONS =====
    
    /// Create a new idea
    pub fn create_idea(
        ctx: Context<CreateIdea>,
        idea_id: u64,
        title: String,
        description: String,
    ) -> Result<()> {
        instructions::create_idea_handler(
            ctx,
            idea_id,
            title,
            description,
            None, // embedding_hash
            None, // embedding_signature
            None, // embedding_provider
            None, // embedding_model
            None, // embedding_model_version
            None, // embedding_provider_pubkey
        )
    }
    
    /// Complete idea (move from InProgress to Completed)
    pub fn complete_idea(
        ctx: Context<CompleteIdea>,
        idea_id: u64,
        completion_report: String,
    ) -> Result<()> {
        instructions::complete_idea_handler(ctx, idea_id, completion_report)
    }
    
    /// Archive idea (move from Completed/Executed/Rejected to Archived)
    pub fn archive_idea(
        ctx: Context<ArchiveIdea>,
        idea_id: u64,
        reason: String,
    ) -> Result<()> {
        instructions::archive_idea_handler(ctx, idea_id, reason)
    }
    
    /// Close Idea account and return rent
    ///
    /// Allows closing an Idea account and returning the rent exemption.
    /// Only the idea author or DAO authority can close the account.
    pub fn close_idea(
        ctx: Context<CloseIdea>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::close_idea_handler(ctx, idea_id)
    }
    
    /// Resubmit idea (move from Rejected to Resubmitted)
    pub fn resubmit_idea(
        ctx: Context<ResubmitIdea>,
        idea_id: u64,
        updated_title: Option<String>,
        updated_description: Option<String>,
    ) -> Result<()> {
        instructions::resubmit_idea_handler(ctx, idea_id, updated_title, updated_description)
    }
    
    /// Execute idea (move from Completed to Executed)
    pub fn execute_idea(
        ctx: Context<ExecuteIdea>,
        idea_id: u64,
        execution_data: String,
    ) -> Result<()> {
        instructions::execute_idea_handler(ctx, idea_id, execution_data)
    }
    
    /// Transfer rights to e.V. without grant (voluntary transfer)
    #[allow(clippy::too_many_arguments)]
    pub fn transfer_rights_to_ev(
        ctx: Context<TransferRightsToEv>,
        can_modify: bool,
        can_distribute: bool,
        can_reproduce: bool,
        can_develop: bool,
        can_sublicense: bool,
        can_gift: bool,
        can_bequeath: bool,
    ) -> Result<()> {
        instructions::transfer_rights_to_ev_handler(
            ctx,
            can_modify,
            can_distribute,
            can_reproduce,
            can_develop,
            can_sublicense,
            can_gift,
            can_bequeath,
        )
    }
    
    // ===== GRANT INSTRUCTIONS =====
    
    /// Create grant request
    #[allow(clippy::too_many_arguments)]
    pub fn create_grant(
        ctx: Context<CreateGrant>,
        grant_id: u64,
        idea_id: u64,
        category: state::grant::GrantCategory,
        grant_type: state::grant::GrantType,
        disbursement_type: state::grant::GrantDisbursementType, // Disbursement type (Urgent/Escrow/Standard)
        base_amount: u64,
        reputation_bonus: u64,
        milestone_id: Option<u64>,
    ) -> Result<()> {
        instructions::create_grant_handler(
            ctx,
            grant_id,
            idea_id,
            category,
            grant_type,
            disbursement_type,
            base_amount,
            reputation_bonus,
            milestone_id,
            None, // semantic_domain_account
            None, // semantic_distance
            None, // phenomenon_membership
        )
    }

    /// Register AI analysis created by the AI program (CPI-only).
    pub fn register_ai_analysis(
        ctx: Context<RegisterAiAnalysis>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::register_ai_analysis_handler(ctx, idea_id)
    }
    
    /// Approve grant
    pub fn approve_grant(
        ctx: Context<ApproveGrant>,
    ) -> Result<()> {
        instructions::approve_grant_handler(ctx)
    }
    
    /// Activate grant
    pub fn activate_grant(
        ctx: Context<ActivateGrant>,
    ) -> Result<()> {
        instructions::activate_grant_handler(ctx)
    }
    
    /// Complete grant (move from Active to Completed)
    pub fn complete_grant(
        ctx: Context<CompleteGrant>,
    ) -> Result<()> {
        instructions::complete_grant_handler(ctx)
    }
    
    /// Disburse grant funds to recipient
    pub fn disburse_grant(
        ctx: Context<DisburseGrant>,
        amount: u64,
    ) -> Result<()> {
        instructions::disburse_grant_handler(ctx, amount)
    }
    
    /// Cast vote on grant
    pub fn cast_grant_vote(
        ctx: Context<CastGrantVote>,
        grant_id: u64,
        vote_choice: voting_types::VoteType,
        voter_type: crate::state::grant::VoterType,
        competency_multiplier: Option<u64>,
    ) -> Result<()> {
        instructions::grants_voting::cast_grant_vote_handler(
            ctx, grant_id, vote_choice, voter_type, competency_multiplier
        )
    }
    
    /// Tally votes for grant
    pub fn tally_grant_votes(
        ctx: Context<TallyGrantVotes>,
        grant_id: u64,
    ) -> Result<()> {
        instructions::grants_voting::tally_grant_votes_handler(ctx, grant_id)
    }
    
    // ===== GRANT REPORT INSTRUCTIONS =====
    
    /// Submit grant report
    ///
    /// Allows mesh group to submit a final report for a grant.
    /// Required for Escrow and Standard grants before closing accounts.
    pub fn submit_grant_report(
        ctx: Context<SubmitGrantReport>,
        grant_id: u64,
        report_content: String,
        attachments_hash: Option<String>,
    ) -> Result<()> {
        instructions::submit_grant_report_handler(ctx, grant_id, report_content, attachments_hash)
    }
    
    /// Approve grant report
    ///
    /// Allows DAO authority to approve a submitted grant report.
    /// After approval, accounts can be closed (if all grants have approved reports).
    pub fn approve_grant_report(
        ctx: Context<ApproveGrantReport>,
        grant_id: u64,
    ) -> Result<()> {
        instructions::approve_grant_report_handler(ctx, grant_id)
    }
    
    /// Reject grant report
    ///
    /// Allows DAO authority to reject a submitted grant report.
    /// Mesh group can resubmit after fixing issues.
    pub fn reject_grant_report(
        ctx: Context<RejectGrantReport>,
        grant_id: u64,
        reason: String,
    ) -> Result<()> {
        instructions::reject_grant_report_handler(ctx, grant_id, reason)
    }
    
    // ===== IDEA VOTING INSTRUCTIONS =====
    
    /// Cast a vote on an idea
    pub fn cast_idea_vote(
        ctx: Context<CastIdeaVote>,
        idea_id: u64,
        vote_type: voting_types::VoteType,
        weight: u64,
    ) -> Result<()> {
        instructions::cast_idea_vote_handler(ctx, idea_id, vote_type, weight)
    }
    
    /// Tally votes for an idea
    pub fn tally_idea_votes(
        ctx: Context<TallyIdeaVotes>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::tally_idea_votes_handler(ctx, idea_id)
    }
    
    /// Start voting on an idea
    pub fn start_idea_voting(
        ctx: Context<StartIdeaVoting>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::start_idea_voting_handler(ctx, idea_id)
    }
    
    // ===== MESH GROUP INSTRUCTIONS =====
    
    /// Create mesh group
    pub fn create_mesh_group(
        ctx: Context<CreateMeshGroup>,
        mesh_group_id: u64,
        name: String,
        description: String,
        group_type: crate::state::mesh_group::GroupType,
    ) -> Result<()> {
        instructions::create_mesh_group_handler(
            ctx,
            mesh_group_id,
            name,
            description,
            group_type,
            None, // embedding_hash
            None, // embedding_signature
            None, // embedding_provider
            None, // embedding_model
            None, // embedding_model_version
            None, // embedding_provider_pubkey
        )
    }
    
    /// Join mesh group
    pub fn join_mesh_group(
        ctx: Context<JoinMeshGroup>,
        role: crate::state::mesh_group::GroupRole,
    ) -> Result<()> {
        instructions::join_mesh_group_handler(ctx, role)
    }
    
    /// Remove member from mesh group
    pub fn remove_mesh_group_member(
        ctx: Context<RemoveMeshGroupMember>,
    ) -> Result<()> {
        instructions::remove_mesh_group_member_handler(ctx)
    }
    
    /// Start mesh group (Forming -> Active)
    pub fn start_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::start_mesh_group_handler(ctx)
    }
    
    /// Pause mesh group (Active -> Paused)
    pub fn pause_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::pause_mesh_group_handler(ctx)
    }
    
    /// Resume mesh group (Paused -> Active)
    pub fn resume_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::resume_mesh_group_handler(ctx)
    }
    
    /// Complete mesh group (Active -> Completed)
    pub fn complete_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::complete_mesh_group_handler(ctx)
    }
    
    /// Close mesh group
    pub fn close_mesh_group(
        ctx: Context<CloseMeshGroup>,
    ) -> Result<()> {
        instructions::close_mesh_group_handler(ctx)
    }
    
    /// Close Mesh Group account and return rent
    ///
    /// Allows closing a Mesh Group account and returning the rent exemption.
    /// Only the group leader, creator, or DAO authority can close the account.
    pub fn close_mesh_group_account(
        ctx: Context<CloseMeshGroupAccount>,
        mesh_group_id: u64,
    ) -> Result<()> {
        instructions::close_mesh_group_account_handler(ctx, mesh_group_id)
    }
    
    /// Add idea to mesh group
    pub fn add_idea_to_mesh_group(
        ctx: Context<LinkIdeaToMeshGroup>,
        idea_id: u64,
    ) -> Result<()> {
        instructions::add_idea_to_mesh_group_handler(ctx, idea_id)
    }
    
    /// Anchor idea in blockchain within mesh group
    pub fn anchor_idea_in_mesh_group(
        ctx: Context<AnchorIdeaInMeshGroup>,
        idea_id: u64,
        content_hash: [u8; 32],
    ) -> Result<()> {
        instructions::anchor_idea_in_mesh_group_handler(ctx, idea_id, content_hash)
    }
    
    /// Add contribution to mesh group
    pub fn add_contribution(
        ctx: Context<AddContribution>,
    ) -> Result<()> {
        instructions::add_contribution_handler(ctx)
    }
    
    /// Add member to mesh group (governance)
    pub fn add_member_to_mesh_group(
        ctx: Context<AddMemberToMeshGroup>,
        member_pubkey: Pubkey,
        role: crate::state::mesh_group::GroupRole,
    ) -> Result<()> {
        instructions::mesh_groups_governance::add_member_to_mesh_group_handler(ctx, member_pubkey, role)
    }
    
    /// Remove member from mesh group (governance)
    pub fn remove_member_from_mesh_group(
        ctx: Context<RemoveMemberFromMeshGroup>,
        member_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::mesh_groups_governance::remove_member_from_mesh_group_handler(ctx, member_pubkey)
    }
    
    /// Disband mesh group
    pub fn disband_mesh_group(
        ctx: Context<ManageMeshGroup>,
    ) -> Result<()> {
        instructions::disband_mesh_group_handler(ctx)
    }
    
    /// Update mesh group development stage
    pub fn update_mesh_group_stage(
        ctx: Context<ManageMeshGroup>,
        new_stage: crate::state::mesh_group::DevelopmentStage,
    ) -> Result<()> {
        instructions::update_mesh_group_stage_handler(ctx, new_stage)
    }
    
    /// Update idea embedding (Track B)
    #[allow(clippy::too_many_arguments)]
    pub fn update_idea_embedding(
        ctx: Context<UpdateIdeaEmbedding>,
        idea_id: u64,
        embedding_hash: [u8; 32],
        embedding_signature: [u8; 64],
        embedding_provider: String,
        embedding_model: Option<String>,
        embedding_model_version: Option<String>,
        embedding_provider_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::update_idea_embedding_handler(
            ctx,
            idea_id,
            embedding_hash,
            embedding_signature,
            embedding_provider,
            embedding_model,
            embedding_model_version,
            embedding_provider_pubkey,
        )
    }
    
    /// Update mesh group embedding (Track B)
    #[allow(clippy::too_many_arguments)]
    pub fn update_mesh_group_embedding(
        ctx: Context<UpdateMeshGroupEmbedding>,
        mesh_group_id: u64,
        embedding_hash: [u8; 32],
        embedding_signature: [u8; 64],
        embedding_provider: String,
        embedding_model: Option<String>,
        embedding_model_version: Option<String>,
        embedding_provider_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::update_mesh_group_embedding_handler(
            ctx,
            mesh_group_id,
            embedding_hash,
            embedding_signature,
            embedding_provider,
            embedding_model,
            embedding_model_version,
            embedding_provider_pubkey,
        )
    }
    
    /// Verify semantic distance bundle (Track B)
    pub fn verify_semantic_distance(
        ctx: Context<VerifySemanticDistance>,
        bundle: state::grant::semantic::SemanticDistanceBundle,
    ) -> Result<()> {
        instructions::verify_semantic_distance_handler(ctx, bundle)
    }
    
    // ===== PHENOMENON INSTRUCTIONS (Track B) =====
    
    /// Create a new phenomenon
    #[allow(clippy::too_many_arguments)]
    // NOTE: create_phenomenon moved to AI program (indrasnet-dao-ai) for modular architecture
    
    // ===== EXPERT REGISTRY INSTRUCTIONS =====
    
    /// Initialize expert registry
    pub fn initialize_expert_registry(
        ctx: Context<InitializeExpertRegistry>,
    ) -> Result<()> {
        instructions::initialize_expert_registry_handler(ctx)
    }
    
    /// Add expert to registry for a semantic domain
    pub fn add_expert(
        ctx: Context<AddExpert>,
        domain_id: String,
        competency_level: crate::state::grant::semantic::CompetencyLevel,
        confidence: u8,
        source: crate::state::grant::semantic::CompetencySource,
    ) -> Result<()> {
        instructions::add_expert_handler(ctx, domain_id, competency_level, confidence, source)
    }
    
    /// Remove expert from registry
    pub fn remove_expert(
        ctx: Context<RemoveExpert>,
        domain_id: String,
    ) -> Result<()> {
        instructions::remove_expert_handler(ctx, domain_id)
    }
    
    /// Update expert entry
    pub fn update_expert(
        ctx: Context<UpdateExpert>,
        new_confidence: Option<u8>,
        new_competency_level: Option<crate::state::grant::semantic::CompetencyLevel>,
    ) -> Result<()> {
        instructions::update_expert_handler(ctx, new_confidence, new_competency_level)
    }
    
    // NOTE: analyze_idea moved to AI program (indrasnet-dao-ai) for modular architecture
    
    /// Update idea status from analysis
    // NOTE: update_idea_status_from_analysis moved to AI program (indrasnet-dao-ai) with analyze_idea
    
    // ===== AI REGISTRY INSTRUCTIONS =====
    
    /// Initialize AI Service Registry
    pub fn initialize_ai_service_registry(
        ctx: Context<InitializeAiServiceRegistry>,
    ) -> Result<()> {
        instructions::initialize_ai_service_registry_handler(ctx)
    }
    
    /// Add AI service to registry
    pub fn add_ai_service(
        ctx: Context<AddAiService>,
        service_pubkey: Pubkey,
        model_ids: Vec<String>,
    ) -> Result<()> {
        instructions::add_ai_service_handler(ctx, service_pubkey, model_ids)
    }

    // ===== MEMBER MANAGEMENT INSTRUCTIONS =====
    
    /// Leave DAO and close member account
    ///
    /// Allows a member to leave the DAO and close their member account.
    /// The rent exemption will be returned to the destination account.
    ///
    /// # Security
    /// - Member must be active
    /// - Member pubkey must match the signer
    /// - Only the member can close their own account
    pub fn leave_dao(
        ctx: Context<LeaveDao>,
    ) -> Result<()> {
        instructions::leave_dao_handler(ctx)
    }

    // ===== TESTING-ONLY INSTRUCTIONS =====

    /// Update proposal status for testing
    #[cfg(feature = "test-bpf")]
    pub fn testing_update_proposal_status(
        ctx: Context<TestingUpdateProposalStatus>,
        new_status: state::proposal::ProposalStatus,
    ) -> Result<()> {
        instructions::testing::testing_update_proposal_status_handler(ctx, new_status)
    }
}
