//! MeshGroup structure and implementation
//!
//! Main MeshGroup structure and all methods (lifecycle, members, resources, queries)

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::{
    GroupType, GroupStatus, DevelopmentStage, 
    GroupMember, MeshGroupParams
};
use super::permissions::MeshGroupPermission;

/// Mesh Group
/// 
/// NOTE: No "Sandbox" term - only mesh groups
/// Mesh group can consist of 1-7 people (if more needed, additional mesh group is created)
#[account]
#[derive(InitSpace)]
pub struct MeshGroup {
    pub id: u64,
    #[max_len(100)]
    pub name: String,
    #[max_len(500)]
    pub description: String,
    pub group_type: GroupType,
    pub status: GroupStatus,
    pub leader: Pubkey,              // Group leader (Owner)
    #[max_len(20)]
    pub members: Vec<GroupMember>,   // Group members (max 7 for main group)
    #[max_len(10)]
    pub ideas: Vec<u64>,             // Linked idea IDs
    #[max_len(10)]
    pub grants: Vec<u64>,            // Linked grant IDs
    #[max_len(5)]
    pub phenomena: Vec<Pubkey>,      // Phenomena that group belongs to (created by AI)
    pub max_members: u8,             // Maximum members (7 for mesh group)
    pub min_members: u8,             // Minimum members (1 - can be one genius)
    /// Parent group (if this is supporting group)
    /// None for main group
    pub parent_group: Option<Pubkey>,
    /// Supporting groups (if main group is full)
    /// Max 10 supporting groups (70 people max: 7 + 10*7 = 77, but limited to 70)
    #[max_len(10)]
    pub supporting_groups: Vec<Pubkey>,
    /// Stage deadline (for mesh groups - max 12 weeks)
    pub stage_deadline: Option<i64>,
    pub current_stage: DevelopmentStage,  // Current development stage
    pub created_at: i64,
    pub started_at: Option<i64>,     // Work start date
    pub completed_at: Option<i64>,   // Completion date
    pub total_contributions: u32,    // Total contributions
    pub total_reputation: u64,       // Total group reputation
    pub created_by: Pubkey,
    // v1: Operating protocol (Track A: simplified)
    pub protocol: super::protocol::OperatingProtocol,
    pub last_meeting_at: Option<i64>, // Last meeting timestamp
    pub last_contribution_at: i64,    // Last contribution timestamp
    
    // v1.1: Rate limiting (SEC-INV-9)
    pub last_member_added_at: Option<i64>,  // For rate limit: 1 member/day per group
    pub last_group_created_at: Option<i64>,  // For rate limit: 1 group/week per creator (stored in creator's context)
    
    // v1.1: Sybil protection (SEC-INV-15)
    pub member_reputation_required: u64,    // Minimum reputation score (default: 10)
    pub member_cooldown_days: u16,           // Cooldown period for former members (default: 30)
    
    // v1.1: Critical moment restrictions (SEC-INV-16)
    pub is_in_critical_moment: bool,        // True if active grants exist
    pub critical_moment_until: Option<i64>, // Timestamp when critical moment ends
    
    // Embedding fields (for Track B)
    pub embedding_hash: Option<[u8; 32]>,
    pub embedding_signature: Option<[u8; 64]>,
    #[max_len(100)]
    pub embedding_provider: Option<String>,
    #[max_len(100)]
    pub embedding_model: Option<String>,
    #[max_len(50)]
    pub embedding_model_version: Option<String>,
    pub embedding_created_at: Option<i64>,
    pub embedding_updated_at: Option<i64>,
    pub embedding_update_count: u64,
    
    pub bump: u8,
}

impl MeshGroup {
    // ===== LIFECYCLE METHODS =====
    
    /// Create a new mesh group
    pub fn new(params: MeshGroupParams) -> Result<Self> {
        Self::new_with_time(params, Clock::get()?.unix_timestamp)
    }

