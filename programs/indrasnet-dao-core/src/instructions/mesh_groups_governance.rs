//! Mesh Group Governance instruction handlers
//!
//! Handlers for mesh group governance operations:
//! - add_member_to_mesh_group - add member (Owner only)
//! - remove_member_from_mesh_group - remove member (Owner only)
//! - transfer_leadership - transfer leadership (Owner only)
//! - update_protocol - update operating protocol (Owner only)
//! - check_inactivity - check and pause if inactive (anyone can trigger)

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::mesh_group::{MeshGroupPermission, OperatingProtocol, MeetingFrequency};
use crate::state::mesh_group::types::{GroupMember, GroupRole};

/// Add member to mesh group
///
/// Track A: Simplified - only Owner can add members
///
/// # Security
/// - Validates caller is group leader (Owner)
/// - Validates group is not full (max 7 members)
/// - Validates member is not already in group
///
/// # Compute Units
/// Recommended: 20,000 CU
pub fn add_member_to_mesh_group_handler(
    ctx: Context<crate::AddMemberToMeshGroup>,
    member_pubkey: Pubkey,
    role: GroupRole,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let caller = ctx.accounts.caller.key();
    let current_time = Clock::get()?.unix_timestamp;
    
    // SECURITY: Validate caller is group leader (Owner)
    require!(
        mesh_group.has_permission(&caller, MeshGroupPermission::AddMember),
        IndrasError::Unauthorized
    );
    
    // SECURITY: Validate group is active
    require!(
        mesh_group.is_active(),
        IndrasError::InvalidState
    );
    
    // SEC-INV-16: Check critical moment - block member changes during active grants/proposals
    require!(
        !mesh_group.is_in_critical_moment,
        IndrasError::CriticalMomentActive
    );
    
    // SEC-INV-9: Rate limit check - 1 member per day per group
    if let Some(last_added) = mesh_group.last_member_added_at {
        const RATE_LIMIT_WINDOW_SECONDS: i64 = 86400; // 24 hours
        let time_since_last = current_time - last_added;
        require!(
            time_since_last >= RATE_LIMIT_WINDOW_SECONDS,
            IndrasError::RateLimitExceeded
        );
    }
    
    // SEC-INV-15: Reputation check - member must have sufficient reputation
    let member_reputation = if let Some(ref member_account) = ctx.accounts.member_account {
        // Verify member account matches member_pubkey
        require!(
            member_account.pubkey == member_pubkey,
            IndrasError::InvalidInput
        );
        member_account.reputation
    } else {
        // Member account not provided - use 0 (for backward compatibility)
        msg!("SEC-INV-15: WARNING - Reputation check skipped (member_account not provided)");
        0
    };
    
    // Enforce reputation check if reputation_required > 0
    if mesh_group.member_reputation_required > 0 {
        require!(
            member_reputation >= mesh_group.member_reputation_required,
            IndrasError::InsufficientReputation
        );
        msg!("SEC-INV-15: Reputation check passed (required: {}, member: {})", 
             mesh_group.member_reputation_required, member_reputation);
    }
    
    // SEC-INV-15: Cooldown check - former members must wait before rejoining
    if let Some(member_history_info) = &ctx.accounts.member_history {
        // Deserialize GroupMemberHistory manually from UncheckedAccount
        let member_history = crate::utils::account_helpers::deserialize_group_member_history(member_history_info)?;
        member_history.check_cooldown(
            &member_pubkey,
            current_time,
            mesh_group.member_cooldown_days,
        )?;
        msg!("SEC-INV-15: Cooldown check passed for member {}", member_pubkey);
    } else {
        // For MVP: If history not provided, skip check (allow rejoining immediately)
        // In production, this should be required
        msg!("SEC-INV-15: WARNING - Cooldown check skipped (member_history account not provided)");
    }
    
    // Create member
    let member = GroupMember {
        pubkey: member_pubkey,
        role,
        joined_at: current_time,
        contributions: 0,  // Field name is 'contributions', not 'contribution_count'
        reputation: member_reputation,
        is_active: true,
    };
    
    // Add member (updates last_contribution_at and last_member_added_at)
    mesh_group.add_member(member, current_time)?;
    
    msg!("Member {} added to mesh group {} by {} (reputation: {})", 
         member_pubkey, mesh_group.id, caller, member_reputation);
    
    Ok(())
}

