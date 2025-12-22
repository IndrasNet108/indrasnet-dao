//! Access Control Module for IndrasNet DAO
//! 
//! This module provides comprehensive access control and authorization
//! mechanisms to prevent unauthorized access to critical functions.
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/access_control.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

// NOTE: Core types are only available when access-control feature is enabled
// In production, Security program is independent and uses IDL for CPI
#[cfg(feature = "access-control")]
use indrasnet_dao_core::state::DaoConfig;
#[cfg(feature = "access-control")]
use indrasnet_dao_core::state::member::Member;
#[cfg(feature = "access-control")]
use indrasnet_dao_core::state::enums::MemberStatus;
// NOTE: MemberRole enum removed - use MemberRole account (member::role::MemberRole) instead

/// Access Control Manager
pub struct AccessControl;

impl AccessControl {
    /// Check if user has admin privileges
    /// 
    /// NOTE: Only available when access-control feature is enabled
    /// In production, use IDL-based CPI for program communication
    #[cfg(feature = "access-control")]
    pub fn is_admin(member: &Member, dao_config: &DaoConfig) -> Result<bool> {
        // NOTE: role field removed from Member - use MemberRole account instead
        // For MVP, check only authority and status
        Ok(member.status == MemberStatus::Active &&
           member.pubkey == dao_config.authority)
    }
    
    /// Check if user can vote
    /// 
    /// NOTE: Only available when access-control feature is enabled
    #[cfg(feature = "access-control")]
    pub fn can_vote(member: &Member) -> Result<bool> {
        // NOTE: can_vote() method removed - check only status and reputation
        Ok(member.status == MemberStatus::Active && 
           member.reputation >= 50)
    }
    
    /// Check if user can create proposals
    /// 
    /// NOTE: Only available when access-control feature is enabled
    #[cfg(feature = "access-control")]
    pub fn can_create_proposal(member: &Member) -> Result<bool> {
        // NOTE: can_propose() method removed - check only status and reputation
        Ok(member.status == MemberStatus::Active && 
           member.reputation >= 100)
    }
    
    /// Check if user can manage treasury
    /// 
    /// NOTE: Only available when access-control feature is enabled
    #[cfg(feature = "access-control")]
    pub fn can_manage_treasury(member: &Member, dao_config: &DaoConfig) -> Result<bool> {
        // NOTE: role field removed from Member - use MemberRole account instead
        // For MVP, check only authority and status
        Ok(member.status == MemberStatus::Active &&
           member.pubkey == dao_config.authority)
    }
    
    /// Check if user can perform DeFi operations
    /// 
    /// NOTE: Only available when access-control feature is enabled
    #[cfg(feature = "access-control")]
    pub fn can_perform_defi(member: &Member, amount: u64) -> Result<bool> {
        require!(member.status == MemberStatus::Active, IndrasError::Unauthorized);
        require!(member.reputation >= 200, IndrasError::Unauthorized); // Higher reputation required
        require!(amount <= 1000000000, IndrasError::AmountTooLarge); // Max 10 SOL
        
        Ok(true)
    }
    
    /// Check if user can manage AI delegations
    /// 
    /// NOTE: Only available when access-control feature is enabled
    #[cfg(feature = "access-control")]
    pub fn can_manage_ai_delegation(member: &Member) -> Result<bool> {
        Ok(member.status == MemberStatus::Active && 
           member.reputation >= 150)
    }
    
    /// Check if user can suspend/ban members
    /// 
    /// NOTE: Only available when access-control feature is enabled
    #[cfg(feature = "access-control")]
    pub fn can_manage_members(member: &Member, dao_config: &DaoConfig) -> Result<bool> {
        // NOTE: role field removed from Member - use MemberRole account instead
        // For MVP, check only authority and status
        Ok(member.status == MemberStatus::Active &&
           member.pubkey == dao_config.authority)
    }
    