    /// Create a new mesh group with explicit timestamp
    pub fn new_with_time(params: MeshGroupParams, current_time: i64) -> Result<Self> {
        let MeshGroupParams { id, name, description, group_type, leader, max_members, min_members, created_by, bump } = params;
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(!description.is_empty(), IndrasError::InvalidInput);
        require!(max_members > 0, IndrasError::InvalidInput);
        require!(min_members > 0, IndrasError::InvalidInput);
        require!(min_members <= max_members, IndrasError::InvalidInput);
        // Mesh group can have maximum 7 members
        require!(max_members <= 7, IndrasError::InvalidInput); // Mesh group max 7 members
        
        Ok(Self {
            id,
            name,
            description,
            group_type,
            status: GroupStatus::Forming,
            leader,
            members: Vec::new(),
            ideas: Vec::new(),
            grants: Vec::new(),
            phenomena: Vec::new(),  // Group not yet in phenomenon (AI will add later)
            max_members,
            min_members,
            parent_group: None,  // Main group by default
            supporting_groups: Vec::new(),  // Supporting groups created later
            stage_deadline: None,  // Stage deadline (set at creation)
            current_stage: DevelopmentStage::Planning,  // Start with planning
            created_at: current_time,
            started_at: None,
            completed_at: None,
            total_contributions: 0,
            total_reputation: 0,
            created_by,
            // v1: Initialize protocol with defaults
            protocol: super::protocol::OperatingProtocol::default(),
            last_meeting_at: None,
            last_contribution_at: current_time,
            
            // v1.1: Initialize rate limiting fields (SEC-INV-9)
            last_member_added_at: None,
            last_group_created_at: Some(current_time), // Track creation time for rate limiting
            
            // v1.1: Initialize Sybil protection fields (SEC-INV-15)
            member_reputation_required: 10,  // Default: 10 reputation points required
            member_cooldown_days: 30,         // Default: 30 days cooldown
            
            // v1.1: Initialize critical moment fields (SEC-INV-16)
            is_in_critical_moment: false,
            critical_moment_until: None,
            
            // Embedding fields - defaults
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            
            bump,
        })
    }

    /// Start group (Forming -> Active)
    pub fn start_group(&mut self) -> Result<()> {
        self.start_group_with_time(Clock::get()?.unix_timestamp)
    }

    /// Start group with explicit timestamp
    pub fn start_group_with_time(&mut self, current_time: i64) -> Result<()> {
        // Group now activates automatically on creation (min_members = 1)
        // This function kept for compatibility
        require!(self.status == GroupStatus::Forming, IndrasError::InvalidState);
        require!(self.members.len() >= self.min_members as usize, IndrasError::InsufficientMembers);
        
        self.status = GroupStatus::Active;
        if self.started_at.is_none() {
            self.started_at = Some(current_time);
        }
        Ok(())
    }

    /// Pause group (Active -> Paused)
    /// 
    /// Can be called manually or automatically via protocol inactivity check
    pub fn pause_group(&mut self) -> Result<()> {
        require!(self.status == GroupStatus::Active, IndrasError::InvalidState);
        self.status = GroupStatus::Paused;
        Ok(())
    }
    
    /// Check if group should be paused due to inactivity
    /// 
    /// Uses protocol.inactivity_timeout_days to determine if pause is needed
    pub fn check_inactivity(&mut self, current_time: i64) -> Result<bool> {
        if self.status != GroupStatus::Active {
            return Ok(false);
        }
        let should_pause = self.protocol.should_pause_for_inactivity(
            self.last_contribution_at,
            current_time
        );
        if should_pause {
            self.pause_group()?;
        }
        Ok(should_pause)
    }

    /// Resume group (Paused -> Active)
    pub fn resume_group(&mut self) -> Result<()> {
        require!(self.status == GroupStatus::Paused, IndrasError::InvalidState);
        self.status = GroupStatus::Active;
        Ok(())
    }

