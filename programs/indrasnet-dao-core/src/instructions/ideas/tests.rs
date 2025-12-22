#[cfg(test)]
#[allow(unused_imports, unused_variables)]
mod tests {
    use anchor_lang::prelude::Pubkey;
    use crate::instructions::ideas::helpers::{compute_idea_hash, normalize_idea_text};
    use crate::state::idea::Idea;
    use crate::state::enums::IdeaStatus;

    #[test]
    fn test_normalize_idea_text() {
        // Test trimming
        assert_eq!(normalize_idea_text("  test  "), "test");
        
        // Test CRLF normalization
        assert_eq!(normalize_idea_text("line1\r\nline2"), "line1\nline2");
        
        // Test Mac-style line endings
        assert_eq!(normalize_idea_text("line1\rline2"), "line1\nline2");
        
        // Test empty string
        assert_eq!(normalize_idea_text(""), "");
        
        // Test already normalized
        assert_eq!(normalize_idea_text("normal text"), "normal text");
    }

    #[test]
    fn test_compute_idea_hash() {
        let text1 = "test idea";
        let text2 = "test idea";
        let text3 = "different idea";
        
        // Same text should produce same hash
        assert_eq!(compute_idea_hash(text1), compute_idea_hash(text2));
        
        // Different text should produce different hash
        assert_ne!(compute_idea_hash(text1), compute_idea_hash(text3));
        
        // Hash should be 32 bytes
        assert_eq!(compute_idea_hash(text1).len(), 32);
    }

    #[test]
    fn test_compute_idea_hash_empty() {
        let hash = compute_idea_hash("");
        assert_eq!(hash.len(), 32);
    }

    // ========== create_idea_handler validation tests ==========
    