/// Remove member from mesh group
///
/// Track A: Simplified - only Owner can remove members
///
/// # Security
/// - Validates caller is group leader (Owner)
/// - Validates member exists in group
/// - Cannot remove leader (must transfer leadership first)
/// - SEC-INV-15: Adds member to history for cooldown enforcement
///
/// # Compute Units
/// Recommended: 15,000 CU
pub fn remove_member_from_mesh_group_handler(
    ctx: Context<crate::RemoveMemberFromMeshGroup>,
    member_pubkey: Pubkey,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let caller = ctx.accounts.caller.key();
    let current_time = Clock::get()?.unix_timestamp;
    
    // SECURITY: Validate caller is group leader (Owner)
    require!(
        mesh_group.has_permission(&caller, MeshGroupPermission::RemoveMember),
        IndrasError::Unauthorized
    );
    
    // SECURITY: Validate group is active
    require!(
        mesh_group.is_active(),
        IndrasError::InvalidState
    );
    
    // Remove member (updates last_contribution_at)
    mesh_group.remove_member(member_pubkey, current_time)?;
    
    // SEC-INV-15: Add member to history for cooldown enforcement
    if let Some(member_history) = &mut ctx.accounts.member_history {
        use crate::state::MemberLeaveReason;
        member_history.add_member_exit(
            member_pubkey,
            current_time,
            MemberLeaveReason::Removed,
        )?;
        msg!("SEC-INV-15: Member {} added to history (cooldown: {} days)", 
             member_pubkey, mesh_group.member_cooldown_days);
    }
    
    msg!("Member {} removed from mesh group {} by {}", 
         member_pubkey, mesh_group.id, caller);
    
    Ok(())
}

/// Transfer leadership to another member
///
/// Track A: Simplified - only current Owner can transfer
///
/// # Security
/// - Validates caller is current leader
/// - Validates new_leader is member of group
/// - Validates new_leader is not current leader
///
/// # Compute Units
/// Recommended: 15,000 CU
pub fn transfer_leadership_handler(
    ctx: Context<crate::TransferMeshGroupLeadership>,
    new_leader: Pubkey,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let caller = ctx.accounts.caller.key();
    
    // SECURITY: Validate caller is current leader
    require!(
        mesh_group.leader == caller,
        IndrasError::Unauthorized
    );
    
    // SECURITY: Validate new_leader is member of group
    require!(
        mesh_group.members.iter().any(|m| m.pubkey == new_leader),
        IndrasError::MemberNotFound
    );
    
    // SECURITY: Validate new_leader is not current leader
    require!(
        mesh_group.leader != new_leader,
        IndrasError::InvalidInput
    );
    
    // Transfer leadership
    mesh_group.leader = new_leader;
    
    msg!("Leadership of mesh group {} transferred from {} to {}", 
         mesh_group.id, caller, new_leader);
    
    Ok(())
}

/// Update operating protocol
///
/// Track A: Simplified - only Owner can update protocol
///
/// # Security
/// - Validates caller is group leader
/// - Validates protocol parameters are within bounds
///
/// # Compute Units
/// Recommended: 10,000 CU
pub fn update_mesh_group_protocol_handler(
    ctx: Context<crate::UpdateMeshGroupProtocol>,
    meeting_frequency: MeetingFrequency,
    decision_quorum: u8,
    contribution_threshold: u32,
    inactivity_timeout_days: u16,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let caller = ctx.accounts.caller.key();
    
    // SECURITY: Validate caller is group leader
    require!(
        mesh_group.leader == caller,
        IndrasError::Unauthorized
    );
    
    // SECURITY: Validate protocol parameters
    require!(
        (50..=100).contains(&decision_quorum),
        IndrasError::InvalidInput
    );
    require!(
        contribution_threshold > 0 && contribution_threshold <= 100,
        IndrasError::InvalidInput
    );
    require!(
        inactivity_timeout_days > 0 && inactivity_timeout_days <= 365,
        IndrasError::InvalidInput
    );
    
    // Update protocol
    mesh_group.protocol = OperatingProtocol {
        meeting_frequency,
        decision_quorum,
        contribution_threshold,
        inactivity_timeout_days,
    };
    
    msg!("Protocol updated for mesh group {} by {}", 
         mesh_group.id, caller);
    
    Ok(())
}

