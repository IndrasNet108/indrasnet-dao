//! Central type definitions for IndrasNet DAO Core
//! 
//! This module consolidates all shared type definitions to avoid duplication
//! and ensure consistency across the codebase.

use anchor_lang::prelude::*;

/// Risk levels for various assessments
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Space for RiskLevel {
    const INIT_SPACE: usize = 1;
}

/// Wallet roles in the DAO
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum WalletRole {
    Creator,      // Creator of the DAO - highest authority
    Admin,        // Administrator - can manage most operations
    Treasurer,    // Treasury manager - can manage treasury operations
    Accountant,   // Accountant - can view and manage financial records
    Member,       // Regular member - basic participation rights
    Delegate,     // Delegated signer - can sign specific transactions
    Observer,     // Observer - read-only access
}

impl Space for WalletRole {
    const INIT_SPACE: usize = 1;
}

/// Wallet role assignment
#[account]
#[derive(InitSpace)]
pub struct WalletRoleAssignment {
    pub wallet: Pubkey,
    pub role: WalletRole,
    pub assigned_by: Pubkey,
    pub assigned_at: i64,
    pub is_active: bool,
    pub permissions: WalletPermissions,
    pub bump: u8,
}

/// Permissions for wallet roles
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub struct WalletPermissions {
    pub can_create_proposals: bool,
    pub can_vote: bool,
    pub can_manage_treasury: bool,
    pub can_manage_members: bool,
    pub can_manage_roles: bool,
    pub can_delegate_signing: bool,
    pub can_approve_transactions: bool,
    pub can_view_analytics: bool,
    pub can_manage_ai: bool,
    pub can_manage_security: bool,
}

impl Space for WalletPermissions {
    const INIT_SPACE: usize = 10; // 10 bool fields
}

/// Member status in the DAO
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum MemberStatus {
    Active,
    Suspended,
    Banned,
    Inactive,
}

impl Space for MemberStatus {
    const INIT_SPACE: usize = 1;
}

/// Member roles in the DAO
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum MemberRole {
    Founder,
    CoreMember,
    Contributor,
    Observer,
}

impl Space for MemberRole {
    const INIT_SPACE: usize = 1;
}

/// Reputation levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum ReputationLevel {
    Newcomer,
    Basic,
    Good,
    High,
    Excellent,
    Expert,
    Master,
    Legendary,
}

impl Space for ReputationLevel {
    const INIT_SPACE: usize = 1;
}

/// Transaction types for treasury operations
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Fee,
    Reward,
    Penalty,
}

impl Space for TransactionType {
    const INIT_SPACE: usize = 1;
}

/// Transaction status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum TransactionStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
    Failed,
}

impl Space for TransactionStatus {
    const INIT_SPACE: usize = 1;
}

/// Proposal status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

impl Space for ProposalStatus {
    const INIT_SPACE: usize = 1;
}

/// Vote types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

impl Space for VoteType {
    const INIT_SPACE: usize = 1;
}

/// Security levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Space for SecurityLevel {
    const INIT_SPACE: usize = 1;
}

/// AI decision types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AIDecisionType {
    Approve,
    Reject,
    Abstain,
    RequestMoreInfo,
}

impl Space for AIDecisionType {
    const INIT_SPACE: usize = 1;
}

/// Consensus status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum ConsensusStatus {
    Pending,
    InProgress,
    Reached,
    Failed,
    Timeout,
}

impl Space for ConsensusStatus {
    const INIT_SPACE: usize = 1;
}

/// Fixed-point arithmetic precision factor
pub const PRECISION_FACTOR: u64 = 1_000_000;

/// Maximum lengths for various fields
pub mod limits {
    pub const MAX_STRING_LEN: usize = 1000;
    pub const MAX_DESCRIPTION_LEN: usize = 500;
    pub const MAX_NAME_LEN: usize = 100;
    pub const MAX_VEC_LEN: usize = 50;
    pub const MAX_SMALL_VEC_LEN: usize = 20;
    pub const MAX_ALERTS_LEN: usize = 20;
    pub const MAX_POLICIES_LEN: usize = 10;
    pub const MAX_BADGES_LEN: usize = 10;
    pub const MAX_HISTORY_LEN: usize = 50;
}
