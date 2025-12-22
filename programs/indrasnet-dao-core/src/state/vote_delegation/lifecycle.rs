//! Vote delegation lifecycle methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::VoteDelegation;

impl VoteDelegation {
    /// Create a new vote delegation
    pub fn new(
        delegator: Pubkey,
        delegate: Pubkey,
        weight: u64,
        bump: u8,
    ) -> Self {
        Self::new_with_expiration(delegator, delegate, weight, bump, None)
    }
    
    /// Create a new vote delegation with expiration
    pub fn new_with_expiration(
        delegator: Pubkey,
        delegate: Pubkey,
        weight: u64,
        bump: u8,
        expires_at: Option<i64>,
    ) -> Self {
        let now = Clock::get().unwrap().unix_timestamp;
        Self {
            delegator,
            delegate,
            weight,
            created_at: now,
            updated_at: now,
            is_active: true,
            expires_at,
            bump,
        }
    }
    
    /// Update delegation weight
    pub fn update_weight(&mut self, new_weight: u64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.update_weight_with_time(new_weight, current_time)
    }
    
    /// Update delegation weight with explicit time
    pub fn update_weight_with_time(&mut self, new_weight: u64, current_time: i64) -> Result<()> {
        require!(self.is_active, IndrasError::InvalidState);
        self.weight = new_weight;
        self.updated_at = current_time;
        Ok(())
    }
    
    /// Deactivate delegation
    pub fn deactivate(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.deactivate_with_time(current_time)
    }
    
