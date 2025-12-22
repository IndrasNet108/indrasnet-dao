//! Progressive Security Unlocking System
//!
//! INNOVATION: Progressive operation unlocking with increasing security
//! Operations unlock in stages with increasing security requirements
//! Protection against sudden critical changes

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Progressive unlock stage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum UnlockStage {
    /// Locked - operation not allowed
    Locked,
    /// Stage 1: Basic unlock (minimal security)
    Stage1Basic,
    /// Stage 2: Enhanced unlock (moderate security)
    Stage2Enhanced,
    /// Stage 3: Full unlock (maximum security)
    Stage3Full,
}

/// Progressive unlock configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ProgressiveUnlock {
    /// Current unlock stage
    pub current_stage: UnlockStage,
    /// Timestamp when current stage was reached
    pub stage_timestamp: i64,
    /// Time required in each stage before next unlock (in seconds)
    pub stage_duration: i64,
    /// Security requirements for each stage
    pub stage_security_requirements: [u8; 3], // [stage1, stage2, stage3]
}

impl ProgressiveUnlock {
    pub fn new(stage_duration: i64) -> Self {
        Self {
            current_stage: UnlockStage::Locked,
            stage_timestamp: 0,
            stage_duration,
            stage_security_requirements: [50, 70, 90], // Progressive security levels
        }
    }
    
    /// Check if operation can proceed to next stage
    pub fn can_advance_stage(&self, current_time: i64, security_score: u8) -> Result<bool> {
        let time_in_stage = current_time
            .checked_sub(self.stage_timestamp)
            .ok_or(error!(IndrasError::InvalidTimestamp))?;
        
        if time_in_stage < self.stage_duration {
            return Ok(false);
        }
        
        // Check security requirements for next stage
        let next_stage_requirement = match self.current_stage {
            UnlockStage::Locked => self.stage_security_requirements[0],
            UnlockStage::Stage1Basic => self.stage_security_requirements[1],
            UnlockStage::Stage2Enhanced => self.stage_security_requirements[2],
            UnlockStage::Stage3Full => return Ok(false), // Already at max
        };
        
        Ok(security_score >= next_stage_requirement)
    }
    
    /// Advance to next stage
    pub fn advance_stage(&mut self, current_time: i64) -> Result<()> {
        self.current_stage = match self.current_stage {
            UnlockStage::Locked => UnlockStage::Stage1Basic,
            UnlockStage::Stage1Basic => UnlockStage::Stage2Enhanced,
            UnlockStage::Stage2Enhanced => UnlockStage::Stage3Full,
            UnlockStage::Stage3Full => return Err(error!(IndrasError::InvalidState)),
        };
        
        self.stage_timestamp = current_time;
        Ok(())
    }
    
    /// Check if operation is allowed at current stage
    pub fn is_operation_allowed(&self, required_stage: UnlockStage) -> bool {
        matches!(
            (&self.current_stage, required_stage),
            (UnlockStage::Stage3Full, _)
                | (UnlockStage::Stage2Enhanced, UnlockStage::Stage1Basic | UnlockStage::Stage2Enhanced)
                | (UnlockStage::Stage1Basic, UnlockStage::Stage1Basic)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progressive_unlock_new() {
        let unlock = ProgressiveUnlock::new(3600);
        
        assert_eq!(unlock.current_stage, UnlockStage::Locked);
        assert_eq!(unlock.stage_timestamp, 0);
        assert_eq!(unlock.stage_duration, 3600);
        assert_eq!(unlock.stage_security_requirements, [50, 70, 90]);
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_locked_to_stage1() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 1000;
        
        // Can advance if enough time passed and security score meets requirement
        let result = unlock.can_advance_stage(5000, 60); // 4000 seconds passed, score 60 >= 50
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_insufficient_time() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 1000;
        
        // Cannot advance if not enough time passed
        let result = unlock.can_advance_stage(2000, 60); // Only 1000 seconds passed
        
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_insufficient_security() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 1000;
        
        // Cannot advance if security score too low
        let result = unlock.can_advance_stage(5000, 40); // Score 40 < 50
        
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_stage1_to_stage2() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage1Basic;
        unlock.stage_timestamp = 1000;
        
        // Can advance if enough time and security score >= 70
        let result = unlock.can_advance_stage(5000, 75);
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_stage3_full() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage3Full;
        unlock.stage_timestamp = 1000;
        
        // Cannot advance from Stage3Full
        let result = unlock.can_advance_stage(5000, 100);
        
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_advance_stage_locked_to_stage1() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 1000;
        
        let result = unlock.advance_stage(2000);
        
        assert!(result.is_ok());
        assert_eq!(unlock.current_stage, UnlockStage::Stage1Basic);
        assert_eq!(unlock.stage_timestamp, 2000);
    }

    #[test]
    fn test_progressive_unlock_advance_stage_stage1_to_stage2() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage1Basic;
        unlock.stage_timestamp = 1000;
        
        let result = unlock.advance_stage(2000);
        
        assert!(result.is_ok());
        assert_eq!(unlock.current_stage, UnlockStage::Stage2Enhanced);
        assert_eq!(unlock.stage_timestamp, 2000);
    }

    #[test]
    fn test_progressive_unlock_advance_stage_stage2_to_stage3() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage2Enhanced;
        unlock.stage_timestamp = 1000;
        
        let result = unlock.advance_stage(2000);
        
        assert!(result.is_ok());
        assert_eq!(unlock.current_stage, UnlockStage::Stage3Full);
        assert_eq!(unlock.stage_timestamp, 2000);
    }

