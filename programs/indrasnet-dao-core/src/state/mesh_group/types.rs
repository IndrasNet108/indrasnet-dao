//! MeshGroup types and enums
//!
//! Types for mesh groups: GroupType, GroupStatus, GroupRole, DevelopmentStage, GroupMember, Milestone

use anchor_lang::prelude::*;
use anchor_lang::Space;

/// Mesh group status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum GroupStatus {
    Forming,        // Forming
    Active,         // Active
    Paused,         // Paused
    Completed,      // Completed
    Disbanded,      // Disbanded
}

impl Space for GroupStatus {
    const INIT_SPACE: usize = 1;
}

/// Member role in group
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum GroupRole {
    Leader,         // Group leader
    Developer,      // Developer
    Designer,       // Designer
    Tester,         // Tester
    Advisor,        // Advisor
    Contributor,    // Contributor
}

impl Space for GroupRole {
    const INIT_SPACE: usize = 1;
}

/// Mesh group type
/// NOTE: NO "Sandbox" - only mesh groups exist
/// Mesh group can have 1-7 members (if more needed, create additional mesh group up to 7 members and so on)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum GroupType {
    Development,    // Development
    Research,       // Research
    Marketing,      // Marketing
    Community,      // Community
    Education,      // Education
    Other,          // Other
}

impl Space for GroupType {
    const INIT_SPACE: usize = 1;
}

impl GroupType {
    /// Get maximum number of members based on group type
    /// Mesh group can have 1-7 members (if more needed, create additional mesh group)
    pub fn get_max_members(&self) -> u8 {
        // All mesh groups have maximum 7 members
        // If more needed, create additional mesh group
        7
    }
    
    /// Get maximum stage duration in weeks
    pub fn get_max_duration_weeks(&self) -> u8 {
        // Default 12 weeks for mesh groups
        12
    }
}

/// Mesh group development stages
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum DevelopmentStage {
    Planning,           // Planning
    InitialDevelopment, // Initial development (MVP, prototype)
    CoreDevelopment,    // Core development
    Finalization,       // Finalization (polish, documentation)
    Completed,          // Completed
}

impl Space for DevelopmentStage {
    const INIT_SPACE: usize = 1;
}

/// Milestone status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum MilestoneStatus {
    Pending,        // Waiting for work to start
    InProgress,     // In progress
    Completed,      // Completed by group
    Verified,       // Verified by DAO/AI
    Rejected,       // Rejected during verification
}

impl Space for MilestoneStatus {
    const INIT_SPACE: usize = 1;
}

/// Milestone for tracking development progress
/// Note: This is a data structure (not Account), stored separately
/// NOTE: max_len does not work for non-Account structures, validation must be in code
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct Milestone {
    pub id: u64,
    pub title: String,              // Max 100 characters (validation in code)
    pub description: String,         // Max 500 characters (validation in code)
    // NOTE: Vec<String> does not support max_len directly
    // Size validation must be in code (max 10 elements, each up to 100 characters)
    pub deliverables: Vec<String>,   // Max 10 elements, each up to 100 characters
    pub grant_amount: u64,          // Grant amount for this milestone
    pub status: MilestoneStatus,
    pub stage: DevelopmentStage,     // Which stage this milestone is in
    pub completed_at: Option<i64>,
    pub verified_by: Option<Pubkey>,
    pub verification_notes: Option<String>, // Max 500 characters (validation in code)
}

/// Group member
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct GroupMember {
    pub pubkey: Pubkey,
    pub role: GroupRole,
    pub joined_at: i64,
    pub contributions: u32,
    pub reputation: u64,
    pub is_active: bool,
}

impl Space for GroupMember {
    const INIT_SPACE: usize = 32 + 1 + 8 + 4 + 8 + 1; // pubkey + role + joined_at + contributions + reputation + is_active
}

/// Parameters for creating mesh group
#[derive(Clone, Debug)]
pub struct MeshGroupParams {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub group_type: GroupType,
    pub leader: Pubkey,
    pub max_members: u8,
    pub min_members: u8,
    pub created_by: Pubkey,
    pub bump: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_status_variants() {
        assert_eq!(GroupStatus::Forming, GroupStatus::Forming);
        assert_eq!(GroupStatus::Active, GroupStatus::Active);
        assert_eq!(GroupStatus::Paused, GroupStatus::Paused);
        assert_eq!(GroupStatus::Completed, GroupStatus::Completed);
        assert_eq!(GroupStatus::Disbanded, GroupStatus::Disbanded);
    }