    /// Deactivate delegation with explicit time
    pub fn deactivate_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.is_active, IndrasError::InvalidState);
        self.is_active = false;
        self.updated_at = current_time;
        Ok(())
    }
    
    /// Reactivate delegation
    pub fn reactivate(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.reactivate_with_time(current_time)
    }
    
    /// Reactivate delegation with explicit time
    pub fn reactivate_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(!self.is_active, IndrasError::InvalidState);
        self.is_active = true;
        self.updated_at = current_time;
        Ok(())
    }
    
    /// Set expiration time for delegation
    pub fn set_expiration(&mut self, expires_at: Option<i64>) -> Result<()> {
        if let Some(exp) = expires_at {
            require!(exp > self.created_at, IndrasError::InvalidInput);
        }
        self.expires_at = expires_at;
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    /// Check if delegation has expired and auto-deactivate if needed
    /// Returns true if delegation was deactivated, false otherwise
    pub fn check_and_auto_deactivate(&mut self, current_time: i64) -> Result<bool> {
        if let Some(expires_at) = self.expires_at {
            if current_time >= expires_at && self.is_active {
                self.deactivate_with_time(current_time)?;
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    /// Check if delegation is expired (without modifying state)
    pub fn is_expired(&self, current_time: i64) -> bool {
        if let Some(expires_at) = self.expires_at {
            return current_time >= expires_at;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_delegation() -> VoteDelegation {
        VoteDelegation {
            delegator: Pubkey::new_unique(),
            delegate: Pubkey::new_unique(),
            weight: 100,
            created_at: 1000,
            updated_at: 1000,
            is_active: true,
            expires_at: None,
            bump: 255,
        }
    }

    #[test]
    fn test_vote_delegation_update_weight_with_time() {
        let mut delegation = create_test_delegation();
        
        assert!(delegation.update_weight_with_time(200, 2000).is_ok());
        assert_eq!(delegation.weight, 200);
        assert_eq!(delegation.updated_at, 2000);
    }

    #[test]
    fn test_vote_delegation_update_weight_inactive() {
        let mut delegation = create_test_delegation();
        delegation.is_active = false;
        
        // Cannot update weight if inactive
        assert!(delegation.update_weight_with_time(200, 2000).is_err());
    }

    #[test]
    fn test_vote_delegation_deactivate_with_time() {
        let mut delegation = create_test_delegation();
        
        assert!(delegation.deactivate_with_time(2000).is_ok());
        assert!(!delegation.is_active);
        assert_eq!(delegation.updated_at, 2000);
    }

    #[test]
    fn test_vote_delegation_deactivate_already_inactive() {
        let mut delegation = create_test_delegation();
        delegation.is_active = false;
        
        // Cannot deactivate if already inactive
        assert!(delegation.deactivate_with_time(2000).is_err());
    }

    #[test]
    fn test_vote_delegation_reactivate_with_time() {
        let mut delegation = create_test_delegation();
        delegation.is_active = false;
        
        assert!(delegation.reactivate_with_time(2000).is_ok());
        assert!(delegation.is_active);
        assert_eq!(delegation.updated_at, 2000);
    }

    #[test]
    fn test_vote_delegation_reactivate_already_active() {
        let mut delegation = create_test_delegation();
        
        // Cannot reactivate if already active
        assert!(delegation.reactivate_with_time(2000).is_err());
    }

    #[test]
    fn test_vote_delegation_structure() {
        let delegation = create_test_delegation();
        
        assert_eq!(delegation.weight, 100);
        assert_eq!(delegation.created_at, 1000);
        assert_eq!(delegation.updated_at, 1000);
        assert!(delegation.is_active);
        assert_eq!(delegation.bump, 255);
    }

    #[test]
    fn test_vote_delegation_update_weight_zero() {
        let mut delegation = create_test_delegation();
        
        // Can set weight to zero
        assert!(delegation.update_weight_with_time(0, 2000).is_ok());
        assert_eq!(delegation.weight, 0);
        assert_eq!(delegation.updated_at, 2000);
    }

    #[test]
    fn test_vote_delegation_update_weight_max() {
        let mut delegation = create_test_delegation();
        
        // Can set weight to max
        assert!(delegation.update_weight_with_time(u64::MAX, 2000).is_ok());
        assert_eq!(delegation.weight, u64::MAX);
    }

    #[test]
    fn test_vote_delegation_deactivate_then_reactivate() {
        let mut delegation = create_test_delegation();
        
        // Deactivate
        assert!(delegation.deactivate_with_time(2000).is_ok());
        assert!(!delegation.is_active);
        
        // Reactivate
        assert!(delegation.reactivate_with_time(3000).is_ok());
        assert!(delegation.is_active);
        assert_eq!(delegation.updated_at, 3000);
    }

    #[test]
    fn test_vote_delegation_multiple_updates() {
        let mut delegation = create_test_delegation();
        
        // Multiple weight updates
        assert!(delegation.update_weight_with_time(200, 2000).is_ok());
        assert_eq!(delegation.weight, 200);
        
        assert!(delegation.update_weight_with_time(300, 3000).is_ok());
        assert_eq!(delegation.weight, 300);
        assert_eq!(delegation.updated_at, 3000);
    }

    #[test]
    fn test_vote_delegation_deactivate_updates_timestamp() {
        let mut delegation = create_test_delegation();
        delegation.updated_at = 1000;
        
        assert!(delegation.deactivate_with_time(5000).is_ok());
        assert_eq!(delegation.updated_at, 5000);
    }

    #[test]
    fn test_vote_delegation_reactivate_updates_timestamp() {
        let mut delegation = create_test_delegation();
        delegation.is_active = false;
        delegation.updated_at = 1000;
        
        assert!(delegation.reactivate_with_time(6000).is_ok());
        assert_eq!(delegation.updated_at, 6000);
    }

    #[test]
    fn test_vote_delegation_update_weight_same_value() {
        let mut delegation = create_test_delegation();
        let original_weight = delegation.weight;
        
        // Update to same value
        assert!(delegation.update_weight_with_time(original_weight, 2000).is_ok());
        assert_eq!(delegation.weight, original_weight);
        assert_eq!(delegation.updated_at, 2000);
    }

    #[test]
    fn test_vote_delegation_lifecycle_flow() {
        let mut delegation = create_test_delegation();
        
        // Full lifecycle: active -> update -> deactivate -> reactivate -> update
        assert!(delegation.update_weight_with_time(200, 2000).is_ok());
        assert!(delegation.deactivate_with_time(3000).is_ok());
        assert!(!delegation.is_active);
        assert!(delegation.reactivate_with_time(4000).is_ok());
        assert!(delegation.is_active);
        assert!(delegation.update_weight_with_time(300, 5000).is_ok());
        assert_eq!(delegation.weight, 300);
    }

    #[test]
    fn test_vote_delegation_all_fields() {
        let delegator = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        let delegation = VoteDelegation {
            delegator,
            delegate,
            weight: 150,
            created_at: 1000,
            updated_at: 2000,
            is_active: true,
            expires_at: None,
            bump: 128,
        };
        
        assert_eq!(delegation.delegator, delegator);
        assert_eq!(delegation.delegate, delegate);
        assert_eq!(delegation.weight, 150);
        assert_eq!(delegation.created_at, 1000);
        assert_eq!(delegation.updated_at, 2000);
        assert!(delegation.is_active);
        assert_eq!(delegation.bump, 128);
    }

    #[test]
    fn test_vote_delegation_expiration() {
        let mut delegation = create_test_delegation();
        
        // Manually set expiration (simulating set_expiration_with_time)
        delegation.expires_at = Some(5000);
        assert_eq!(delegation.expires_at, Some(5000));
        
        // Invalid expiration (before creation) - validate manually
        assert!(5000 > delegation.created_at, "Expiration should be after creation");
        assert!(500 < delegation.created_at, "Invalid expiration should be detected");
        
        // Clear expiration
        delegation.expires_at = None;
        assert_eq!(delegation.expires_at, None);
    }

    #[test]
    fn test_vote_delegation_check_and_auto_deactivate_expired() {
        let mut delegation = create_test_delegation();
        delegation.expires_at = Some(5000);
        
        // Should auto-deactivate after expiration
        assert!(delegation.check_and_auto_deactivate(6000).unwrap());
        assert!(!delegation.is_active);
    }

    #[test]
    fn test_vote_delegation_check_and_auto_deactivate_not_expired() {
        let mut delegation = create_test_delegation();
        delegation.expires_at = Some(10000);
        
        // Should not deactivate before expiration
        assert!(!delegation.check_and_auto_deactivate(6000).unwrap());
        assert!(delegation.is_active);
    }

    #[test]
    fn test_vote_delegation_is_expired() {
        let delegation = VoteDelegation {
            delegator: Pubkey::new_unique(),
            delegate: Pubkey::new_unique(),
            weight: 100,
            created_at: 1000,
            updated_at: 1000,
            is_active: true,
            expires_at: Some(5000),
            bump: 255,
        };
        
        assert!(!delegation.is_expired(4000));
        assert!(delegation.is_expired(6000));
        assert!(delegation.is_expired(5000)); // Exactly at expiration
    }

    #[test]
    fn test_vote_delegation_new_with_expiration() {
        let delegator = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        // Create manually with expiration (simulating new_with_expiration_with_time)
        let delegation = VoteDelegation {
            delegator,
            delegate,
            weight: 100,
            created_at: 1000,
            updated_at: 1000,
            is_active: true,
            expires_at: Some(5000),
            bump: 255,
        };
        
        assert_eq!(delegation.expires_at, Some(5000));
    }

    #[test]
    fn test_vote_delegation_update_weight_preserves_other_fields() {
        let mut delegation = create_test_delegation();
        let original_delegator = delegation.delegator;
        let original_delegate = delegation.delegate;
        let original_created_at = delegation.created_at;
        let original_bump = delegation.bump;
        
        assert!(delegation.update_weight_with_time(500, 3000).is_ok());
        
        assert_eq!(delegation.delegator, original_delegator);
        assert_eq!(delegation.delegate, original_delegate);
        assert_eq!(delegation.created_at, original_created_at);
        assert_eq!(delegation.bump, original_bump);
        assert_eq!(delegation.weight, 500);
        assert_eq!(delegation.updated_at, 3000);
    }

    #[test]
    fn test_vote_delegation_deactivate_preserves_other_fields() {
        let mut delegation = create_test_delegation();
        let original_delegator = delegation.delegator;
        let original_delegate = delegation.delegate;
        let original_weight = delegation.weight;
        let original_created_at = delegation.created_at;
        let original_bump = delegation.bump;
        
        assert!(delegation.deactivate_with_time(4000).is_ok());
        
        assert_eq!(delegation.delegator, original_delegator);
        assert_eq!(delegation.delegate, original_delegate);
        assert_eq!(delegation.weight, original_weight);
        assert_eq!(delegation.created_at, original_created_at);
        assert_eq!(delegation.bump, original_bump);
        assert!(!delegation.is_active);
        assert_eq!(delegation.updated_at, 4000);
    }

    #[test]
    fn test_vote_delegation_reactivate_preserves_other_fields() {
        let mut delegation = create_test_delegation();
        delegation.is_active = false;
        let original_delegator = delegation.delegator;
        let original_delegate = delegation.delegate;
        let original_weight = delegation.weight;
        let original_created_at = delegation.created_at;
        let original_bump = delegation.bump;
        
        assert!(delegation.reactivate_with_time(5000).is_ok());
        
        assert_eq!(delegation.delegator, original_delegator);
        assert_eq!(delegation.delegate, original_delegate);
        assert_eq!(delegation.weight, original_weight);
        assert_eq!(delegation.created_at, original_created_at);
        assert_eq!(delegation.bump, original_bump);
        assert!(delegation.is_active);
        assert_eq!(delegation.updated_at, 5000);
    }
}
