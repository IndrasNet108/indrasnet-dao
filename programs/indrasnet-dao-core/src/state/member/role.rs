//! Member Role with Permission Bitmask
//!
//! This module defines the MemberRole account structure with bitmask-based permissions.
//! Roles are stored in PDA and can only be changed through governance proposals.

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Permission flags (bitmask)
/// 
/// Each permission is a single bit in a u64 mask.
/// This allows for up to 64 different permissions.
pub mod role_permissions {
    /// Access admin panel
    pub const CAN_ACCESS_ADMIN: u64 = 1 << 0;           // 0x01
    
    /// Manage roles (assign/remove roles)
    pub const CAN_MANAGE_ROLES: u64 = 1 << 1;           // 0x02
    
    /// Manage treasury (propose payments)
    pub const CAN_MANAGE_TREASURY: u64 = 1 << 2;         // 0x04
    
    /// Moderate content (hide/flag/queue for vote)
    pub const CAN_MODERATE: u64 = 1 << 3;               // 0x08
    
    /// Create ideas
    pub const CAN_CREATE_IDEA: u64 = 1 << 4;            // 0x10
    
    /// Vote on proposals/grants
    pub const CAN_VOTE: u64 = 1 << 5;                   // 0x20
    
    /// Create proposals
    pub const CAN_PROPOSE: u64 = 1 << 6;                // 0x40
    
    /// Execute treasury transactions (after proposal execution)
    pub const CAN_EXECUTE_TREASURY_TX: u64 = 1 << 7;    // 0x80
    
    /// Emergency pause system
    pub const CAN_EMERGENCY_PAUSE: u64 = 1 << 8;       // 0x100
    
    /// Create grant requests
    pub const CAN_CREATE_GRANT: u64 = 1 << 9;           // 0x200
    
    /// Approve grants (through voting)
    pub const CAN_APPROVE_GRANT: u64 = 1 << 10;         // 0x400
    
    /// Manage mesh groups
    pub const CAN_MANAGE_MESH_GROUPS: u64 = 1 << 11;    // 0x800
    
    /// Manage experts (add/remove/update experts in registry)
    pub const CAN_MANAGE_EXPERTS: u64 = 1 << 14;       // 0x4000
    
    /// Default role masks
    pub mod roles {
        /// Observer: no permissions
        pub const OBSERVER: u64 = 0x00;
        
        /// Member: can create ideas and vote
        pub const MEMBER: u64 = super::CAN_CREATE_IDEA | super::CAN_VOTE; // 0x30
        
        /// Contributor: Member + can propose
        pub const CONTRIBUTOR: u64 = MEMBER | super::CAN_PROPOSE; // 0x70
        
        /// Moderator: Member + can moderate
        pub const MODERATOR: u64 = MEMBER | super::CAN_MODERATE; // 0x38
        
        /// Treasurer: can vote, propose, and manage treasury
        pub const TREASURER: u64 = super::CAN_VOTE | super::CAN_PROPOSE | super::CAN_MANAGE_TREASURY; // 0x66
        
        /// Admin: can access admin, moderate, create ideas, vote
        pub const ADMIN: u64 = super::CAN_ACCESS_ADMIN | super::CAN_MODERATE | super::CAN_CREATE_IDEA | super::CAN_VOTE; // 0x39
        
        /// Super Admin: emergency pause + admin access
        pub const SUPER_ADMIN: u64 = super::CAN_EMERGENCY_PAUSE | super::CAN_ACCESS_ADMIN; // 0x101
        
        /// Creator: all permissions (only during init)
        pub const CREATOR: u64 = 0xFFFFFFFFFFFFFFFF;
    }
}

/// MemberRole account structure
/// 
/// Stores role permissions for a member as a bitmask.
/// PDA seeds: [b"member_role", member_wallet]
#[account]
#[derive(InitSpace)]
pub struct MemberRole {
    pub member: Pubkey,
    pub role_mask: u64,              // Bitmask of permissions
    pub assigned_at: i64,            // When role was assigned
    pub assigned_by: Pubkey,         // Who assigned the role (governance proposal)
    pub last_updated: i64,           // Last update timestamp
    pub bump: u8,
}

