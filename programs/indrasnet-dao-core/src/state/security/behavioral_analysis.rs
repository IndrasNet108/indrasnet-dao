//! Behavioral Pattern Analysis System
//!
//! INNOVATION: Behavioral pattern analysis for anomaly detection
//! System tracks normal patterns and detects deviations
//! Protection against unusual behavior and potential attacks

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Behavioral pattern for a user
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct BehavioralPattern {
    /// User public key
    pub user: Pubkey,
    /// Average time between operations (in seconds)
    pub avg_operation_interval: i64,
    /// Typical operation types
    pub typical_operation_types: Vec<u8>,
    /// Typical operation amounts (if applicable)
    pub typical_amount_range: (u64, u64),
    /// Number of operations analyzed
    pub operation_count: u64,
    /// Last operation timestamp
    pub last_operation_timestamp: i64,
    /// Pattern confidence (0-100)
    pub confidence: u8,
}

impl BehavioralPattern {
    pub fn new(user: Pubkey) -> Self {
        Self {
            user,
            avg_operation_interval: 0,
            typical_operation_types: Vec::new(),
            typical_amount_range: (0, 0),
            operation_count: 0,
            last_operation_timestamp: 0,
            confidence: 0,
        }
    }
    
    /// Update pattern with new operation
    pub fn update_pattern(
        &mut self,
        operation_type: u8,
        amount: Option<u64>,
        current_time: i64,
    ) -> Result<()> {
        // Update average interval
        if self.operation_count > 0 && self.last_operation_timestamp > 0 {
            let interval = current_time
                .checked_sub(self.last_operation_timestamp)
                .ok_or(error!(IndrasError::InvalidTimestamp))?;
            
            // Weighted average: new_avg = (old_avg * (n-1) + new_value) / n
            let total_interval = self.avg_operation_interval
                .checked_mul(self.operation_count as i64)
                .ok_or(error!(IndrasError::Overflow))?;
            
            let new_total = total_interval
                .checked_add(interval)
                .ok_or(error!(IndrasError::Overflow))?;
            
            self.avg_operation_interval = new_total
                .checked_div((self.operation_count + 1) as i64)
                .unwrap_or(0);
        }
        
        // Update typical operation types (keep last 10)
        if !self.typical_operation_types.contains(&operation_type) {
            if self.typical_operation_types.len() >= 10 {
                self.typical_operation_types.remove(0);
            }
            self.typical_operation_types.push(operation_type);
        }
        
        // Update amount range
        if let Some(amt) = amount {
            if self.operation_count == 0 {
                self.typical_amount_range = (amt, amt);
            } else {
                let (min, max) = self.typical_amount_range;
                self.typical_amount_range = (
                    min.min(amt),
                    max.max(amt),
                );
            }
        }
        
        self.operation_count = self.operation_count
            .checked_add(1)
            .ok_or(error!(IndrasError::Overflow))?;
        self.last_operation_timestamp = current_time;
        
        // Increase confidence with more operations (capped at 100)
        if self.operation_count > 0 {
            self.confidence = (self.operation_count.min(100) as u8)
                .min(100);
        }
        
        Ok(())
    }
    
    /// Analyze if operation matches pattern (returns anomaly score 0-100, higher = more anomalous)
    pub fn analyze_anomaly(
        &self,
        operation_type: u8,
        amount: Option<u64>,
        current_time: i64,
    ) -> Result<u8> {
        if self.operation_count < 3 {
            return Ok(0); // Not enough data
        }
        
        let mut anomaly_score = 0u8;
        
        // Check timing anomaly
        if self.last_operation_timestamp > 0 {
            let time_since_last = current_time
                .checked_sub(self.last_operation_timestamp)
                .ok_or(error!(IndrasError::InvalidTimestamp))?;
            
            // If operation happens much faster than average, it's suspicious
            if self.avg_operation_interval > 0 && time_since_last < self.avg_operation_interval / 10 {
                anomaly_score = anomaly_score.saturating_add(30);
            }
        }
        
        // Check operation type anomaly
        if !self.typical_operation_types.contains(&operation_type) {
            anomaly_score = anomaly_score.saturating_add(25);
        }
        
        // Check amount anomaly
        if let Some(amt) = amount {
            let (min, max) = self.typical_amount_range;
            if amt < min || amt > max {
                // Calculate how far outside the range
                let range_size = max.saturating_sub(min);
                if range_size > 0 {
                    let deviation = if amt < min {
                        min.saturating_sub(amt)
                    } else {
                        amt.saturating_sub(max)
                    };
                    
                    // If deviation is more than 50% of range, it's anomalous
                    if deviation > range_size / 2 {
                        anomaly_score = anomaly_score.saturating_add(25);
                    }
                } else if amt != min {
                    anomaly_score = anomaly_score.saturating_add(20);
                }
            }
        }
        
        Ok(anomaly_score.min(100))
    }
}

