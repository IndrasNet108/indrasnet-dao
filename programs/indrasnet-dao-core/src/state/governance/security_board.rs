//! Security Board module
//!
//! Security board management for governance
//!
//! On-chain: Metadata for security board members, decisions
//! Off-chain: Actual security analysis, recommendations

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Security board member role
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum SecurityBoardMemberRole {
    /// Chairperson
    Chairperson,
    /// Member
    Member,
    /// Advisor
    Advisor,
}

/// Security board decision status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum SecurityBoardDecisionStatus {
    /// Decision pending
    Pending,
    /// Decision approved
    Approved,
    /// Decision rejected
    Rejected,
    /// Decision deferred
    Deferred,
}

/// Security board member metadata (on-chain)
///
/// Stores metadata for security board members
#[account]
#[derive(InitSpace)]
pub struct SecurityBoardMemberMetadata {
    /// Member ID
    pub member_id: u64,
    /// Member pubkey
    pub member_pubkey: Pubkey,
    /// Role
    pub role: SecurityBoardMemberRole,
    /// Joined at
    pub joined_at: i64,
    /// Last active at
    pub last_active_at: i64,
    /// Decisions participated
    pub decisions_participated: u32,
    /// Bump seed
    pub bump: u8,
}

/// Security board decision metadata (on-chain)
///
/// Stores metadata for security board decisions
#[account]
#[derive(InitSpace)]
pub struct SecurityBoardDecisionMetadata {
    /// Decision ID
    pub decision_id: u64,
    /// Proposal ID (if related)
    pub proposal_id: Option<u64>,
    /// Status
    pub status: SecurityBoardDecisionStatus,
    /// Created at
    pub created_at: i64,
    /// Decided at
    pub decided_at: Option<i64>,
    /// Decision data hash
    pub decision_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for security board
pub mod onchain {
    use super::*;

    /// Initialize security board member
    pub fn initialize_board_member(
        member: &mut SecurityBoardMemberMetadata,
        member_id: u64,
        member_pubkey: Pubkey,
        role: SecurityBoardMemberRole,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(member_id > 0, IndrasError::InvalidInput);
        
        member.member_id = member_id;
        member.member_pubkey = member_pubkey;
        member.role = role;
        member.joined_at = current_time;
        member.last_active_at = current_time;
        member.decisions_participated = 0;
        member.bump = bump;
        
        Ok(())
    }