    #[test]
    fn test_group_role_variants() {
        assert_eq!(GroupRole::Leader, GroupRole::Leader);
        assert_eq!(GroupRole::Developer, GroupRole::Developer);
        assert_eq!(GroupRole::Designer, GroupRole::Designer);
        assert_eq!(GroupRole::Tester, GroupRole::Tester);
        assert_eq!(GroupRole::Advisor, GroupRole::Advisor);
        assert_eq!(GroupRole::Contributor, GroupRole::Contributor);
    }

    #[test]
    fn test_group_type_variants() {
        assert_eq!(GroupType::Development, GroupType::Development);
        assert_eq!(GroupType::Research, GroupType::Research);
        assert_eq!(GroupType::Marketing, GroupType::Marketing);
        assert_eq!(GroupType::Community, GroupType::Community);
        assert_eq!(GroupType::Education, GroupType::Education);
        assert_eq!(GroupType::Other, GroupType::Other);
    }

    #[test]
    fn test_group_type_get_max_members() {
        // All group types have max 7 members
        assert_eq!(GroupType::Development.get_max_members(), 7);
        assert_eq!(GroupType::Research.get_max_members(), 7);
        assert_eq!(GroupType::Marketing.get_max_members(), 7);
        assert_eq!(GroupType::Community.get_max_members(), 7);
        assert_eq!(GroupType::Education.get_max_members(), 7);
        assert_eq!(GroupType::Other.get_max_members(), 7);
    }

    #[test]
    fn test_group_type_get_max_duration_weeks() {
        // All group types have max 12 weeks duration
        assert_eq!(GroupType::Development.get_max_duration_weeks(), 12);
        assert_eq!(GroupType::Research.get_max_duration_weeks(), 12);
        assert_eq!(GroupType::Marketing.get_max_duration_weeks(), 12);
    }

    #[test]
    fn test_development_stage_variants() {
        assert_eq!(DevelopmentStage::Planning, DevelopmentStage::Planning);
        assert_eq!(DevelopmentStage::InitialDevelopment, DevelopmentStage::InitialDevelopment);
        assert_eq!(DevelopmentStage::CoreDevelopment, DevelopmentStage::CoreDevelopment);
        assert_eq!(DevelopmentStage::Finalization, DevelopmentStage::Finalization);
        assert_eq!(DevelopmentStage::Completed, DevelopmentStage::Completed);
    }

    #[test]
    fn test_milestone_status_variants() {
        assert_eq!(MilestoneStatus::Pending, MilestoneStatus::Pending);
        assert_eq!(MilestoneStatus::InProgress, MilestoneStatus::InProgress);
        assert_eq!(MilestoneStatus::Completed, MilestoneStatus::Completed);
        assert_eq!(MilestoneStatus::Verified, MilestoneStatus::Verified);
        assert_eq!(MilestoneStatus::Rejected, MilestoneStatus::Rejected);
    }

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_group_member_structure() {
        let member_pubkey = create_test_pubkey(1);
        let member = GroupMember {
            pubkey: member_pubkey,
            role: GroupRole::Leader,
            joined_at: 1000,
            contributions: 10,
            reputation: 500,
            is_active: true,
        };
        
        assert_eq!(member.pubkey, member_pubkey);
        assert_eq!(member.role, GroupRole::Leader);
        assert_eq!(member.joined_at, 1000);
        assert_eq!(member.contributions, 10);
        assert_eq!(member.reputation, 500);
        assert!(member.is_active);
    }

    #[test]
    fn test_group_member_all_roles() {
        let roles = vec![
            GroupRole::Leader,
            GroupRole::Developer,
            GroupRole::Designer,
            GroupRole::Tester,
            GroupRole::Advisor,
            GroupRole::Contributor,
        ];
        
        for role in &roles {
            let member = GroupMember {
                pubkey: create_test_pubkey(1),
                role: role.clone(),
                joined_at: 1000,
                contributions: 0,
                reputation: 0,
                is_active: true,
            };
            assert_eq!(member.role, *role);
        }
    }