/// SECURITY: Anomaly thresholds
pub const LOW_ANOMALY_THRESHOLD: u8 = 30;
pub const MEDIUM_ANOMALY_THRESHOLD: u8 = 50;
pub const HIGH_ANOMALY_THRESHOLD: u8 = 70;

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_behavioral_pattern_new() {
        let user = create_test_pubkey(1);
        let pattern = BehavioralPattern::new(user);
        
        assert_eq!(pattern.user, user);
        assert_eq!(pattern.avg_operation_interval, 0);
        assert_eq!(pattern.typical_operation_types.len(), 0);
        assert_eq!(pattern.typical_amount_range, (0, 0));
        assert_eq!(pattern.operation_count, 0);
        assert_eq!(pattern.last_operation_timestamp, 0);
        assert_eq!(pattern.confidence, 0);
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_first_operation() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        let result = pattern.update_pattern(1, Some(100), 1000);
        
        assert!(result.is_ok());
        assert_eq!(pattern.operation_count, 1);
        assert_eq!(pattern.last_operation_timestamp, 1000);
        assert_eq!(pattern.typical_amount_range, (100, 100));
        assert!(pattern.typical_operation_types.contains(&1));
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_multiple_operations() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // First operation
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        
        // Second operation after 100 seconds
        pattern.update_pattern(1, Some(150), 1100).unwrap();
        
        assert_eq!(pattern.operation_count, 2);
        assert_eq!(pattern.last_operation_timestamp, 1100);
        assert_eq!(pattern.typical_amount_range, (100, 150));
        assert!(pattern.avg_operation_interval > 0);
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_overflow_protection() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        pattern.operation_count = u64::MAX;
        
        // Should fail on overflow
        let result = pattern.update_pattern(1, Some(100), 1000);
        assert!(result.is_err());
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_confidence_increase() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Add multiple operations to increase confidence
        for i in 0..50 {
            pattern.update_pattern(1, Some(100 + i as u64), 1000 + (i as i64 * 10)).unwrap();
        }
        
        // Confidence should increase (capped at 100)
        assert!(pattern.confidence > 0);
        assert!(pattern.confidence <= 100);
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_insufficient_data() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Less than 3 operations
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        
        let result = pattern.analyze_anomaly(1, Some(100), 1200);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // Not enough data
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_normal_operation() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with normal operations
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Normal operation (same type, similar amount, normal timing)
        let result = pattern.analyze_anomaly(1, Some(100), 1300);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score < MEDIUM_ANOMALY_THRESHOLD);
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_different_operation_type() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with type 1 operations
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Different operation type
        let result = pattern.analyze_anomaly(2, Some(100), 1300);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score >= 25); // Should add 25 points for different type
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_amount_outside_range() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with amounts 100-150
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(150), 1100).unwrap();
        pattern.update_pattern(1, Some(120), 1200).unwrap();
        
        // Amount way outside range
        let result = pattern.analyze_anomaly(1, Some(1000), 1300);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score >= 25); // Should add points for amount anomaly
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_timing_anomaly() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with 100 second intervals
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Operation happens too fast (5 seconds after last, much less than avg/10)
        let result = pattern.analyze_anomaly(1, Some(100), 1205);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score >= 30); // Should add 30 points for timing anomaly
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_capped_at_100() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Multiple anomalies: different type, wrong amount, wrong timing
        let result = pattern.analyze_anomaly(2, Some(10000), 1205);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score <= 100); // Should be capped at 100
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_no_amount() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Update without amount
        let result = pattern.update_pattern(1, None, 1000);
        
        assert!(result.is_ok());
        assert_eq!(pattern.operation_count, 1);
        assert_eq!(pattern.typical_amount_range, (0, 0)); // Should remain (0, 0)
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_operation_types_limit() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Add 12 different operation types (should keep last 10)
        for i in 0..12 {
            pattern.update_pattern(i, Some(100), 1000 + (i as i64 * 10)).unwrap();
        }
        
        // Should have max 10 operation types
        assert!(pattern.typical_operation_types.len() <= 10);
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_interval_calculation() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // First operation
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        
        // Second operation after 100 seconds
        // Formula: new_avg = (old_avg * (n-1) + new_value) / n
        // After first: avg = 0, count = 1
        // After second: avg = (0 * 1 + 100) / 2 = 50
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        
        // Average interval should be 50 (100 / 2)
        assert_eq!(pattern.avg_operation_interval, 50);
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_negative_interval() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // First operation sets timestamp
        pattern.update_pattern(1, Some(100), 2000).unwrap();
        assert_eq!(pattern.operation_count, 1);
        assert_eq!(pattern.last_operation_timestamp, 2000);
        
        // Current time before last operation
        // For i64, checked_sub(1000, 2000) = Some(-1000) (valid negative value)
        // Code allows negative intervals (time can go backwards in test scenarios)
        let result = pattern.update_pattern(1, Some(100), 1000);
        
        // Should succeed (negative interval is valid for i64)
        assert!(result.is_ok());
        assert_eq!(pattern.operation_count, 2);
        assert_eq!(pattern.last_operation_timestamp, 1000);
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_no_amount() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Analyze without amount
        let result = pattern.analyze_anomaly(1, None, 1300);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score < 50); // Should be low without amount anomaly
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_amount_within_range() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with amounts 100-200
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(200), 1100).unwrap();
        pattern.update_pattern(1, Some(150), 1200).unwrap();
        
        // Amount within range
        let result = pattern.analyze_anomaly(1, Some(150), 1300);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score < 25); // Should be low (no amount anomaly)
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_amount_slightly_outside_range() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with amounts 100-200
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(200), 1100).unwrap();
        pattern.update_pattern(1, Some(150), 1200).unwrap();
        
        // Amount slightly outside range (but deviation < 50% of range)
        let result = pattern.analyze_anomaly(1, Some(250), 1300);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        // Should be low because deviation is not > 50% of range
        assert!(score < 50);
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_timing_normal() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with 100 second intervals
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Normal timing (100 seconds after last)
        let result = pattern.analyze_anomaly(1, Some(100), 1300);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score < 30); // Should be low (no timing anomaly)
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_zero_interval() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern with zero interval (first operation)
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        pattern.avg_operation_interval = 0; // Set to zero
        
        // Should not trigger timing anomaly if interval is 0
        let result = pattern.analyze_anomaly(1, Some(100), 1205);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        assert!(score < 30); // Should not add timing anomaly points
    }

    #[test]
    fn test_behavioral_pattern_analyze_anomaly_combined_anomalies() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Build pattern
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Combined: different type + wrong amount + wrong timing
        let result = pattern.analyze_anomaly(2, Some(10000), 1205);
        
        assert!(result.is_ok());
        let score = result.unwrap();
        // Should have high score (25 + 25 + 30 = 80, capped at 100)
        assert!(score >= 50);
        assert!(score <= 100);
    }

    #[test]
    fn test_behavioral_pattern_update_pattern_duplicate_operation_type() {
        let user = create_test_pubkey(1);
        let mut pattern = BehavioralPattern::new(user);
        
        // Add same operation type multiple times
        pattern.update_pattern(1, Some(100), 1000).unwrap();
        pattern.update_pattern(1, Some(100), 1100).unwrap();
        pattern.update_pattern(1, Some(100), 1200).unwrap();
        
        // Should only have one unique operation type
        assert_eq!(pattern.typical_operation_types.len(), 1);
        assert!(pattern.typical_operation_types.contains(&1));
    }
}
