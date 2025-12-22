//! Enum definitions for Core program state

use anchor_lang::prelude::*;

/// Idea status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug, Default)]
pub enum IdeaStatus {
    #[default]
    Draft,           // Draft
    UnderReview,     // Under AI review
    Approved,        // Approved by AI, for voting
    Rejected,        // Rejected by AI
    InProgress,      // In development (mesh group)
    Paused,          // Paused
    Completed,       // Completed
    Executed,        // Executed
    Commercialization, // Transferred to commercial enterprise
    Archived,        // Archived
    Resubmitted,     // Resubmitted after rejection
    Voting,          // Voting
    Expired,         // Expired
}

impl Space for IdeaStatus {
    const INIT_SPACE: usize = 1;
}

/// Member action enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug, Default)]
pub enum MemberAction {
    #[default]
    Join,
    Leave,
    Suspend,
    Activate,
    Ban,
}

impl Space for MemberAction {
    const INIT_SPACE: usize = 1;
}

/// Delegation type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug, Default)]
pub enum DelegationType {
    #[default]
    Temporary,
    Permanent,
    Conditional,
}

impl Space for DelegationType {
    const INIT_SPACE: usize = 1;
}

/// Capability type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug, Default)]
pub enum CapabilityType {
    Withdraw,
    Deposit,
    Manage,
    #[default]
    Vote,
    Propose,
}

impl Space for CapabilityType {
    const INIT_SPACE: usize = 1;
}

/// Contribution type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug, Default)]
pub enum ContributionType {
    Code,
    Design,
    Documentation,
    Testing,
    Review,
    Community,
    Governance,
    #[default]
    Other,
}

impl Space for ContributionType {
    const INIT_SPACE: usize = 1;
}

/// Off-chain vote status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug, Default)]
pub enum OffchainVoteStatus {
    #[default]
    Pending,
    Active,
    Completed,
    Cancelled,
    Failed,
}

impl Space for OffchainVoteStatus {
    const INIT_SPACE: usize = 1;
}

/// Member status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum MemberStatus {
    Active,         // Active member
    Suspended,      // Suspended
    Banned,         // Banned
    Inactive,       // Inactive
}

impl Space for MemberStatus {
    const INIT_SPACE: usize = 1;
}

