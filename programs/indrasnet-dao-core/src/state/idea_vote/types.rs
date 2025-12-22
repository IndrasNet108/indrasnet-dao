//! Idea Vote Types
//!
//! This module defines the IdeaVote account structure
//! for tracking votes on ideas in the DAO.

use anchor_lang::prelude::*;
use crate::voting_types::VoteType;

#[account]
#[derive(InitSpace)]
pub struct IdeaVote {
    pub idea_id: u64,
    pub voter: Pubkey,
    pub vote_type: VoteType,
    pub weight: u64,
    pub cast_at: i64,
    pub bump: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_idea_vote_structure() {
        let voter = create_test_pubkey(1);
        let vote = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        
        assert_eq!(vote.idea_id, 1);
        assert_eq!(vote.voter, voter);
        assert_eq!(vote.weight, 100);
        assert_eq!(vote.cast_at, 1000);
    }

    #[test]
    fn test_idea_vote_all_fields() {
        let voter = create_test_pubkey(10);
        let vote = IdeaVote {
            idea_id: 999,
            voter,
            vote_type: crate::voting_types::VoteType::No,
            weight: 500,
            cast_at: 5000,
            bump: 128,
        };
        
        assert_eq!(vote.idea_id, 999);
        assert_eq!(vote.voter, voter);
        assert_eq!(vote.vote_type, crate::voting_types::VoteType::No);
        assert_eq!(vote.weight, 500);
        assert_eq!(vote.cast_at, 5000);
        assert_eq!(vote.bump, 128);
    }

    #[test]
    fn test_idea_vote_vote_types() {
        let voter = create_test_pubkey(1);
        
        let yes_vote = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        assert_eq!(yes_vote.vote_type, crate::voting_types::VoteType::Yes);
        
        let no_vote = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::No,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        assert_eq!(no_vote.vote_type, crate::voting_types::VoteType::No);
    }

    #[test]
    fn test_idea_vote_zero_weight() {
        let voter = create_test_pubkey(1);
        let vote = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 0,
            cast_at: 1000,
            bump: 255,
        };
        
        assert_eq!(vote.weight, 0);
    }

    #[test]
    fn test_idea_vote_max_weight() {
        let voter = create_test_pubkey(1);
        let vote = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: u64::MAX,
            cast_at: 1000,
            bump: 255,
        };
        
        assert_eq!(vote.weight, u64::MAX);
    }

    #[test]
    fn test_idea_vote_different_ideas() {
        let voter = create_test_pubkey(1);
        
        let vote1 = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        
        let vote2 = IdeaVote {
            idea_id: 2,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        
        assert_ne!(vote1.idea_id, vote2.idea_id);
    }

    #[test]
    fn test_idea_vote_different_voters() {
        let voter1 = create_test_pubkey(1);
        let voter2 = create_test_pubkey(2);
        
        let vote1 = IdeaVote {
            idea_id: 1,
            voter: voter1,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        
        let vote2 = IdeaVote {
            idea_id: 1,
            voter: voter2,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        
        assert_ne!(vote1.voter, vote2.voter);
    }

    #[test]
    fn test_idea_vote_different_timestamps() {
        let voter = create_test_pubkey(1);
        
        let vote1 = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        
        let vote2 = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 2000,
            bump: 255,
        };
        
        assert_ne!(vote1.cast_at, vote2.cast_at);
    }

    #[test]
    fn test_idea_vote_different_bumps() {
        let voter = create_test_pubkey(1);
        
        for bump in [0u8, 50u8, 128u8, 255u8] {
            let vote = IdeaVote {
                idea_id: 1,
                voter,
                vote_type: crate::voting_types::VoteType::Yes,
                weight: 100,
                cast_at: 1000,
                bump,
            };
            
            assert_eq!(vote.bump, bump);
        }
    }

    #[test]
    fn test_idea_vote_negative_timestamp() {
        let voter = create_test_pubkey(1);
        let vote = IdeaVote {
            idea_id: 1,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: -1000,
            bump: 255,
        };
        
        assert_eq!(vote.cast_at, -1000);
    }

    #[test]
    fn test_idea_vote_large_idea_id() {
        let voter = create_test_pubkey(1);
        let vote = IdeaVote {
            idea_id: u64::MAX,
            voter,
            vote_type: crate::voting_types::VoteType::Yes,
            weight: 100,
            cast_at: 1000,
            bump: 255,
        };
        
        assert_eq!(vote.idea_id, u64::MAX);
    }

    #[test]
    fn test_idea_vote_all_vote_types() {
        let voter = create_test_pubkey(1);
        let vote_types = vec![
            crate::voting_types::VoteType::Yes,
            crate::voting_types::VoteType::No,
            crate::voting_types::VoteType::Abstain,
        ];
        
        for vote_type in vote_types {
            let vote = IdeaVote {
                idea_id: 1,
                voter,
                vote_type,
                weight: 100,
                cast_at: 1000,
                bump: 255,
            };
            assert_eq!(vote.vote_type, vote_type);
        }
    }
}
