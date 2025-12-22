//! Proposal account structures

use anchor_lang::prelude::*;

/// Proposal status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, InitSpace)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
    Archived,
    Tied,
}

/// Proposal account structure
#[account]
#[derive(Debug, PartialEq, InitSpace)]
pub struct Proposal {
    pub id: u64,
    #[max_len(200)]
    pub title: String,
    #[max_len(2000)]
    pub description: String,
    #[max_len(50)]
    pub proposal_type: String,
    pub author: Pubkey,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub submitted_at: Option<i64>,
    pub cancelled_at: Option<i64>,
    pub executed_at: Option<i64>,
    pub archived_at: Option<i64>,
    pub voting_duration: i64,
    pub status: ProposalStatus,
    pub bump: u8,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_votes: u64,
    pub last_tallied_at: Option<i64>,
    #[max_len(200)]
    pub cancellation_reason: Option<String>,
    /// Execution data (JSON-encoded data for proposal execution)
    /// For role changes: {"type": "role_change", "target": "...", "role_mask": 123}
    #[max_len(500)]
    pub execution_data: Option<String>,
    /// Expiration timestamp - proposal will be auto-archived after this time
    /// None means proposal never expires
    pub expires_at: Option<i64>,
    /// Optional: ID of the Idea this proposal was created from (rare case)
    /// None means proposal was created directly, not from an Idea
    pub idea_id: Option<u64>,
    /// Optional: Treasury operation data for Treasury proposals
    /// None means this is not a Treasury proposal
    pub treasury_operation: Option<crate::state::proposal::treasury::TreasuryOperationData>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_status_variants() {
        let variants = vec![
            ProposalStatus::Draft,
            ProposalStatus::Active,
            ProposalStatus::Passed,
            ProposalStatus::Rejected,
            ProposalStatus::Executed,
            ProposalStatus::Cancelled,
            ProposalStatus::Archived,
            ProposalStatus::Tied,
        ];
        
        // Check all variants are unique
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    fn create_test_pubkey(seed: u8) -> anchor_lang::prelude::Pubkey {
        anchor_lang::prelude::Pubkey::from([seed; 32])
    }

    fn create_test_proposal() -> Proposal {
        Proposal {
            id: 1,
            title: "Test Proposal".to_string(),
            description: "Test Description".to_string(),
            proposal_type: "governance".to_string(),
            author: create_test_pubkey(1),
            created_at: 1000,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 168,
            status: ProposalStatus::Draft,
            bump: 255,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
            expires_at: None,
            idea_id: None,
            treasury_operation: None,
        }
    }

    #[test]
    fn test_proposal_structure() {
        let proposal = create_test_proposal();
        assert_eq!(proposal.id, 1);
        assert_eq!(proposal.title, "Test Proposal");
        assert_eq!(proposal.description, "Test Description");
        assert_eq!(proposal.proposal_type, "governance");
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert_eq!(proposal.bump, 255);
    }

    #[test]
    fn test_proposal_all_status_variants() {
        let statuses = vec![
            ProposalStatus::Draft,
            ProposalStatus::Active,
            ProposalStatus::Passed,
            ProposalStatus::Rejected,
            ProposalStatus::Executed,
            ProposalStatus::Cancelled,
            ProposalStatus::Archived,
            ProposalStatus::Tied,
        ];
        
        for status in &statuses {
            let mut proposal = create_test_proposal();
            proposal.status = status.clone();
            assert_eq!(proposal.status, *status);
        }
    }

    #[test]
    fn test_proposal_votes() {
        let mut proposal = create_test_proposal();
        
        proposal.yes_votes = 100;
        proposal.no_votes = 50;
        proposal.total_votes = 150;
        
        assert_eq!(proposal.yes_votes, 100);
        assert_eq!(proposal.no_votes, 50);
        assert_eq!(proposal.total_votes, 150);
    }

    #[test]
    fn test_proposal_timestamps() {
        let mut proposal = create_test_proposal();
        
        proposal.created_at = 1000;
        proposal.updated_at = Some(2000);
        proposal.submitted_at = Some(3000);
        proposal.executed_at = Some(4000);
        proposal.archived_at = Some(5000);
        proposal.last_tallied_at = Some(6000);
        
        assert_eq!(proposal.created_at, 1000);
        assert_eq!(proposal.updated_at, Some(2000));
        assert_eq!(proposal.submitted_at, Some(3000));
        assert_eq!(proposal.executed_at, Some(4000));
        assert_eq!(proposal.archived_at, Some(5000));
        assert_eq!(proposal.last_tallied_at, Some(6000));
    }

    #[test]
    fn test_proposal_cancellation_reason() {
        let mut proposal = create_test_proposal();
        
        proposal.cancellation_reason = Some("Test reason".to_string());
        assert_eq!(proposal.cancellation_reason, Some("Test reason".to_string()));
        
        proposal.cancellation_reason = None;
        assert_eq!(proposal.cancellation_reason, None);
    }

    #[test]
    fn test_proposal_execution_data() {
        let mut proposal = create_test_proposal();
        
        proposal.execution_data = Some(r#"{"type": "role_change", "target": "..."}"#.to_string());
        assert!(proposal.execution_data.is_some());
        assert!(proposal.execution_data.as_ref().unwrap().contains("role_change"));
        
        proposal.execution_data = None;
        assert_eq!(proposal.execution_data, None);
    }

    #[test]
    fn test_proposal_author() {
        let author1 = create_test_pubkey(10);
        let author2 = create_test_pubkey(20);
        
        let mut proposal1 = create_test_proposal();
        proposal1.author = author1;
        
        let mut proposal2 = create_test_proposal();
        proposal2.author = author2;
        
        assert_ne!(proposal1.author, proposal2.author);
    }

    #[test]
    fn test_proposal_voting_duration() {
        let mut proposal = create_test_proposal();
        
        proposal.voting_duration = 24; // 1 day
        assert_eq!(proposal.voting_duration, 24);
        
        proposal.voting_duration = 720; // 30 days
        assert_eq!(proposal.voting_duration, 720);
    }

    #[test]
    fn test_proposal_id_boundary() {
        let mut proposal = create_test_proposal();
        
        proposal.id = u64::MAX;
        assert_eq!(proposal.id, u64::MAX);
        
        proposal.id = 1;
        assert_eq!(proposal.id, 1);
    }

    #[test]
    fn test_proposal_status_equality() {
        assert_eq!(ProposalStatus::Draft, ProposalStatus::Draft);
        assert_ne!(ProposalStatus::Draft, ProposalStatus::Active);
        assert_eq!(ProposalStatus::Active, ProposalStatus::Active);
        assert_ne!(ProposalStatus::Active, ProposalStatus::Passed);
    }

    #[test]
    fn test_proposal_status_all_equality_checks() {
        let statuses = vec![
            ProposalStatus::Draft,
            ProposalStatus::Active,
            ProposalStatus::Passed,
            ProposalStatus::Rejected,
            ProposalStatus::Executed,
            ProposalStatus::Cancelled,
            ProposalStatus::Archived,
            ProposalStatus::Tied,
        ];
        
        // Test equality
        for status in &statuses {
            assert_eq!(*status, status.clone());
        }
        
        // Test inequality
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j]);
            }
        }
    }

    #[test]
    fn test_proposal_status_clone() {
        let status1 = ProposalStatus::Draft;
        let status2 = status1.clone();
        assert_eq!(status1, status2);
        
        let status3 = ProposalStatus::Active;
        let status4 = status3.clone();
        assert_eq!(status3, status4);
    }

    #[test]
    fn test_proposal_status_space() {
        assert_eq!(<ProposalStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_proposal_all_fields() {
        let proposal = Proposal {
            id: 123,
            title: "Title".to_string(),
            description: "Description".to_string(),
            proposal_type: "type".to_string(),
            author: create_test_pubkey(5),
            created_at: 1000,
            updated_at: Some(2000),
            submitted_at: Some(3000),
            cancelled_at: Some(4000),
            executed_at: Some(5000),
            archived_at: Some(6000),
            voting_duration: 168,
            status: ProposalStatus::Active,
            bump: 128,
            yes_votes: 100,
            no_votes: 50,
            total_votes: 150,
            last_tallied_at: Some(7000),
            cancellation_reason: Some("Reason".to_string()),
            execution_data: Some("Data".to_string()),
            expires_at: None,
            idea_id: None,
            treasury_operation: None,
        };
        
        assert_eq!(proposal.id, 123);
        assert_eq!(proposal.title, "Title");
        assert_eq!(proposal.description, "Description");
        assert_eq!(proposal.proposal_type, "type");
        assert_eq!(proposal.author, create_test_pubkey(5));
        assert_eq!(proposal.created_at, 1000);
        assert_eq!(proposal.updated_at, Some(2000));
        assert_eq!(proposal.submitted_at, Some(3000));
        assert_eq!(proposal.cancelled_at, Some(4000));
        assert_eq!(proposal.executed_at, Some(5000));
        assert_eq!(proposal.archived_at, Some(6000));
        assert_eq!(proposal.voting_duration, 168);
        assert_eq!(proposal.status, ProposalStatus::Active);
        assert_eq!(proposal.bump, 128);
        assert_eq!(proposal.yes_votes, 100);
        assert_eq!(proposal.no_votes, 50);
        assert_eq!(proposal.total_votes, 150);
        assert_eq!(proposal.last_tallied_at, Some(7000));
        assert_eq!(proposal.cancellation_reason, Some("Reason".to_string()));
        assert_eq!(proposal.execution_data, Some("Data".to_string()));
    }

    #[test]
    fn test_proposal_with_all_none_fields() {
        let proposal = Proposal {
            id: 1,
            title: "Title".to_string(),
            description: "Description".to_string(),
            proposal_type: "type".to_string(),
            author: create_test_pubkey(1),
            created_at: 1000,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 168,
            status: ProposalStatus::Draft,
            bump: 255,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
            expires_at: None,
            idea_id: None,
            treasury_operation: None,
        };
        
        assert_eq!(proposal.updated_at, None);
        assert_eq!(proposal.submitted_at, None);
        assert_eq!(proposal.cancelled_at, None);
        assert_eq!(proposal.executed_at, None);
        assert_eq!(proposal.archived_at, None);
        assert_eq!(proposal.last_tallied_at, None);
        assert_eq!(proposal.cancellation_reason, None);
        assert_eq!(proposal.execution_data, None);
    }

    #[test]
    fn test_proposal_vote_calculations() {
        let mut proposal = create_test_proposal();
        
        proposal.yes_votes = 75;
        proposal.no_votes = 25;
        proposal.total_votes = 100;
        
        // Verify vote totals
        assert_eq!(proposal.yes_votes + proposal.no_votes, proposal.total_votes);
    }

    #[test]
    fn test_proposal_large_id() {
        let mut proposal = create_test_proposal();
        proposal.id = u64::MAX;
        assert_eq!(proposal.id, u64::MAX);
    }

    #[test]
    fn test_proposal_zero_votes() {
        let proposal = create_test_proposal();
        assert_eq!(proposal.yes_votes, 0);
        assert_eq!(proposal.no_votes, 0);
        assert_eq!(proposal.total_votes, 0);
    }

    #[test]
    fn test_proposal_status_all_variants_unique() {
        let statuses = vec![
            ProposalStatus::Draft,
            ProposalStatus::Active,
            ProposalStatus::Passed,
            ProposalStatus::Rejected,
            ProposalStatus::Executed,
            ProposalStatus::Cancelled,
            ProposalStatus::Archived,
            ProposalStatus::Tied,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_proposal_all_fields_with_values() {
        let author = create_test_pubkey(15);
        let proposal = Proposal {
            id: 999,
            title: "Advanced Proposal".to_string(),
            description: "Advanced Description".to_string(),
            proposal_type: "advanced".to_string(),
            author,
            created_at: 5000,
            updated_at: Some(6000),
            submitted_at: Some(7000),
            cancelled_at: None,
            executed_at: Some(8000),
            archived_at: Some(9000),
            voting_duration: 720,
            status: ProposalStatus::Executed,
            bump: 200,
            yes_votes: 200,
            no_votes: 100,
            total_votes: 300,
            last_tallied_at: Some(8500),
            cancellation_reason: None,
            execution_data: Some(r#"{"type": "test"}"#.to_string()),
            expires_at: None,
            idea_id: None,
            treasury_operation: None,
        };
        
        assert_eq!(proposal.id, 999);
        assert_eq!(proposal.title, "Advanced Proposal");
        assert_eq!(proposal.description, "Advanced Description");
        assert_eq!(proposal.proposal_type, "advanced");
        assert_eq!(proposal.author, author);
        assert_eq!(proposal.created_at, 5000);
        assert_eq!(proposal.updated_at, Some(6000));
        assert_eq!(proposal.submitted_at, Some(7000));
        assert_eq!(proposal.executed_at, Some(8000));
        assert_eq!(proposal.archived_at, Some(9000));
        assert_eq!(proposal.voting_duration, 720);
        assert_eq!(proposal.status, ProposalStatus::Executed);
        assert_eq!(proposal.bump, 200);
        assert_eq!(proposal.yes_votes, 200);
        assert_eq!(proposal.no_votes, 100);
        assert_eq!(proposal.total_votes, 300);
        assert_eq!(proposal.last_tallied_at, Some(8500));
        assert_eq!(proposal.execution_data, Some(r#"{"type": "test"}"#.to_string()));
    }
}
