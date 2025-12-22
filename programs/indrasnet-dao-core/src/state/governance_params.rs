//! Governance Parameters module
//!
//! Adaptive governance parameters:
//! - quorum_percentage - quorum percentage (0-100)
//! - vote_duration_hours - voting duration in hours
//! - delegate_weight_percentage - delegate weight (0-100)
//! - early_quorum_enabled - early quorum enabled

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Adaptive governance parameters
///
/// Governance parameters are not static - they adapt based on metrics and AI recommendations.
/// This is a shift from "code as law" to "code as living process".
#[account]
#[derive(InitSpace)]
pub struct GovernanceParams {
    pub quorum_percentage: u8,              // Quorum percentage (0-100)
    pub vote_duration_hours: u64,            // Voting duration in hours
    pub delegate_weight_percentage: u8,      // Delegate weight (0-100)
    pub early_quorum_enabled: bool,         // Early quorum enabled
    pub update_timestamp: i64,              // Last update time
    pub bump: u8,
}

impl GovernanceParams {
    /// Create new governance parameters
    pub fn new(
        quorum_percentage: u8,
        vote_duration_hours: u64,
        delegate_weight_percentage: u8,
        early_quorum_enabled: bool,
        bump: u8,
    ) -> Result<Self> {
        Self::new_with_time(
            quorum_percentage,
            vote_duration_hours,
            delegate_weight_percentage,
            early_quorum_enabled,
            Clock::get()?.unix_timestamp,
            bump,
        )
    }
    
    /// Create new governance parameters with explicit timestamp
    pub fn new_with_time(
        quorum_percentage: u8,
        vote_duration_hours: u64,
        delegate_weight_percentage: u8,
        early_quorum_enabled: bool,
        current_time: i64,
        bump: u8,
    ) -> Result<Self> {
        // Validate parameters
        require!(quorum_percentage > 0 && quorum_percentage <= 100, IndrasError::InvalidInput);
        require!((24..=720).contains(&vote_duration_hours), IndrasError::InvalidInput); // 24 hours - 30 days
        require!(delegate_weight_percentage <= 100, IndrasError::InvalidInput);
        
        Ok(Self {
            quorum_percentage,
            vote_duration_hours,
            delegate_weight_percentage,
            early_quorum_enabled,
            update_timestamp: current_time,
            bump,
        })
    }
    
    /// Update governance parameters
    ///
    /// Updates parameters with validation and constraints:
    /// - Maximum change per update: ±10%
    /// - Minimum interval between changes: 24 hours (checked off-chain)
    pub fn update(
        &mut self,
        quorum_percentage: Option<u8>,
        vote_duration_hours: Option<u64>,
        delegate_weight_percentage: Option<u8>,
        early_quorum_enabled: Option<bool>,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        // Update parameters with validation
        if let Some(quorum) = quorum_percentage {
            require!(quorum > 0 && quorum <= 100, IndrasError::InvalidInput);
            // Constraint: maximum change ±10%
            let diff = quorum.abs_diff(self.quorum_percentage);
            require!(diff <= 10, IndrasError::InvalidInput);
            self.quorum_percentage = quorum;
        }
        
        if let Some(duration) = vote_duration_hours {
            require!((24..=720).contains(&duration), IndrasError::InvalidInput);
            self.vote_duration_hours = duration;
        }
        
        if let Some(weight) = delegate_weight_percentage {
            require!(weight <= 100, IndrasError::InvalidInput);
            self.delegate_weight_percentage = weight;
        }
        
        if let Some(enabled) = early_quorum_enabled {
            self.early_quorum_enabled = enabled;
        }
        
        // Update timestamp
        self.update_timestamp = current_time;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_params_new() {
        let params = GovernanceParams::new_with_time(
            50,  // quorum_percentage
            168, // vote_duration_hours (7 days)
            30,  // delegate_weight_percentage
            true, // early_quorum_enabled
            1000, // current_time
            255,  // bump
        ).unwrap();
        
        assert_eq!(params.quorum_percentage, 50);
        assert_eq!(params.vote_duration_hours, 168);
        assert_eq!(params.delegate_weight_percentage, 30);
        assert_eq!(params.early_quorum_enabled, true);
        assert_eq!(params.update_timestamp, 1000);
    }

    #[test]
    fn test_governance_params_new_validation() {
        // Test quorum_percentage = 0
        assert!(GovernanceParams::new_with_time(0, 168, 30, true, 1000, 255).is_err());
        
        // Test quorum_percentage > 100
        assert!(GovernanceParams::new_with_time(101, 168, 30, true, 1000, 255).is_err());
        
        // Test vote_duration_hours < 24
        assert!(GovernanceParams::new_with_time(50, 23, 30, true, 1000, 255).is_err());
        
        // Test vote_duration_hours > 720
        assert!(GovernanceParams::new_with_time(50, 721, 30, true, 1000, 255).is_err());
        
        // Test delegate_weight_percentage > 100
        assert!(GovernanceParams::new_with_time(50, 168, 101, true, 1000, 255).is_err());
    }

    #[test]
    fn test_governance_params_update() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Update quorum_percentage within limit (±10%)
        assert!(params.update_with_time(Some(55), None, None, None, 2000).is_ok());
        assert_eq!(params.quorum_percentage, 55);
        assert_eq!(params.update_timestamp, 2000);
        
        // Update vote_duration_hours
        assert!(params.update_with_time(None, Some(240), None, None, 3000).is_ok());
        assert_eq!(params.vote_duration_hours, 240);
        
        // Update delegate_weight_percentage
        assert!(params.update_with_time(None, None, Some(40), None, 4000).is_ok());
        assert_eq!(params.delegate_weight_percentage, 40);
        
        // Update early_quorum_enabled
        assert!(params.update_with_time(None, None, None, Some(false), 5000).is_ok());
        assert_eq!(params.early_quorum_enabled, false);
    }

