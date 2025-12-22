//! Mesh Group Operating Protocol
//!
//! Defines operating protocol for mesh groups: meeting frequency, quorum, inactivity handling.
//! Simplified version for Track A (basic protocol only).

use anchor_lang::prelude::*;

/// Meeting frequency for mesh group
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MeetingFrequency {
    /// Weekly meetings
    Weekly,
    /// Bi-weekly meetings
    BiWeekly,
    /// Monthly meetings
    Monthly,
    /// On-demand meetings (no fixed schedule)
    OnDemand,
}

/// Operating protocol for mesh group
/// 
/// Simplified version for Track A: basic protocol only
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OperatingProtocol {
    /// Meeting frequency
    pub meeting_frequency: MeetingFrequency,
    /// Decision quorum (0.5 = 50%, 1.0 = 100%)
    pub decision_quorum: u8,  // 50-100 (representing 0.5-1.0 as percentage)
    /// Minimum contributions required for grant eligibility
    pub contribution_threshold: u32,
    /// Inactivity timeout in days (auto-pause if no activity)
    pub inactivity_timeout_days: u16,
}

impl OperatingProtocol {
    /// Default protocol for Track A
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            meeting_frequency: MeetingFrequency::OnDemand,
            decision_quorum: 60,  // 60% quorum
            contribution_threshold: 3,  // Minimum 3 contributions
            inactivity_timeout_days: 90,  // 90 days inactivity = pause
        }
    }
    
    /// Check if quorum is reached
    pub fn is_quorum_reached(&self, votes: u64, total_members: u64) -> bool {
        if total_members == 0 {
            return false;
        }
        let required = (total_members as u128 * self.decision_quorum as u128) / 100;
        votes >= required as u64
    }
    
    /// Check if group should be paused due to inactivity
    pub fn should_pause_for_inactivity(&self, last_activity: i64, current_time: i64) -> bool {
        let days_since_activity = (current_time - last_activity) / (24 * 60 * 60);
        days_since_activity >= self.inactivity_timeout_days as i64
    }
}

impl anchor_lang::Space for OperatingProtocol {
    const INIT_SPACE: usize = 1 + 1 + 4 + 2; // meeting_frequency + decision_quorum + contribution_threshold + inactivity_timeout_days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operating_protocol_default() {
        let protocol = OperatingProtocol::default();
        