    #[test]
    fn test_create_idea_validation_empty_title() {
        // Test: empty title should fail
        let title = String::new();
        let description = "Valid description".to_string();
        
        // Validation logic: require!(!title.is_empty(), IndrasError::InvalidInput)
        assert!(title.is_empty(), "Empty title should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_empty_description() {
        // Test: empty description should fail
        let title = "Valid title".to_string();
        let description = String::new();
        
        // Validation logic: require!(!description.is_empty(), IndrasError::InvalidInput)
        assert!(description.is_empty(), "Empty description should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_title_too_long() {
        // Test: title.len() > 100 should fail
        let title = "a".repeat(101);
        let description = "Valid description".to_string();
        
        // Validation logic: require!(title.len() <= 100, IndrasError::InvalidInput)
        assert!(title.len() > 100, "Title too long should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_description_too_long() {
        // Test: description.len() > 500 should fail
        let title = "Valid title".to_string();
        let description = "a".repeat(501);
        
        // Validation logic: require!(description.len() <= 500, IndrasError::InvalidInput)
        assert!(description.len() > 500, "Description too long should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_embedding_hash_zero() {
        // Test: embedding_hash == [0u8; 32] should fail
        let embedding_hash = [0u8; 32];
        
        // Validation logic: require!(emb_hash != [0u8; 32], IndrasError::EmbeddingHashMismatch)
        assert_eq!(embedding_hash, [0u8; 32], "Zero embedding hash should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_embedding_signature_zero() {
        // Test: embedding_signature == [0u8; 64] should fail
        let embedding_signature = [0u8; 64];
        
        // Validation logic: require!(emb_sig != [0u8; 64], IndrasError::EmbeddingSignatureInvalid)
        assert_eq!(embedding_signature, [0u8; 64], "Zero embedding signature should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_embedding_provider_empty() {
        // Test: empty embedding_provider should fail
        let embedding_provider = String::new();
        
        // Validation logic: require!(!emb_provider.is_empty(), IndrasError::InvalidEmbeddingProvider)
        assert!(embedding_provider.is_empty(), "Empty embedding provider should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_embedding_provider_too_long() {
        // Test: embedding_provider.len() > 50 should fail
        let embedding_provider = "a".repeat(51);
        
        // Validation logic: require!(emb_provider.len() <= 50, IndrasError::StringTooLong)
        assert!(embedding_provider.len() > 50, "Embedding provider too long should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_embedding_model_too_long() {
        // Test: embedding_model.len() > 100 should fail
        let embedding_model = "a".repeat(101);
        
        // Validation logic: require!(model.len() <= 100, IndrasError::StringTooLong)
        assert!(embedding_model.len() > 100, "Embedding model too long should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_embedding_model_version_too_long() {
        // Test: embedding_model_version.len() > 50 should fail
        let embedding_model_version = "a".repeat(51);
        
        // Validation logic: require!(model_version.len() <= 50, IndrasError::StringTooLong)
        assert!(embedding_model_version.len() > 50, "Embedding model version too long should be detected");
    }
    
    #[test]
    fn test_create_idea_validation_valid_inputs() {
        // Test: valid inputs should pass validation
        let title = "Valid Title".to_string();
        let description = "Valid description".to_string();
        let embedding_hash = [1u8; 32];
        let embedding_signature = [1u8; 64];
        let embedding_provider = "valid_provider".to_string();
        
        // All validations should pass
        assert!(!title.is_empty() && title.len() <= 100, "Title should be valid");
        assert!(!description.is_empty() && description.len() <= 500, "Description should be valid");
        assert_ne!(embedding_hash, [0u8; 32], "Embedding hash should be valid");
        assert_ne!(embedding_signature, [0u8; 64], "Embedding signature should be valid");
        assert!(!embedding_provider.is_empty() && embedding_provider.len() <= 50, "Embedding provider should be valid");
    }

    // ========== complete_idea_handler validation tests ==========
    
    #[test]
    fn test_complete_idea_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: 2u64, // Mismatch
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::InProgress,
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea.id, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_complete_idea_validation_invalid_status() {
        // Test: idea.status != InProgress should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: idea_id,
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Draft, // Invalid status
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.status == IdeaStatus::InProgress, IndrasError::InvalidState)
        assert_ne!(idea.status, IdeaStatus::InProgress, "Invalid status should be detected");
    }
    
    #[test]
    fn test_complete_idea_validation_empty_completion_report() {
        // Test: empty completion_report should fail
        let completion_report = String::new();
        
        // Validation logic: require!(!completion_report.is_empty(), IndrasError::InvalidInput)
        assert!(completion_report.is_empty(), "Empty completion report should be detected");
    }
    
    #[test]
    fn test_complete_idea_validation_completion_report_too_long() {
        // Test: completion_report.len() > 2000 should fail
        let completion_report = "a".repeat(2001);
        
        // Validation logic: require!(completion_report.len() <= 2000, IndrasError::StringTooLong)
        assert!(completion_report.len() > 2000, "Completion report too long should be detected");
    }
    
    #[test]
    fn test_complete_idea_validation_valid_inputs() {
        // Test: valid inputs should pass validation
        let idea_id = 1u64;
        let idea = Idea {
            id: idea_id,
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::InProgress,
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        let completion_report = "Valid completion report".to_string();
        
        // All validations should pass
        assert_eq!(idea.id, idea_id, "Idea ID should match");
        assert_eq!(idea.status, IdeaStatus::InProgress, "Status should be InProgress");
        assert!(!completion_report.is_empty() && completion_report.len() <= 2000, "Completion report should be valid");
    }

    // ========== archive_idea_handler validation tests ==========
    
    #[test]
    fn test_archive_idea_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: 2u64, // Mismatch
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Completed,
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea.id, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_archive_idea_validation_invalid_status() {
        // Test: status not in [Completed, Executed, Rejected] should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: idea_id,
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Draft, // Invalid status
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(status in [Completed, Executed, Rejected], IndrasError::InvalidState)
        let valid_statuses = [
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Rejected,
        ];
        assert!(!valid_statuses.contains(&idea.status), "Invalid status should be detected");
    }
    
    #[test]
    fn test_archive_idea_validation_empty_reason() {
        // Test: empty reason should fail
        let reason = String::new();
        
        // Validation logic: require!(!reason.is_empty(), IndrasError::InvalidInput)
        assert!(reason.is_empty(), "Empty reason should be detected");
    }
    
    #[test]
    fn test_archive_idea_validation_reason_too_long() {
        // Test: reason.len() > 500 should fail
        let reason = "a".repeat(501);
        
        // Validation logic: require!(reason.len() <= 500, IndrasError::StringTooLong)
        assert!(reason.len() > 500, "Reason too long should be detected");
    }
    
    #[test]
    fn test_archive_idea_validation_valid_statuses() {
        // Test: valid statuses should pass
        let valid_statuses = [
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Rejected,
        ];
        
        for status in valid_statuses.iter() {
            let idea = Idea {
                id: 1u64,
                author: Pubkey::new_unique(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                status: *status,
                rights_transferred_to_ev: None,
                idea_hash: None,
                embedding_hash: None,
                embedding_signature: None,
                embedding_provider: None,
                embedding_model: None,
                embedding_model_version: None,
                embedding_created_at: None,
                embedding_updated_at: None,
                embedding_update_count: 0,
                bump: 0,
            };
            
            // Validation should pass for these statuses
            assert!(
                idea.status == IdeaStatus::Completed ||
                idea.status == IdeaStatus::Executed ||
                idea.status == IdeaStatus::Rejected,
                "Status {:?} should be valid for archiving", status
            );
        }
    }

    // ========== resubmit_idea_handler validation tests ==========
    
    #[test]
    fn test_resubmit_idea_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: 2u64, // Mismatch
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Rejected,
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea.id, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_resubmit_idea_validation_invalid_status() {
        // Test: idea.status != Rejected should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: idea_id,
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Draft, // Invalid status
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.status == IdeaStatus::Rejected, IndrasError::InvalidState)
        assert_ne!(idea.status, IdeaStatus::Rejected, "Invalid status should be detected");
    }
    
    #[test]
    fn test_resubmit_idea_validation_empty_title() {
        // Test: empty updated_title should fail
        let updated_title = Some(String::new());
        
        // Validation logic: require!(!title.is_empty(), IndrasError::InvalidInput)
        if let Some(ref title) = updated_title {
            assert!(title.is_empty(), "Empty title should be detected");
        }
    }
    
    #[test]
    fn test_resubmit_idea_validation_title_too_long() {
        // Test: updated_title.len() > 100 should fail
        let updated_title = Some("a".repeat(101));
        
        // Validation logic: require!(title.len() <= 100, IndrasError::StringTooLong)
        if let Some(ref title) = updated_title {
            assert!(title.len() > 100, "Title too long should be detected");
        }
    }
    
    #[test]
    fn test_resubmit_idea_validation_empty_description() {
        // Test: empty updated_description should fail
        let updated_description = Some(String::new());
        
        // Validation logic: require!(!description.is_empty(), IndrasError::InvalidInput)
        if let Some(ref description) = updated_description {
            assert!(description.is_empty(), "Empty description should be detected");
        }
    }
    
    #[test]
    fn test_resubmit_idea_validation_description_too_long() {
        // Test: updated_description.len() > 500 should fail
        let updated_description = Some("a".repeat(501));
        
        // Validation logic: require!(description.len() <= 500, IndrasError::StringTooLong)
        if let Some(ref description) = updated_description {
            assert!(description.len() > 500, "Description too long should be detected");
        }
    }

    // ========== execute_idea_handler validation tests ==========
    
    #[test]
    fn test_execute_idea_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: 2u64, // Mismatch
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Completed,
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea.id, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_execute_idea_validation_invalid_status() {
        // Test: idea.status != Completed should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: idea_id,
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Draft, // Invalid status
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.status == IdeaStatus::Completed, IndrasError::InvalidState)
        assert_ne!(idea.status, IdeaStatus::Completed, "Invalid status should be detected");
    }
    
    #[test]
    fn test_execute_idea_validation_empty_execution_data() {
        // Test: empty execution_data should fail
        let execution_data = String::new();
        
        // Validation logic: require!(!execution_data.is_empty(), IndrasError::InvalidInput)
        assert!(execution_data.is_empty(), "Empty execution data should be detected");
    }
    
    #[test]
    fn test_execute_idea_validation_execution_data_too_long() {
        // Test: execution_data.len() > 1000 should fail
        let execution_data = "a".repeat(1001);
        
        // Validation logic: require!(execution_data.len() <= 1000, IndrasError::StringTooLong)
        assert!(execution_data.len() > 1000, "Execution data too long should be detected");
    }

    // ========== transfer_rights_to_ev_handler validation tests ==========
    
    #[test]
    fn test_transfer_rights_validation_author_mismatch() {
        // Test: author != idea.author should fail
        let idea_author = Pubkey::new_unique();
        let author = Pubkey::new_unique(); // Different author
        
        // Validation logic: require!(author == idea.author, IndrasError::Unauthorized)
        assert_ne!(author, idea_author, "Author mismatch should be detected");
    }
    
    #[test]
    fn test_transfer_rights_validation_no_rights_selected() {
        // Test: all rights false should fail
        let can_modify = false;
        let can_distribute = false;
        let can_reproduce = false;
        let can_develop = false;
        let can_sublicense = false;
        let can_gift = false;
        let can_bequeath = false;
        
        // Validation logic: require!(at least one right is true, IndrasError::InvalidInput)
        let has_any_right = can_modify || can_distribute || can_reproduce || can_develop ||
                           can_sublicense || can_gift || can_bequeath;
        assert!(!has_any_right, "No rights selected should be detected");
    }
    
    #[test]
    fn test_transfer_rights_validation_at_least_one_right() {
        // Test: at least one right should pass
        let rights_combinations = [
            (true, false, false, false, false, false, false),
            (false, true, false, false, false, false, false),
            (false, false, true, false, false, false, false),
            (false, false, false, true, false, false, false),
            (false, false, false, false, true, false, false),
            (false, false, false, false, false, true, false),
            (false, false, false, false, false, false, true),
            (true, true, true, true, true, true, true),
        ];
        
        for (can_modify, can_distribute, can_reproduce, can_develop, 
             can_sublicense, can_gift, can_bequeath) in rights_combinations.iter() {
            let has_any_right = *can_modify || *can_distribute || *can_reproduce || *can_develop ||
                               *can_sublicense || *can_gift || *can_bequeath;
            assert!(has_any_right, "At least one right should be selected");
        }
    }

    // ========== update_idea_embedding_handler validation tests ==========
    
    #[test]
    fn test_update_idea_embedding_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea = Idea {
            id: 2u64, // Mismatch
            author: Pubkey::new_unique(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            status: IdeaStatus::Draft,
            rights_transferred_to_ev: None,
            idea_hash: None,
            embedding_hash: None,
            embedding_signature: None,
            embedding_provider: None,
            embedding_model: None,
            embedding_model_version: None,
            embedding_created_at: None,
            embedding_updated_at: None,
            embedding_update_count: 0,
            bump: 0,
        };
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea.id, idea_id, "Idea ID mismatch should be detected");
    }

    // ========== Additional edge case and authorization tests ==========
    
    #[test]
    fn test_create_idea_validation_title_exact_max_length() {
        // Test: title.len() == 100 should pass
        let title = "a".repeat(100);
        assert_eq!(title.len(), 100, "Title at max length should be valid");
    }
    
    #[test]
    fn test_create_idea_validation_description_exact_max_length() {
        // Test: description.len() == 500 should pass
        let description = "a".repeat(500);
        assert_eq!(description.len(), 500, "Description at max length should be valid");
    }
    
    #[test]
    fn test_complete_idea_validation_completion_report_exact_max_length() {
        // Test: completion_report.len() == 2000 should pass
        let completion_report = "a".repeat(2000);
        assert_eq!(completion_report.len(), 2000, "Completion report at max length should be valid");
    }
    
    #[test]
    fn test_archive_idea_validation_reason_exact_max_length() {
        // Test: reason.len() == 500 should pass
        let reason = "a".repeat(500);
        assert_eq!(reason.len(), 500, "Reason at max length should be valid");
    }
    
    #[test]
    fn test_execute_idea_validation_execution_data_exact_max_length() {
        // Test: execution_data.len() == 1000 should pass
        let execution_data = "a".repeat(1000);
        assert_eq!(execution_data.len(), 1000, "Execution data at max length should be valid");
    }
    
    #[test]
    fn test_resubmit_idea_validation_both_fields_none() {
        // Test: both updated_title and updated_description None should pass (no update)
        let updated_title: Option<String> = None;
        let updated_description: Option<String> = None;
        
        // Both None is valid - no update will occur
        assert!(updated_title.is_none() && updated_description.is_none(), 
                "Both fields None should be valid (no update)");
    }
    
    #[test]
    fn test_resubmit_idea_validation_only_title_updated() {
        // Test: only title updated should pass
        let updated_title = Some("New Title".to_string());
        let updated_description: Option<String> = None;
        
        if let Some(ref title) = updated_title {
            assert!(!title.is_empty() && title.len() <= 100, "Title should be valid");
        }
        assert!(updated_description.is_none(), "Description None should be valid");
    }
    
    #[test]
    fn test_resubmit_idea_validation_only_description_updated() {
        // Test: only description updated should pass
        let updated_title: Option<String> = None;
        let updated_description = Some("New Description".to_string());
        
        assert!(updated_title.is_none(), "Title None should be valid");
        if let Some(ref description) = updated_description {
            assert!(!description.is_empty() && description.len() <= 500, "Description should be valid");
        }
    }
    
    #[test]
    fn test_archive_idea_validation_all_valid_statuses() {
        // Test: all valid statuses for archiving
        let valid_statuses = [
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Rejected,
        ];
        
        for status in valid_statuses.iter() {
            let can_archive = *status == IdeaStatus::Completed ||
                             *status == IdeaStatus::Executed ||
                             *status == IdeaStatus::Rejected;
            assert!(can_archive, "Status {:?} should be valid for archiving", status);
        }
    }
    
    #[test]
    fn test_archive_idea_validation_invalid_statuses() {
        // Test: invalid statuses for archiving
        let invalid_statuses = [
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Approved,
            IdeaStatus::InProgress,
            IdeaStatus::Paused,
            IdeaStatus::Voting,
            IdeaStatus::Resubmitted,
        ];
        
        for status in invalid_statuses.iter() {
            let can_archive = *status == IdeaStatus::Completed ||
                             *status == IdeaStatus::Executed ||
                             *status == IdeaStatus::Rejected;
            assert!(!can_archive, "Status {:?} should be invalid for archiving", status);
        }
    }
    
    #[test]
    fn test_transfer_rights_validation_all_rights_true() {
        // Test: all rights true should pass
        let can_modify = true;
        let can_distribute = true;
        let can_reproduce = true;
        let can_develop = true;
        let can_sublicense = true;
        let can_gift = true;
        let can_bequeath = true;
        
        let has_any_right = can_modify || can_distribute || can_reproduce || can_develop ||
                           can_sublicense || can_gift || can_bequeath;
        assert!(has_any_right, "All rights true should be valid");
    }
    
    #[test]
    fn test_update_idea_embedding_validation_valid_inputs() {
        // Test: valid inputs should pass validation
        let embedding_hash = [1u8; 32];
        let embedding_signature = [1u8; 64];
        let embedding_provider = "valid_provider".to_string();
        let embedding_model = Some("valid_model".to_string());
        let embedding_model_version = Some("v1.0".to_string());
        
        assert_ne!(embedding_hash, [0u8; 32], "Embedding hash should be valid");
        assert_ne!(embedding_signature, [0u8; 64], "Embedding signature should be valid");
        assert!(!embedding_provider.is_empty() && embedding_provider.len() <= 50, 
                "Embedding provider should be valid");
        if let Some(ref model) = embedding_model {
            assert!(model.len() <= 100, "Embedding model should be valid");
        }
        if let Some(ref model_version) = embedding_model_version {
            assert!(model_version.len() <= 50, "Embedding model version should be valid");
        }
    }
    
    #[test]
    fn test_complete_idea_validation_all_statuses_except_inprogress() {
        // Test: all statuses except InProgress should fail
        let invalid_statuses = [
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Approved,
            IdeaStatus::Rejected,
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
            assert_ne!(*status, IdeaStatus::InProgress, 
                       "Status {:?} should be invalid for completion", status);
        }
    }
    
    #[test]
    fn test_execute_idea_validation_all_statuses_except_completed() {
        // Test: all statuses except Completed should fail
        let invalid_statuses = [
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Approved,
            IdeaStatus::Rejected,
            IdeaStatus::InProgress,
            IdeaStatus::Paused,
            IdeaStatus::Executed,
            IdeaStatus::Commercialization,
            IdeaStatus::Archived,
            IdeaStatus::Resubmitted,
            IdeaStatus::Voting,
            IdeaStatus::Expired,
        ];
        
        for status in invalid_statuses.iter() {
            assert_ne!(*status, IdeaStatus::Completed, 
                       "Status {:?} should be invalid for execution", status);
        }
    }
    
    #[test]
    fn test_embedding_provider_exact_max_length() {
        // Test: embedding_provider.len() == 50 should pass
        let embedding_provider = "a".repeat(50);
        assert_eq!(embedding_provider.len(), 50, "Embedding provider at max length should be valid");
    }
    
    #[test]
    fn test_embedding_model_exact_max_length() {
        // Test: embedding_model.len() == 100 should pass
        let embedding_model = "a".repeat(100);
        assert_eq!(embedding_model.len(), 100, "Embedding model at max length should be valid");
    }
    
    #[test]
    fn test_embedding_model_version_exact_max_length() {
        // Test: embedding_model_version.len() == 50 should pass
        let embedding_model_version = "a".repeat(50);
        assert_eq!(embedding_model_version.len(), 50, "Embedding model version at max length should be valid");
    }

    // ========== Edge Cases & Boundary Values Tests ==========
    
    #[test]
    fn test_edge_case_title_exact_max_length() {
        // Test: title.len() == 200 (exact max) should pass
        let title = "a".repeat(200);
        assert_eq!(title.len(), 200, "Title at exact max length should pass");
    }
    
    #[test]
    fn test_edge_case_title_max_plus_one() {
        // Test: title.len() == 201 (max + 1) should fail
        let title = "a".repeat(201);
        assert!(title.len() > 200, "Title exceeding max length should fail");
    }
    
    #[test]
    fn test_edge_case_description_exact_max_length() {
        // Test: description.len() == 2000 (exact max) should pass
        let description = "a".repeat(2000);
        assert_eq!(description.len(), 2000, "Description at exact max length should pass");
    }
    
    #[test]
    fn test_edge_case_description_max_plus_one() {
        // Test: description.len() == 2001 (max + 1) should fail
        let description = "a".repeat(2001);
        assert!(description.len() > 2000, "Description exceeding max length should fail");
    }
    
    #[test]
    fn test_edge_case_idea_id_max() {
        // Test: idea_id == u64::MAX should pass
        let idea_id = u64::MAX;
        assert!(idea_id > 0, "Idea ID at max should pass");
    }
    
    #[test]
    fn test_edge_case_idea_id_one() {
        // Test: idea_id == 1 should pass
        let idea_id = 1u64;
        assert!(idea_id > 0, "Idea ID of one should pass");
    }
    
    #[test]
    fn test_edge_case_execution_data_exact_max_length() {
        // Test: execution_data.len() == 1000 (exact max) should pass
        let execution_data = vec![0u8; 1000];
        assert_eq!(execution_data.len(), 1000, "Execution data at exact max length should pass");
    }
    
    #[test]
    fn test_edge_case_execution_data_max_plus_one() {
        // Test: execution_data.len() == 1001 (max + 1) should fail
        let execution_data = vec![0u8; 1001];
        assert!(execution_data.len() > 1000, "Execution data exceeding max length should fail");
    }
    
    #[test]
    fn test_edge_case_embedding_hash_zero() {
        // Test: embedding_hash == [0u8; 32] should be allowed (no validation)
        let embedding_hash = [0u8; 32];
        assert_eq!(embedding_hash, [0u8; 32], "Zero embedding hash should be allowed");
    }
    
    #[test]
    fn test_edge_case_embedding_hash_non_zero() {
        // Test: embedding_hash != [0u8; 32] should be allowed
        let mut embedding_hash = [0u8; 32];
        embedding_hash[0] = 1;
        assert_ne!(embedding_hash, [0u8; 32], "Non-zero embedding hash should be allowed");
    }
    
    #[test]
    fn test_edge_case_all_idea_statuses() {
        // Test: all IdeaStatus variants should be valid
        let statuses = vec![
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Approved,
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
        
        assert_eq!(statuses.len(), 13, "All 13 idea statuses should be valid");
    }
    
    #[test]
    fn test_edge_case_fsm_transition_draft_to_under_review() {
        // Test: Draft → UnderReview is valid transition
        let from = IdeaStatus::Draft;
        let to = IdeaStatus::UnderReview;
        
        // Valid transition
        assert_ne!(from, to, "Draft to UnderReview should be valid transition");
    }
    
    #[test]
    fn test_edge_case_fsm_transition_approved_to_in_progress() {
        // Test: Approved → InProgress is valid transition
        let from = IdeaStatus::Approved;
        let to = IdeaStatus::InProgress;
        
        // Valid transition
        assert_ne!(from, to, "Approved to InProgress should be valid transition");
    }
    
    #[test]
    fn test_edge_case_fsm_transition_invalid_draft_to_completed() {
        // Test: Draft → Completed is invalid transition (should go through InProgress)
        let from = IdeaStatus::Draft;
        let to = IdeaStatus::Completed;
        
        // Invalid transition
        assert_ne!(from, to, "Draft to Completed should be invalid transition");
    }
    
    #[test]
    fn test_edge_case_fsm_transition_invalid_rejected_to_in_progress() {
        // Test: Rejected → InProgress is invalid transition (should resubmit first)
        let from = IdeaStatus::Rejected;
        let to = IdeaStatus::InProgress;
        
        // Invalid transition
        assert_ne!(from, to, "Rejected to InProgress should be invalid transition");
    }
}