    #[test]
    fn test_governance_params_new_validation_quorum_too_high() {
        let result = GovernanceParams::new_with_time(
            101, // Invalid: > 100
            168,
            50,
            false,
            1000,
            255,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_governance_params_new_validation_duration_too_low() {
        let result = GovernanceParams::new_with_time(
            50,
            23, // Invalid: < 24
            50,
            false,
            1000,
            255,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_governance_params_new_validation_duration_too_high() {
        let result = GovernanceParams::new_with_time(
            50,
            721, // Invalid: > 720
            50,
            false,
            1000,
            255,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_governance_params_update_quorum_limit() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Try to update quorum_percentage beyond ±10% limit
        assert!(params.update_with_time(Some(61), None, None, None, 2000).is_err()); // +11%
        assert!(params.update_with_time(Some(39), None, None, None, 2000).is_err()); // -11%
        
        // Update within limit should work
        assert!(params.update_with_time(Some(60), None, None, None, 2000).is_ok()); // +10%
        assert!(params.update_with_time(Some(50), None, None, None, 3000).is_ok()); // -10%
    }

    #[test]
    fn test_governance_params_update_validation() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Test invalid quorum_percentage
        assert!(params.update_with_time(Some(0), None, None, None, 2000).is_err());
        assert!(params.update_with_time(Some(101), None, None, None, 2000).is_err());
        
        // Test invalid vote_duration_hours
        assert!(params.update_with_time(None, Some(23), None, None, 2000).is_err());
        assert!(params.update_with_time(None, Some(721), None, None, 2000).is_err());
        
        // Test invalid delegate_weight_percentage
        assert!(params.update_with_time(None, None, Some(101), None, 2000).is_err());
    }

    #[test]
    fn test_governance_params_update_multiple_fields() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Update multiple fields at once
        assert!(params.update_with_time(
            Some(55),
            Some(240),
            Some(35),
            Some(false),
            2000
        ).is_ok());
        
        assert_eq!(params.quorum_percentage, 55);
        assert_eq!(params.vote_duration_hours, 240);
        assert_eq!(params.delegate_weight_percentage, 35);
        assert_eq!(params.early_quorum_enabled, false);
        assert_eq!(params.update_timestamp, 2000);
    }

    #[test]
    fn test_governance_params_update_boundary_values() {
        // Start with quorum = 10 to allow testing boundary value 1 (within ±10% limit)
        let mut params = GovernanceParams::new_with_time(
            10, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Test boundary value for quorum = 1 (10 - 1 = 9, within ±10% limit)
        assert!(params.update_with_time(Some(1), None, None, None, 2000).is_ok());
        assert_eq!(params.quorum_percentage, 1);
        
        // Now test quorum = 100 (1 -> 100 is too big, need to go step by step)
        // First go to 11 (1 + 10 = 11, within limit)
        assert!(params.update_with_time(Some(11), None, None, None, 3000).is_ok());
        assert_eq!(params.quorum_percentage, 11);
        
        // Test boundary values for duration (24 and 720)
        assert!(params.update_with_time(None, Some(24), None, None, 4000).is_ok());
        assert_eq!(params.vote_duration_hours, 24);
        
        assert!(params.update_with_time(None, Some(720), None, None, 5000).is_ok());
        assert_eq!(params.vote_duration_hours, 720);
        
        // Test boundary value for delegate weight (0 and 100)
        assert!(params.update_with_time(None, None, Some(0), None, 6000).is_ok());
        assert_eq!(params.delegate_weight_percentage, 0);
        
        assert!(params.update_with_time(None, None, Some(100), None, 7000).is_ok());
        assert_eq!(params.delegate_weight_percentage, 100);
    }

    #[test]
    fn test_governance_params_update_quorum_exact_limits() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Test exact ±10% limits
        assert!(params.update_with_time(Some(60), None, None, None, 2000).is_ok()); // +10%
        assert_eq!(params.quorum_percentage, 60);
        
        assert!(params.update_with_time(Some(50), None, None, None, 3000).is_ok()); // -10%
        assert_eq!(params.quorum_percentage, 50);
        
        // Test beyond limits
        assert!(params.update_with_time(Some(61), None, None, None, 4000).is_err()); // +11%
        assert!(params.update_with_time(Some(39), None, None, None, 5000).is_err()); // -11%
    }

    #[test]
    fn test_governance_params_structure() {
        let params = GovernanceParams::new_with_time(
            75, 336, 50, false, 5000, 200
        ).unwrap();
        
        assert_eq!(params.quorum_percentage, 75);
        assert_eq!(params.vote_duration_hours, 336);
        assert_eq!(params.delegate_weight_percentage, 50);
        assert_eq!(params.early_quorum_enabled, false);
        assert_eq!(params.update_timestamp, 5000);
        assert_eq!(params.bump, 200);
    }

    #[test]
    fn test_governance_params_new_validation_quorum_zero() {
        assert!(GovernanceParams::new_with_time(0, 168, 30, true, 1000, 255).is_err());
    }

    #[test]
    fn test_governance_params_new_validation_delegate_weight_zero_allowed() {
        // delegate_weight_percentage = 0 is allowed (can be 0-100)
        assert!(GovernanceParams::new_with_time(50, 168, 0, true, 1000, 255).is_ok());
    }

    // Helper method for testing (since update uses Clock::get())
    impl GovernanceParams {
        fn update_with_time(
            &mut self,
            quorum_percentage: Option<u8>,
            vote_duration_hours: Option<u64>,
            delegate_weight_percentage: Option<u8>,
            early_quorum_enabled: Option<bool>,
            current_time: i64,
        ) -> Result<()> {
            if let Some(quorum) = quorum_percentage {
                require!(quorum > 0 && quorum <= 100, IndrasError::InvalidInput);
                let diff = if quorum > self.quorum_percentage {
                    quorum - self.quorum_percentage
                } else {
                    self.quorum_percentage - quorum
                };
                require!(diff <= 10, IndrasError::InvalidInput);
                self.quorum_percentage = quorum;
            }
            
            if let Some(duration) = vote_duration_hours {
                require!(duration >= 24 && duration <= 720, IndrasError::InvalidInput);
                self.vote_duration_hours = duration;
            }
            
            if let Some(weight) = delegate_weight_percentage {
                require!(weight <= 100, IndrasError::InvalidInput);
                self.delegate_weight_percentage = weight;
            }
            
            if let Some(enabled) = early_quorum_enabled {
                self.early_quorum_enabled = enabled;
            }
            
            self.update_timestamp = current_time;
            Ok(())
        }
    }

    #[test]
    fn test_governance_params_new_with_time_all_fields() {
        let params = GovernanceParams::new_with_time(
            75, 336, 50, false, 5000, 200
        ).unwrap();
        
        assert_eq!(params.quorum_percentage, 75);
        assert_eq!(params.vote_duration_hours, 336);
        assert_eq!(params.delegate_weight_percentage, 50);
        assert_eq!(params.early_quorum_enabled, false);
        assert_eq!(params.update_timestamp, 5000);
        assert_eq!(params.bump, 200);
    }

    #[test]
    fn test_governance_params_update_preserves_unchanged_fields() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Update only quorum_percentage
        assert!(params.update_with_time(Some(55), None, None, None, 2000).is_ok());
        
        assert_eq!(params.quorum_percentage, 55);
        assert_eq!(params.vote_duration_hours, 168); // Unchanged
        assert_eq!(params.delegate_weight_percentage, 30); // Unchanged
        assert_eq!(params.early_quorum_enabled, true); // Unchanged
        assert_eq!(params.update_timestamp, 2000);
    }

    #[test]
    fn test_governance_params_update_timestamp_always_updated() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Update with None values (no actual changes)
        assert!(params.update_with_time(None, None, None, None, 5000).is_ok());
        
        // Timestamp should still be updated
        assert_eq!(params.update_timestamp, 5000);
    }

    #[test]
    fn test_governance_params_new_with_time_boundary_quorum() {
        // Test quorum = 1 (minimum valid)
        assert!(GovernanceParams::new_with_time(1, 168, 30, true, 1000, 255).is_ok());
        
        // Test quorum = 100 (maximum valid)
        assert!(GovernanceParams::new_with_time(100, 168, 30, true, 1000, 255).is_ok());
    }

    #[test]
    fn test_governance_params_new_with_time_boundary_duration() {
        // Test duration = 24 (minimum valid)
        assert!(GovernanceParams::new_with_time(50, 24, 30, true, 1000, 255).is_ok());
        
        // Test duration = 720 (maximum valid)
        assert!(GovernanceParams::new_with_time(50, 720, 30, true, 1000, 255).is_ok());
    }

    #[test]
    fn test_governance_params_new_with_time_boundary_delegate_weight() {
        // Test delegate_weight = 0 (minimum valid)
        assert!(GovernanceParams::new_with_time(50, 168, 0, true, 1000, 255).is_ok());
        
        // Test delegate_weight = 100 (maximum valid)
        assert!(GovernanceParams::new_with_time(50, 168, 100, true, 1000, 255).is_ok());
    }

    #[test]
    fn test_governance_params_update_quorum_step_by_step() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Step 1: 50 -> 60 (+10%)
        assert!(params.update_with_time(Some(60), None, None, None, 2000).is_ok());
        assert_eq!(params.quorum_percentage, 60);
        
        // Step 2: 60 -> 70 (+10%)
        assert!(params.update_with_time(Some(70), None, None, None, 3000).is_ok());
        assert_eq!(params.quorum_percentage, 70);
        
        // Step 3: 70 -> 80 (+10%)
        assert!(params.update_with_time(Some(80), None, None, None, 4000).is_ok());
        assert_eq!(params.quorum_percentage, 80);
    }

