//! Grant Vote Types
//!
//! This module defines the GrantVote account structure
//! for tracking votes on grants in the DAO.

use anchor_lang::prelude::*;
use crate::voting_types::VoteType;

/// Voter type enum - determines vote weight
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VoterType {
    MeshGroupMember,  // Weight 2x
    DaoMember,        // Weight 1x
    IdeaAuthor,       // Weight 1x
    Expert,           // Weight depends on competency
}

impl Space for VoterType {
    const INIT_SPACE: usize = 1;
}

/// Grant Vote account structure
/// 
/// Stores a single vote on a grant request.
/// PDA seeds: [b"grant_vote", grant.key(), voter.key()]
#[account]
#[derive(InitSpace)]
pub struct GrantVote {
    pub grant_id: u64,
    pub voter: Pubkey,
    pub vote_type: VoteType,      // Yes, No, Abstain
    pub weight: u64,               // 1x for DAO members, 2x for mesh group
    pub voter_type: VoterType,    // Voter type
    pub cast_at: i64,
    pub bump: u8,
}

impl GrantVote {
    /// Calculate vote weight based on voter type
    pub fn calculate_weight(voter_type: VoterType) -> u64 {
        match voter_type {
            VoterType::MeshGroupMember => 2,  // Double weight for mesh group members
            VoterType::DaoMember => 1,        // Standard weight for DAO members
            VoterType::IdeaAuthor => 1,       // Standard weight for idea author
            VoterType::Expert => 1,           // Base weight for expert (multiplied by competency)
        }
    }
    
    /// Calculate base weight for voter type
    pub fn calculate_base_weight(voter_type: VoterType) -> u64 {
        Self::calculate_weight(voter_type)
    }
    