    /// Complete group (Active -> Completed)
    pub fn complete_group(&mut self) -> Result<()> {
        self.complete_group_with_time(Clock::get()?.unix_timestamp)
    }

    /// Complete group with explicit timestamp
    pub fn complete_group_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == GroupStatus::Active, IndrasError::InvalidState);
        self.status = GroupStatus::Completed;
        self.completed_at = Some(current_time);
        Ok(())
    }

    /// Disband group (any status -> Disbanded)
    pub fn disband_group(&mut self) -> Result<()> {
        self.disband_group_with_time(Clock::get()?.unix_timestamp)
    }

    /// Disband group with explicit timestamp
    pub fn disband_group_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status != GroupStatus::Disbanded, IndrasError::InvalidState);
        self.status = GroupStatus::Disbanded;
        self.completed_at = Some(current_time);
        Ok(())
    }

    // ===== MEMBER MANAGEMENT METHODS =====

    /// Add member to group
    ///
    /// Track A: Simplified - only Owner can add members
    /// Updates last_contribution_at timestamp
    ///
    /// # Security
    /// - Validates group status
    /// - Checks member limit (max_members)
    /// - Prevents duplicate members
    /// - Uses checked arithmetic for reputation to prevent overflow
    pub fn add_member(&mut self, member: GroupMember, current_time: i64) -> Result<()> {
        require!(self.status == GroupStatus::Forming || self.status == GroupStatus::Active, IndrasError::InvalidState);
        require!(self.members.len() < self.max_members as usize, IndrasError::GroupFull);
        require!(!self.is_member(member.pubkey), IndrasError::AlreadyMember);
        
        // SECURITY: Use checked arithmetic to prevent overflow
        let reputation = member.reputation;
        self.members.push(member);
        self.total_reputation = self.total_reputation
            .checked_add(reputation)
            .ok_or(error!(IndrasError::Overflow))?;
        // Update last contribution timestamp
        self.last_contribution_at = current_time;
        // SEC-INV-9: Update rate limit timestamp
        self.last_member_added_at = Some(current_time);
        Ok(())
    }

    /// Remove member from group
    /// 
    /// Track A: Simplified - only Owner can remove members
    /// Updates last_contribution_at timestamp
    pub fn remove_member(&mut self, member_pubkey: Pubkey, current_time: i64) -> Result<()> {
        require!(self.status == GroupStatus::Forming || self.status == GroupStatus::Active, IndrasError::InvalidState);
        // Cannot remove leader (must transfer leadership first)
        require!(
            self.leader != member_pubkey,
            IndrasError::InvalidInput
        );
        
        if let Some(index) = self.members.iter().position(|m| m.pubkey == member_pubkey) {
            let member = self.members.remove(index);
            self.total_reputation = self.total_reputation.saturating_sub(member.reputation);
        }
        // Update last contribution timestamp
        self.last_contribution_at = current_time;
        Ok(())
    }

    /// Add contribution from member
    ///
    /// Updates total_contributions and last_contribution_at timestamp
    ///
    /// # Security
    /// - Validates group is active
    /// - Uses checked arithmetic to prevent overflow
    pub fn add_contribution(&mut self, member_pubkey: Pubkey, current_time: i64) -> Result<()> {
        require!(self.status == GroupStatus::Active, IndrasError::InvalidState);
        
        if let Some(member) = self.members.iter_mut().find(|m| m.pubkey == member_pubkey) {
            // SECURITY: Use checked arithmetic to prevent overflow
            member.contributions = member.contributions
                .checked_add(1)
                .ok_or(error!(IndrasError::Overflow))?;
            
            member.reputation = member.reputation
                .checked_add(10) // +10 reputation per contribution
                .ok_or(error!(IndrasError::Overflow))?;
            
            self.total_contributions = self.total_contributions
                .checked_add(1)
                .ok_or(error!(IndrasError::Overflow))?;
            
            self.total_reputation = self.total_reputation
                .checked_add(10)
                .ok_or(error!(IndrasError::Overflow))?;
        }
        // Update last contribution timestamp
        self.last_contribution_at = current_time;
        Ok(())
    }

    // ===== RESOURCE MANAGEMENT METHODS =====

    /// Add idea to group
    ///
    /// # Security
    /// - Validates group status
    /// - Prevents duplicate ideas
    /// - Enforces max_len limit (checked by Anchor, but explicit check for clarity)
    pub fn add_idea(&mut self, idea_id: u64) -> Result<()> {
        require!(self.status == GroupStatus::Forming || self.status == GroupStatus::Active, IndrasError::InvalidState);
        require!(!self.ideas.contains(&idea_id), IndrasError::AlreadyExists);
        
        // SECURITY: Check max_len limit (max_len(10) in struct definition)
        require!(
            self.ideas.len() < 10,
            IndrasError::DataTooLarge
        );
        
        self.ideas.push(idea_id);
        Ok(())
    }

    /// Remove idea from group
    pub fn remove_idea(&mut self, idea_id: u64) -> Result<()> {
        if let Some(index) = self.ideas.iter().position(|&id| id == idea_id) {
            self.ideas.remove(index);
        }
        Ok(())
    }

    /// Add grant to group
    ///
    /// # Security
    /// - Validates group status
    /// - Prevents duplicate grants
    /// - Enforces max_len limit (checked by Anchor, but explicit check for clarity)
    pub fn add_grant(&mut self, grant_id: u64) -> Result<()> {
        require!(self.status == GroupStatus::Forming || self.status == GroupStatus::Active, IndrasError::InvalidState);
        require!(!self.grants.contains(&grant_id), IndrasError::AlreadyExists);
        
        // SECURITY: Check max_len limit (max_len(10) in struct definition)
        require!(
            self.grants.len() < 10,
            IndrasError::DataTooLarge
        );
        
        self.grants.push(grant_id);
        Ok(())
    }

    /// Remove grant from group
    pub fn remove_grant(&mut self, grant_id: u64) -> Result<()> {
        if let Some(index) = self.grants.iter().position(|&id| id == grant_id) {
            self.grants.remove(index);
        }
        Ok(())
    }

    // ===== CRITICAL MOMENT MANAGEMENT (SEC-INV-16) =====
    
    /// Update critical moment status based on active grants
    /// 
    /// SEC-INV-16: Sets is_in_critical_moment = true if group has active grants
    /// Critical moment ends when all grants are completed or expired
    /// NOTE: Proposals are for DAO governance, not mesh groups. Mesh groups work with Ideas and Grants only.
    pub fn update_critical_moment_status(
        &mut self,
        active_grants: &[u64],  // Grant IDs that are pending/active
        current_time: i64,
    ) {
        // Check if group has active grants
        let has_active_grants = active_grants.iter()
            .any(|&grant_id| self.grants.contains(&grant_id));
        
        // Update critical moment status
        self.is_in_critical_moment = has_active_grants;
        
        if self.is_in_critical_moment {
            // Set expiration to max grant end time
            // For MVP, we set a default expiration (7 days from now)
            // In production, this would be computed from actual grant end times
            self.critical_moment_until = Some(current_time + (7 * 24 * 60 * 60)); // 7 days
        } else {
            self.critical_moment_until = None;
        }
    }
    
    // ===== QUERY METHODS =====

    /// Check if pubkey is a member
    pub fn is_member(&self, member_pubkey: Pubkey) -> bool {
        self.members.iter().any(|m| m.pubkey == member_pubkey)
    }

    /// Check if pubkey is the leader
    pub fn is_leader(&self, member_pubkey: Pubkey) -> bool {
        self.leader == member_pubkey
    }

    /// Get member by pubkey
    pub fn get_member(&self, member_pubkey: Pubkey) -> Option<&GroupMember> {
        self.members.iter().find(|m| m.pubkey == member_pubkey)
    }

    /// Get mutable member by pubkey
    pub fn get_member_mut(&mut self, member_pubkey: Pubkey) -> Option<&mut GroupMember> {
        self.members.iter_mut().find(|m| m.pubkey == member_pubkey)
    }

    /// Check if group can start (has enough members)
    pub fn can_start(&self) -> bool {
        self.status == GroupStatus::Forming && self.members.len() >= self.min_members as usize
    }

    /// Check if group is active
    pub fn is_active(&self) -> bool {
        self.status == GroupStatus::Active
    }
    
    /// Check if member has permission to perform action
    /// 
    /// Track A: Simplified - only Owner has permissions
    pub fn has_permission(&self, member: &Pubkey, permission: MeshGroupPermission) -> bool {
        match permission {
            MeshGroupPermission::AddMember => self.leader == *member,
            MeshGroupPermission::RemoveMember => self.leader == *member,
            MeshGroupPermission::PauseGroup => self.leader == *member,
            MeshGroupPermission::ResumeGroup => self.leader == *member,
            MeshGroupPermission::TransferLeadership => self.leader == *member,
            MeshGroupPermission::CreateGrant => {
                // Owner or any member (Track A: simplified)
                self.leader == *member || self.members.iter().any(|m| m.pubkey == *member)
            }
        }
    }
    
    /// Check if group meets contribution threshold for grant eligibility
    pub fn meets_contribution_threshold(&self) -> bool {
        self.total_contributions >= self.protocol.contribution_threshold
    }

    /// Check if group is completed
    pub fn is_completed(&self) -> bool {
        self.status == GroupStatus::Completed
    }

    /// Check if group is disbanded
    pub fn is_disbanded(&self) -> bool {
        self.status == GroupStatus::Disbanded
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use crate::state::mesh_group::types::{GroupType, GroupRole};

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_mesh_group_params(id: u64, leader: Pubkey, created_by: Pubkey) -> MeshGroupParams {
        MeshGroupParams {
            id,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by,
            bump: 255,
        }
    }

    #[test]
    fn test_mesh_group_new() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        
        // Mock Clock::get() by using new_with_time
        let current_time = 1000i64;
        let group = MeshGroup::new_with_time(params.clone(), current_time).unwrap();
        
        assert_eq!(group.id, 1);
        assert_eq!(group.name, "Test Group");
        assert_eq!(group.status, GroupStatus::Forming);
        assert_eq!(group.leader, leader);
        assert_eq!(group.created_by, created_by);
        assert_eq!(group.max_members, 7);
        assert_eq!(group.min_members, 1);
        assert_eq!(group.members.len(), 0); // Members added separately
        assert_eq!(group.created_at, current_time);
    }

    #[test]
    fn test_mesh_group_new_validation() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        
        // Test empty name
        let mut params = create_test_mesh_group_params(1, leader, created_by);
        params.name = "".to_string();
        assert!(MeshGroup::new_with_time(params, 1000).is_err());
        
        // Test max_members > 7
        let mut params = create_test_mesh_group_params(1, leader, created_by);
        params.max_members = 8;
        assert!(MeshGroup::new_with_time(params, 1000).is_err());
        
        // Test min_members > max_members
        let mut params = create_test_mesh_group_params(1, leader, created_by);
        params.min_members = 8;
        assert!(MeshGroup::new_with_time(params, 1000).is_err());
    }

    #[test]
    fn test_mesh_group_add_member() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        let member_pubkey = create_test_pubkey(3);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 100,
            is_active: true,
        };
        
        assert!(group.add_member(member.clone(), 1000).is_ok());
        assert_eq!(group.members.len(), 1);
        assert_eq!(group.total_reputation, 100);
        assert!(group.is_member(member_pubkey));
    }

    #[test]
    fn test_mesh_group_add_member_full() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Add 7 members (max)
        for i in 0..7 {
            let member_pubkey = create_test_pubkey(i + 3);
            let member = GroupMember {
                pubkey: member_pubkey,
                role: GroupRole::Contributor,
                joined_at: 1000,
                contributions: 0,
                reputation: 10,
                is_active: true,
            };
            assert!(group.add_member(member, 1000).is_ok());
        }
        
        // Try to add 8th member - should fail
        let member_pubkey = create_test_pubkey(10);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 10,
            is_active: true,
        };
        assert!(group.add_member(member, 1000).is_err());
    }

    #[test]
    fn test_mesh_group_remove_member() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        let member_pubkey = create_test_pubkey(3);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 100,
            is_active: true,
        };
        
        group.add_member(member, 1000).unwrap();
        assert_eq!(group.total_reputation, 100);
        
        assert!(group.remove_member(member_pubkey, 2000).is_ok());
        assert_eq!(group.members.len(), 0);
        assert_eq!(group.total_reputation, 0);
        assert!(!group.is_member(member_pubkey));
    }

    #[test]
    fn test_mesh_group_cannot_remove_leader() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Try to remove leader - should fail
        assert!(group.remove_member(leader, 2000).is_err());
    }

    #[test]
    fn test_mesh_group_lifecycle() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Add member to meet min_members
        let member = GroupMember {
            pubkey: create_test_pubkey(3),
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        
        // Start group
        assert!(group.start_group_with_time(2000).is_ok());
        assert_eq!(group.status, GroupStatus::Active);
        assert_eq!(group.started_at, Some(2000));
        
        // Pause group
        assert!(group.pause_group().is_ok());
        assert_eq!(group.status, GroupStatus::Paused);
        
        // Resume group
        assert!(group.resume_group().is_ok());
        assert_eq!(group.status, GroupStatus::Active);
        
        // Complete group
        assert!(group.complete_group_with_time(3000).is_ok());
        assert_eq!(group.status, GroupStatus::Completed);
        assert_eq!(group.completed_at, Some(3000));
    }

    #[test]
    fn test_mesh_group_add_idea() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        assert!(group.add_idea(1).is_ok());
        assert_eq!(group.ideas.len(), 1);
        assert!(group.ideas.contains(&1));
        
        // Try to add duplicate - should fail
        assert!(group.add_idea(1).is_err());
    }

    #[test]
    fn test_mesh_group_add_grant() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        assert!(group.add_grant(1).is_ok());
        assert_eq!(group.grants.len(), 1);
        assert!(group.grants.contains(&1));
        
        // Try to add duplicate - should fail
        assert!(group.add_grant(1).is_err());
    }

    #[test]
    fn test_mesh_group_pause_group() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Add leader as member (required for start_group)
        let leader_member = GroupMember {
            pubkey: leader,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(leader_member, 1000).unwrap();
        
        // Start group first
        group.start_group_with_time(2000).unwrap();
        assert_eq!(group.status, GroupStatus::Active);
        
        // Pause group
        assert!(group.pause_group().is_ok());
        assert_eq!(group.status, GroupStatus::Paused);
    }

    #[test]
    fn test_mesh_group_pause_group_invalid_state() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Try to pause non-active group - should fail
        assert!(group.pause_group().is_err());
    }

    #[test]
    fn test_mesh_group_resume_group() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Add leader as member (required for start_group)
        let leader_member = GroupMember {
            pubkey: leader,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(leader_member, 1000).unwrap();
        
        // Start and pause group
        group.start_group_with_time(2000).unwrap();
        group.pause_group().unwrap();
        assert_eq!(group.status, GroupStatus::Paused);
        
        // Resume group
        assert!(group.resume_group().is_ok());
        assert_eq!(group.status, GroupStatus::Active);
    }

    #[test]
    fn test_mesh_group_resume_group_invalid_state() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Try to resume non-paused group - should fail
        assert!(group.resume_group().is_err());
    }

    #[test]
    fn test_mesh_group_complete_group() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Add leader as member (required for start_group)
        let leader_member = GroupMember {
            pubkey: leader,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(leader_member, 1000).unwrap();
        
        // Start group first
        group.start_group_with_time(2000).unwrap();
        assert_eq!(group.status, GroupStatus::Active);
        
        // Complete group
        assert!(group.complete_group_with_time(3000).is_ok());
        assert_eq!(group.status, GroupStatus::Completed);
        assert_eq!(group.completed_at, Some(3000));
    }

    #[test]
    fn test_mesh_group_complete_group_invalid_state() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Try to complete non-active group - should fail
        assert!(group.complete_group_with_time(3000).is_err());
    }

    #[test]
    fn test_mesh_group_disband_group() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Disband group from any status
        assert!(group.disband_group_with_time(3000).is_ok());
        assert_eq!(group.status, GroupStatus::Disbanded);
        assert_eq!(group.completed_at, Some(3000));
    }

    #[test]
    fn test_mesh_group_disband_group_already_disbanded() {
        let leader = create_test_pubkey(1);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by: leader,
            bump: 255,
        };
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Disband once
        group.disband_group_with_time(3000).unwrap();
        
        // Try to disband again - should fail
        assert!(group.disband_group_with_time(4000).is_err());
    }

    #[test]
    fn test_mesh_group_add_contribution() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, created_by);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        let member_pubkey = create_test_pubkey(3);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        group.status = GroupStatus::Active;
        
        assert!(group.add_contribution(member_pubkey, 2000).is_ok());
        assert_eq!(group.total_contributions, 1);
        assert_eq!(group.total_reputation, 10);
        
        let member = group.get_member(member_pubkey).unwrap();
        assert_eq!(member.contributions, 1);
        assert_eq!(member.reputation, 10);
    }

    #[test]
    fn test_mesh_group_is_member() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        let member_pubkey = create_test_pubkey(2);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        
        assert!(group.is_member(member_pubkey));
        assert!(!group.is_member(create_test_pubkey(3)));
    }

    #[test]
    fn test_mesh_group_is_leader() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        assert!(group.is_leader(leader));
        assert!(!group.is_leader(create_test_pubkey(2)));
    }

    #[test]
    fn test_mesh_group_get_member() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        let member_pubkey = create_test_pubkey(2);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member.clone(), 1000).unwrap();
        
        let retrieved = group.get_member(member_pubkey);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().pubkey, member_pubkey);
        
        assert!(group.get_member(create_test_pubkey(3)).is_none());
    }

    #[test]
    fn test_mesh_group_can_start() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Initially cannot start (no members)
        assert!(!group.can_start());
        
        // Add member
        let member = GroupMember {
            pubkey: leader,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        
        // Now can start
        assert!(group.can_start());
        
        // After starting, cannot start again
        group.start_group_with_time(2000).unwrap();
        assert!(!group.can_start());
    }

    #[test]
    fn test_mesh_group_is_active() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        assert!(!group.is_active());
        
        let member = GroupMember {
            pubkey: leader,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        group.start_group_with_time(2000).unwrap();
        
        assert!(group.is_active());
    }

    #[test]
    fn test_mesh_group_has_permission() {
        let leader = create_test_pubkey(1);
        let member_pubkey = create_test_pubkey(2);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        
        // Leader has all permissions
        assert!(group.has_permission(&leader, MeshGroupPermission::AddMember));
        assert!(group.has_permission(&leader, MeshGroupPermission::RemoveMember));
        assert!(group.has_permission(&leader, MeshGroupPermission::PauseGroup));
        
        // Member has only CreateGrant permission
        assert!(!group.has_permission(&member_pubkey, MeshGroupPermission::AddMember));
        assert!(group.has_permission(&member_pubkey, MeshGroupPermission::CreateGrant));
    }

    #[test]
    fn test_mesh_group_meets_contribution_threshold() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        // Initially doesn't meet threshold
        assert!(!group.meets_contribution_threshold());
        
        // Add contributions
        let member_pubkey = create_test_pubkey(2);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Contributor,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        group.status = GroupStatus::Active;
        
        // Add enough contributions to meet threshold
        let threshold = group.protocol.contribution_threshold;
        for _ in 0..threshold {
            group.add_contribution(member_pubkey, 2000).unwrap();
        }
        
        assert!(group.meets_contribution_threshold());
    }

    #[test]
    fn test_mesh_group_is_completed() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        assert!(!group.is_completed());
        
        let member = GroupMember {
            pubkey: leader,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        group.start_group_with_time(2000).unwrap();
        group.complete_group_with_time(3000).unwrap();
        
        assert!(group.is_completed());
    }

    #[test]
    fn test_mesh_group_is_disbanded() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        assert!(!group.is_disbanded());
        
        group.disband_group_with_time(2000).unwrap();
        
        assert!(group.is_disbanded());
    }

    #[test]
    fn test_mesh_group_remove_idea() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        group.add_idea(1).unwrap();
        assert_eq!(group.ideas.len(), 1);
        
        group.remove_idea(1).unwrap();
        assert_eq!(group.ideas.len(), 0);
    }

    #[test]
    fn test_mesh_group_remove_grant() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        group.add_grant(1).unwrap();
        assert_eq!(group.grants.len(), 1);
        
        group.remove_grant(1).unwrap();
        assert_eq!(group.grants.len(), 0);
    }

    #[test]
    fn test_mesh_group_new_with_time_validation_empty_name() {
        let leader = create_test_pubkey(1);
        let mut params = create_test_mesh_group_params(1, leader, leader);
        params.name = String::new();
        
        assert!(MeshGroup::new_with_time(params, 1000).is_err());
    }

    #[test]
    fn test_mesh_group_new_with_time_validation_empty_description() {
        let leader = create_test_pubkey(1);
        let mut params = create_test_mesh_group_params(1, leader, leader);
        params.description = String::new();
        
        assert!(MeshGroup::new_with_time(params, 1000).is_err());
    }

    #[test]
    fn test_mesh_group_new_with_time_validation_zero_max_members() {
        let leader = create_test_pubkey(1);
        let mut params = create_test_mesh_group_params(1, leader, leader);
        params.max_members = 0;
        
        assert!(MeshGroup::new_with_time(params, 1000).is_err());
    }

    #[test]
    fn test_mesh_group_new_with_time_all_fields() {
        let leader = create_test_pubkey(5);
        let created_by = create_test_pubkey(6);
        let params = MeshGroupParams {
            id: 999,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by,
            bump: 128,
        };
        let group = MeshGroup::new_with_time(params, 5000).unwrap();
        
        assert_eq!(group.id, 999);
        assert_eq!(group.name, "Test Group");
        assert_eq!(group.description, "Test Description");
        assert_eq!(group.group_type, GroupType::Development);
        assert_eq!(group.status, GroupStatus::Forming);
        assert_eq!(group.leader, leader);
        assert_eq!(group.max_members, 7);
        assert_eq!(group.min_members, 1);
        assert_eq!(group.created_by, created_by);
        assert_eq!(group.created_at, 5000);
        assert_eq!(group.bump, 128);
        assert_eq!(group.members.len(), 0);
        assert_eq!(group.ideas.len(), 0);
        assert_eq!(group.grants.len(), 0);
    }

    #[test]
    fn test_mesh_group_is_completed_not_started() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        assert!(!group.is_completed());
    }

    #[test]
    fn test_mesh_group_is_completed_active() {
        let leader = create_test_pubkey(1);
        let params = create_test_mesh_group_params(1, leader, leader);
        let mut group = MeshGroup::new_with_time(params, 1000).unwrap();
        
        let member = GroupMember {
            pubkey: leader,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 0,
            reputation: 0,
            is_active: true,
        };
        group.add_member(member, 1000).unwrap();
        group.start_group_with_time(2000).unwrap();
        
        assert!(!group.is_completed());
    }
}