    /// Validate quorum requirements
    /// Validate quorum for voting
    /// 
    /// NOTE: This function doesn't require Core types, so it's always available
    pub fn validate_quorum(
        total_members: u64,
        active_members: u64,
        min_quorum_percent: u8,
    ) -> Result<bool> {
        require!(min_quorum_percent <= 100, IndrasError::InvalidInput);
        
        let min_required = (total_members * min_quorum_percent as u64) / 100;
        Ok(active_members >= min_required)
    }
    
    /// Check if proposal meets execution requirements
    /// Check if proposal can be executed based on votes
    /// 
    /// NOTE: This function doesn't require Core types, so it's always available
    pub fn can_execute_proposal(
        yes_votes: u64,
        _no_votes: u64,
        total_votes: u64,
        min_approval_percent: u8,
    ) -> Result<bool> {
        require!(min_approval_percent <= 100, IndrasError::InvalidInput);
        
        if total_votes == 0 {
            return Ok(false);
        }
        
        let approval_percent = (yes_votes * 100) / total_votes;
        Ok(approval_percent >= min_approval_percent as u64)
    }
    
    /// Rate limiting for critical operations
    /// Check rate limit
    /// 
    /// NOTE: This function doesn't require Core types, so it's always available
    pub fn check_rate_limit(
        last_operation: i64,
        min_interval_seconds: i64,
    ) -> Result<bool> {
        let current_time = Clock::get()?.unix_timestamp;
        Self::check_rate_limit_with_time(last_operation, min_interval_seconds, current_time)
    }

    /// Check rate limit with explicit time (for testing)
    /// Check rate limit with explicit timestamp
    /// 
    /// NOTE: This function doesn't require Core types, so it's always available
    pub fn check_rate_limit_with_time(
        last_operation: i64,
        min_interval_seconds: i64,
        current_time: i64,
    ) -> Result<bool> {
        Ok(current_time - last_operation >= min_interval_seconds)
    }
    
    /// Validate amount limits
    /// Validate amount is within limits
    /// 
    /// NOTE: This function doesn't require Core types, so it's always available
    pub fn validate_amount(amount: u64, min_amount: u64, max_amount: u64) -> Result<()> {
        require!(amount >= min_amount, IndrasError::AmountTooSmall);
        require!(amount <= max_amount, IndrasError::AmountTooLarge);
        Ok(())
    }
    
    /// Check if operation is within daily limits
    /// Check daily limit
    /// 
    /// NOTE: This function doesn't require Core types, so it's always available
    pub fn check_daily_limit(
        daily_used: u64,
        daily_limit: u64,
        operation_amount: u64,
    ) -> Result<bool> {
        Ok(daily_used + operation_amount <= daily_limit)
    }
}

/// Security constants
pub mod constants {
    pub const MIN_VOTING_REPUTATION: u64 = 50;
    pub const MIN_PROPOSAL_REPUTATION: u64 = 100;
    pub const MIN_DEFI_REPUTATION: u64 = 200;
    pub const MIN_AI_DELEGATION_REPUTATION: u64 = 150;
    
    pub const MAX_DEFI_AMOUNT: u64 = 1000000000; // 10 SOL
    pub const MIN_DEFI_AMOUNT: u64 = 1000; // 0.000001 SOL
    
    pub const DEFAULT_QUORUM_PERCENT: u8 = 20; // 20%
    pub const DEFAULT_APPROVAL_PERCENT: u8 = 51; // 51%
    
    pub const RATE_LIMIT_INTERVAL: i64 = 3600; // 1 hour
    pub const DAILY_LIMIT_MULTIPLIER: u64 = 10; // 10x daily limit
}

#[cfg(all(test, feature = "access-control"))]
mod tests {
    use super::*;
    use indrasnet_dao_core::state::{Member, MemberStatus, DaoConfig};
    // NOTE: MemberRole enum removed - use MemberRole account instead

