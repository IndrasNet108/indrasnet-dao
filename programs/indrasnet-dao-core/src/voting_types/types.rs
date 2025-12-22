//! Voting types

use anchor_lang::prelude::*;

/// Vote type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

impl Space for VoteType {
    const INIT_SPACE: usize = 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vote_type_variants() {
        assert_eq!(VoteType::Yes, VoteType::Yes);
        assert_eq!(VoteType::No, VoteType::No);
        assert_eq!(VoteType::Abstain, VoteType::Abstain);
    }
}