    #[test]
    fn test_progressive_unlock_advance_stage_stage3_full_error() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage3Full;
        
        // Cannot advance from Stage3Full
        let result = unlock.advance_stage(2000);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_progressive_unlock_is_operation_allowed_stage3_full() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage3Full;
        
        // Stage3Full allows all operations
        assert!(unlock.is_operation_allowed(UnlockStage::Stage1Basic));
        assert!(unlock.is_operation_allowed(UnlockStage::Stage2Enhanced));
        assert!(unlock.is_operation_allowed(UnlockStage::Stage3Full));
    }

    #[test]
    fn test_progressive_unlock_is_operation_allowed_stage2_enhanced() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage2Enhanced;
        
        // Stage2Enhanced allows Stage1 and Stage2 operations
        assert!(unlock.is_operation_allowed(UnlockStage::Stage1Basic));
        assert!(unlock.is_operation_allowed(UnlockStage::Stage2Enhanced));
        assert!(!unlock.is_operation_allowed(UnlockStage::Stage3Full));
    }

    #[test]
    fn test_progressive_unlock_is_operation_allowed_stage1_basic() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage1Basic;
        
        // Stage1Basic only allows Stage1 operations
        assert!(unlock.is_operation_allowed(UnlockStage::Stage1Basic));
        assert!(!unlock.is_operation_allowed(UnlockStage::Stage2Enhanced));
        assert!(!unlock.is_operation_allowed(UnlockStage::Stage3Full));
    }

    #[test]
    fn test_progressive_unlock_is_operation_allowed_locked() {
        let unlock = ProgressiveUnlock::new(3600);
        
        // Locked allows no operations
        assert!(!unlock.is_operation_allowed(UnlockStage::Stage1Basic));
        assert!(!unlock.is_operation_allowed(UnlockStage::Stage2Enhanced));
        assert!(!unlock.is_operation_allowed(UnlockStage::Stage3Full));
    }

    #[test]
    fn test_unlock_stage_variants() {
        assert_eq!(UnlockStage::Locked, UnlockStage::Locked);
        assert_eq!(UnlockStage::Stage1Basic, UnlockStage::Stage1Basic);
        assert_eq!(UnlockStage::Stage2Enhanced, UnlockStage::Stage2Enhanced);
        assert_eq!(UnlockStage::Stage3Full, UnlockStage::Stage3Full);
    }

    #[test]
    fn test_progressive_unlock_all_fields() {
        let unlock = ProgressiveUnlock::new(7200);
        
        assert_eq!(unlock.current_stage, UnlockStage::Locked);
        assert_eq!(unlock.stage_timestamp, 0);
        assert_eq!(unlock.stage_duration, 7200);
        assert_eq!(unlock.stage_security_requirements, [50, 70, 90]);
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_exact_time() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 1000;
        
        // Exactly 3600 seconds passed
        let result = unlock.can_advance_stage(4600, 60);
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_exact_security() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 1000;
        
        // Exactly 50 security score (minimum requirement)
        let result = unlock.can_advance_stage(5000, 50);
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_stage2_to_stage3() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.current_stage = UnlockStage::Stage2Enhanced;
        unlock.stage_timestamp = 1000;
        
        // Can advance if enough time and security score >= 90
        let result = unlock.can_advance_stage(5000, 95);
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_progressive_unlock_advance_stage_preserves_other_fields() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 1000;
        let original_duration = unlock.stage_duration;
        let original_requirements = unlock.stage_security_requirements;
        
        unlock.advance_stage(2000).unwrap();
        
        assert_eq!(unlock.stage_duration, original_duration);
        assert_eq!(unlock.stage_security_requirements, original_requirements);
        assert_eq!(unlock.current_stage, UnlockStage::Stage1Basic);
        assert_eq!(unlock.stage_timestamp, 2000);
    }

    #[test]
    fn test_progressive_unlock_can_advance_stage_negative_time() {
        let mut unlock = ProgressiveUnlock::new(3600);
        unlock.stage_timestamp = 2000;
        
        // Current time before stage timestamp
        // checked_sub for i64 returns negative value, not None
        // So time_in_stage will be negative, which is < stage_duration, so returns Ok(false)
        let result = unlock.can_advance_stage(1000, 60);
        
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false (cannot advance)
    }
}
