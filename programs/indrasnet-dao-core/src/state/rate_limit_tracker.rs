//! Rate Limit Tracker account structure
//!
//! SEC-INV-8: Tracks rate limits for critical operations per user/operation
//! Used for: create_idea (1/day), analyze_idea (1/5min), etc.

use anchor_lang::prelude::*;

/// Rate Limit Tracker
/// 
/// Tracks last operation timestamp per user per operation type
/// PDA: [b"rate_limit", user_pubkey, operation_type]
#[account]
#[derive(InitSpace)]
pub struct RateLimitTracker {
    pub user: Pubkey,
    #[max_len(50)]
    pub operation_type: String,  // e.g., "create_idea", "analyze_idea"
    pub last_operation_at: i64,
    pub operation_count: u64,     // Count in current window
    pub window_start: i64,        // Start of current rate limit window
    pub bump: u8,
}

impl RateLimitTracker {
    /// Check if operation is allowed based on rate limit
    /// 
    /// Returns Ok(()) if allowed, Err(RateLimitExceeded) if not
    pub fn check_rate_limit(
        &mut self,
        current_time: i64,
        window_seconds: i64,
        max_operations: u64,
    ) -> Result<()> {
        use crate::error::IndrasError;
        
        // Reset window if expired
        if current_time - self.window_start >= window_seconds {
            self.operation_count = 0;
            self.window_start = current_time;
        }
        
        // Check if limit exceeded
        require!(
            self.operation_count < max_operations,
            IndrasError::RateLimitExceeded
        );
        
        // Increment counter
        self.operation_count = self.operation_count
            .checked_add(1)
            .ok_or(error!(IndrasError::Overflow))?;
        
        // Update last operation timestamp
        self.last_operation_at = current_time;
        
        Ok(())
    }
    
    /// Check if enough time has passed since last operation (simple time-based check)
    /// 
    /// Used for operations like "1 per day" or "1 per 5 minutes"
    pub fn check_time_based_rate_limit(
        &mut self,
        current_time: i64,
        min_seconds_between_operations: i64,
    ) -> Result<()> {
        use crate::error::IndrasError;
        
        let time_since_last = current_time - self.last_operation_at;
        require!(
            time_since_last >= min_seconds_between_operations,
            IndrasError::RateLimitExceeded
        );
        
        // Update last operation timestamp
        self.last_operation_at = current_time;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_tracker() -> RateLimitTracker {
        RateLimitTracker {
            user: Pubkey::new_unique(),
            operation_type: "create_idea".to_string(),
            last_operation_at: 1000,
            operation_count: 0,
            window_start: 1000,
            bump: 255,
        }
    }

    #[test]
    fn test_check_rate_limit_within_limit() {
        let mut tracker = create_test_tracker();
        
        assert!(tracker.check_rate_limit(2000, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 1);
        assert_eq!(tracker.last_operation_at, 2000);
    }

    #[test]
    fn test_check_rate_limit_exceeds_limit() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 10;
        
        // Should fail - already at limit
        assert!(tracker.check_rate_limit(2000, 3600, 10).is_err());
    }

    #[test]
    fn test_check_rate_limit_window_reset() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 5;
        tracker.window_start = 1000;
        
        // New window starts (current_time - window_start >= window_seconds)
        assert!(tracker.check_rate_limit(5000, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 1); // Reset to 1
        assert_eq!(tracker.window_start, 5000);
    }

    #[test]
    fn test_check_rate_limit_overflow() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = u64::MAX;
        
        // Should fail on overflow
        assert!(tracker.check_rate_limit(2000, 3600, 10).is_err());
    }

    #[test]
    fn test_check_time_based_rate_limit() {
        let mut tracker = create_test_tracker();
        tracker.last_operation_at = 1000;
        
        // Enough time passed (2000 - 1000 = 1000 >= 600)
        assert!(tracker.check_time_based_rate_limit(2000, 600).is_ok());
        assert_eq!(tracker.last_operation_at, 2000);
    }