impl MemberRole {
    /// Check if member has a specific permission
    pub fn has_permission(&self, permission: u64) -> bool {
        (self.role_mask & permission) != 0
    }
    
    /// Check if member has any of the given permissions
    pub fn has_any_permission(&self, permissions: u64) -> bool {
        (self.role_mask & permissions) != 0
    }
    
    /// Check if member has all of the given permissions
    pub fn has_all_permissions(&self, permissions: u64) -> bool {
        (self.role_mask & permissions) == permissions
    }
    
    /// Add permissions to role mask
    pub fn add_permissions(&mut self, permissions: u64) {
        let current_time = Clock::get().unwrap().unix_timestamp;
        self.add_permissions_with_time(permissions, current_time);
    }
    
    /// Add permissions to role mask with specified time
    pub fn add_permissions_with_time(&mut self, permissions: u64, current_time: i64) {
        self.role_mask |= permissions;
        self.last_updated = current_time;
    }
    
    /// Remove permissions from role mask
    pub fn remove_permissions(&mut self, permissions: u64) {
        let current_time = Clock::get().unwrap().unix_timestamp;
        self.remove_permissions_with_time(permissions, current_time);
    }
    
    /// Remove permissions from role mask with specified time
    pub fn remove_permissions_with_time(&mut self, permissions: u64, current_time: i64) {
        self.role_mask &= !permissions;
        self.last_updated = current_time;
    }
    
    /// Update role mask (used in governance proposals)
    pub fn update_role_mask(&mut self, new_mask: u64, updated_by: Pubkey, dao_authority: Pubkey) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.update_role_mask_with_time(new_mask, updated_by, dao_authority, current_time)
    }
    
    /// Update role mask with specified time
    pub fn update_role_mask_with_time(&mut self, new_mask: u64, updated_by: Pubkey, dao_authority: Pubkey, current_time: i64) -> Result<()> {
        // Prevent removing emergency pause from super admin without proper governance
        if self.has_permission(role_permissions::CAN_EMERGENCY_PAUSE) && 
           (new_mask & role_permissions::CAN_EMERGENCY_PAUSE) == 0 {
            require!(
                updated_by == dao_authority,
                IndrasError::Unauthorized
            );
        }
        
        self.role_mask = new_mask;
        self.assigned_by = updated_by;
        self.last_updated = current_time;
        Ok(())
    }
    
    /// Create new MemberRole with initial mask
    pub fn new(
        member: Pubkey,
        role_mask: u64,
        assigned_by: Pubkey,
        bump: u8,
    ) -> Result<Self> {
        let current_time = Clock::get()?.unix_timestamp;
        Self::new_with_time(member, role_mask, assigned_by, bump, current_time)
    }
    
    /// Create new MemberRole with initial mask and specified time
    pub fn new_with_time(
        member: Pubkey,
        role_mask: u64,
        assigned_by: Pubkey,
        bump: u8,
        current_time: i64,
    ) -> Result<Self> {
        Ok(Self {
            member,
            role_mask,
            assigned_at: current_time,
            assigned_by,
            last_updated: current_time,
            bump,
        })
    }
}