    #[test]
    fn test_milestone_structure() {
        let verified_by = create_test_pubkey(10);
        let milestone = Milestone {
            id: 1,
            title: "Test Milestone".to_string(),
            description: "Test Description".to_string(),
            deliverables: vec!["deliverable1".to_string(), "deliverable2".to_string()],
            grant_amount: 1000,
            status: MilestoneStatus::Verified,
            stage: DevelopmentStage::CoreDevelopment,
            completed_at: Some(2000),
            verified_by: Some(verified_by),
            verification_notes: Some("Verified".to_string()),
        };
        
        assert_eq!(milestone.id, 1);
        assert_eq!(milestone.title, "Test Milestone");
        assert_eq!(milestone.deliverables.len(), 2);
        assert_eq!(milestone.grant_amount, 1000);
        assert_eq!(milestone.status, MilestoneStatus::Verified);
        assert_eq!(milestone.stage, DevelopmentStage::CoreDevelopment);
        assert_eq!(milestone.completed_at, Some(2000));
        assert_eq!(milestone.verified_by, Some(verified_by));
    }

    #[test]
    fn test_milestone_all_statuses() {
        let statuses = vec![
            MilestoneStatus::Pending,
            MilestoneStatus::InProgress,
            MilestoneStatus::Completed,
            MilestoneStatus::Verified,
            MilestoneStatus::Rejected,
        ];
        
        for status in &statuses {
            let milestone = Milestone {
                id: 1,
                title: "Test".to_string(),
                description: "Test".to_string(),
                deliverables: vec![],
                grant_amount: 0,
                status: status.clone(),
                stage: DevelopmentStage::Planning,
                completed_at: None,
                verified_by: None,
                verification_notes: None,
            };
            assert_eq!(milestone.status, *status);
        }
    }

    #[test]
    fn test_milestone_all_stages() {
        let stages = vec![
            DevelopmentStage::Planning,
            DevelopmentStage::InitialDevelopment,
            DevelopmentStage::CoreDevelopment,
            DevelopmentStage::Finalization,
            DevelopmentStage::Completed,
        ];
        
        for stage in &stages {
            let milestone = Milestone {
                id: 1,
                title: "Test".to_string(),
                description: "Test".to_string(),
                deliverables: vec![],
                grant_amount: 0,
                status: MilestoneStatus::Pending,
                stage: stage.clone(),
                completed_at: None,
                verified_by: None,
                verification_notes: None,
            };
            assert_eq!(milestone.stage, *stage);
        }
    }

    #[test]
    fn test_mesh_group_params_structure() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = MeshGroupParams {
            id: 1,
            name: "Test Group".to_string(),
            description: "Test Description".to_string(),
            group_type: GroupType::Development,
            leader,
            max_members: 7,
            min_members: 1,
            created_by,
            bump: 255,
        };
        
