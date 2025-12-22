//! Grant lifecycle methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::{GrantStatus, GrantCategory, GrantType, GrantDisbursementType, VerificationStatus};
use super::{Grant, VotingLayer};

/// Parameters for creating a grant
pub struct GrantParams {
    pub id: u64,
    pub idea_id: u64,
    pub mesh_group: Pubkey,
    pub category: GrantCategory,
    pub grant_type: GrantType,
    pub disbursement_type: GrantDisbursementType, // Disbursement type
    pub base_amount: u64,
    pub reputation_bonus: u64,
    pub milestone_id: Option<u64>,
    pub bump: u8,
}

impl Grant {
    /// Create a new grant
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u64,
        idea_id: u64,
        mesh_group: Pubkey,
        category: GrantCategory,
        grant_type: GrantType,
        disbursement_type: GrantDisbursementType,
        base_amount: u64,
        reputation_bonus: u64,
        milestone_id: Option<u64>,
        bump: u8,
    ) -> Result<Self> {
        let current_time = Clock::get()?.unix_timestamp;
        let params = GrantParams {
            id,
            idea_id,
            mesh_group,
            category,
            grant_type,
            disbursement_type,
            base_amount,
            reputation_bonus,
            milestone_id,
            bump,
        };
        Ok(Self::new_with_time(params, current_time))
    }

    /// Create a new grant with explicit timestamp
    pub fn new_with_time(
        params: GrantParams,
        current_time: i64,
    ) -> Self {
        let total_amount = params.base_amount + params.reputation_bonus;
        
        Self {
            id: params.id,
            idea_id: params.idea_id,
            mesh_group: params.mesh_group,
            category: params.category,
            status: GrantStatus::Pending,
            base_amount: params.base_amount,
            reputation_bonus: params.reputation_bonus,
            total_amount,
            disbursed_amount: 0,
            grant_type: params.grant_type,
            disbursement_type: params.disbursement_type,
            milestone_id: params.milestone_id,
            verification_status: VerificationStatus::Pending,
            // NOTE: When grant is approved, author MUST transfer commercialization right to e.V.
            // Author remains copyright owner (does not transfer)
            // e.V. receives right to transfer Idea to commercial enterprise
            commercialization_right_transferred: false, // Will be set on approval
            created_at: current_time,
            approved_at: None,
            completed_at: None,
            // Grant voting fields - initialize with voting period (7 days = 604800 seconds)
            voting_end: current_time + 604800, // 7 days in seconds
            total_votes: 0,
            total_yes_weight: 0,
            total_no_weight: 0,
            total_abstain_weight: 0,
            quorum_reached: false,
            // Semantic domain and voting layer - defaults
            semantic_domain: None,
            semantic_domain_account: None,
            semantic_distance: None,
            phenomenon_membership: None,
            grant_level: 1, // Default to Level 1 (author only)
            voting_layer: VotingLayer::AuthorOnly, // Default to author only
            // Grant report fields
            final_report_submitted: false,
            final_report_approved: false,
            final_report_submitted_at: None,
            final_report_approved_at: None,
            escrow_account: None, // Will be created on activation for Escrow type
            bump: params.bump,
        }
    }

    /// Approve grant (Pending -> Approved)
    pub fn approve(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.approve_with_time(current_time)
    }

    /// Approve grant with explicit timestamp
    pub fn approve_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.status == GrantStatus::Pending, IndrasError::InvalidState);
        self.status = GrantStatus::Approved;
        self.approved_at = Some(current_time);
        Ok(())
    }

    /// Activate grant (Approved -> Active)
    pub fn activate(&mut self) -> Result<()> {
        require!(self.status == GrantStatus::Approved, IndrasError::InvalidState);
        self.status = GrantStatus::Active;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use crate::state::grant::types::GrantDisbursementType;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_grant_new() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let grant = Grant::new_with_time(params, 1000);
        
        assert_eq!(grant.id, 1);
        assert_eq!(grant.idea_id, 10);
        assert_eq!(grant.mesh_group, mesh_group);
        assert_eq!(grant.status, GrantStatus::Pending);
        assert_eq!(grant.base_amount, 1000);
        assert_eq!(grant.reputation_bonus, 200);
        assert_eq!(grant.total_amount, 1200);
        assert_eq!(grant.disbursed_amount, 0);
        assert_eq!(grant.verification_status, VerificationStatus::Pending);
        assert_eq!(grant.commercialization_right_transferred, false);
        assert_eq!(grant.created_at, 1000);
    }

    #[test]
    fn test_grant_approve() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        assert_eq!(grant.status, GrantStatus::Pending);
        assert!(grant.approve_with_time(2000).is_ok());
        assert_eq!(grant.status, GrantStatus::Approved);
        assert_eq!(grant.approved_at, Some(2000));
    }

    #[test]
    fn test_grant_approve_invalid_state() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        grant.status = GrantStatus::Active;
        // Try to approve non-pending grant - should fail
        assert!(grant.approve_with_time(2000).is_err());
    }

    #[test]
    fn test_grant_activate() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        grant.approve_with_time(2000).unwrap();
        assert!(grant.activate().is_ok());
        assert_eq!(grant.status, GrantStatus::Active);
    }

    #[test]
    fn test_grant_activate_invalid_state() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Try to activate non-approved grant - should fail
        assert!(grant.activate().is_err());
    }

    #[test]
    fn test_grant_lifecycle() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Pending -> Approved
        assert_eq!(grant.status, GrantStatus::Pending);
        grant.approve_with_time(2000).unwrap();
        assert_eq!(grant.status, GrantStatus::Approved);
        
        // Approved -> Active
        grant.activate().unwrap();
        assert_eq!(grant.status, GrantStatus::Active);
    }

    #[test]
    fn test_grant_new_with_time_all_categories() {
        let mesh_group = create_test_pubkey(1);
        let categories = vec![
            GrantCategory::Research,
            GrantCategory::Development,
            GrantCategory::Community,
        ];
        
        for (idx, category) in categories.iter().enumerate() {
            let params = GrantParams {
                id: idx as u64 + 1,
                idea_id: 10,
                mesh_group,
                category: *category,
                grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: 1000,
                reputation_bonus: 200,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, 1000);
            assert_eq!(grant.category, *category);
            assert_eq!(grant.status, GrantStatus::Pending);
        }
    }

    #[test]
    fn test_grant_new_with_time_all_types() {
        let mesh_group = create_test_pubkey(1);
        let types = vec![
            GrantType::Initial,
            GrantType::Core,
            GrantType::Final,
        ];
        
        for (idx, grant_type) in types.iter().enumerate() {
            let params = GrantParams {
                id: idx as u64 + 1,
                idea_id: 10,
                mesh_group,
                category: GrantCategory::Research,
                grant_type: *grant_type,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: 1000,
                reputation_bonus: 200,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, 1000);
            assert_eq!(grant.grant_type, *grant_type);
        }
    }

    #[test]
    fn test_grant_new_with_time_milestone_id() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Core,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: Some(5),
            bump: 255,
        };
        let grant = Grant::new_with_time(params, 1000);
        
        assert_eq!(grant.milestone_id, Some(5));
    }

    #[test]
    fn test_grant_new_with_time_zero_amounts() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 0,
            reputation_bonus: 0,
            milestone_id: None,
            bump: 255,
        };
        let grant = Grant::new_with_time(params, 1000);
        
        assert_eq!(grant.base_amount, 0);
        assert_eq!(grant.reputation_bonus, 0);
        assert_eq!(grant.total_amount, 0);
    }

    #[test]
    fn test_grant_new_with_time_large_amounts() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1_000_000,
            reputation_bonus: 500_000,
            milestone_id: None,
            bump: 255,
        };
        let grant = Grant::new_with_time(params, 1000);
        
        assert_eq!(grant.base_amount, 1_000_000);
        assert_eq!(grant.reputation_bonus, 500_000);
        assert_eq!(grant.total_amount, 1_500_000);
    }

    #[test]
    fn test_grant_new_with_time_voting_end() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let current_time = 1000;
        let grant = Grant::new_with_time(params, current_time);
        
        // Voting end should be 7 days (604800 seconds) after creation
        assert_eq!(grant.voting_end, current_time + 604800);
    }

    #[test]
    fn test_grant_new_with_time_default_fields() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let grant = Grant::new_with_time(params, 1000);
        
        assert_eq!(grant.disbursed_amount, 0);
        assert_eq!(grant.total_votes, 0);
        assert_eq!(grant.total_yes_weight, 0);
        assert_eq!(grant.total_no_weight, 0);
        assert_eq!(grant.total_abstain_weight, 0);
        assert_eq!(grant.quorum_reached, false);
        assert_eq!(grant.grant_level, 1);
        assert_eq!(grant.voting_layer, VotingLayer::AuthorOnly);
        assert_eq!(grant.semantic_domain, None);
        assert_eq!(grant.semantic_domain_account, None);
        assert_eq!(grant.semantic_distance, None);
        assert_eq!(grant.phenomenon_membership, None);
    }

    #[test]
    fn test_grant_approve_with_time_different_timestamps() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Approve with different timestamp
        assert!(grant.approve_with_time(5000).is_ok());
        assert_eq!(grant.approved_at, Some(5000));
    }

    #[test]
    fn test_grant_approve_with_time_negative_timestamp() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Approve with negative timestamp (allowed, not validated)
        assert!(grant.approve_with_time(-1000).is_ok());
        assert_eq!(grant.approved_at, Some(-1000));
    }

    #[test]
    fn test_grant_approve_with_time_already_approved() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Approve once
        grant.approve_with_time(2000).unwrap();
        assert_eq!(grant.status, GrantStatus::Approved);
        
        // Try to approve again - should fail
        assert!(grant.approve_with_time(3000).is_err());
    }

    #[test]
    fn test_grant_activate_from_pending() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Try to activate from Pending - should fail
        assert_eq!(grant.status, GrantStatus::Pending);
        assert!(grant.activate().is_err());
    }

    #[test]
    fn test_grant_activate_from_active() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Approve and activate
        grant.approve_with_time(2000).unwrap();
        grant.activate().unwrap();
        assert_eq!(grant.status, GrantStatus::Active);
        
        // Try to activate again - should fail (already active)
        assert!(grant.activate().is_err());
    }

    #[test]
    fn test_grant_new_with_time_different_ids() {
        let mesh_group = create_test_pubkey(1);
        
        for id in 1..=10 {
            let params = GrantParams {
                id,
                idea_id: 10,
                mesh_group,
                category: GrantCategory::Research,
                grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: 1000,
                reputation_bonus: 200,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, 1000);
            assert_eq!(grant.id, id);
        }
    }

    #[test]
    fn test_grant_new_with_time_different_idea_ids() {
        let mesh_group = create_test_pubkey(1);
        
        for idea_id in 1..=10 {
            let params = GrantParams {
                id: 1,
                idea_id,
                mesh_group,
                category: GrantCategory::Research,
                grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: 1000,
                reputation_bonus: 200,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, 1000);
            assert_eq!(grant.idea_id, idea_id);
        }
    }

    #[test]
    fn test_grant_new_with_time_different_mesh_groups() {
        let mesh_groups: Vec<Pubkey> = (1..=5).map(|seed| create_test_pubkey(seed)).collect();
        
        for mesh_group in &mesh_groups {
            let params = GrantParams {
                id: 1,
                idea_id: 10,
                mesh_group: *mesh_group,
                category: GrantCategory::Research,
                grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: 1000,
                reputation_bonus: 200,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, 1000);
            assert_eq!(grant.mesh_group, *mesh_group);
        }
    }

    #[test]
    fn test_grant_new_with_time_created_at() {
        let mesh_group = create_test_pubkey(1);
        // Use timestamps that won't overflow when adding 604800 (7 days)
        let timestamps = vec![0, 1000, 2000, i64::MAX - 604800, -1000];
        
        for timestamp in timestamps {
            let params = GrantParams {
                id: 1,
                idea_id: 10,
                mesh_group,
                category: GrantCategory::Research,
                grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: 1000,
                reputation_bonus: 200,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, timestamp);
            assert_eq!(grant.created_at, timestamp);
        }
    }

    #[test]
    fn test_grant_approve_with_time_preserves_other_fields() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: Some(5),
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        let original_id = grant.id;
        let original_idea_id = grant.idea_id;
        let original_mesh_group = grant.mesh_group;
        let original_base_amount = grant.base_amount;
        let original_total_amount = grant.total_amount;
        
        grant.approve_with_time(2000).unwrap();
        
        // Other fields should be preserved
        assert_eq!(grant.id, original_id);
        assert_eq!(grant.idea_id, original_idea_id);
        assert_eq!(grant.mesh_group, original_mesh_group);
        assert_eq!(grant.base_amount, original_base_amount);
        assert_eq!(grant.total_amount, original_total_amount);
    }

    #[test]
    fn test_grant_activate_preserves_other_fields() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        grant.approve_with_time(2000).unwrap();
        let approved_at = grant.approved_at;
        
        grant.activate().unwrap();
        
        // approved_at should be preserved
        assert_eq!(grant.approved_at, approved_at);
        assert_eq!(grant.status, GrantStatus::Active);
    }

    #[test]
    fn test_grant_new_with_time_total_amount_calculation() {
        let mesh_group = create_test_pubkey(1);
        let test_cases = vec![
            (0u64, 0u64, 0u64),
            (1000u64, 200u64, 1200u64),
            (1_000_000u64, 500_000u64, 1_500_000u64),
            (u64::MAX, 0u64, u64::MAX),
        ];
        
        for (base, bonus, expected_total) in test_cases {
            let params = GrantParams {
                id: 1,
                idea_id: 10,
                mesh_group,
                category: GrantCategory::Research,
                grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: base,
                reputation_bonus: bonus,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, 1000);
            assert_eq!(grant.total_amount, expected_total);
        }
    }

    #[test]
    fn test_grant_new_with_time_voting_end_calculation() {
        let mesh_group = create_test_pubkey(1);
        let test_times = vec![0i64, 1000i64, 1000000i64];
        
        for current_time in test_times {
            let params = GrantParams {
                id: 1,
                idea_id: 10,
                mesh_group,
                category: GrantCategory::Research,
                grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
                base_amount: 1000,
                reputation_bonus: 200,
                milestone_id: None,
                bump: 255,
            };
            let grant = Grant::new_with_time(params, current_time);
            assert_eq!(grant.voting_end, current_time + 604800);
        }
    }

    #[test]
    fn test_grant_new_with_time_verification_status_default() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let grant = Grant::new_with_time(params, 1000);
        assert_eq!(grant.verification_status, VerificationStatus::Pending);
    }

    #[test]
    fn test_grant_new_with_time_commercialization_right_not_transferred() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let grant = Grant::new_with_time(params, 1000);
        assert_eq!(grant.commercialization_right_transferred, false);
    }

    #[test]
    fn test_grant_new_with_time_optional_fields() {
        let mesh_group = create_test_pubkey(1);
        
        // With milestone_id
        let params_with_milestone = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Core,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: Some(5),
            bump: 255,
        };
        let grant_with = Grant::new_with_time(params_with_milestone, 1000);
        assert_eq!(grant_with.milestone_id, Some(5));
        
        // Without milestone_id
        let params_without = GrantParams {
            id: 2,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let grant_without = Grant::new_with_time(params_without, 1000);
        assert_eq!(grant_without.milestone_id, None);
    }

    #[test]
    fn test_grant_new_with_time_all_fields_comprehensive() {
        let mesh_group = create_test_pubkey(5);
        let params = GrantParams {
            id: 999,
            idea_id: 888,
            mesh_group,
            category: GrantCategory::Development,
            grant_type: GrantType::Core,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 5000,
            reputation_bonus: 1000,
            milestone_id: Some(42),
            bump: 128,
        };
        let grant = Grant::new_with_time(params, 5000);
        
        assert_eq!(grant.id, 999);
        assert_eq!(grant.idea_id, 888);
        assert_eq!(grant.mesh_group, mesh_group);
        assert_eq!(grant.category, GrantCategory::Development);
        assert_eq!(grant.grant_type, GrantType::Core);
        assert_eq!(grant.base_amount, 5000);
        assert_eq!(grant.reputation_bonus, 1000);
        assert_eq!(grant.total_amount, 6000);
        assert_eq!(grant.disbursed_amount, 0);
        assert_eq!(grant.status, GrantStatus::Pending);
        assert_eq!(grant.milestone_id, Some(42));
        assert_eq!(grant.verification_status, VerificationStatus::Pending);
        assert_eq!(grant.commercialization_right_transferred, false);
        assert_eq!(grant.created_at, 5000);
        assert_eq!(grant.approved_at, None);
        assert_eq!(grant.completed_at, None);
        assert_eq!(grant.voting_end, 5000 + 604800);
        assert_eq!(grant.bump, 128);
    }

    #[test]
    fn test_grant_approve_with_time_preserves_other_fields_comprehensive() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        let original_id = grant.id;
        let original_idea_id = grant.idea_id;
        let original_mesh_group = grant.mesh_group;
        let original_total_amount = grant.total_amount;
        let original_bump = grant.bump;
        
        assert!(grant.approve_with_time(2000).is_ok());
        
        assert_eq!(grant.id, original_id);
        assert_eq!(grant.idea_id, original_idea_id);
        assert_eq!(grant.mesh_group, original_mesh_group);
        assert_eq!(grant.total_amount, original_total_amount);
        assert_eq!(grant.bump, original_bump);
        assert_eq!(grant.status, GrantStatus::Approved);
        assert_eq!(grant.approved_at, Some(2000));
    }

}