/// Helper function to check permission in instruction handlers
pub fn require_permission(role: &MemberRole, permission: u64) -> Result<()> {
    require!(
        role.has_permission(permission),
        IndrasError::Unauthorized
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_role() -> MemberRole {
        MemberRole {
            member: Pubkey::new_unique(),
            role_mask: role_permissions::roles::MEMBER,
            assigned_at: 1000,
            assigned_by: Pubkey::new_unique(),
            last_updated: 1000,
            bump: 255,
        }
    }

    #[test]
    fn test_member_role_has_permission() {
        let role = create_test_role();
        
        // MEMBER role has CAN_CREATE_IDEA and CAN_VOTE
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(!role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
    }

    #[test]
    fn test_member_role_has_any_permission() {
        let role = create_test_role();
        
        // Has CAN_CREATE_IDEA
        assert!(role.has_any_permission(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_PROPOSE));
        // Doesn't have CAN_PROPOSE or CAN_MANAGE_TREASURY
        assert!(!role.has_any_permission(role_permissions::CAN_PROPOSE | role_permissions::CAN_MANAGE_TREASURY));
    }

    #[test]
    fn test_member_role_has_all_permissions() {
        let role = create_test_role();
        
        // Has both CAN_CREATE_IDEA and CAN_VOTE
        assert!(role.has_all_permissions(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE));
        // Doesn't have CAN_PROPOSE
        assert!(!role.has_all_permissions(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_add_permissions_with_time() {
        let mut role = create_test_role();
        
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
        role.add_permissions_with_time(role_permissions::CAN_PROPOSE, 2000);
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        assert_eq!(role.last_updated, 2000);
    }

    #[test]
    fn test_member_role_remove_permissions_with_time() {
        let mut role = create_test_role();
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        role.remove_permissions_with_time(role_permissions::CAN_CREATE_IDEA, 3000);
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert_eq!(role.last_updated, 3000);
    }

    #[test]
    fn test_member_role_add_multiple_permissions_with_time() {
        let mut role = create_test_role();
        
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
        role.add_permissions_with_time(role_permissions::CAN_PROPOSE | role_permissions::CAN_MANAGE_TREASURY, 4000);
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
        assert_eq!(role.last_updated, 4000);
    }

    #[test]
    fn test_member_role_remove_multiple_permissions_with_time() {
        let mut role = create_test_role();
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        role.remove_permissions_with_time(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE, 5000);
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(!role.has_permission(role_permissions::CAN_VOTE));
        assert_eq!(role.last_updated, 5000);
    }

    #[test]
    fn test_member_role_update_role_mask_with_time() {
        let mut role = create_test_role();
        let dao_authority = Pubkey::new_unique();
        let updated_by = Pubkey::new_unique();
        
        let new_mask = role_permissions::roles::CONTRIBUTOR;
        
        // Normal update (not removing emergency pause) - should succeed
        assert!(role.update_role_mask_with_time(new_mask, updated_by, dao_authority, 6000).is_ok());
        assert_eq!(role.role_mask, new_mask);
        assert_eq!(role.assigned_by, updated_by);
        assert_eq!(role.last_updated, 6000);
    }

    #[test]
    fn test_member_role_update_role_mask_remove_emergency_pause_without_authority() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::SUPER_ADMIN;
        let dao_authority = Pubkey::new_unique();
        let unauthorized = Pubkey::new_unique();
        
        let new_mask = role_permissions::roles::ADMIN;
        
        // Has emergency pause and trying to remove it without authority - should fail
        assert!(role.has_permission(role_permissions::CAN_EMERGENCY_PAUSE));
        assert_eq!(new_mask & role_permissions::CAN_EMERGENCY_PAUSE, 0);
        
        assert!(role.update_role_mask_with_time(new_mask, unauthorized, dao_authority, 7000).is_err());
    }

    #[test]
    fn test_member_role_update_role_mask_remove_emergency_pause_with_authority() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::SUPER_ADMIN;
        let dao_authority = Pubkey::new_unique();
        
        let new_mask = role_permissions::roles::ADMIN;
        
        // With authority - should succeed
        assert!(role.update_role_mask_with_time(new_mask, dao_authority, dao_authority, 8000).is_ok());
        assert_eq!(role.role_mask, new_mask);
        assert_eq!(role.assigned_by, dao_authority);
        assert_eq!(role.last_updated, 8000);
        assert!(!role.has_permission(role_permissions::CAN_EMERGENCY_PAUSE));
    }

    #[test]
    fn test_member_role_new_with_time() {
        let member = Pubkey::new_unique();
        let assigned_by = Pubkey::new_unique();
        let role_mask = role_permissions::roles::MEMBER;
        
        let role = MemberRole::new_with_time(member, role_mask, assigned_by, 255, 9000).unwrap();
        
        assert_eq!(role.member, member);
        assert_eq!(role.role_mask, role_mask);
        assert_eq!(role.assigned_at, 9000);
        assert_eq!(role.assigned_by, assigned_by);
        assert_eq!(role.last_updated, 9000);
        assert_eq!(role.bump, 255);
    }

    #[test]
    fn test_member_role_role_masks() {
        // Test predefined role masks
        assert_eq!(role_permissions::roles::OBSERVER, 0x00);
        assert_eq!(role_permissions::roles::MEMBER, 
                   role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE);
        assert_eq!(role_permissions::roles::CONTRIBUTOR,
                   role_permissions::roles::MEMBER | role_permissions::CAN_PROPOSE);
    }

    #[test]
    fn test_member_role_permission_flags() {
        // Test permission flags are unique powers of 2
        assert_eq!(role_permissions::CAN_ACCESS_ADMIN, 1 << 0);
        assert_eq!(role_permissions::CAN_MANAGE_ROLES, 1 << 1);
        assert_eq!(role_permissions::CAN_MANAGE_TREASURY, 1 << 2);
        assert_eq!(role_permissions::CAN_MODERATE, 1 << 3);
    }

    #[test]
    fn test_require_permission() {
        let role = create_test_role();
        
        // Has permission - should succeed
        assert!(require_permission(&role, role_permissions::CAN_CREATE_IDEA).is_ok());
        
        // Doesn't have permission - should fail
        assert!(require_permission(&role, role_permissions::CAN_PROPOSE).is_err());
    }

    #[test]
    fn test_member_role_all_permission_flags() {
        // Test all permission flags are unique powers of 2
        assert_eq!(role_permissions::CAN_ACCESS_ADMIN, 1 << 0);
        assert_eq!(role_permissions::CAN_MANAGE_ROLES, 1 << 1);
        assert_eq!(role_permissions::CAN_MANAGE_TREASURY, 1 << 2);
        assert_eq!(role_permissions::CAN_MODERATE, 1 << 3);
        assert_eq!(role_permissions::CAN_CREATE_IDEA, 1 << 4);
        assert_eq!(role_permissions::CAN_VOTE, 1 << 5);
        assert_eq!(role_permissions::CAN_PROPOSE, 1 << 6);
        assert_eq!(role_permissions::CAN_EXECUTE_TREASURY_TX, 1 << 7);
        assert_eq!(role_permissions::CAN_EMERGENCY_PAUSE, 1 << 8);
        assert_eq!(role_permissions::CAN_CREATE_GRANT, 1 << 9);
        assert_eq!(role_permissions::CAN_APPROVE_GRANT, 1 << 10);
        assert_eq!(role_permissions::CAN_MANAGE_MESH_GROUPS, 1 << 11);
        assert_eq!(role_permissions::CAN_MANAGE_EXPERTS, 1 << 14);
    }

    #[test]
    fn test_member_role_all_role_masks() {
        // Test all predefined role masks
        assert_eq!(role_permissions::roles::OBSERVER, 0x00);
        assert_eq!(role_permissions::roles::MEMBER, 
                   role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE);
        assert_eq!(role_permissions::roles::CONTRIBUTOR,
                   role_permissions::roles::MEMBER | role_permissions::CAN_PROPOSE);
        assert_eq!(role_permissions::roles::MODERATOR,
                   role_permissions::roles::MEMBER | role_permissions::CAN_MODERATE);
        assert_eq!(role_permissions::roles::TREASURER,
                   role_permissions::CAN_VOTE | role_permissions::CAN_PROPOSE | role_permissions::CAN_MANAGE_TREASURY);
        assert_eq!(role_permissions::roles::ADMIN,
                   role_permissions::CAN_ACCESS_ADMIN | role_permissions::CAN_MODERATE | 
                   role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE);
        assert_eq!(role_permissions::roles::SUPER_ADMIN,
                   role_permissions::CAN_EMERGENCY_PAUSE | role_permissions::CAN_ACCESS_ADMIN);
        assert_eq!(role_permissions::roles::CREATOR, 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn test_member_role_has_permission_multiple_flags() {
        let mut role = create_test_role();
        
        // Add multiple permissions
        role.role_mask = role_permissions::CAN_CREATE_IDEA | 
                         role_permissions::CAN_VOTE | 
                         role_permissions::CAN_PROPOSE;
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(!role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
    }

    #[test]
    fn test_member_role_has_any_permission_single() {
        let role = create_test_role();
        
        // Test with single permission
        assert!(role.has_any_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(!role.has_any_permission(role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_has_all_permissions_single() {
        let role = create_test_role();
        
        // Test with single permission
        assert!(role.has_all_permissions(role_permissions::CAN_CREATE_IDEA));
        assert!(!role.has_all_permissions(role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_has_all_permissions_partial() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE;
        
        // Has CAN_CREATE_IDEA but not CAN_PROPOSE
        assert!(!role.has_all_permissions(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_PROPOSE));
        
        // Has both CAN_CREATE_IDEA and CAN_VOTE
        assert!(role.has_all_permissions(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE));
    }

    #[test]
    fn test_member_role_add_permissions_preserves_existing() {
        let mut role = create_test_role();
        
        // Start with MEMBER role (CAN_CREATE_IDEA | CAN_VOTE)
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        
        // Add CAN_PROPOSE - should preserve existing permissions
        role.add_permissions_with_time(role_permissions::CAN_PROPOSE, 10000);
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_remove_permissions_preserves_others() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE | role_permissions::CAN_PROPOSE;
        
        // Remove only CAN_PROPOSE
        role.remove_permissions_with_time(role_permissions::CAN_PROPOSE, 11000);
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_update_role_mask_keeps_emergency_pause() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::SUPER_ADMIN;
        let dao_authority = Pubkey::new_unique();
        let updated_by = Pubkey::new_unique();
        
        // Update mask but keep emergency pause - should succeed without authority check
        let new_mask = role_permissions::CAN_EMERGENCY_PAUSE | role_permissions::CAN_ACCESS_ADMIN | role_permissions::CAN_CREATE_IDEA;
        
        assert!(role.update_role_mask_with_time(new_mask, updated_by, dao_authority, 12000).is_ok());
        assert_eq!(role.role_mask, new_mask);
        assert!(role.has_permission(role_permissions::CAN_EMERGENCY_PAUSE));
    }

    #[test]
    fn test_member_role_permission_combinations() {
        let mut role = create_test_role();
        
        // Test various permission combinations
        role.role_mask = role_permissions::CAN_VOTE | role_permissions::CAN_PROPOSE;
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
        
        // Add more permissions
        role.role_mask |= role_permissions::CAN_MANAGE_TREASURY;
        assert!(role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
    }

    #[test]
    fn test_member_role_empty_mask() {
        let mut role = create_test_role();
        role.role_mask = 0;
        
        // No permissions
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(!role.has_permission(role_permissions::CAN_VOTE));
        assert!(!role.has_any_permission(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE));
        assert!(!role.has_all_permissions(role_permissions::CAN_CREATE_IDEA));
    }

    #[test]
    fn test_member_role_full_mask() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::CREATOR;
        
        // Has all permissions
        assert!(role.has_permission(role_permissions::CAN_ACCESS_ADMIN));
        assert!(role.has_permission(role_permissions::CAN_EMERGENCY_PAUSE));
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_all_permissions(role_permissions::CAN_ACCESS_ADMIN | 
                                         role_permissions::CAN_EMERGENCY_PAUSE));
    }

    #[test]
    fn test_member_role_remove_single_permission() {
        let mut role = create_test_role();
        
        // Start with MEMBER role (CAN_CREATE_IDEA | CAN_VOTE)
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        
        // Remove only CAN_CREATE_IDEA
        role.role_mask &= !role_permissions::CAN_CREATE_IDEA;
        
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE)); // Still has this
    }

    #[test]
    fn test_member_role_structure() {
        let role = create_test_role();
        assert_eq!(role.role_mask, role_permissions::roles::MEMBER);
        assert_eq!(role.assigned_at, 1000);
        assert_eq!(role.last_updated, 1000);
        assert_eq!(role.bump, 255);
    }

    #[test]
    fn test_member_role_has_any_permission_empty() {
        let mut role = create_test_role();
        role.role_mask = 0;
        
        assert!(!role.has_any_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(!role.has_any_permission(role_permissions::CAN_VOTE | role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_has_all_permissions_empty() {
        let mut role = create_test_role();
        role.role_mask = 0;
        
        assert!(!role.has_all_permissions(role_permissions::CAN_CREATE_IDEA));
        assert!(!role.has_all_permissions(role_permissions::CAN_VOTE | role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_has_all_permissions_zero() {
        let role = create_test_role();
        
        // Zero permissions should always pass
        assert!(role.has_all_permissions(0));
    }

    #[test]
    fn test_member_role_has_any_permission_zero() {
        let role = create_test_role();
        
        // Zero permissions should always fail
        assert!(!role.has_any_permission(0));
    }

    #[test]
    fn test_member_role_add_remove_cycle() {
        let mut role = create_test_role();
        
        // Add permission
        role.role_mask |= role_permissions::CAN_PROPOSE;
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        
        // Remove permission
        role.role_mask &= !role_permissions::CAN_PROPOSE;
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
        
        // Add again
        role.role_mask |= role_permissions::CAN_PROPOSE;
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_multiple_add_remove() {
        let mut role = create_test_role();
        
        // Add multiple permissions
        role.role_mask |= role_permissions::CAN_PROPOSE | role_permissions::CAN_MANAGE_TREASURY;
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
        
        // Remove one
        role.role_mask &= !role_permissions::CAN_PROPOSE;
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
        
        // Remove the other
        role.role_mask &= !role_permissions::CAN_MANAGE_TREASURY;
        assert!(!role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
    }

    #[test]
    fn test_member_role_update_mask_preserves_other_permissions() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE | role_permissions::CAN_PROPOSE;
        
        // Update to remove CAN_PROPOSE but keep others
        let new_mask = role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE;
        role.role_mask = new_mask;
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_observer_role() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::OBSERVER;
        
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(!role.has_permission(role_permissions::CAN_VOTE));
        assert!(!role.has_any_permission(role_permissions::CAN_CREATE_IDEA | role_permissions::CAN_VOTE));
    }

    #[test]
    fn test_member_role_contributor_role() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::CONTRIBUTOR;
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(!role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
    }

    #[test]
    fn test_member_role_moderator_role() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::MODERATOR;
        
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(role.has_permission(role_permissions::CAN_MODERATE));
        assert!(!role.has_permission(role_permissions::CAN_PROPOSE));
    }

    #[test]
    fn test_member_role_treasurer_role() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::TREASURER;
        
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(role.has_permission(role_permissions::CAN_PROPOSE));
        assert!(role.has_permission(role_permissions::CAN_MANAGE_TREASURY));
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
    }

    #[test]
    fn test_member_role_admin_role() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::ADMIN;
        
        assert!(role.has_permission(role_permissions::CAN_ACCESS_ADMIN));
        assert!(role.has_permission(role_permissions::CAN_MODERATE));
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
    }

    #[test]
    fn test_member_role_super_admin_role() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::SUPER_ADMIN;
        
        assert!(role.has_permission(role_permissions::CAN_EMERGENCY_PAUSE));
        assert!(role.has_permission(role_permissions::CAN_ACCESS_ADMIN));
        assert!(!role.has_permission(role_permissions::CAN_CREATE_IDEA));
    }

    #[test]
    fn test_member_role_creator_role() {
        let mut role = create_test_role();
        role.role_mask = role_permissions::roles::CREATOR;
        
        // Creator has all permissions
        assert!(role.has_permission(role_permissions::CAN_ACCESS_ADMIN));
        assert!(role.has_permission(role_permissions::CAN_EMERGENCY_PAUSE));
        assert!(role.has_permission(role_permissions::CAN_CREATE_IDEA));
        assert!(role.has_permission(role_permissions::CAN_VOTE));
        assert!(role.has_all_permissions(role_permissions::CAN_ACCESS_ADMIN | 
                                         role_permissions::CAN_EMERGENCY_PAUSE |
                                         role_permissions::CAN_CREATE_IDEA));
    }

    #[test]
    fn test_member_role_permission_bit_operations() {
        let mut role = create_test_role();
        
        // Test bitwise operations
        assert_eq!(role.role_mask & role_permissions::CAN_CREATE_IDEA, role_permissions::CAN_CREATE_IDEA);
        assert_eq!(role.role_mask & role_permissions::CAN_PROPOSE, 0);
        
        // Test OR operation
        role.role_mask |= role_permissions::CAN_PROPOSE;
        assert_eq!(role.role_mask & role_permissions::CAN_PROPOSE, role_permissions::CAN_PROPOSE);
        
        // Test AND NOT operation
        role.role_mask &= !role_permissions::CAN_PROPOSE;
        assert_eq!(role.role_mask & role_permissions::CAN_PROPOSE, 0);
    }

    #[test]
    fn test_member_role_require_permission_success() {
        let role = create_test_role();
        
        // Has permission
        assert!(require_permission(&role, role_permissions::CAN_CREATE_IDEA).is_ok());
        assert!(require_permission(&role, role_permissions::CAN_VOTE).is_ok());
    }

    #[test]
    fn test_member_role_require_permission_failure() {
        let role = create_test_role();
        
        // Doesn't have permission
        assert!(require_permission(&role, role_permissions::CAN_PROPOSE).is_err());
        assert!(require_permission(&role, role_permissions::CAN_MANAGE_TREASURY).is_err());
    }

    #[test]
    fn test_member_role_add_permissions_with_time_preserves_other_fields() {
        let mut role = create_test_role();
        let original_member = role.member;
        let original_assigned_by = role.assigned_by;
        let original_assigned_at = role.assigned_at;
        let original_bump = role.bump;
        
        role.add_permissions_with_time(role_permissions::CAN_PROPOSE, 2000);
        
        assert_eq!(role.member, original_member);
        assert_eq!(role.assigned_by, original_assigned_by);
        assert_eq!(role.assigned_at, original_assigned_at);
        assert_eq!(role.bump, original_bump);
        assert_eq!(role.last_updated, 2000);
    }

    #[test]
    fn test_member_role_remove_permissions_with_time_preserves_other_fields() {
        let mut role = create_test_role();
        let original_member = role.member;
        let original_assigned_by = role.assigned_by;
        let original_assigned_at = role.assigned_at;
        let original_bump = role.bump;
        
        role.remove_permissions_with_time(role_permissions::CAN_CREATE_IDEA, 3000);
        
        assert_eq!(role.member, original_member);
        assert_eq!(role.assigned_by, original_assigned_by);
        assert_eq!(role.assigned_at, original_assigned_at);
        assert_eq!(role.bump, original_bump);
        assert_eq!(role.last_updated, 3000);
    }

    #[test]
    fn test_member_role_update_role_mask_with_time_preserves_other_fields() {
        let mut role = create_test_role();
        let original_member = role.member;
        let original_bump = role.bump;
        let dao_authority = Pubkey::new_unique();
        let updated_by = Pubkey::new_unique();
        
        let new_mask = role_permissions::roles::CONTRIBUTOR;
        assert!(role.update_role_mask_with_time(new_mask, updated_by, dao_authority, 6000).is_ok());
        
        assert_eq!(role.member, original_member);
        assert_eq!(role.bump, original_bump);
        assert_eq!(role.assigned_by, updated_by);
        assert_eq!(role.last_updated, 6000);
    }

    #[test]
    fn test_member_role_new_with_time_all_fields() {
        let member = Pubkey::new_unique();
        let assigned_by = Pubkey::new_unique();
        let role_mask = role_permissions::roles::ADMIN;
        
        let role = MemberRole::new_with_time(member, role_mask, assigned_by, 128, 10000).unwrap();
        
        assert_eq!(role.member, member);
        assert_eq!(role.role_mask, role_mask);
        assert_eq!(role.assigned_at, 10000);
        assert_eq!(role.assigned_by, assigned_by);
        assert_eq!(role.last_updated, 10000);
        assert_eq!(role.bump, 128);
    }
}