    /// Initialize security board decision
    pub fn initialize_decision(
        decision: &mut SecurityBoardDecisionMetadata,
        decision_id: u64,
        proposal_id: Option<u64>,
        decision_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(decision_id > 0, IndrasError::InvalidInput);
        
        decision.decision_id = decision_id;
        decision.proposal_id = proposal_id;
        decision.status = SecurityBoardDecisionStatus::Pending;
        decision.created_at = current_time;
        decision.decided_at = None;
        decision.decision_data_hash = decision_data_hash;
        decision.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for security board
///
/// These functions should be implemented in off-chain service
/// for actual security analysis and recommendations.
pub mod offchain {
    // Off-chain functions will be implemented in separate service
    
    /// Analyze security proposal
    pub fn analyze_security_proposal(_proposal_id: u64) -> Vec<String> {
        // Implementation in off-chain service
        // Analyzes security proposal and returns recommendations
        vec![]
    }

    /// Generate security board report
    pub fn generate_board_report(_board_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        // Generates security board report
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_board_member() -> SecurityBoardMemberMetadata {
        SecurityBoardMemberMetadata {
            member_id: 1,
            member_pubkey: create_test_pubkey(1),
            role: SecurityBoardMemberRole::Member,
            joined_at: 1000,
            last_active_at: 1000,
            decisions_participated: 0,
            bump: 255,
        }
    }

    fn create_test_decision() -> SecurityBoardDecisionMetadata {
        SecurityBoardDecisionMetadata {
            decision_id: 1,
            proposal_id: Some(100),
            status: SecurityBoardDecisionStatus::Pending,
            created_at: 1000,
            decided_at: None,
            decision_data_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_security_board_member_role_variants() {
        assert_eq!(SecurityBoardMemberRole::Chairperson, SecurityBoardMemberRole::Chairperson);
        assert_eq!(SecurityBoardMemberRole::Member, SecurityBoardMemberRole::Member);
        assert_eq!(SecurityBoardMemberRole::Advisor, SecurityBoardMemberRole::Advisor);
    }

    #[test]
    fn test_security_board_decision_status_variants() {
        assert_eq!(SecurityBoardDecisionStatus::Pending, SecurityBoardDecisionStatus::Pending);
        assert_eq!(SecurityBoardDecisionStatus::Approved, SecurityBoardDecisionStatus::Approved);
        assert_eq!(SecurityBoardDecisionStatus::Rejected, SecurityBoardDecisionStatus::Rejected);
        assert_eq!(SecurityBoardDecisionStatus::Deferred, SecurityBoardDecisionStatus::Deferred);
    }

    #[test]
    fn test_security_board_member_metadata_structure() {
        let member = create_test_board_member();
        assert_eq!(member.member_id, 1);
        assert_eq!(member.role, SecurityBoardMemberRole::Member);
        assert_eq!(member.joined_at, 1000);
        assert_eq!(member.last_active_at, 1000);
        assert_eq!(member.decisions_participated, 0);
        assert_eq!(member.bump, 255);
    }

    #[test]
    fn test_initialize_board_member() {
        let mut member = SecurityBoardMemberMetadata {
            member_id: 0,
            member_pubkey: create_test_pubkey(0),
            role: SecurityBoardMemberRole::Member,
            joined_at: 0,
            last_active_at: 0,
            decisions_participated: 0,
            bump: 0,
        };

        let pubkey = create_test_pubkey(5);
        let result = onchain::initialize_board_member(
            &mut member,
            800,
            pubkey,
            SecurityBoardMemberRole::Chairperson,
            13000,
            48,
        );

        assert!(result.is_ok());
        assert_eq!(member.member_id, 800);
        assert_eq!(member.member_pubkey, pubkey);
        assert_eq!(member.role, SecurityBoardMemberRole::Chairperson);
        assert_eq!(member.joined_at, 13000);
        assert_eq!(member.last_active_at, 13000);
        assert_eq!(member.decisions_participated, 0);
        assert_eq!(member.bump, 48);
    }

    #[test]
    fn test_initialize_board_member_invalid_id() {
        let mut member = create_test_board_member();

        let result = onchain::initialize_board_member(
            &mut member,
            0, // Invalid: member_id must be > 0
            create_test_pubkey(1),
            SecurityBoardMemberRole::Member,
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_board_member_all_roles() {
        let roles = vec![
            SecurityBoardMemberRole::Chairperson,
            SecurityBoardMemberRole::Member,
            SecurityBoardMemberRole::Advisor,
        ];

        for role in roles {
            let mut member = create_test_board_member();
            let result = onchain::initialize_board_member(
                &mut member,
                1,
                create_test_pubkey(1),
                role,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(member.role, role);
        }
    }

    #[test]
    fn test_initialize_decision() {
        let mut decision = SecurityBoardDecisionMetadata {
            decision_id: 0,
            proposal_id: None,
            status: SecurityBoardDecisionStatus::Pending,
            created_at: 0,
            decided_at: None,
            decision_data_hash: [0u8; 32],
            bump: 0,
        };

        let data_hash = [6u8; 32];
        let result = onchain::initialize_decision(
            &mut decision,
            900,
            Some(1000),
            data_hash,
            14000,
            112,
        );

        assert!(result.is_ok());
        assert_eq!(decision.decision_id, 900);
        assert_eq!(decision.proposal_id, Some(1000));
        assert_eq!(decision.status, SecurityBoardDecisionStatus::Pending);
        assert_eq!(decision.created_at, 14000);
        assert_eq!(decision.decided_at, None);
        assert_eq!(decision.decision_data_hash, data_hash);
        assert_eq!(decision.bump, 112);
    }

    #[test]
    fn test_initialize_decision_invalid_id() {
        let mut decision = create_test_decision();

        let result = onchain::initialize_decision(
            &mut decision,
            0, // Invalid: decision_id must be > 0
            Some(1000),
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_decision_without_proposal_id() {
        let mut decision = create_test_decision();

        let result = onchain::initialize_decision(
            &mut decision,
            1,
            None, // No related proposal
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(decision.proposal_id, None);
    }

    #[test]
    fn test_initialize_decision_status_always_pending_on_init() {
        let mut decision = create_test_decision();
        decision.status = SecurityBoardDecisionStatus::Approved;

        let result = onchain::initialize_decision(
            &mut decision,
            1,
            Some(100),
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Pending on initialization
        assert_eq!(decision.status, SecurityBoardDecisionStatus::Pending);
    }

    #[test]
    fn test_initialize_decision_decided_at_none_on_init() {
        let mut decision = create_test_decision();
        decision.decided_at = Some(5000);

        let result = onchain::initialize_decision(
            &mut decision,
            1,
            Some(100),
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // decided_at should always be None on initialization
        assert_eq!(decision.decided_at, None);
    }

    #[test]
    fn test_security_board_decision_all_statuses() {
        let statuses = vec![
            SecurityBoardDecisionStatus::Pending,
            SecurityBoardDecisionStatus::Approved,
            SecurityBoardDecisionStatus::Rejected,
            SecurityBoardDecisionStatus::Deferred,
        ];

        for status in statuses {
            let mut decision = create_test_decision();
            decision.status = status;
            assert_eq!(decision.status, status);
        }
    }

    #[test]
    fn test_initialize_board_member_timestamps() {
        let mut member = create_test_board_member();

        let result = onchain::initialize_board_member(
            &mut member,
            1,
            create_test_pubkey(1),
            SecurityBoardMemberRole::Member,
            65432,
            200,
        );

        assert!(result.is_ok());
        // Both joined_at and last_active_at should be set to current_time
        assert_eq!(member.joined_at, 65432);
        assert_eq!(member.last_active_at, 65432);
    }

    #[test]
    fn test_initialize_decision_data_hash() {
        let mut decision = create_test_decision();
        let custom_hash = [111u8; 32];

        let result = onchain::initialize_decision(
            &mut decision,
            1,
            Some(200),
            custom_hash,
            2000,
            150,
        );

        assert!(result.is_ok());
        assert_eq!(decision.decision_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_board_member_all_roles_initialization() {
        let roles = vec![
            SecurityBoardMemberRole::Chairperson,
            SecurityBoardMemberRole::Member,
            SecurityBoardMemberRole::Advisor,
        ];
        
        for role in roles {
            let mut member = SecurityBoardMemberMetadata {
                member_id: 0,
                member_pubkey: create_test_pubkey(0),
                role: SecurityBoardMemberRole::Member,
                joined_at: 0,
                last_active_at: 0,
                decisions_participated: 5, // Should be reset to 0
                bump: 0,
            };
            
            let result = onchain::initialize_board_member(
                &mut member,
                1,
                create_test_pubkey(1),
                role,
                2000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(member.role, role);
            assert_eq!(member.decisions_participated, 0); // Should be reset
        }
    }

    #[test]
    fn test_initialize_board_member_large_member_id() {
        let mut member = create_test_board_member();
        
        let result = onchain::initialize_board_member(
            &mut member,
            u64::MAX,
            create_test_pubkey(1),
            SecurityBoardMemberRole::Member,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(member.member_id, u64::MAX);
    }

    #[test]
    fn test_initialize_decision_all_statuses_initialization() {
        // Initialize should always set status to Pending
        let mut decision = SecurityBoardDecisionMetadata {
            decision_id: 0,
            proposal_id: None,
            status: SecurityBoardDecisionStatus::Approved, // Will be reset
            created_at: 0,
            decided_at: Some(5000), // Will be reset
            decision_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_decision(
            &mut decision,
            1,
            Some(100),
            [1u8; 32],
            2000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(decision.status, SecurityBoardDecisionStatus::Pending);
        assert_eq!(decision.decided_at, None);
    }

    #[test]
    fn test_initialize_decision_large_decision_id() {
        let mut decision = create_test_decision();
        
        let result = onchain::initialize_decision(
            &mut decision,
            u64::MAX,
            Some(100),
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(decision.decision_id, u64::MAX);
    }

    #[test]
    fn test_security_board_member_role_equality() {
        assert_eq!(SecurityBoardMemberRole::Chairperson, SecurityBoardMemberRole::Chairperson);
        assert_ne!(SecurityBoardMemberRole::Chairperson, SecurityBoardMemberRole::Member);
        assert_ne!(SecurityBoardMemberRole::Chairperson, SecurityBoardMemberRole::Advisor);
        assert_eq!(SecurityBoardMemberRole::Member, SecurityBoardMemberRole::Member);
        assert_ne!(SecurityBoardMemberRole::Member, SecurityBoardMemberRole::Advisor);
        assert_eq!(SecurityBoardMemberRole::Advisor, SecurityBoardMemberRole::Advisor);
    }

    #[test]
    fn test_security_board_decision_status_equality() {
        assert_eq!(SecurityBoardDecisionStatus::Pending, SecurityBoardDecisionStatus::Pending);
        assert_ne!(SecurityBoardDecisionStatus::Pending, SecurityBoardDecisionStatus::Approved);
        assert_ne!(SecurityBoardDecisionStatus::Pending, SecurityBoardDecisionStatus::Rejected);
        assert_eq!(SecurityBoardDecisionStatus::Approved, SecurityBoardDecisionStatus::Approved);
        assert_eq!(SecurityBoardDecisionStatus::Rejected, SecurityBoardDecisionStatus::Rejected);
        assert_eq!(SecurityBoardDecisionStatus::Deferred, SecurityBoardDecisionStatus::Deferred);
    }

    #[test]
    fn test_security_board_member_metadata_all_fields() {
        let member = SecurityBoardMemberMetadata {
            member_id: 999,
            member_pubkey: create_test_pubkey(99),
            role: SecurityBoardMemberRole::Chairperson,
            joined_at: 5000,
            last_active_at: 6000,
            decisions_participated: 50,
            bump: 128,
        };
        
        assert_eq!(member.member_id, 999);
        assert_eq!(member.member_pubkey, create_test_pubkey(99));
        assert_eq!(member.role, SecurityBoardMemberRole::Chairperson);
        assert_eq!(member.joined_at, 5000);
        assert_eq!(member.last_active_at, 6000);
        assert_eq!(member.decisions_participated, 50);
        assert_eq!(member.bump, 128);
    }

    #[test]
    fn test_security_board_decision_metadata_all_fields() {
        let decision = SecurityBoardDecisionMetadata {
            decision_id: 888,
            proposal_id: Some(777),
            status: SecurityBoardDecisionStatus::Approved,
            created_at: 3000,
            decided_at: Some(4000),
            decision_data_hash: [99u8; 32],
            bump: 200,
        };
        
        assert_eq!(decision.decision_id, 888);
        assert_eq!(decision.proposal_id, Some(777));
        assert_eq!(decision.status, SecurityBoardDecisionStatus::Approved);
        assert_eq!(decision.created_at, 3000);
        assert_eq!(decision.decided_at, Some(4000));
        assert_eq!(decision.decision_data_hash, [99u8; 32]);
        assert_eq!(decision.bump, 200);
    }

    #[test]
    fn test_initialize_board_member_different_pubkeys() {
        let mut member = create_test_board_member();
        let pubkey1 = create_test_pubkey(10);
        let pubkey2 = create_test_pubkey(20);
        
        let result1 = onchain::initialize_board_member(
            &mut member,
            1,
            pubkey1,
            SecurityBoardMemberRole::Member,
            1000,
            255,
        );
        assert!(result1.is_ok());
        assert_eq!(member.member_pubkey, pubkey1);
        
        let result2 = onchain::initialize_board_member(
            &mut member,
            2,
            pubkey2,
            SecurityBoardMemberRole::Member,
            2000,
            255,
        );
        assert!(result2.is_ok());
        assert_eq!(member.member_pubkey, pubkey2);
    }

    #[test]
    fn test_initialize_decision_different_proposal_ids() {
        let mut decision = create_test_decision();
        
        // With proposal_id
        let result1 = onchain::initialize_decision(
            &mut decision,
            1,
            Some(100),
            [0u8; 32],
            1000,
            255,
        );
        assert!(result1.is_ok());
        assert_eq!(decision.proposal_id, Some(100));
        
        // Without proposal_id
        let result2 = onchain::initialize_decision(
            &mut decision,
            2,
            None,
            [0u8; 32],
            2000,
            255,
        );
        assert!(result2.is_ok());
        assert_eq!(decision.proposal_id, None);
    }

    #[test]
    fn test_security_board_member_role_all_variants_unique() {
        let roles = vec![
            SecurityBoardMemberRole::Chairperson,
            SecurityBoardMemberRole::Member,
            SecurityBoardMemberRole::Advisor,
        ];
        
        for i in 0..roles.len() {
            for j in (i + 1)..roles.len() {
                assert_ne!(roles[i], roles[j], "Duplicate role found");
            }
        }
    }

    #[test]
    fn test_security_board_decision_status_all_variants_unique() {
        let statuses = vec![
            SecurityBoardDecisionStatus::Pending,
            SecurityBoardDecisionStatus::Approved,
            SecurityBoardDecisionStatus::Rejected,
            SecurityBoardDecisionStatus::Deferred,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_security_board_member_role_copy() {
        let role1 = SecurityBoardMemberRole::Chairperson;
        let role2 = role1; // Copy trait
        assert_eq!(role1, role2);
    }

    #[test]
    fn test_security_board_decision_status_copy() {
        let status1 = SecurityBoardDecisionStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_security_board_member_role_space() {
        assert_eq!(<SecurityBoardMemberRole as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_security_board_decision_status_space() {
        assert_eq!(<SecurityBoardDecisionStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_board_member_decisions_participated_always_zero() {
        let mut member = SecurityBoardMemberMetadata {
            member_id: 0,
            member_pubkey: create_test_pubkey(0),
            role: SecurityBoardMemberRole::Member,
            joined_at: 0,
            last_active_at: 0,
            decisions_participated: 999, // Will be reset
            bump: 0,
        };
        
        let result = onchain::initialize_board_member(
            &mut member,
            1,
            create_test_pubkey(1),
            SecurityBoardMemberRole::Member,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(member.decisions_participated, 0);
    }

    #[test]
    fn test_offchain_analyze_security_proposal() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::analyze_security_proposal(1);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_offchain_analyze_security_proposal_different_ids() {
        // Test with different IDs
        let result1 = offchain::analyze_security_proposal(1);
        let result2 = offchain::analyze_security_proposal(999);
        assert_eq!(result1, Vec::<String>::new());
        assert_eq!(result2, Vec::<String>::new());
    }

    #[test]
    fn test_offchain_generate_board_report() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::generate_board_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_generate_board_report_different_ids() {
        // Test with different IDs
        let result1 = offchain::generate_board_report(1);
        let result2 = offchain::generate_board_report(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
