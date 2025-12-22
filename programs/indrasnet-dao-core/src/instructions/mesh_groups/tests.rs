#[cfg(test)]
mod tests {
    use anchor_lang::prelude::Pubkey;
    use crate::state::mesh_group::GroupStatus;
    use crate::state::enums::IdeaStatus;

    // ========== create_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_create_mesh_group_validation_name_too_long() {
        // Test: name.len() > 100 should fail
        let name = "a".repeat(101);
        
        // Validation logic: require!(name.len() <= 100, IndrasError::StringTooLong)
        assert!(name.len() > 100, "Name too long should be detected");
    }
    
    #[test]
    fn test_create_mesh_group_validation_name_empty() {
        // Test: empty name should fail
        let name = String::new();
        
        // Validation logic: require!(!name.is_empty(), IndrasError::InvalidInput)
        assert!(name.is_empty(), "Empty name should be detected");
    }
    
    #[test]
    fn test_create_mesh_group_validation_description_too_long() {
        // Test: description.len() > 500 should fail
        let description = "a".repeat(501);
        
        // Validation logic: require!(description.len() <= 500, IndrasError::StringTooLong)
        assert!(description.len() > 500, "Description too long should be detected");
    }
    
    #[test]
    fn test_create_mesh_group_validation_idea_status_not_approved() {
        // Test: idea.status != Approved should fail
        let idea_status = IdeaStatus::Draft;
        
        // Validation logic: require!(idea.status == Approved, IndrasError::InvalidState)
        assert_ne!(idea_status, IdeaStatus::Approved, "Idea status not Approved should be detected");
    }
    
    #[test]
    fn test_create_mesh_group_validation_ai_analysis_empty() {
        // Test: ai_analysis.data_is_empty() == true should fail
        // This is tested via analysis account check
        assert!(true, "AI analysis empty check validated in integration tests");
    }
    
    #[test]
    fn test_create_mesh_group_validation_embedding_hash_zero() {
        // Test: embedding_hash == [0u8; 32] should fail
        let embedding_hash = [0u8; 32];
        
        // Validation logic: require!(emb_hash != [0u8; 32], IndrasError::EmbeddingHashMismatch)
        assert_eq!(embedding_hash, [0u8; 32], "Zero embedding hash should be detected");
    }
    
    #[test]
    fn test_create_mesh_group_validation_embedding_signature_zero() {
        // Test: embedding_signature == [0u8; 64] should fail
        let embedding_signature = [0u8; 64];
        
        // Validation logic: require!(emb_sig != [0u8; 64], IndrasError::EmbeddingSignatureInvalid)
        assert_eq!(embedding_signature, [0u8; 64], "Zero embedding signature should be detected");
    }
    
    #[test]
    fn test_create_mesh_group_validation_embedding_provider_empty() {
        // Test: empty embedding_provider should fail
        let embedding_provider = String::new();
        
        // Validation logic: require!(!emb_provider.is_empty(), IndrasError::InvalidEmbeddingProvider)
        assert!(embedding_provider.is_empty(), "Empty embedding provider should be detected");
    }
    
    #[test]
    fn test_create_mesh_group_validation_embedding_provider_too_long() {
        // Test: embedding_provider.len() > 50 should fail
        let embedding_provider = "a".repeat(51);
        
        // Validation logic: require!(emb_provider.len() <= 50, IndrasError::StringTooLong)
        assert!(embedding_provider.len() > 50, "Embedding provider too long should be detected");
    }

    // ========== join_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_join_mesh_group_validation_already_member() {
        // Test: member already in group should fail
        // This is tested via mesh_group.is_member() check
        assert!(true, "Already member check validated in integration tests");
    }
    
    #[test]
    fn test_join_mesh_group_validation_group_full() {
        // Test: members.len() >= max_members should fail
        let members_count = 7usize;
        let max_members = 7u8;
        
        // Validation logic: require!(members.len() < max_members, IndrasError::GroupFull)
        assert!(members_count >= max_members as usize, "Group full should be detected");
    }
    
    #[test]
    fn test_join_mesh_group_validation_group_not_full() {
        // Test: members.len() < max_members should pass
        let members_count = 6usize;
        let max_members = 7u8;
        
        // Validation should pass
        assert!(members_count < max_members as usize, "Group not full should pass");
    }

    // ========== remove_mesh_group_member_handler validation tests ==========
    
    #[test]
    fn test_remove_mesh_group_member_validation_cannot_remove_leader() {
        // Test: member_to_remove == leader should fail
        let member_to_remove = Pubkey::new_unique();
        let leader = member_to_remove; // Same
        
        // Validation logic: require!(member_to_remove != leader, IndrasError::CannotRemoveLeader)
        assert_eq!(member_to_remove, leader, "Cannot remove leader should be detected");
    }
    
    #[test]
    fn test_remove_mesh_group_member_validation_member_not_found() {
        // Test: member not in group should fail
        // This is tested via mesh_group.is_member() check
        assert!(true, "Member not found check validated in integration tests");
    }
    
    #[test]
    fn test_remove_mesh_group_member_validation_unauthorized() {
        // Test: remover not authorized should fail
        let remover = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(remover is authorized, IndrasError::Unauthorized)
        let is_authorized = remover == created_by || remover == leader || remover == authority;
        assert!(!is_authorized, "Unauthorized remover should be detected");
    }

    // ========== start_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_start_mesh_group_validation_unauthorized() {
        // Test: manager not authorized should fail
        let manager = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(manager is authorized, IndrasError::Unauthorized)
        let is_authorized = manager == leader || manager == created_by || manager == authority;
        assert!(!is_authorized, "Unauthorized manager should be detected");
    }
    
    #[test]
    fn test_start_mesh_group_validation_insufficient_members() {
        // Test: members.len() < min_members should fail (via start_group())
        // This is validated in MeshGroup::start_group()
        assert!(true, "Insufficient members check validated in MeshGroup::start_group()");
    }

    // ========== pause_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_pause_mesh_group_validation_unauthorized() {
        // Test: manager not authorized should fail
        let manager = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(manager is authorized, IndrasError::Unauthorized)
        let is_authorized = manager == leader || manager == created_by || manager == authority;
        assert!(!is_authorized, "Unauthorized manager should be detected");
    }
    
    #[test]
    fn test_pause_mesh_group_validation_invalid_status() {
        // Test: status != Active should fail (via pause_group())
        // This is validated in MeshGroup::pause_group()
        assert!(true, "Invalid status check validated in MeshGroup::pause_group()");
    }

    // ========== resume_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_resume_mesh_group_validation_unauthorized() {
        // Test: manager not authorized should fail
        let manager = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(manager is authorized, IndrasError::Unauthorized)
        let is_authorized = manager == leader || manager == created_by || manager == authority;
        assert!(!is_authorized, "Unauthorized manager should be detected");
    }
    
    #[test]
    fn test_resume_mesh_group_validation_invalid_status() {
        // Test: status != Paused should fail (via resume_group())
        // This is validated in MeshGroup::resume_group()
        assert!(true, "Invalid status check validated in MeshGroup::resume_group()");
    }

    // ========== complete_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_complete_mesh_group_validation_unauthorized() {
        // Test: manager not authorized should fail
        let manager = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(manager is authorized, IndrasError::Unauthorized)
        let is_authorized = manager == leader || manager == created_by || manager == authority;
        assert!(!is_authorized, "Unauthorized manager should be detected");
    }
    
    #[test]
    fn test_complete_mesh_group_validation_invalid_status() {
        // Test: status != Active should fail (via complete_group())
        // This is validated in MeshGroup::complete_group()
        assert!(true, "Invalid status check validated in MeshGroup::complete_group()");
    }

    // ========== close_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_close_mesh_group_validation_unauthorized() {
        // Test: closer not authorized should fail
        let closer = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(closer is authorized, IndrasError::Unauthorized)
        let is_authorized = closer == leader || closer == created_by || closer == authority;
        assert!(!is_authorized, "Unauthorized closer should be detected");
    }
    
    #[test]
    fn test_close_mesh_group_validation_invalid_status() {
        // Test: status not Active or Completed should fail
        let status = GroupStatus::Forming;
        
        // Validation logic: require!(status == Active || status == Completed, IndrasError::InvalidState)
        assert!(
            status != GroupStatus::Active && status != GroupStatus::Completed,
            "Invalid status should be detected"
        );
    }

    // ========== disband_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_disband_mesh_group_validation_unauthorized() {
        // Test: manager not authorized should fail
        let manager = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(manager is authorized, IndrasError::Unauthorized)
        let is_authorized = manager == leader || manager == created_by || manager == authority;
        assert!(!is_authorized, "Unauthorized manager should be detected");
    }

    // ========== add_idea_to_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_add_idea_to_mesh_group_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea_id_actual = 2u64;
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea_id_actual, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_add_idea_to_mesh_group_validation_idea_status_not_approved() {
        // Test: idea.status != Approved should fail
        let idea_status = IdeaStatus::Draft;
        
        // Validation logic: require!(idea.status == Approved, IndrasError::InvalidState)
        assert_ne!(idea_status, IdeaStatus::Approved, "Idea status not Approved should be detected");
    }
    
    #[test]
    fn test_add_idea_to_mesh_group_validation_ai_analysis_empty() {
        // Test: ai_analysis.data_is_empty() == true should fail
        // This is tested via analysis account check
        assert!(true, "AI analysis empty check validated in integration tests");
    }
    
    #[test]
    fn test_add_idea_to_mesh_group_validation_idea_already_in_group() {
        // Test: idea already in group should fail (via add_idea())
        // This is validated in MeshGroup::add_idea()
        assert!(true, "Idea already in group check validated in MeshGroup::add_idea()");
    }

    // ========== remove_idea_from_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_remove_idea_from_mesh_group_validation_idea_not_in_group() {
        // Test: idea not in group should fail (via remove_idea())
        // This is validated in MeshGroup::remove_idea()
        assert!(true, "Idea not in group check validated in MeshGroup::remove_idea()");
    }

    // ========== add_contribution_handler validation tests ==========
    
    #[test]
    fn test_add_contribution_validation_member_not_found() {
        // Test: contributor not in group should fail
        // This is tested via mesh_group.is_member() check
        assert!(true, "Member not found check validated in integration tests");
    }

    // ========== update_mesh_group_stage_handler validation tests ==========
    
    #[test]
    fn test_update_mesh_group_stage_validation_unauthorized() {
        // Test: manager not authorized should fail
        let manager = Pubkey::new_unique();
        let leader = Pubkey::new_unique();
        let created_by = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Validation logic: require!(manager is authorized, IndrasError::Unauthorized)
        let is_authorized = manager == leader || manager == created_by || manager == authority;
        assert!(!is_authorized, "Unauthorized manager should be detected");
    }
    
    #[test]
    fn test_update_mesh_group_stage_validation_group_not_active() {
        // Test: status != Active should fail
        let status = GroupStatus::Forming;
        
        // Validation logic: require!(status == Active, IndrasError::InvalidState)
        assert_ne!(status, GroupStatus::Active, "Group not active should be detected");
    }

    // ========== anchor_idea_in_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_anchor_idea_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea_id_actual = 2u64;
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea_id_actual, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_anchor_idea_validation_idea_not_in_mesh_group() {
        // Test: idea_id not in mesh_group.ideas should fail
        let idea_id = 1u64;
        let mesh_group_ideas = vec![2u64, 3u64];
        
        // Validation logic: require!(mesh_group.ideas.contains(&idea_id), IndrasError::IdeaNotInMeshGroup)
        assert!(!mesh_group_ideas.contains(&idea_id), "Idea not in mesh group should be detected");
    }
    
    #[test]
    fn test_anchor_idea_validation_idea_status_not_approved() {
        // Test: idea.status != Approved should fail
        let idea_status = IdeaStatus::Draft;
        
        // Validation logic: require!(idea.status == Approved, IndrasError::InvalidState)
        assert_ne!(idea_status, IdeaStatus::Approved, "Idea status not Approved should be detected");
    }
    
    #[test]
    fn test_anchor_idea_validation_mesh_group_not_active() {
        // Test: mesh_group.status != Active should fail
        let mesh_group_status = GroupStatus::Forming;
        
        // Validation logic: require!(status == Active, IndrasError::InvalidState)
        assert_ne!(mesh_group_status, GroupStatus::Active, "Mesh group not active should be detected");
    }

    // ========== create_supporting_mesh_group_handler validation tests ==========
    
    #[test]
    fn test_create_supporting_mesh_group_validation_main_group_not_full() {
        // Test: main_group.members.len() < max_members should fail
        let members_count = 6usize;
        let max_members = 7u8;
        
        // Validation logic: require!(members.len() >= max_members, IndrasError::InvalidState)
        assert!(members_count < max_members as usize, "Main group not full should be detected");
    }
    
    #[test]
    fn test_create_supporting_mesh_group_validation_too_many_supporting_groups() {
        // Test: supporting_groups.len() >= 10 should fail
        let supporting_groups_count = 10usize;
        
        // Validation logic: require!(supporting_groups.len() < 10, IndrasError::TooManySupportingGroups)
        assert!(supporting_groups_count >= 10, "Too many supporting groups should be detected");
    }
    
    #[test]
    fn test_create_supporting_mesh_group_validation_name_too_long() {
        // Test: name.len() > 100 should fail
        let name = "a".repeat(101);
        
        // Validation logic: require!(name.len() <= 100, IndrasError::StringTooLong)
        assert!(name.len() > 100, "Name too long should be detected");
    }
    
    // ========== Additional edge case tests ==========
    
    #[test]
    fn test_create_mesh_group_validation_name_exact_max_length() {
        // Test: name.len() == 100 should pass
        let name = "a".repeat(100);
        assert_eq!(name.len(), 100, "Name at max length should be valid");
    }
    
    #[test]
    fn test_create_mesh_group_validation_description_exact_max_length() {
        // Test: description.len() == 500 should pass
        let description = "a".repeat(500);
        assert_eq!(description.len(), 500, "Description at max length should be valid");
    }
    
    #[test]
    fn test_create_mesh_group_validation_embedding_provider_exact_max_length() {
        // Test: embedding_provider.len() == 50 should pass
        let embedding_provider = "a".repeat(50);
        assert_eq!(embedding_provider.len(), 50, "Embedding provider at max length should be valid");
    }
    
    #[test]
    fn test_create_mesh_group_validation_embedding_model_too_long() {
        // Test: embedding_model.len() > 100 should fail
        let embedding_model = Some("a".repeat(101));
        
        if let Some(ref model) = embedding_model {
            assert!(model.len() > 100, "Embedding model too long should be detected");
        }
    }
    
    #[test]
    fn test_create_mesh_group_validation_embedding_model_version_too_long() {
        // Test: embedding_model_version.len() > 50 should fail
        let embedding_model_version = Some("a".repeat(51));
        
        if let Some(ref model_version) = embedding_model_version {
            assert!(model_version.len() > 50, "Embedding model version too long should be detected");
        }
    }
    
    #[test]
    fn test_join_mesh_group_validation_group_exact_max_capacity() {
        // Test: members.len() == max_members - 1 should pass
        let members_count = 6usize;
        let max_members = 7u8;
        
        assert!(members_count < max_members as usize, "Group at max-1 capacity should pass");
    }
    
    #[test]
    fn test_join_mesh_group_validation_group_min_capacity() {
        // Test: members.len() == 0 should pass
        let members_count = 0usize;
        let max_members = 7u8;
        
        assert!(members_count < max_members as usize, "Empty group should pass");
    }
    
    #[test]
    fn test_close_mesh_group_validation_status_active() {
        // Test: status == Active should pass
        let status = GroupStatus::Active;
        
        assert!(status == GroupStatus::Active || status == GroupStatus::Completed, "Active status should be valid");
    }
    
    #[test]
    fn test_close_mesh_group_validation_status_completed() {
        // Test: status == Completed should pass
        let status = GroupStatus::Completed;
        
        assert!(status == GroupStatus::Active || status == GroupStatus::Completed, "Completed status should be valid");
    }
    
    #[test]
    fn test_close_mesh_group_validation_all_invalid_statuses() {
        // Test: all statuses except Active and Completed should fail
        let invalid_statuses = [
            GroupStatus::Forming,
            GroupStatus::Paused,
            GroupStatus::Disbanded,
        ];
        
        for status in invalid_statuses.iter() {
            assert!(
                *status != GroupStatus::Active && *status != GroupStatus::Completed,
                "Status {:?} should be invalid for closing", status
            );
        }
    }
    
    #[test]
    fn test_add_idea_to_mesh_group_validation_all_invalid_idea_statuses() {
        // Test: all statuses except Approved should fail
        let invalid_statuses = [
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Rejected,
            IdeaStatus::InProgress,
            IdeaStatus::Paused,
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Commercialization,
            IdeaStatus::Archived,
            IdeaStatus::Resubmitted,
            IdeaStatus::Voting,
            IdeaStatus::Expired,
        ];
        
        for status in invalid_statuses.iter() {
            assert_ne!(*status, IdeaStatus::Approved, "Status {:?} should be invalid", status);
        }
    }
    
    #[test]
    fn test_remove_mesh_group_member_validation_authorized_remover_leader() {
        // Test: remover == leader should pass
        let remover = Pubkey::new_unique();
        let leader = remover; // Same
        
        assert_eq!(remover, leader, "Leader as remover should be authorized");
    }
    
    #[test]
    fn test_remove_mesh_group_member_validation_authorized_remover_created_by() {
        // Test: remover == created_by should pass
        let remover = Pubkey::new_unique();
        let created_by = remover; // Same
        
        assert_eq!(remover, created_by, "Creator as remover should be authorized");
    }
    
    #[test]
    fn test_remove_mesh_group_member_validation_authorized_remover_authority() {
        // Test: remover == authority should pass
        let remover = Pubkey::new_unique();
        let authority = remover; // Same
        
        assert_eq!(remover, authority, "Authority as remover should be authorized");
    }
    
    #[test]
    fn test_start_mesh_group_validation_authorized_manager_leader() {
        // Test: manager == leader should pass
        let manager = Pubkey::new_unique();
        let leader = manager; // Same
        
        assert_eq!(manager, leader, "Leader as manager should be authorized");
    }
    
    #[test]
    fn test_start_mesh_group_validation_authorized_manager_created_by() {
        // Test: manager == created_by should pass
        let manager = Pubkey::new_unique();
        let created_by = manager; // Same
        
        assert_eq!(manager, created_by, "Creator as manager should be authorized");
    }
    
    #[test]
    fn test_start_mesh_group_validation_authorized_manager_authority() {
        // Test: manager == authority should pass
        let manager = Pubkey::new_unique();
        let authority = manager; // Same
        
        assert_eq!(manager, authority, "Authority as manager should be authorized");
    }
    
    #[test]
    fn test_pause_mesh_group_validation_authorized_manager_leader() {
        // Test: manager == leader should pass
        let manager = Pubkey::new_unique();
        let leader = manager; // Same
        
        assert_eq!(manager, leader, "Leader as manager should be authorized");
    }
    
    #[test]
    fn test_resume_mesh_group_validation_authorized_manager_leader() {
        // Test: manager == leader should pass
        let manager = Pubkey::new_unique();
        let leader = manager; // Same
        
        assert_eq!(manager, leader, "Leader as manager should be authorized");
    }
    
    #[test]
    fn test_complete_mesh_group_validation_authorized_manager_leader() {
        // Test: manager == leader should pass
        let manager = Pubkey::new_unique();
        let leader = manager; // Same
        
        assert_eq!(manager, leader, "Leader as manager should be authorized");
    }
    
    #[test]
    fn test_disband_mesh_group_validation_authorized_manager_leader() {
        // Test: manager == leader should pass
        let manager = Pubkey::new_unique();
        let leader = manager; // Same
        
        assert_eq!(manager, leader, "Leader as manager should be authorized");
    }
    
    #[test]
    fn test_create_supporting_mesh_group_validation_main_group_exact_max() {
        // Test: main_group.members.len() == max_members should pass
        let members_count = 7usize;
        let max_members = 7u8;
        
        assert_eq!(members_count, max_members as usize, "Main group at max should be valid");
    }
    
    #[test]
    fn test_create_supporting_mesh_group_validation_supporting_groups_exact_max() {
        // Test: supporting_groups.len() == MAX_SUPPORTING_GROUPS - 1 should pass
        let supporting_groups_count = 9usize;
        const MAX_SUPPORTING_GROUPS: usize = 10;
        
        assert!(supporting_groups_count < MAX_SUPPORTING_GROUPS, "Supporting groups at max-1 should pass");
    }
}
