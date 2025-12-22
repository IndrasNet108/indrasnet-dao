//! Integration tests for Proposal functionality
//!
//! Tests for:
//! - Proposal lifecycle (create, activate, pass, reject, cancel, archive)
//! - Auto-transition after voting
//! - Auto-archive on expiration
//! - Proposal amendments
//! - Proposal templates

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::state::proposal::*;
    use anchor_lang::prelude::*;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    // ========== Proposal Lifecycle Tests ==========

    #[test]
    fn test_proposal_full_lifecycle() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        // Draft -> Active
        assert_eq!(proposal.status, ProposalStatus::Draft);
        proposal.activate_with_time(10, 20, 2000).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Active);
        assert_eq!(proposal.submitted_at, Some(2000));

        // Active -> Passed (after voting period)
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        proposal.yes_votes = 100;
        proposal.no_votes = 50;
        proposal.auto_transition_after_voting(voting_end + 1).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Passed);

        // Passed -> Executed
        proposal.execute_with_time(5000).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Executed);
        assert_eq!(proposal.executed_at, Some(5000));

        // Executed -> Archived
        proposal.archive_with_time(6000).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Archived);
        assert_eq!(proposal.archived_at, Some(6000));
    }

    #[test]
    fn test_proposal_rejection_lifecycle() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        proposal.yes_votes = 50;
        proposal.no_votes = 100;

        // Active -> Rejected
        proposal.auto_transition_after_voting(voting_end + 1).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Rejected);

        // Rejected -> Archived
        proposal.archive_with_time(4000).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }

    #[test]
    fn test_proposal_tied_lifecycle() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        proposal.yes_votes = 100;
        proposal.no_votes = 100;

        // Active -> Tied
        proposal.auto_transition_after_voting(voting_end + 1).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Tied);
    }

    #[test]
    fn test_proposal_cancellation() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        // Draft -> Cancelled
        proposal.cancel_with_time("Changed mind".to_string(), 2000).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Cancelled);
        assert_eq!(proposal.cancellation_reason, Some("Changed mind".to_string()));

        // Cancelled -> Archived
        proposal.archive_with_time(3000).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }

    // ========== Auto-Transition Tests ==========

    #[test]
    fn test_auto_transition_before_voting_end() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        proposal.activate_with_time(10, 20, 2000).unwrap();
        proposal.yes_votes = 100;
        proposal.no_votes = 50;

        // Should not transition before voting ends
        assert!(!proposal.auto_transition_after_voting(2000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    #[test]
    fn test_auto_transition_not_active() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        // Should not transition if not Active
        assert!(!proposal.auto_transition_after_voting(5000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Draft);
    }

    // ========== Expiration and Auto-Archive Tests ==========

    #[test]
    fn test_proposal_expiration_set() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        // Set expiration
        proposal.set_expiration(Some(5000)).unwrap();
        assert_eq!(proposal.expires_at, Some(5000));

        // Invalid expiration (before creation)
        assert!(proposal.set_expiration(Some(500)).is_err());
    }

    #[test]
    fn test_auto_archive_expired_executed() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();
        proposal.execute_with_time(3000).unwrap();
        proposal.expires_at = Some(5000);

        // Should auto-archive after expiration
        assert!(proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }

    #[test]
    fn test_auto_archive_expired_rejected() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();
        proposal.expires_at = Some(5000);

        // Should auto-archive after expiration
        assert!(proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }

    #[test]
    fn test_auto_archive_not_expired() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        proposal.activate_with_time(10, 20, 2000).unwrap();
        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();
        proposal.expires_at = Some(10000);

        // Should not archive before expiration
        assert!(!proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }

    #[test]
    fn test_auto_archive_active_proposal() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        proposal.activate_with_time(10, 20, 2000).unwrap();
        proposal.expires_at = Some(5000);

        // Should not archive Active proposal even if expired
        assert!(!proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    // ========== Proposal Amendment Tests ==========

    #[test]
    fn test_proposal_amendment_creation() {
        let author = create_test_pubkey(1);
        let amendment = ProposalAmendment::new_with_time(
            1,
            100,
            author,
            "Amendment content".to_string(),
            255,
            1000,
        ).unwrap();

        assert_eq!(amendment.amendment_id, 1);
        assert_eq!(amendment.proposal_id, 100);
        assert_eq!(amendment.author, author);
        assert_eq!(amendment.content, "Amendment content");
        assert_eq!(amendment.created_at, 1000);
    }

    #[test]
    fn test_proposal_amendment_validation() {
        let author = create_test_pubkey(1);

        // Empty content should fail
        assert!(ProposalAmendment::new_with_time(
            1, 100, author, String::new(), 255, 1000
        ).is_err());

        // Content too long should fail
        let long_content = "a".repeat(2001);
        assert!(ProposalAmendment::new_with_time(
            1, 100, author, long_content, 255, 1000
        ).is_err());
    }

    // ========== Proposal Template Tests ==========

    #[test]
    fn test_proposal_template_creation() {
        let author = create_test_pubkey(1);
        let template = ProposalTemplate::new_with_time(
            1,
            "Test Template".to_string(),
            "Test Description".to_string(),
            "governance".to_string(),
            vec![],
            author,
            255,
            1000,
        ).unwrap();

        assert_eq!(template.template_id, 1);
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.description, "Test Description");
        assert_eq!(template.proposal_type, "governance");
        assert_eq!(template.fields.len(), 0);
        assert_eq!(template.created_by, author);
        assert!(template.is_active);
    }

    #[test]
    fn test_proposal_template_update() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::new_with_time(
            1,
            "Old Name".to_string(),
            "Old Description".to_string(),
            "governance".to_string(),
            vec![],
            author,
            255,
            1000,
        ).unwrap();

        // Update name and description using update_with_time
        template.update_with_time(
            Some("New Name".to_string()),
            Some("New Description".to_string()),
            None,
            2000,
        ).unwrap();

        assert_eq!(template.name, "New Name");
        assert_eq!(template.description, "New Description");
        assert_eq!(template.updated_at, Some(2000));
    }

    #[test]
    fn test_proposal_template_activation() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            vec![],
            author,
            255,
            1000,
        ).unwrap();

        // Deactivate using deactivate_with_time
        template.deactivate_with_time(2000).unwrap();
        assert!(!template.is_active);
        assert_eq!(template.updated_at, Some(2000));

        // Reactivate using activate_with_time
        template.activate_with_time(3000).unwrap();
        assert!(template.is_active);
        assert_eq!(template.updated_at, Some(3000));
    }

    #[test]
    fn test_proposal_template_field_limits() {
        let author = create_test_pubkey(1);
        
        // Too many fields should fail
        let too_many_fields: Vec<TemplateField> = (0..21)
            .map(|i| TemplateField {
                name: format!("field{}", i),
                description: "Test".to_string(),
                field_type: TemplateFieldType::Text,
                required: false,
            })
            .collect();

        assert!(ProposalTemplate::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            too_many_fields,
            author,
            255,
            1000,
        ).is_err());
    }

    // ========== Idea ID Link Tests ==========

    #[test]
    fn test_proposal_with_idea_id() {
        let author = create_test_pubkey(1);
        let proposal = Proposal::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            255,
            1000,
        ).unwrap();

        // idea_id should be None by default
        assert_eq!(proposal.idea_id, None);
    }
}