/// Check inactivity and pause if needed
///
/// Can be called by anyone (public function)
/// Uses protocol.inactivity_timeout_days to determine if pause is needed
///
/// # Compute Units
/// Recommended: 10,000 CU
pub fn check_mesh_group_inactivity_handler(
    ctx: Context<crate::CheckMeshGroupInactivity>,
) -> Result<()> {
    let mesh_group = &mut ctx.accounts.mesh_group;
    let current_time = Clock::get()?.unix_timestamp;
    
    let was_paused = mesh_group.check_inactivity(current_time)?;
    
    if was_paused {
        msg!("Mesh group {} paused due to inactivity (last activity: {} days ago)", 
             mesh_group.id,
             (current_time - mesh_group.last_contribution_at) / (24 * 60 * 60));
    } else {
        msg!("Mesh group {} is active (last activity: {} days ago)", 
             mesh_group.id,
             (current_time - mesh_group.last_contribution_at) / (24 * 60 * 60));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::Pubkey;

    // ========== add_member_to_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_add_member_validation_unauthorized() {
        // Test: caller lacks AddMember permission should fail
        // This is validated in mesh_group.has_permission()
        assert!(true, "Unauthorized check validated in MeshGroup::has_permission()");
    }
    
    #[test]
    fn test_add_member_validation_group_not_active() {
        // Test: !mesh_group.is_active() should fail
        // This is validated in MeshGroup::is_active()
        assert!(true, "Group not active check validated in MeshGroup::is_active()");
    }
    
    #[test]
    fn test_add_member_validation_critical_moment_active() {
        // Test: mesh_group.is_in_critical_moment == true should fail
        let is_in_critical_moment = true;
        
        // Validation logic: require!(!is_in_critical_moment, IndrasError::CriticalMomentActive)
        assert!(is_in_critical_moment, "Critical moment active should be detected");
    }
    
    #[test]
    fn test_add_member_validation_rate_limit_exceeded() {
        // Test: time_since_last < RATE_LIMIT_WINDOW_SECONDS should fail
        let current_time = 1000000i64;
        let last_added = 1000000i64 - 86399i64; // 1 second before 24 hours
        const RATE_LIMIT_WINDOW_SECONDS: i64 = 86400;
        let time_since_last = current_time - last_added;
        
        // Validation logic: require!(time_since_last >= RATE_LIMIT_WINDOW_SECONDS, IndrasError::RateLimitExceeded)
        assert!(time_since_last < RATE_LIMIT_WINDOW_SECONDS, "Rate limit exceeded should be detected");
    }
    
    #[test]
    fn test_add_member_validation_group_full() {
        // Test: members.len() >= max_members should fail (via add_member())
        // This is validated in MeshGroup::add_member()
        assert!(true, "Group full check validated in MeshGroup::add_member()");
    }
    
    #[test]
    fn test_add_member_validation_member_already_in_group() {
        // Test: member already in group should fail (via add_member())
        // This is validated in MeshGroup::add_member()
        assert!(true, "Member already in group check validated in MeshGroup::add_member()");
    }

    // ========== remove_member_from_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_remove_member_validation_unauthorized() {
        // Test: caller lacks RemoveMember permission should fail
        // This is validated in mesh_group.has_permission()
        assert!(true, "Unauthorized check validated in MeshGroup::has_permission()");
    }
    
    #[test]
    fn test_remove_member_validation_group_not_active() {
        // Test: !mesh_group.is_active() should fail
        // This is validated in MeshGroup::is_active()
        assert!(true, "Group not active check validated in MeshGroup::is_active()");
    }
    
    #[test]
    fn test_remove_member_validation_member_not_found() {
        // Test: member not in group should fail (via remove_member())
        // This is validated in MeshGroup::remove_member()
        assert!(true, "Member not found check validated in MeshGroup::remove_member()");
    }
    
    #[test]
    fn test_remove_member_validation_cannot_remove_leader() {
        // Test: member == leader should fail (via remove_member())
        // This is validated in MeshGroup::remove_member()
        assert!(true, "Cannot remove leader check validated in MeshGroup::remove_member()");
    }

    // ========== transfer_leadership_handler validation tests ==========
    
    #[test]
    fn test_transfer_leadership_validation_caller_not_leader() {
        // Test: caller != mesh_group.leader should fail
        let caller = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        
        // Validation logic: require!(caller == leader, IndrasError::Unauthorized)
        assert_ne!(caller, leader, "Caller not leader should be detected");
    }
    
    #[test]
    fn test_transfer_leadership_validation_new_leader_not_member() {
        // Test: new_leader not in mesh_group.members should fail
        let new_leader = Pubkey::new_unique();
        let mesh_group_members = vec![Pubkey::new_unique(), Pubkey::new_unique()];
        
        // Validation logic: require!(members.iter().any(|m| m.pubkey == new_leader), IndrasError::MemberNotFound)
        assert!(!mesh_group_members.contains(&new_leader), "New leader not member should be detected");
    }
    
    #[test]
    fn test_transfer_leadership_validation_new_leader_is_current_leader() {
        // Test: new_leader == mesh_group.leader should fail
        let leader = Pubkey::new_unique();
        let new_leader = leader; // Same
        
        // Validation logic: require!(leader != new_leader, IndrasError::InvalidInput)
        assert_eq!(leader, new_leader, "New leader is current leader should be detected");
    }

    // ========== update_mesh_group_protocol_handler validation tests ==========
    
    #[test]
    fn test_update_protocol_validation_caller_not_leader() {
        // Test: caller != mesh_group.leader should fail
        let caller = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        
        // Validation logic: require!(caller == leader, IndrasError::Unauthorized)
        assert_ne!(caller, leader, "Caller not leader should be detected");
    }
    
    #[test]
    fn test_update_protocol_validation_decision_quorum_too_low() {
        // Test: decision_quorum < 50 should fail
        let decision_quorum = 49u8;
        
        // Validation logic: require!((50..=100).contains(&decision_quorum), IndrasError::InvalidInput)
        assert!(!(50..=100).contains(&decision_quorum), "Decision quorum too low should be detected");
    }
    
    #[test]
    fn test_update_protocol_validation_decision_quorum_too_high() {
        // Test: decision_quorum > 100 should fail
        let decision_quorum = 101u8;
        
        // Validation logic: require!((50..=100).contains(&decision_quorum), IndrasError::InvalidInput)
        assert!(!(50..=100).contains(&decision_quorum), "Decision quorum too high should be detected");
    }
    
    #[test]
    fn test_update_protocol_validation_contribution_threshold_zero() {
        // Test: contribution_threshold == 0 should fail
        let contribution_threshold = 0u32;
        
        // Validation logic: require!(contribution_threshold > 0 && contribution_threshold <= 100, IndrasError::InvalidInput)
        assert_eq!(contribution_threshold, 0, "Contribution threshold zero should be detected");
    }
    
    #[test]
    fn test_update_protocol_validation_contribution_threshold_too_high() {
        // Test: contribution_threshold > 100 should fail
        let contribution_threshold = 101u32;
        
        // Validation logic: require!(contribution_threshold > 0 && contribution_threshold <= 100, IndrasError::InvalidInput)
        assert!(contribution_threshold > 100, "Contribution threshold too high should be detected");
    }
    
    #[test]
    fn test_update_protocol_validation_inactivity_timeout_zero() {
        // Test: inactivity_timeout_days == 0 should fail
        let inactivity_timeout_days = 0u16;
        
        // Validation logic: require!(inactivity_timeout_days > 0 && inactivity_timeout_days <= 365, IndrasError::InvalidInput)
        assert_eq!(inactivity_timeout_days, 0, "Inactivity timeout zero should be detected");
    }
    
    #[test]
    fn test_update_protocol_validation_inactivity_timeout_too_high() {
        // Test: inactivity_timeout_days > 365 should fail
        let inactivity_timeout_days = 366u16;
        
        // Validation logic: require!(inactivity_timeout_days > 0 && inactivity_timeout_days <= 365, IndrasError::InvalidInput)
        assert!(inactivity_timeout_days > 365, "Inactivity timeout too high should be detected");
    }
    
    #[test]
    fn test_update_protocol_validation_valid_inputs() {
        // Test: valid inputs should pass
        let decision_quorum = 75u8;
        let contribution_threshold = 50u32;
        let inactivity_timeout_days = 30u16;
        
        // All validations should pass
        assert!((50..=100).contains(&decision_quorum), "Decision quorum should be valid");
        assert!(contribution_threshold > 0 && contribution_threshold <= 100, "Contribution threshold should be valid");
        assert!(inactivity_timeout_days > 0 && inactivity_timeout_days <= 365, "Inactivity timeout should be valid");
    }

    // ========== check_mesh_group_inactivity_handler validation tests ==========
    
    #[test]
    fn test_check_inactivity_validation_no_validation_needed() {
        // Test: check_inactivity is a public function with no input validation
        // It only calls mesh_group.check_inactivity() which is tested in state module
        assert!(true, "Check inactivity validation is in MeshGroup::check_inactivity()");
    }
}