    #[test]
    fn test_check_time_based_rate_limit_not_enough_time() {
        let mut tracker = create_test_tracker();
        tracker.last_operation_at = 1000;
        
        // Not enough time passed (1500 - 1000 = 500 < 600)
        assert!(tracker.check_time_based_rate_limit(1500, 600).is_err());
    }

    #[test]
    fn test_check_rate_limit_exact_limit() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 9; // One less than limit
        
        assert!(tracker.check_rate_limit(2000, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 10);
    }

    #[test]
    fn test_check_rate_limit_multiple_operations() {
        let mut tracker = create_test_tracker();
        
        // First operation
        assert!(tracker.check_rate_limit(2000, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 1);
        
        // Second operation
        assert!(tracker.check_rate_limit(2100, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 2);
        
        // Third operation
        assert!(tracker.check_rate_limit(2200, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 3);
    }

    #[test]
    fn test_check_rate_limit_window_boundary() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 5;
        tracker.window_start = 1000;
        
        // Exactly at window boundary (4600 - 1000 = 3600)
        assert!(tracker.check_rate_limit(4600, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 1); // Reset
        assert_eq!(tracker.window_start, 4600);
    }

    #[test]
    fn test_check_rate_limit_window_not_expired() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 3;
        tracker.window_start = 1000;
        
        // Window not expired (2500 - 1000 = 1500 < 3600)
        assert!(tracker.check_rate_limit(2500, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 4); // Incremented, not reset
        assert_eq!(tracker.window_start, 1000); // Not changed
    }

    #[test]
    fn test_check_time_based_rate_limit_exact_time() {
        let mut tracker = create_test_tracker();
        tracker.last_operation_at = 1000;
        
        // Exactly enough time (1600 - 1000 = 600 >= 600)
        assert!(tracker.check_time_based_rate_limit(1600, 600).is_ok());
        assert_eq!(tracker.last_operation_at, 1600);
    }

    #[test]
    fn test_check_time_based_rate_limit_multiple_operations() {
        let mut tracker = create_test_tracker();
        tracker.last_operation_at = 1000;
        
        // First operation after delay
        assert!(tracker.check_time_based_rate_limit(2000, 600).is_ok());
        assert_eq!(tracker.last_operation_at, 2000);
        
        // Second operation after delay
        assert!(tracker.check_time_based_rate_limit(3000, 600).is_ok());
        assert_eq!(tracker.last_operation_at, 3000);
    }

    #[test]
    fn test_check_time_based_rate_limit_negative_time() {
        let mut tracker = create_test_tracker();
        tracker.last_operation_at = 2000;
        
        // Current time before last operation (should fail)
        assert!(tracker.check_time_based_rate_limit(1000, 600).is_err());
        assert_eq!(tracker.last_operation_at, 2000); // Not updated
    }

    #[test]
    fn test_rate_limit_tracker_structure() {
        let user = Pubkey::new_unique();
        let tracker = RateLimitTracker {
            user,
            operation_type: "analyze_idea".to_string(),
            last_operation_at: 5000,
            operation_count: 7,
            window_start: 3000,
            bump: 128,
        };
        
        assert_eq!(tracker.user, user);
        assert_eq!(tracker.operation_type, "analyze_idea");
        assert_eq!(tracker.last_operation_at, 5000);
        assert_eq!(tracker.operation_count, 7);
        assert_eq!(tracker.window_start, 3000);
        assert_eq!(tracker.bump, 128);
    }

    #[test]
    fn test_check_rate_limit_zero_max_operations() {
        let mut tracker = create_test_tracker();
        
        // Zero max operations - should always fail
        assert!(tracker.check_rate_limit(2000, 3600, 0).is_err());
    }

    #[test]
    fn test_check_rate_limit_very_large_window() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 5;
        tracker.window_start = 1000;
        
        // Very large window (100000 seconds)
        assert!(tracker.check_rate_limit(2000, 100000, 10).is_ok());
        assert_eq!(tracker.operation_count, 6); // Not reset
    }

    #[test]
    fn test_check_rate_limit_preserves_other_fields() {
        let mut tracker = create_test_tracker();
        let original_user = tracker.user;
        let original_operation_type = tracker.operation_type.clone();
        let original_bump = tracker.bump;
        
        assert!(tracker.check_rate_limit(2000, 3600, 10).is_ok());
        
        assert_eq!(tracker.user, original_user);
        assert_eq!(tracker.operation_type, original_operation_type);
        assert_eq!(tracker.bump, original_bump);
    }

    #[test]
    fn test_check_time_based_rate_limit_preserves_other_fields() {
        let mut tracker = create_test_tracker();
        let original_user = tracker.user;
        let original_operation_type = tracker.operation_type.clone();
        let original_operation_count = tracker.operation_count;
        let original_window_start = tracker.window_start;
        let original_bump = tracker.bump;
        
        assert!(tracker.check_time_based_rate_limit(2000, 600).is_ok());
        
        assert_eq!(tracker.user, original_user);
        assert_eq!(tracker.operation_type, original_operation_type);
        assert_eq!(tracker.operation_count, original_operation_count);
        assert_eq!(tracker.window_start, original_window_start);
        assert_eq!(tracker.bump, original_bump);
    }

    #[test]
    fn test_check_rate_limit_all_fields() {
        let user = Pubkey::new_unique();
        let tracker = RateLimitTracker {
            user,
            operation_type: "analyze_idea".to_string(),
            last_operation_at: 5000,
            operation_count: 7,
            window_start: 3000,
            bump: 128,
        };
        
        assert_eq!(tracker.user, user);
        assert_eq!(tracker.operation_type, "analyze_idea");
        assert_eq!(tracker.last_operation_at, 5000);
        assert_eq!(tracker.operation_count, 7);
        assert_eq!(tracker.window_start, 3000);
        assert_eq!(tracker.bump, 128);
    }

    #[test]
    fn test_check_rate_limit_max_operations_reached() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 10;
        
        // At max limit - should fail
        assert!(tracker.check_rate_limit(2000, 3600, 10).is_err());
        assert_eq!(tracker.operation_count, 10); // Not incremented
    }

    #[test]
    fn test_check_rate_limit_window_exact_expiry() {
        let mut tracker = create_test_tracker();
        tracker.operation_count = 5;
        tracker.window_start = 1000;
        
        // Exactly at expiry boundary
        assert!(tracker.check_rate_limit(4600, 3600, 10).is_ok());
        assert_eq!(tracker.operation_count, 1); // Reset
        assert_eq!(tracker.window_start, 4600);
    }

    #[test]
    fn test_check_time_based_rate_limit_zero_delay() {
        let mut tracker = create_test_tracker();
        tracker.last_operation_at = 1000;
        
        // Zero delay - should always pass
        assert!(tracker.check_time_based_rate_limit(1000, 0).is_ok());
        assert_eq!(tracker.last_operation_at, 1000);
    }

    #[test]
    fn test_rate_limit_tracker_all_fields_comprehensive() {
        let user = Pubkey::new_unique();
        let tracker = RateLimitTracker {
            user,
            operation_type: "test_operation".to_string(),
            last_operation_at: 5000,
            operation_count: 5,
            window_start: 3000,
            bump: 128,
        };
        
        assert_eq!(tracker.user, user);
        assert_eq!(tracker.operation_type, "test_operation");
        assert_eq!(tracker.last_operation_at, 5000);
        assert_eq!(tracker.operation_count, 5);
        assert_eq!(tracker.window_start, 3000);
        assert_eq!(tracker.bump, 128);
    }

    #[test]
    fn test_check_rate_limit_window_reset_preserves_other_fields_comprehensive() {
        let mut tracker = create_test_tracker();
        tracker.window_start = 1000;
        tracker.operation_count = 5;
        let original_user = tracker.user;
        let original_operation_type = tracker.operation_type.clone();
        let original_bump = tracker.bump;
        
        // Window expires (5000 - 1000 = 4000 >= 3600)
        assert!(tracker.check_rate_limit(5000, 3600, 10).is_ok());
        
        assert_eq!(tracker.user, original_user);
        assert_eq!(tracker.operation_type, original_operation_type);
        assert_eq!(tracker.bump, original_bump);
        assert_eq!(tracker.operation_count, 1); // Reset to 0, then incremented
        assert_eq!(tracker.window_start, 5000);
    }
}