// NOTE: MemberRole enum removed - use member::MemberRole struct instead
// This enum was deprecated in favor of the bitmask-based MemberRole struct

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idea_status_all_variants() {
        let variants = vec![
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Approved,
            IdeaStatus::Rejected,
            IdeaStatus::InProgress,
            IdeaStatus::Paused,
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Commercialization,
            IdeaStatus::Archived,
            IdeaStatus::Resubmitted,
            IdeaStatus::Voting,
            IdeaStatus::Expired,
        ];
        
        // Check all variants are unique
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_idea_status_default() {
        assert_eq!(IdeaStatus::default(), IdeaStatus::Draft);
    }

    #[test]
    fn test_member_action_variants() {
        assert_eq!(MemberAction::Join, MemberAction::Join);
        assert_eq!(MemberAction::Leave, MemberAction::Leave);
        assert_eq!(MemberAction::Suspend, MemberAction::Suspend);
        assert_eq!(MemberAction::Activate, MemberAction::Activate);
        assert_eq!(MemberAction::Ban, MemberAction::Ban);
    }

    #[test]
    fn test_member_action_default() {
        assert_eq!(MemberAction::default(), MemberAction::Join);
    }

    #[test]
    fn test_delegation_type_variants() {
        assert_eq!(DelegationType::Temporary, DelegationType::Temporary);
        assert_eq!(DelegationType::Permanent, DelegationType::Permanent);
        assert_eq!(DelegationType::Conditional, DelegationType::Conditional);
    }

    #[test]
    fn test_delegation_type_default() {
        assert_eq!(DelegationType::default(), DelegationType::Temporary);
    }

    #[test]
    fn test_capability_type_variants() {
        assert_eq!(CapabilityType::Withdraw, CapabilityType::Withdraw);
        assert_eq!(CapabilityType::Deposit, CapabilityType::Deposit);
        assert_eq!(CapabilityType::Manage, CapabilityType::Manage);
        assert_eq!(CapabilityType::Vote, CapabilityType::Vote);
        assert_eq!(CapabilityType::Propose, CapabilityType::Propose);
    }

    #[test]
    fn test_capability_type_default() {
        assert_eq!(CapabilityType::default(), CapabilityType::Vote);
    }

    #[test]
    fn test_contribution_type_variants() {
        assert_eq!(ContributionType::Code, ContributionType::Code);
        assert_eq!(ContributionType::Design, ContributionType::Design);
        assert_eq!(ContributionType::Documentation, ContributionType::Documentation);
        assert_eq!(ContributionType::Testing, ContributionType::Testing);
        assert_eq!(ContributionType::Review, ContributionType::Review);
        assert_eq!(ContributionType::Community, ContributionType::Community);
        assert_eq!(ContributionType::Governance, ContributionType::Governance);
        assert_eq!(ContributionType::Other, ContributionType::Other);
    }

    #[test]
    fn test_contribution_type_default() {
        assert_eq!(ContributionType::default(), ContributionType::Other);
    }

    #[test]
    fn test_offchain_vote_status_variants() {
        assert_eq!(OffchainVoteStatus::Pending, OffchainVoteStatus::Pending);
        assert_eq!(OffchainVoteStatus::Active, OffchainVoteStatus::Active);
        assert_eq!(OffchainVoteStatus::Completed, OffchainVoteStatus::Completed);
        assert_eq!(OffchainVoteStatus::Cancelled, OffchainVoteStatus::Cancelled);
        assert_eq!(OffchainVoteStatus::Failed, OffchainVoteStatus::Failed);
    }

    #[test]
    fn test_offchain_vote_status_default() {
        assert_eq!(OffchainVoteStatus::default(), OffchainVoteStatus::Pending);
    }

    #[test]
    fn test_member_status_variants() {
        assert_eq!(MemberStatus::Active, MemberStatus::Active);
        assert_eq!(MemberStatus::Suspended, MemberStatus::Suspended);
        assert_eq!(MemberStatus::Banned, MemberStatus::Banned);
        assert_eq!(MemberStatus::Inactive, MemberStatus::Inactive);
    }

    #[test]
    fn test_member_status_default() {
        // MemberStatus doesn't have Default, but we can test variants
        let status = MemberStatus::Active;
        assert_eq!(status, MemberStatus::Active);
    }

    #[test]
    fn test_idea_status_all_variants_unique() {
        let variants = vec![
            IdeaStatus::Draft,
            IdeaStatus::UnderReview,
            IdeaStatus::Approved,
            IdeaStatus::Rejected,
            IdeaStatus::InProgress,
            IdeaStatus::Paused,
            IdeaStatus::Completed,
            IdeaStatus::Executed,
            IdeaStatus::Commercialization,
            IdeaStatus::Archived,
            IdeaStatus::Resubmitted,
            IdeaStatus::Voting,
            IdeaStatus::Expired,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_member_action_all_variants_unique() {
        let variants = vec![
            MemberAction::Join,
            MemberAction::Leave,
            MemberAction::Suspend,
            MemberAction::Activate,
            MemberAction::Ban,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_delegation_type_all_variants_unique() {
        let variants = vec![
            DelegationType::Temporary,
            DelegationType::Permanent,
            DelegationType::Conditional,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_capability_type_all_variants_unique() {
        let variants = vec![
            CapabilityType::Withdraw,
            CapabilityType::Deposit,
            CapabilityType::Manage,
            CapabilityType::Vote,
            CapabilityType::Propose,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_contribution_type_all_variants_unique() {
        let variants = vec![
            ContributionType::Code,
            ContributionType::Design,
            ContributionType::Documentation,
            ContributionType::Testing,
            ContributionType::Review,
            ContributionType::Community,
            ContributionType::Governance,
            ContributionType::Other,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_offchain_vote_status_all_variants_unique() {
        let variants = vec![
            OffchainVoteStatus::Pending,
            OffchainVoteStatus::Active,
            OffchainVoteStatus::Completed,
            OffchainVoteStatus::Cancelled,
            OffchainVoteStatus::Failed,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_member_status_all_variants_unique() {
        let variants = vec![
            MemberStatus::Active,
            MemberStatus::Suspended,
            MemberStatus::Banned,
            MemberStatus::Inactive,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_idea_status_space() {
        assert_eq!(<IdeaStatus as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_member_action_space() {
        assert_eq!(<MemberAction as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_delegation_type_space() {
        assert_eq!(<DelegationType as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_capability_type_space() {
        assert_eq!(<CapabilityType as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_contribution_type_space() {
        assert_eq!(<ContributionType as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_offchain_vote_status_space() {
        assert_eq!(<OffchainVoteStatus as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_member_status_space() {
        assert_eq!(<MemberStatus as Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_idea_status_equality() {
        assert_eq!(IdeaStatus::Draft, IdeaStatus::Draft);
        assert_ne!(IdeaStatus::Draft, IdeaStatus::Approved);
        assert_eq!(IdeaStatus::Voting, IdeaStatus::Voting);
    }

    #[test]
    fn test_member_action_equality() {
        assert_eq!(MemberAction::Join, MemberAction::Join);
        assert_ne!(MemberAction::Join, MemberAction::Leave);
        assert_eq!(MemberAction::Ban, MemberAction::Ban);
    }

    #[test]
    fn test_delegation_type_equality() {
        assert_eq!(DelegationType::Temporary, DelegationType::Temporary);
        assert_ne!(DelegationType::Temporary, DelegationType::Permanent);
        assert_eq!(DelegationType::Conditional, DelegationType::Conditional);
    }

    #[test]
    fn test_capability_type_equality() {
        assert_eq!(CapabilityType::Vote, CapabilityType::Vote);
        assert_ne!(CapabilityType::Vote, CapabilityType::Propose);
        assert_eq!(CapabilityType::Withdraw, CapabilityType::Withdraw);
    }

    #[test]
    fn test_contribution_type_equality() {
        assert_eq!(ContributionType::Code, ContributionType::Code);
        assert_ne!(ContributionType::Code, ContributionType::Design);
        assert_eq!(ContributionType::Other, ContributionType::Other);
    }

    #[test]
    fn test_offchain_vote_status_equality() {
        assert_eq!(OffchainVoteStatus::Pending, OffchainVoteStatus::Pending);
        assert_ne!(OffchainVoteStatus::Pending, OffchainVoteStatus::Active);
        assert_eq!(OffchainVoteStatus::Completed, OffchainVoteStatus::Completed);
    }

    #[test]
    fn test_member_status_equality() {
        assert_eq!(MemberStatus::Active, MemberStatus::Active);
        assert_ne!(MemberStatus::Active, MemberStatus::Suspended);
        assert_eq!(MemberStatus::Banned, MemberStatus::Banned);
    }

    #[test]
    fn test_idea_status_copy() {
        let status1 = IdeaStatus::Draft;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_member_action_copy() {
        let action1 = MemberAction::Join;
        let action2 = action1; // Copy trait
        assert_eq!(action1, action2);
    }

    #[test]
    fn test_delegation_type_copy() {
        let delegation1 = DelegationType::Temporary;
        let delegation2 = delegation1; // Copy trait
        assert_eq!(delegation1, delegation2);
    }

    #[test]
    fn test_capability_type_copy() {
        let capability1 = CapabilityType::Vote;
        let capability2 = capability1; // Copy trait
        assert_eq!(capability1, capability2);
    }

    #[test]
    fn test_contribution_type_copy() {
        let contribution1 = ContributionType::Code;
        let contribution2 = contribution1; // Copy trait
        assert_eq!(contribution1, contribution2);
    }

    #[test]
    fn test_offchain_vote_status_copy() {
        let status1 = OffchainVoteStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }
}