    fn create_test_member(reputation: u64) -> Member {
        Member {
            pubkey: Pubkey::from([1u8; 32]),
            status: MemberStatus::Active,
            // NOTE: role field removed - use MemberRole account instead
            reputation,
            joined_at: 1234567890,
            last_activity: 1234567890,
            contributions_count: 0,
            votes_cast: 0,
            ideas_created: 0,
            proposals_created: 0,
            suspension_reason: None,
            suspension_until: None,
            created_by: Pubkey::from([2u8; 32]),
            bump: 0,
        }
    }

    fn create_test_dao_config() -> DaoConfig {
        DaoConfig {
            authority: Pubkey::from([3u8; 32]),
            name: "Test DAO".to_string(),
            description: "Test Description".to_string(),
            is_active: true,
            is_paused: false,
            last_operation_timestamp: None,
            operation_count: 0,
            execution_delay_seconds: 0,
            adaptive_security_enabled: false,
            progressive_unlock_enabled: false,
            behavioral_analysis_enabled: false,
            created_at: 1234567890,
            updated_at: None,
            deactivated_at: None,
            reactivated_at: None,
            authority_transferred_at: None,
            security_enhancement_count: 0,
            bump: 0,
        }
    }

    #[test]
    fn test_is_admin() {
        let dao_config = create_test_dao_config();
        let mut member = create_test_member(1000);
        member.pubkey = dao_config.authority;
        
        assert!(AccessControl::is_admin(&member, &dao_config).unwrap());
        
        // Test with wrong authority
        let member_wrong = create_test_member(1000);
        assert!(!AccessControl::is_admin(&member_wrong, &dao_config).unwrap());
    }

    #[test]
    fn test_can_vote() {
        let member = create_test_member(60);
        assert!(AccessControl::can_vote(&member).unwrap());
        
        let member_low_rep = create_test_member(30);
        assert!(!AccessControl::can_vote(&member_low_rep).unwrap());
    }

    #[test]
    fn test_can_create_proposal() {
        let member = create_test_member(120);
        assert!(AccessControl::can_create_proposal(&member).unwrap());
        
        let member_low_rep = create_test_member(80);
        assert!(!AccessControl::can_create_proposal(&member_low_rep).unwrap());
    }

    #[test]
    fn test_validate_quorum() {
        assert!(AccessControl::validate_quorum(100, 25, 20).unwrap()); // 25% >= 20%
        assert!(!AccessControl::validate_quorum(100, 15, 20).unwrap()); // 15% < 20%
    }

    #[test]
    fn test_can_execute_proposal() {
        assert!(AccessControl::can_execute_proposal(60, 40, 100, 51).unwrap()); // 60% >= 51%
        assert!(!AccessControl::can_execute_proposal(40, 60, 100, 51).unwrap()); // 40% < 51%
    }

    #[test]
    fn test_validate_amount() {
        assert!(AccessControl::validate_amount(1000, 100, 10000).is_ok());
        assert!(AccessControl::validate_amount(50, 100, 10000).is_err()); // Too small
        assert!(AccessControl::validate_amount(20000, 100, 10000).is_err()); // Too large
    }

    #[test]
    fn test_check_rate_limit() {
        let last_operation = 1234567890;
        let min_interval = 3600; // 1 hour
        
        let current_time = last_operation + 1800; // 30 minutes later
        assert!(!AccessControl::check_rate_limit_with_time(last_operation, min_interval, current_time).unwrap());
        
        let current_time = last_operation + 3600; // Exactly 1 hour later
        assert!(AccessControl::check_rate_limit_with_time(last_operation, min_interval, current_time).unwrap());
    }

    #[test]
    fn test_check_daily_limit() {
        // Within limit
        assert!(AccessControl::check_daily_limit(100_000_000, 1_000_000_000, 500_000_000).unwrap());
        
        // At limit
        assert!(AccessControl::check_daily_limit(500_000_000, 1_000_000_000, 500_000_000).unwrap());
        
        // Over limit
        assert!(!AccessControl::check_daily_limit(600_000_000, 1_000_000_000, 500_000_000).unwrap());
    }
}