        assert_eq!(params.id, 1);
        assert_eq!(params.name, "Test Group");
        assert_eq!(params.group_type, GroupType::Development);
        assert_eq!(params.leader, leader);
        assert_eq!(params.max_members, 7);
        assert_eq!(params.min_members, 1);
        assert_eq!(params.created_by, created_by);
        assert_eq!(params.bump, 255);
    }

    #[test]
    fn test_group_status_equality() {
        assert_eq!(GroupStatus::Forming, GroupStatus::Forming);
        assert_ne!(GroupStatus::Forming, GroupStatus::Active);
        assert_eq!(GroupStatus::Active, GroupStatus::Active);
        assert_ne!(GroupStatus::Active, GroupStatus::Paused);
    }

    #[test]
    fn test_group_role_equality() {
        assert_eq!(GroupRole::Leader, GroupRole::Leader);
        assert_ne!(GroupRole::Leader, GroupRole::Developer);
        assert_eq!(GroupRole::Developer, GroupRole::Developer);
    }

    #[test]
    fn test_group_type_equality() {
        assert_eq!(GroupType::Development, GroupType::Development);
        assert_ne!(GroupType::Development, GroupType::Research);
        assert_eq!(GroupType::Research, GroupType::Research);
    }

    #[test]
    fn test_development_stage_equality() {
        assert_eq!(DevelopmentStage::Planning, DevelopmentStage::Planning);
        assert_ne!(DevelopmentStage::Planning, DevelopmentStage::InitialDevelopment);
        assert_eq!(DevelopmentStage::Completed, DevelopmentStage::Completed);
    }

    #[test]
    fn test_milestone_status_equality() {
        assert_eq!(MilestoneStatus::Pending, MilestoneStatus::Pending);
        assert_ne!(MilestoneStatus::Pending, MilestoneStatus::InProgress);
        assert_eq!(MilestoneStatus::Verified, MilestoneStatus::Verified);
    }

    #[test]
    fn test_group_status_all_variants_unique() {
        let statuses = vec![
            GroupStatus::Forming,
            GroupStatus::Active,
            GroupStatus::Paused,
            GroupStatus::Completed,
            GroupStatus::Disbanded,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_group_role_all_variants_unique() {
        let roles = vec![
            GroupRole::Leader,
            GroupRole::Developer,
            GroupRole::Designer,
            GroupRole::Tester,
            GroupRole::Advisor,
            GroupRole::Contributor,
        ];
        
        for i in 0..roles.len() {
            for j in (i + 1)..roles.len() {
                assert_ne!(roles[i], roles[j], "Duplicate role found");
            }
        }
    }

    #[test]
    fn test_group_type_all_variants_unique() {
        let types = vec![
            GroupType::Development,
            GroupType::Research,
            GroupType::Marketing,
            GroupType::Community,
            GroupType::Education,
            GroupType::Other,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_development_stage_all_variants_unique() {
        let stages = vec![
            DevelopmentStage::Planning,
            DevelopmentStage::InitialDevelopment,
            DevelopmentStage::CoreDevelopment,
            DevelopmentStage::Finalization,
            DevelopmentStage::Completed,
        ];
        
        for i in 0..stages.len() {
            for j in (i + 1)..stages.len() {
                assert_ne!(stages[i], stages[j], "Duplicate stage found");
            }
        }
    }

    #[test]
    fn test_milestone_status_all_variants_unique() {
        let statuses = vec![
            MilestoneStatus::Pending,
            MilestoneStatus::InProgress,
            MilestoneStatus::Completed,
            MilestoneStatus::Verified,
            MilestoneStatus::Rejected,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_group_type_get_max_members_all_types() {
        let types = vec![
            GroupType::Development,
            GroupType::Research,
            GroupType::Marketing,
            GroupType::Community,
            GroupType::Education,
            GroupType::Other,
        ];
        
        for group_type in &types {
            assert_eq!(group_type.get_max_members(), 7);
        }
    }

    #[test]
    fn test_group_type_get_max_duration_weeks_all_types() {
        let types = vec![
            GroupType::Development,
            GroupType::Research,
            GroupType::Marketing,
            GroupType::Community,
            GroupType::Education,
            GroupType::Other,
        ];
        
        for group_type in &types {
            assert_eq!(group_type.get_max_duration_weeks(), 12);
        }
    }

    #[test]
    fn test_group_member_space() {
        assert_eq!(<GroupMember as Space>::INIT_SPACE, 32 + 1 + 8 + 4 + 8 + 1);
    }

    #[test]
    fn test_group_status_space() {
        assert_eq!(<GroupStatus as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_group_role_space() {
        assert_eq!(<GroupRole as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_group_type_space() {
        assert_eq!(<GroupType as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_development_stage_space() {
        assert_eq!(<DevelopmentStage as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_milestone_status_space() {
        assert_eq!(<MilestoneStatus as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_group_member_inactive() {
        let member = GroupMember {
            pubkey: create_test_pubkey(1),
            role: GroupRole::Developer,
            joined_at: 1000,
            contributions: 5,
            reputation: 200,
            is_active: false,
        };
        
        assert!(!member.is_active);
    }

    #[test]
    fn test_milestone_without_optional_fields() {
        let milestone = Milestone {
            id: 1,
            title: "Test".to_string(),
            description: "Test".to_string(),
            deliverables: vec![],
            grant_amount: 0,
            status: MilestoneStatus::Pending,
            stage: DevelopmentStage::Planning,
            completed_at: None,
            verified_by: None,
            verification_notes: None,
        };
        
        assert_eq!(milestone.completed_at, None);
        assert_eq!(milestone.verified_by, None);
        assert_eq!(milestone.verification_notes, None);
    }

    #[test]
    fn test_mesh_group_params_all_fields() {
        let leader = create_test_pubkey(1);
        let created_by = create_test_pubkey(2);
        let params = MeshGroupParams {
            id: 999,
            name: "Advanced Group".to_string(),
            description: "Advanced Description".to_string(),
            group_type: GroupType::Research,
            leader,
            max_members: 7,
            min_members: 2,
            created_by,
            bump: 128,
        };
        
        assert_eq!(params.id, 999);
        assert_eq!(params.name, "Advanced Group");
        assert_eq!(params.description, "Advanced Description");
        assert_eq!(params.group_type, GroupType::Research);
        assert_eq!(params.leader, leader);
        assert_eq!(params.max_members, 7);
        assert_eq!(params.min_members, 2);
        assert_eq!(params.created_by, created_by);
        assert_eq!(params.bump, 128);
    }
}
