//! Grant disbursement methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::GrantStatus;
use super::Grant;

impl Grant {
    /// Disburse grant funds
    ///
    /// Updates disbursed_amount and completes grant if fully disbursed.
    pub fn disburse(&mut self, amount: u64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.disburse_with_time(amount, current_time)
    }

    /// Disburse grant funds with explicit timestamp
    pub fn disburse_with_time(&mut self, amount: u64, current_time: i64) -> Result<()> {
        require!(self.status == GrantStatus::Active, IndrasError::InvalidState);
        
        let new_disbursed = self.disbursed_amount
            .checked_add(amount)
            .ok_or(error!(IndrasError::Overflow))?;
        require!(new_disbursed <= self.total_amount, IndrasError::InsufficientFunds);
        
        self.disbursed_amount = new_disbursed;
        
        // If fully disbursed, complete the grant
        if self.disbursed_amount >= self.total_amount {
            self.status = GrantStatus::Completed;
            self.completed_at = Some(current_time);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use crate::state::grant::lifecycle::GrantParams;
    use crate::state::grant::types::{GrantCategory, GrantType, GrantDisbursementType};

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_active_grant() -> Grant {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            disbursement_type: GrantDisbursementType::Standard,
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        grant.approve_with_time(2000).unwrap();
        grant.activate().unwrap();
        grant
    }

    #[test]
    fn test_grant_disburse() {
        let mut grant = create_active_grant();
        
        assert_eq!(grant.disbursed_amount, 0);
        assert!(grant.disburse_with_time(500, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 500);
        assert_eq!(grant.status, GrantStatus::Active); // Not completed yet
    }

    #[test]
    fn test_grant_disburse_complete() {
        let mut grant = create_active_grant();
        
        // Disburse full amount
        assert!(grant.disburse_with_time(1200, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 1200);
        assert_eq!(grant.status, GrantStatus::Completed);
        assert_eq!(grant.completed_at, Some(3000));
    }

    #[test]
    fn test_grant_disburse_partial_then_complete() {
        let mut grant = create_active_grant();
        
        // First partial disbursement
        assert!(grant.disburse_with_time(600, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 600);
        assert_eq!(grant.status, GrantStatus::Active);
        
        // Second disbursement completes it
        assert!(grant.disburse_with_time(600, 4000).is_ok());
        assert_eq!(grant.disbursed_amount, 1200);
        assert_eq!(grant.status, GrantStatus::Completed);
        assert_eq!(grant.completed_at, Some(4000));
    }

    #[test]
    fn test_grant_disburse_exceeds_total() {
        let mut grant = create_active_grant();
        
        // Try to disburse more than total - should fail
        assert!(grant.disburse_with_time(1300, 3000).is_err());
        assert_eq!(grant.disbursed_amount, 0);
    }

    #[test]
    fn test_grant_disburse_invalid_state() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            disbursement_type: GrantDisbursementType::Standard,
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        
        // Try to disburse non-active grant - should fail
        assert!(grant.disburse_with_time(500, 3000).is_err());
    }

    #[test]
    fn test_grant_disburse_overflow() {
        let mut grant = create_active_grant();
        
        // Set disbursed_amount near max to test overflow
        grant.disbursed_amount = u64::MAX - 100;
        
        // Try to disburse amount that would overflow - should fail
        assert!(grant.disburse_with_time(200, 3000).is_err());
    }

    #[test]
    fn test_grant_disburse_exact_total() {
        let mut grant = create_active_grant();
        
        // Disburse exactly the total amount
        assert!(grant.disburse_with_time(1200, 5000).is_ok());
        assert_eq!(grant.disbursed_amount, 1200);
        assert_eq!(grant.status, GrantStatus::Completed);
        assert_eq!(grant.completed_at, Some(5000));
    }

    #[test]
    fn test_grant_disburse_multiple_partial() {
        let mut grant = create_active_grant();
        
        // Multiple partial disbursements
        assert!(grant.disburse_with_time(300, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 300);
        assert_eq!(grant.status, GrantStatus::Active);
        
        assert!(grant.disburse_with_time(400, 4000).is_ok());
        assert_eq!(grant.disbursed_amount, 700);
        assert_eq!(grant.status, GrantStatus::Active);
        
        assert!(grant.disburse_with_time(500, 5000).is_ok());
        assert_eq!(grant.disbursed_amount, 1200);
        assert_eq!(grant.status, GrantStatus::Completed);
        assert_eq!(grant.completed_at, Some(5000));
    }

    #[test]
    fn test_grant_disburse_zero_amount() {
        let mut grant = create_active_grant();
        
        // Disburse zero amount - should succeed but not change anything
        assert!(grant.disburse_with_time(0, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 0);
        assert_eq!(grant.status, GrantStatus::Active);
    }

    #[test]
    fn test_grant_disburse_one_short_of_complete() {
        let mut grant = create_active_grant();
        
        // Disburse one less than total - should not complete
        assert!(grant.disburse_with_time(1199, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 1199);
        assert_eq!(grant.status, GrantStatus::Active);
        assert_eq!(grant.completed_at, None);
    }

    #[test]
    fn test_grant_disburse_one_more_than_complete() {
        let mut grant = create_active_grant();
        
        // Disburse one more than total - should fail
        assert!(grant.disburse_with_time(1201, 3000).is_err());
        assert_eq!(grant.disbursed_amount, 0);
        assert_eq!(grant.status, GrantStatus::Active);
    }

    #[test]
    fn test_grant_disburse_after_partial_exceeds_remaining() {
        let mut grant = create_active_grant();
        
        // First partial disbursement
        assert!(grant.disburse_with_time(800, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 800);
        
        // Try to disburse more than remaining (400) - should fail
        assert!(grant.disburse_with_time(500, 4000).is_err());
        assert_eq!(grant.disbursed_amount, 800); // Should remain unchanged
    }

    #[test]
    fn test_grant_disburse_timestamp_preserved() {
        let mut grant = create_active_grant();
        
        // Disburse with specific timestamp
        assert!(grant.disburse_with_time(1200, 7777).is_ok());
        assert_eq!(grant.completed_at, Some(7777));
    }

    #[test]
    fn test_grant_disburse_status_not_active() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            disbursement_type: GrantDisbursementType::Standard,
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            base_amount: 1000,
            reputation_bonus: 200,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        grant.approve_with_time(2000).unwrap();
        // Grant is approved but not activated
        
        // Try to disburse - should fail (not active)
        assert!(grant.disburse_with_time(500, 3000).is_err());
    }

    #[test]
    fn test_grant_disburse_already_completed() {
        let mut grant = create_active_grant();
        
        // Complete the grant first
        assert!(grant.disburse_with_time(1200, 3000).is_ok());
        assert_eq!(grant.status, GrantStatus::Completed);
        
        // Try to disburse again - should fail (not active anymore)
        assert!(grant.disburse_with_time(100, 4000).is_err());
        assert_eq!(grant.disbursed_amount, 1200); // Should remain unchanged
    }

    #[test]
    fn test_grant_disburse_large_amount() {
        let mesh_group = create_test_pubkey(1);
        let params = GrantParams {
            disbursement_type: GrantDisbursementType::Standard,
            id: 1,
            idea_id: 10,
            mesh_group,
            category: GrantCategory::Research,
            grant_type: GrantType::Initial,
            base_amount: 1_000_000_000,
            reputation_bonus: 0,
            milestone_id: None,
            bump: 255,
        };
        let mut grant = Grant::new_with_time(params, 1000);
        grant.approve_with_time(2000).unwrap();
        grant.activate().unwrap();
        
        // Disburse large amount
        assert!(grant.disburse_with_time(500_000_000, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 500_000_000);
        assert_eq!(grant.status, GrantStatus::Active);
    }

    #[test]
    fn test_grant_disburse_multiple_partial_disbursements() {
        let mut grant = create_active_grant();
        
        // Multiple partial disbursements
        assert!(grant.disburse_with_time(100, 3000).is_ok());
        assert_eq!(grant.disbursed_amount, 100);
        assert_eq!(grant.status, GrantStatus::Active);
        
        assert!(grant.disburse_with_time(200, 4000).is_ok());
        assert_eq!(grant.disbursed_amount, 300);
        assert_eq!(grant.status, GrantStatus::Active);
        
        assert!(grant.disburse_with_time(300, 5000).is_ok());
        assert_eq!(grant.disbursed_amount, 600);
        assert_eq!(grant.status, GrantStatus::Active);
        
        // Final disbursement completes it
        assert!(grant.disburse_with_time(600, 6000).is_ok());
        assert_eq!(grant.disbursed_amount, 1200);
        assert_eq!(grant.status, GrantStatus::Completed);
        assert_eq!(grant.completed_at, Some(6000));
    }

    #[test]
    fn test_grant_disburse_overflow_protection() {
        let mut grant = create_active_grant();
        grant.disbursed_amount = u64::MAX - 100;
        
        // Try to disburse amount that would cause overflow
        assert!(grant.disburse_with_time(200, 3000).is_err());
        assert_eq!(grant.disbursed_amount, u64::MAX - 100); // Should remain unchanged
    }

    #[test]
    fn test_grant_disburse_preserves_other_fields() {
        let mut grant = create_active_grant();
        let original_id = grant.id;
        let original_idea_id = grant.idea_id;
        let original_mesh_group = grant.mesh_group;
        let original_total_amount = grant.total_amount;
        let original_bump = grant.bump;
        
        assert!(grant.disburse_with_time(500, 3000).is_ok());
        
        assert_eq!(grant.id, original_id);
        assert_eq!(grant.idea_id, original_idea_id);
        assert_eq!(grant.mesh_group, original_mesh_group);
        assert_eq!(grant.total_amount, original_total_amount);
        assert_eq!(grant.bump, original_bump);
        assert_eq!(grant.disbursed_amount, 500);
    }
}
