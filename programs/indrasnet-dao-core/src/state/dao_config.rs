//! DAO Configuration account structure

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// DAO Configuration account
/// Stores the main configuration and metadata for the DAO
pub const DAO_CONFIG_SCHEMA_VERSION: u8 = 2;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DaoConfigV1 {
    pub authority: Pubkey,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_paused: bool,
    pub last_operation_timestamp: Option<i64>,
    pub operation_count: u64,
    pub execution_delay_seconds: i64,
    pub adaptive_security_enabled: bool,
    pub progressive_unlock_enabled: bool,
    pub behavioral_analysis_enabled: bool,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deactivated_at: Option<i64>,
    pub reactivated_at: Option<i64>,
    pub authority_transferred_at: Option<i64>,
    pub security_enhancement_count: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct DaoConfig {
    pub schema_version: u8,
    pub authority: Pubkey,
    #[max_len(100)]
    pub name: String,
    #[max_len(500)]
    pub description: String,
    pub is_active: bool,
    /// SECURITY: Dev mode flag (bypasses some production-only checks)
    pub dev_mode: bool,
    /// SECURITY: Circuit breaker - emergency pause mechanism
    pub is_paused: bool,
    /// SECURITY: Last operation timestamp for rate limiting
    pub last_operation_timestamp: Option<i64>,
    /// SECURITY: Operation count in current window (for rate limiting)
    pub operation_count: u64,
    /// SECURITY: Execution delay for proposals (timelock in seconds)
    pub execution_delay_seconds: i64,
    /// SECURITY: Adaptive security scoring enabled
    pub adaptive_security_enabled: bool,
    /// SECURITY: Progressive unlock enabled
    pub progressive_unlock_enabled: bool,
    /// SECURITY: Behavioral analysis enabled
    pub behavioral_analysis_enabled: bool,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deactivated_at: Option<i64>,
    pub reactivated_at: Option<i64>,
    pub authority_transferred_at: Option<i64>,
    pub security_enhancement_count: u64,
    pub bump: u8,
}

impl DaoConfig {
    /// SECURITY: Check if DAO is paused (circuit breaker)
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
    
    /// SECURITY: Pause DAO (emergency circuit breaker)
    pub fn pause(&mut self) -> Result<()> {
        self.is_paused = true;
        Ok(())
    }
    
    /// SECURITY: Unpause DAO
    pub fn unpause(&mut self) -> Result<()> {
        self.is_paused = false;
        Ok(())
    }
    
    /// SECURITY: Rate limiting check - prevent DoS attacks
    pub fn check_rate_limit(&mut self, current_time: i64, max_operations_per_hour: u64) -> Result<()> {
        const RATE_LIMIT_WINDOW: i64 = 3600; // 1 hour window
        
        if let Some(last_timestamp) = self.last_operation_timestamp {
            // Reset counter if window expired
            if current_time - last_timestamp >= RATE_LIMIT_WINDOW {
                self.operation_count = 0;
                self.last_operation_timestamp = Some(current_time);
            } else {
                // Check if limit exceeded
                require!(
                    self.operation_count < max_operations_per_hour,
                    IndrasError::OperationNotAllowed
                );
            }
        } else {
            // First operation in window
            self.last_operation_timestamp = Some(current_time);
        }
        
        // Increment operation count
        self.operation_count = self.operation_count
            .checked_add(1)
            .ok_or(error!(IndrasError::Overflow))?;
        
        Ok(())
    }
    
    /// SECURITY: Update execution delay (timelock)
    pub fn set_execution_delay(&mut self, delay_seconds: i64) -> Result<()> {
        const MAX_EXECUTION_DELAY_SECONDS: i64 = 7 * 24 * 3600; // 7 days max
        
        require!(
            (0..=MAX_EXECUTION_DELAY_SECONDS).contains(&delay_seconds),
            IndrasError::InvalidInput
        );
        
        self.execution_delay_seconds = delay_seconds;
        Ok(())
    }
    
    /// SECURITY: Enable/disable adaptive security scoring
    pub fn set_adaptive_security(&mut self, enabled: bool) -> Result<()> {
        self.adaptive_security_enabled = enabled;
        Ok(())
    }
    
    /// SECURITY: Enable/disable progressive unlock
    pub fn set_progressive_unlock(&mut self, enabled: bool) -> Result<()> {
        self.progressive_unlock_enabled = enabled;
        Ok(())
    }
    
    /// SECURITY: Enable/disable behavioral analysis
    pub fn set_behavioral_analysis(&mut self, enabled: bool) -> Result<()> {
        self.behavioral_analysis_enabled = enabled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_dao_config() -> DaoConfig {
        DaoConfig {
            schema_version: DAO_CONFIG_SCHEMA_VERSION,
            authority: Pubkey::new_unique(),
            name: "Test DAO".to_string(),
            description: "Test Description".to_string(),
            is_active: true,
            dev_mode: false,
            is_paused: false,
            last_operation_timestamp: None,
            operation_count: 0,
            execution_delay_seconds: 0,
            adaptive_security_enabled: false,
            progressive_unlock_enabled: false,
            behavioral_analysis_enabled: false,
            created_at: 1000,
            updated_at: None,
            deactivated_at: None,
            reactivated_at: None,
            authority_transferred_at: None,
            security_enhancement_count: 0,
            bump: 255,
        }
    }

    #[test]
    fn test_dao_config_is_paused() {
        let mut config = create_test_dao_config();
        
        assert!(!config.is_paused());
        config.pause().unwrap();
        assert!(config.is_paused());
        config.unpause().unwrap();
        assert!(!config.is_paused());
    }

    #[test]
    fn test_dao_config_check_rate_limit_first_operation() {
        let mut config = create_test_dao_config();
        
        assert!(config.check_rate_limit(2000, 10).is_ok());
        assert_eq!(config.operation_count, 1);
        assert_eq!(config.last_operation_timestamp, Some(2000));
    }

    #[test]
    fn test_dao_config_check_rate_limit_within_limit() {
        let mut config = create_test_dao_config();
        config.last_operation_timestamp = Some(2000);
        config.operation_count = 5;
        
        assert!(config.check_rate_limit(2500, 10).is_ok());
        assert_eq!(config.operation_count, 6);
    }

    #[test]
    fn test_dao_config_check_rate_limit_exceeds_limit() {
        let mut config = create_test_dao_config();
        config.last_operation_timestamp = Some(2000);
        config.operation_count = 10;
        
        // Should fail - already at limit
        assert!(config.check_rate_limit(2500, 10).is_err());
    }

    #[test]
    fn test_dao_config_check_rate_limit_window_reset() {
        let mut config = create_test_dao_config();
        config.last_operation_timestamp = Some(2000);
        config.operation_count = 5;
        
        // New window starts (current_time - last_timestamp >= 3600)
        assert!(config.check_rate_limit(6000, 10).is_ok());
        assert_eq!(config.operation_count, 1); // Reset to 1
        assert_eq!(config.last_operation_timestamp, Some(6000));
    }

    #[test]
    fn test_dao_config_set_execution_delay() {
        let mut config = create_test_dao_config();
        
        assert!(config.set_execution_delay(86400).is_ok()); // 1 day
        assert_eq!(config.execution_delay_seconds, 86400);
    }

    #[test]
    fn test_dao_config_set_execution_delay_invalid_negative() {
        let mut config = create_test_dao_config();
        
        assert!(config.set_execution_delay(-1).is_err());
    }

    #[test]
    fn test_dao_config_set_execution_delay_invalid_too_large() {
        let mut config = create_test_dao_config();
        
        // More than 7 days
        assert!(config.set_execution_delay(8 * 24 * 3600).is_err());
    }

    #[test]
    fn test_dao_config_set_adaptive_security() {
        let mut config = create_test_dao_config();
        
        config.set_adaptive_security(true).unwrap();
        assert!(config.adaptive_security_enabled);
        
        config.set_adaptive_security(false).unwrap();
        assert!(!config.adaptive_security_enabled);
    }

    #[test]
    fn test_dao_config_set_progressive_unlock() {
        let mut config = create_test_dao_config();
        
        config.set_progressive_unlock(true).unwrap();
        assert!(config.progressive_unlock_enabled);
        
        config.set_progressive_unlock(false).unwrap();
        assert!(!config.progressive_unlock_enabled);
    }

    #[test]
    fn test_dao_config_set_behavioral_analysis() {
        let mut config = create_test_dao_config();
        
        config.set_behavioral_analysis(true).unwrap();
        assert!(config.behavioral_analysis_enabled);
        
        config.set_behavioral_analysis(false).unwrap();
        assert!(!config.behavioral_analysis_enabled);
    }

    #[test]
    fn test_dao_config_all_fields() {
        let authority = Pubkey::new_unique();
        let config = DaoConfig {
            schema_version: DAO_CONFIG_SCHEMA_VERSION,
            authority,
            name: "Test DAO".to_string(),
            description: "Test Description".to_string(),
            is_active: true,
            dev_mode: false,
            is_paused: false,
            last_operation_timestamp: Some(1000),
            operation_count: 5,
            execution_delay_seconds: 86400,
            adaptive_security_enabled: true,
            progressive_unlock_enabled: true,
            behavioral_analysis_enabled: true,
            created_at: 1000,
            updated_at: Some(2000),
            deactivated_at: None,
            reactivated_at: None,
            authority_transferred_at: None,
            security_enhancement_count: 10,
            bump: 128,
        };
        
        assert_eq!(config.authority, authority);
        assert_eq!(config.name, "Test DAO");
        assert_eq!(config.description, "Test Description");
        assert!(config.is_active);
        assert!(!config.is_paused);
        assert_eq!(config.last_operation_timestamp, Some(1000));
        assert_eq!(config.operation_count, 5);
        assert_eq!(config.execution_delay_seconds, 86400);
        assert!(config.adaptive_security_enabled);
        assert!(config.progressive_unlock_enabled);
        assert!(config.behavioral_analysis_enabled);
        assert_eq!(config.created_at, 1000);
        assert_eq!(config.updated_at, Some(2000));
        assert_eq!(config.security_enhancement_count, 10);
        assert_eq!(config.bump, 128);
    }

    #[test]
    fn test_dao_config_check_rate_limit_overflow() {
        let mut config = create_test_dao_config();
        config.operation_count = u64::MAX;
        
        // Should fail on overflow
        assert!(config.check_rate_limit(2000, 10).is_err());
    }

    #[test]
    fn test_dao_config_check_rate_limit_exact_window() {
        let mut config = create_test_dao_config();
        config.last_operation_timestamp = Some(2000);
        config.operation_count = 5;
        
        // Exactly at window boundary (3600 seconds)
        assert!(config.check_rate_limit(5600, 10).is_ok());
        assert_eq!(config.operation_count, 1); // Reset
        assert_eq!(config.last_operation_timestamp, Some(5600));
    }

    #[test]
    fn test_dao_config_set_execution_delay_zero() {
        let mut config = create_test_dao_config();
        
        assert!(config.set_execution_delay(0).is_ok());
        assert_eq!(config.execution_delay_seconds, 0);
    }

    #[test]
    fn test_dao_config_set_execution_delay_max() {
        let mut config = create_test_dao_config();
        let max_delay = 7 * 24 * 3600; // 7 days
        
        assert!(config.set_execution_delay(max_delay).is_ok());
        assert_eq!(config.execution_delay_seconds, max_delay);
    }

    #[test]
    fn test_dao_config_pause_preserves_other_fields() {
        let mut config = create_test_dao_config();
        let original_authority = config.authority;
        let original_name = config.name.clone();
        let original_bump = config.bump;
        
        assert!(config.pause().is_ok());
        
        assert_eq!(config.authority, original_authority);
        assert_eq!(config.name, original_name);
        assert_eq!(config.bump, original_bump);
        assert!(config.is_paused);
    }

    #[test]
    fn test_dao_config_unpause_preserves_other_fields() {
        let mut config = create_test_dao_config();
        config.is_paused = true;
        let original_authority = config.authority;
        let original_name = config.name.clone();
        let original_bump = config.bump;
        
        assert!(config.unpause().is_ok());
        
        assert_eq!(config.authority, original_authority);
        assert_eq!(config.name, original_name);
        assert_eq!(config.bump, original_bump);
        assert!(!config.is_paused);
    }

    #[test]
    fn test_dao_config_check_rate_limit_multiple_operations() {
        let mut config = create_test_dao_config();
        
        // First operation
        assert!(config.check_rate_limit(2000, 10).is_ok());
        assert_eq!(config.operation_count, 1);
        
        // Second operation
        assert!(config.check_rate_limit(2100, 10).is_ok());
        assert_eq!(config.operation_count, 2);
        
        // Third operation
        assert!(config.check_rate_limit(2200, 10).is_ok());
        assert_eq!(config.operation_count, 3);
    }

    #[test]
    fn test_dao_config_check_rate_limit_boundary_conditions() {
        let mut config = create_test_dao_config();
        config.last_operation_timestamp = Some(2000);
        config.operation_count = 9; // One below limit
        
        // Should succeed - at limit - 1
        assert!(config.check_rate_limit(2500, 10).is_ok());
        assert_eq!(config.operation_count, 10);
        
        // Next operation should fail - at limit
        assert!(config.check_rate_limit(2600, 10).is_err());
    }

    #[test]
    fn test_dao_config_check_rate_limit_window_edge() {
        let mut config = create_test_dao_config();
        config.last_operation_timestamp = Some(2000);
        config.operation_count = 5;
        
        // Just before window expires (3599 seconds)
        assert!(config.check_rate_limit(5599, 10).is_ok());
        assert_eq!(config.operation_count, 6);
        
        // Exactly at window boundary (3600 seconds)
        assert!(config.check_rate_limit(5600, 10).is_ok());
        assert_eq!(config.operation_count, 1); // Reset
    }

    #[test]
    fn test_dao_config_set_execution_delay_boundaries() {
        let mut config = create_test_dao_config();
        
        // Test minimum (0)
        assert!(config.set_execution_delay(0).is_ok());
        assert_eq!(config.execution_delay_seconds, 0);
        
        // Test maximum (7 days)
        let max_delay = 7 * 24 * 3600;
        assert!(config.set_execution_delay(max_delay).is_ok());
        assert_eq!(config.execution_delay_seconds, max_delay);
        
        // Test just over maximum
        assert!(config.set_execution_delay(max_delay + 1).is_err());
    }

    #[test]
    fn test_dao_config_security_flags_combinations() {
        let mut config = create_test_dao_config();
        
        // Test all combinations of security flags
        for adaptive in [true, false] {
            for progressive in [true, false] {
                for behavioral in [true, false] {
                    config.set_adaptive_security(adaptive).unwrap();
                    config.set_progressive_unlock(progressive).unwrap();
                    config.set_behavioral_analysis(behavioral).unwrap();
                    
                    assert_eq!(config.adaptive_security_enabled, adaptive);
                    assert_eq!(config.progressive_unlock_enabled, progressive);
                    assert_eq!(config.behavioral_analysis_enabled, behavioral);
                }
            }
        }
    }

    #[test]
    fn test_dao_config_name_length_limits() {
        let mut config = create_test_dao_config();
        
        // Test max length name (100 chars)
        let max_name = "a".repeat(100);
        config.name = max_name.clone();
        assert_eq!(config.name.len(), 100);
        assert_eq!(config.name, max_name);
    }

    #[test]
    fn test_dao_config_description_length_limits() {
        let mut config = create_test_dao_config();
        
        // Test max length description (500 chars)
        let max_desc = "a".repeat(500);
        config.description = max_desc.clone();
        assert_eq!(config.description.len(), 500);
        assert_eq!(config.description, max_desc);
    }

    #[test]
    fn test_dao_config_timestamp_operations() {
        let mut config = create_test_dao_config();
        
        // Test with various timestamps
        let timestamps = vec![0i64, 1i64, 1234567890i64, i64::MAX, -1i64];
        
        for timestamp in timestamps {
            config.last_operation_timestamp = Some(timestamp);
            assert_eq!(config.last_operation_timestamp, Some(timestamp));
        }
    }

    #[test]
    fn test_dao_config_optional_timestamps() {
        let mut config = create_test_dao_config();
        
        // Test setting optional timestamps
        config.updated_at = Some(2000);
        config.deactivated_at = Some(3000);
        config.reactivated_at = Some(4000);
        config.authority_transferred_at = Some(5000);
        
        assert_eq!(config.updated_at, Some(2000));
        assert_eq!(config.deactivated_at, Some(3000));
        assert_eq!(config.reactivated_at, Some(4000));
        assert_eq!(config.authority_transferred_at, Some(5000));
        
        // Test clearing optional timestamps
        config.updated_at = None;
        assert_eq!(config.updated_at, None);
    }

    #[test]
    fn test_dao_config_security_enhancement_count() {
        let mut config = create_test_dao_config();
        
        // Test incrementing security enhancement count
        config.security_enhancement_count = 0;
        assert_eq!(config.security_enhancement_count, 0);
        
        config.security_enhancement_count = 100;
        assert_eq!(config.security_enhancement_count, 100);
        
        config.security_enhancement_count = u64::MAX;
        assert_eq!(config.security_enhancement_count, u64::MAX);
    }

    #[test]
    fn test_dao_config_bump_variations() {
        let mut config = create_test_dao_config();
        
        let bumps = vec![0u8, 1u8, 128u8, 255u8];
        
        for bump in bumps {
            config.bump = bump;
            assert_eq!(config.bump, bump);
        }
    }

    #[test]
    fn test_dao_config_authority_operations() {
        let mut config = create_test_dao_config();
        let original_authority = config.authority;
        
        // Test authority doesn't change on pause/unpause
        config.pause().unwrap();
        assert_eq!(config.authority, original_authority);
        
        config.unpause().unwrap();
        assert_eq!(config.authority, original_authority);
        
        // Test setting new authority
        let new_authority = Pubkey::new_unique();
        config.authority = new_authority;
        assert_eq!(config.authority, new_authority);
        assert_ne!(config.authority, original_authority);
    }
}