        assert_eq!(protocol.meeting_frequency, MeetingFrequency::OnDemand);
        assert_eq!(protocol.decision_quorum, 60);
        assert_eq!(protocol.contribution_threshold, 3);
        assert_eq!(protocol.inactivity_timeout_days, 90);
    }

    #[test]
    fn test_operating_protocol_is_quorum_reached() {
        let protocol = OperatingProtocol::default();
        
        // 60% quorum: 6 votes out of 10 members = 60% = reached
        assert!(protocol.is_quorum_reached(6, 10));
        
        // 5 votes out of 10 = 50% < 60% = not reached
        assert!(!protocol.is_quorum_reached(5, 10));
        
        // 0 members = not reached
        assert!(!protocol.is_quorum_reached(0, 0));
    }

    #[test]
    fn test_operating_protocol_should_pause_for_inactivity() {
        let protocol = OperatingProtocol::default();
        
        // 90 days passed = should pause
        assert!(protocol.should_pause_for_inactivity(1000, 1000 + 90 * 24 * 60 * 60));
        
        // 89 days passed = should not pause
        assert!(!protocol.should_pause_for_inactivity(1000, 1000 + 89 * 24 * 60 * 60));
        
        // 0 days passed = should not pause
        assert!(!protocol.should_pause_for_inactivity(1000, 1000));
    }

    #[test]
    fn test_meeting_frequency_variants() {
        assert_eq!(MeetingFrequency::Weekly, MeetingFrequency::Weekly);
        assert_eq!(MeetingFrequency::BiWeekly, MeetingFrequency::BiWeekly);
        assert_eq!(MeetingFrequency::Monthly, MeetingFrequency::Monthly);
        assert_eq!(MeetingFrequency::OnDemand, MeetingFrequency::OnDemand);
    }

    #[test]
    fn test_operating_protocol_is_quorum_reached_exact_threshold() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 60,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        // Exactly 60% = reached
        assert!(protocol.is_quorum_reached(6, 10));
    }

    #[test]
    fn test_operating_protocol_is_quorum_reached_above_threshold() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 60,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        // Above 60% = reached
        assert!(protocol.is_quorum_reached(7, 10));
        assert!(protocol.is_quorum_reached(10, 10));
    }

    #[test]
    fn test_operating_protocol_is_quorum_reached_below_threshold() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 60,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        // Below 60% = not reached
        assert!(!protocol.is_quorum_reached(5, 10));
        assert!(!protocol.is_quorum_reached(0, 10));
    }

    #[test]
    fn test_operating_protocol_is_quorum_reached_different_quorum() {
        // Test with 50% quorum
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 50,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        assert!(protocol.is_quorum_reached(5, 10));
        assert!(!protocol.is_quorum_reached(4, 10));
        
        // Test with 100% quorum
        let protocol_100 = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 100,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        assert!(protocol_100.is_quorum_reached(10, 10));
        assert!(!protocol_100.is_quorum_reached(9, 10));
    }

    #[test]
    fn test_operating_protocol_should_pause_for_inactivity_exact_threshold() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 60,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        // Exactly 90 days = should pause
        assert!(protocol.should_pause_for_inactivity(1000, 1000 + 90 * 24 * 60 * 60));
    }

    #[test]
    fn test_operating_protocol_should_pause_for_inactivity_different_timeout() {
        // Test with 30 days timeout
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 60,
            contribution_threshold: 3,
            inactivity_timeout_days: 30,
        };
        
        assert!(protocol.should_pause_for_inactivity(1000, 1000 + 30 * 24 * 60 * 60));
        assert!(!protocol.should_pause_for_inactivity(1000, 1000 + 29 * 24 * 60 * 60));
    }

    #[test]
    fn test_operating_protocol_should_pause_for_inactivity_negative_time() {
        let protocol = OperatingProtocol::default();
        
        // Negative time (future activity) = should not pause
        assert!(!protocol.should_pause_for_inactivity(2000, 1000));
    }

    #[test]
    fn test_meeting_frequency_all_variants_unique() {
        let frequencies = vec![
            MeetingFrequency::Weekly,
            MeetingFrequency::BiWeekly,
            MeetingFrequency::Monthly,
            MeetingFrequency::OnDemand,
        ];
        
        for i in 0..frequencies.len() {
            for j in (i + 1)..frequencies.len() {
                assert_ne!(frequencies[i], frequencies[j], "Duplicate frequency found");
            }
        }
    }

    #[test]
    fn test_meeting_frequency_equality() {
        assert_eq!(MeetingFrequency::Weekly, MeetingFrequency::Weekly);
        assert_ne!(MeetingFrequency::Weekly, MeetingFrequency::BiWeekly);
        assert_eq!(MeetingFrequency::OnDemand, MeetingFrequency::OnDemand);
    }

    #[test]
    fn test_operating_protocol_structure() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Monthly,
            decision_quorum: 75,
            contribution_threshold: 5,
            inactivity_timeout_days: 60,
        };
        
        assert_eq!(protocol.meeting_frequency, MeetingFrequency::Monthly);
        assert_eq!(protocol.decision_quorum, 75);
        assert_eq!(protocol.contribution_threshold, 5);
        assert_eq!(protocol.inactivity_timeout_days, 60);
    }

    #[test]
    fn test_operating_protocol_space() {
        assert_eq!(<OperatingProtocol as anchor_lang::Space>::INIT_SPACE, 1 + 1 + 4 + 2);
    }

    #[test]
    fn test_operating_protocol_clone() {
        let protocol1 = OperatingProtocol::default();
        let protocol2 = protocol1.clone();
        
        assert_eq!(protocol1.meeting_frequency, protocol2.meeting_frequency);
        assert_eq!(protocol1.decision_quorum, protocol2.decision_quorum);
        assert_eq!(protocol1.contribution_threshold, protocol2.contribution_threshold);
        assert_eq!(protocol1.inactivity_timeout_days, protocol2.inactivity_timeout_days);
    }

    #[test]
    fn test_operating_protocol_is_quorum_reached_large_numbers() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 50,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        // Large numbers
        assert!(protocol.is_quorum_reached(5000, 10000));
        assert!(!protocol.is_quorum_reached(4999, 10000));
    }

    #[test]
    fn test_operating_protocol_should_pause_for_inactivity_large_times() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::Weekly,
            decision_quorum: 60,
            contribution_threshold: 3,
            inactivity_timeout_days: 90,
        };
        
        let large_time = 1_000_000_000;
        assert!(protocol.should_pause_for_inactivity(large_time, large_time + 90 * 24 * 60 * 60));
    }

    #[test]
    fn test_meeting_frequency_copy() {
        let freq1 = MeetingFrequency::Weekly;
        let freq2 = freq1; // Copy trait
        assert_eq!(freq1, freq2);
    }

    #[test]
    fn test_meeting_frequency_space() {
        assert_eq!(<MeetingFrequency as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_operating_protocol_all_fields() {
        let protocol = OperatingProtocol {
            meeting_frequency: MeetingFrequency::BiWeekly,
            decision_quorum: 80,
            contribution_threshold: 10,
            inactivity_timeout_days: 120,
        };
        
        assert_eq!(protocol.meeting_frequency, MeetingFrequency::BiWeekly);
        assert_eq!(protocol.decision_quorum, 80);
        assert_eq!(protocol.contribution_threshold, 10);
        assert_eq!(protocol.inactivity_timeout_days, 120);
    }

    #[test]
    fn test_operating_protocol_is_quorum_reached_all_quorum_levels() {
        for quorum in [50u8, 60u8, 70u8, 80u8, 90u8, 100u8] {
            let protocol = OperatingProtocol {
                meeting_frequency: MeetingFrequency::Weekly,
                decision_quorum: quorum,
                contribution_threshold: 3,
                inactivity_timeout_days: 90,
            };
            
            let total = 100u64;
            let required = (total as u128 * quorum as u128) / 100;
            assert!(protocol.is_quorum_reached(required as u64, total));
            assert!(!protocol.is_quorum_reached((required - 1) as u64, total));
        }
    }
}