    /// Calculate final weight with competency multiplier
    pub fn calculate_final_weight(base_weight: u64, competency_multiplier: u64) -> u64 {
        base_weight.saturating_mul(competency_multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_weight_mesh_group_member() {
        assert_eq!(GrantVote::calculate_weight(VoterType::MeshGroupMember), 2);
    }

    #[test]
    fn test_calculate_weight_dao_member() {
        assert_eq!(GrantVote::calculate_weight(VoterType::DaoMember), 1);
    }

    #[test]
    fn test_calculate_weight_idea_author() {
        assert_eq!(GrantVote::calculate_weight(VoterType::IdeaAuthor), 1);
    }

    #[test]
    fn test_calculate_weight_expert() {
        assert_eq!(GrantVote::calculate_weight(VoterType::Expert), 1);
    }

    #[test]
    fn test_calculate_base_weight() {
        assert_eq!(GrantVote::calculate_base_weight(VoterType::MeshGroupMember), 2);
        assert_eq!(GrantVote::calculate_base_weight(VoterType::DaoMember), 1);
    }

    #[test]
    fn test_calculate_final_weight() {
        assert_eq!(GrantVote::calculate_final_weight(2, 3), 6);
        assert_eq!(GrantVote::calculate_final_weight(1, 5), 5);
        assert_eq!(GrantVote::calculate_final_weight(0, 10), 0);
    }

    #[test]
    fn test_calculate_final_weight_overflow() {
        // Test saturation on overflow
        let max = u64::MAX;
        let result = GrantVote::calculate_final_weight(max, 2);
        assert_eq!(result, max); // Should saturate, not panic
    }

    #[test]
    fn test_voter_type_variants() {
        assert_eq!(VoterType::MeshGroupMember, VoterType::MeshGroupMember);
        assert_eq!(VoterType::DaoMember, VoterType::DaoMember);
        assert_eq!(VoterType::IdeaAuthor, VoterType::IdeaAuthor);
        assert_eq!(VoterType::Expert, VoterType::Expert);
    }

    #[test]
    fn test_voter_type_all_variants_unique() {
        let variants = vec![
            VoterType::MeshGroupMember,
            VoterType::DaoMember,
            VoterType::IdeaAuthor,
            VoterType::Expert,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_voter_type_copy() {
        let type1 = VoterType::MeshGroupMember;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_voter_type_space() {
        assert_eq!(<VoterType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_grant_vote_structure() {
        use crate::voting_types::VoteType;
        use anchor_lang::prelude::Pubkey;
        
        let voter = Pubkey::from([1u8; 32]);
        let vote = GrantVote {
            grant_id: 1,
            voter,
            vote_type: VoteType::Yes,
            weight: 2,
            voter_type: VoterType::MeshGroupMember,
            cast_at: 1000,
            bump: 255,
        };
        
        assert_eq!(vote.grant_id, 1);
        assert_eq!(vote.voter, voter);
        assert_eq!(vote.vote_type, VoteType::Yes);
        assert_eq!(vote.weight, 2);
        assert_eq!(vote.voter_type, VoterType::MeshGroupMember);
        assert_eq!(vote.cast_at, 1000);
        assert_eq!(vote.bump, 255);
    }

    #[test]
    fn test_calculate_weight_all_types() {
        assert_eq!(GrantVote::calculate_weight(VoterType::MeshGroupMember), 2);
        assert_eq!(GrantVote::calculate_weight(VoterType::DaoMember), 1);
        assert_eq!(GrantVote::calculate_weight(VoterType::IdeaAuthor), 1);
        assert_eq!(GrantVote::calculate_weight(VoterType::Expert), 1);
    }

    #[test]
    fn test_calculate_base_weight_all_types() {
        assert_eq!(GrantVote::calculate_base_weight(VoterType::MeshGroupMember), 2);
        assert_eq!(GrantVote::calculate_base_weight(VoterType::DaoMember), 1);
        assert_eq!(GrantVote::calculate_base_weight(VoterType::IdeaAuthor), 1);
        assert_eq!(GrantVote::calculate_base_weight(VoterType::Expert), 1);
    }

    #[test]
    fn test_calculate_final_weight_zero_base() {
        assert_eq!(GrantVote::calculate_final_weight(0, 5), 0);
    }

    #[test]
    fn test_calculate_final_weight_zero_multiplier() {
        assert_eq!(GrantVote::calculate_final_weight(2, 0), 0);
    }

    #[test]
    fn test_calculate_final_weight_one_multiplier() {
        assert_eq!(GrantVote::calculate_final_weight(2, 1), 2);
        assert_eq!(GrantVote::calculate_final_weight(1, 1), 1);
    }

    #[test]
    fn test_calculate_final_weight_large_values() {
        assert_eq!(GrantVote::calculate_final_weight(100, 50), 5000);
        assert_eq!(GrantVote::calculate_final_weight(10, 100), 1000);
    }

    #[test]
    fn test_grant_vote_all_fields() {
        use crate::voting_types::VoteType;
        use anchor_lang::prelude::Pubkey;
        
        let voter = Pubkey::from([5u8; 32]);
        let vote = GrantVote {
            grant_id: 999,
            voter,
            vote_type: VoteType::No,
            weight: 1,
            voter_type: VoterType::DaoMember,
            cast_at: 5000,
            bump: 128,
        };
        
        assert_eq!(vote.grant_id, 999);
        assert_eq!(vote.voter, voter);
        assert_eq!(vote.vote_type, VoteType::No);
        assert_eq!(vote.weight, 1);
        assert_eq!(vote.voter_type, VoterType::DaoMember);
        assert_eq!(vote.cast_at, 5000);
        assert_eq!(vote.bump, 128);
    }

    #[test]
    fn test_voter_type_equality() {
        assert_eq!(VoterType::MeshGroupMember, VoterType::MeshGroupMember);
        assert_ne!(VoterType::MeshGroupMember, VoterType::DaoMember);
        assert_eq!(VoterType::DaoMember, VoterType::DaoMember);
        assert_ne!(VoterType::DaoMember, VoterType::IdeaAuthor);
    }

    #[test]
    fn test_calculate_final_weight_all_combinations() {
        let base_weights = vec![1, 2, 5, 10];
        let multipliers = vec![1, 2, 3, 5, 10];
        
        for base in &base_weights {
            for mult in &multipliers {
                let result = GrantVote::calculate_final_weight(*base, *mult);
                assert_eq!(result, base.saturating_mul(*mult));
            }
        }
    }
}