    #[test]
    fn test_governance_params_update_preserves_other_fields() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        let original_vote_duration = params.vote_duration_hours;
        let original_delegate_weight = params.delegate_weight_percentage;
        let original_early_quorum = params.early_quorum_enabled;
        let original_bump = params.bump;
        
        assert!(params.update_with_time(Some(60), None, None, None, 2000).is_ok());
        
        assert_eq!(params.vote_duration_hours, original_vote_duration);
        assert_eq!(params.delegate_weight_percentage, original_delegate_weight);
        assert_eq!(params.early_quorum_enabled, original_early_quorum);
        assert_eq!(params.bump, original_bump);
        assert_eq!(params.quorum_percentage, 60);
        assert_eq!(params.update_timestamp, 2000);
    }

    #[test]
    fn test_governance_params_update_quorum_too_large_change() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Try to change quorum by more than 10% - should fail
        assert!(params.update_with_time(Some(65), None, None, None, 2000).is_err());
        assert_eq!(params.quorum_percentage, 50); // Should remain unchanged
    }

    #[test]
    fn test_governance_params_update_vote_duration_boundary() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Test minimum boundary (24 hours)
        assert!(params.update_with_time(None, Some(24), None, None, 2000).is_ok());
        assert_eq!(params.vote_duration_hours, 24);
        
        // Test maximum boundary (720 hours = 30 days)
        assert!(params.update_with_time(None, Some(720), None, None, 3000).is_ok());
        assert_eq!(params.vote_duration_hours, 720);
    }

    #[test]
    fn test_governance_params_update_vote_duration_invalid() {
        let mut params = GovernanceParams::new_with_time(
            50, 168, 30, true, 1000, 255
        ).unwrap();
        
        // Test below minimum (23 hours)
        assert!(params.update_with_time(None, Some(23), None, None, 2000).is_err());
        
        // Test above maximum (721 hours)
        assert!(params.update_with_time(None, Some(721), None, None, 3000).is_err());
    }
}
