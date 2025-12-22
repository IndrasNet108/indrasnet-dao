//! Partnership Governance Participation Module
//! 
//! Comprehensive governance participation system for partnerships.
//! Manages voting rights, proposal participation, and governance power distribution.
//! 
//! Hybrid model: Off-chain complex governance analysis, on-chain voting records
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Voting records, governance power allocation, participation tracking
//! - OFF-CHAIN: Complex governance analysis, proposal evaluation, detailed reporting
//!
//! Improved version with enhanced governance participation logic and comprehensive testing

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::constants::PRECISION_FACTOR;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Governance participation level
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum GovernanceLevel {
    /// Observer level (no voting rights)
    Observer,
    /// Contributor level (limited voting rights)
    Contributor,
    /// Member level (full voting rights)
    Member,
    /// Strategic level (enhanced voting rights)
    Strategic,
}

impl Space for GovernanceLevel {
    const INIT_SPACE: usize = 1;
}

/// Governance participation record
#[account]
#[derive(InitSpace)]
pub struct GovernanceParticipation {
    /// Participation ID
    pub participation_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Partner address
    pub partner_address: Pubkey,
    /// Governance level
    pub governance_level: GovernanceLevel,
    /// Voting power (in fixed-point, 0 to PRECISION_FACTOR)
    pub voting_power: u64,
    /// Total votes cast
    pub total_votes_cast: u64,
    /// Total proposals participated
    pub total_proposals_participated: u64,
    /// Participation score (0 to PRECISION_FACTOR)
    pub participation_score: u64,
    /// Last participation timestamp
    pub last_participation: i64,
    /// Participation start timestamp
    pub participation_start: i64,
    /// Whether participation is active
    pub is_active: bool,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Governance vote record
#[account]
#[derive(InitSpace)]
pub struct GovernanceVote {
    /// Vote ID
    pub vote_id: u64,
    /// Proposal ID
    pub proposal_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Partner address
    pub partner_address: Pubkey,
    /// Vote choice (Yes/No/Abstain)
    pub vote_choice: VoteChoice,
    /// Vote weight (in fixed-point)
    pub vote_weight: u64,
    /// Vote timestamp
    pub voted_at: i64,
    /// Vote hash (for verification)
    pub vote_hash: [u8; 32],
    /// Bump seed for PDA
    pub bump: u8,
}

/// Vote choice
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum VoteChoice {
    /// Yes vote
    Yes,
    /// No vote
    No,
    /// Abstain
    Abstain,
}

impl Space for VoteChoice {
    const INIT_SPACE: usize = 1;
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Register governance participation
pub fn register_governance_participation(
    participation: &mut GovernanceParticipation,
    participation_id: u64,
    partnership_id: u64,
    partner_address: Pubkey,
    governance_level: GovernanceLevel,
    voting_power: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    register_governance_participation_with_time(
        participation,
        participation_id,
        partnership_id,
        partner_address,
        governance_level,
        voting_power,
        current_time,
    )
}

/// Register governance participation with explicit timestamp (for testing)
pub fn register_governance_participation_with_time(
    participation: &mut GovernanceParticipation,
    participation_id: u64,
    partnership_id: u64,
    partner_address: Pubkey,
    governance_level: GovernanceLevel,
    voting_power: u64,
    current_time: i64,
) -> Result<()> {
    require!(voting_power <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(partner_address != Pubkey::default(), IndrasError::InvalidInput);
    
    // Calculate voting power based on governance level
    let calculated_power = match governance_level {
        GovernanceLevel::Observer => 0,
        GovernanceLevel::Contributor => PRECISION_FACTOR / 4, // 25%
        GovernanceLevel::Member => PRECISION_FACTOR / 2, // 50%
        GovernanceLevel::Strategic => PRECISION_FACTOR * 3 / 4, // 75%
    };
    
    // Use provided voting_power if it's within level limits, otherwise use calculated
    let final_power = if voting_power <= calculated_power || governance_level == GovernanceLevel::Strategic {
        voting_power
    } else {
        calculated_power
    };
    
    participation.participation_id = participation_id;
    participation.partnership_id = partnership_id;
    participation.partner_address = partner_address;
    participation.governance_level = governance_level;
    participation.voting_power = final_power;
    participation.total_votes_cast = 0;
    participation.total_proposals_participated = 0;
    participation.participation_score = 0;
    participation.last_participation = current_time;
    participation.participation_start = current_time;
    participation.is_active = true;
    
    msg!("Governance participation {} registered for partnership {} (level: {:?}, voting power: {}%)", 
         participation_id, partnership_id, governance_level, final_power * 100 / PRECISION_FACTOR);
    
    Ok(())
}

/// Record governance vote
pub fn record_governance_vote(
    vote: &mut GovernanceVote,
    vote_id: u64,
    proposal_id: u64,
    partnership_id: u64,
    partner_address: Pubkey,
    vote_choice: VoteChoice,
    vote_weight: u64,
    vote_hash: [u8; 32],
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    record_governance_vote_with_time(
        vote,
        vote_id,
        proposal_id,
        partnership_id,
        partner_address,
        vote_choice,
        vote_weight,
        vote_hash,
        current_time,
    )
}

/// Record governance vote with explicit timestamp (for testing)
pub fn record_governance_vote_with_time(
    vote: &mut GovernanceVote,
    vote_id: u64,
    proposal_id: u64,
    partnership_id: u64,
    partner_address: Pubkey,
    vote_choice: VoteChoice,
    vote_weight: u64,
    vote_hash: [u8; 32],
    current_time: i64,
) -> Result<()> {
    require!(vote_weight > 0, IndrasError::InvalidInput);
    require!(vote_weight <= PRECISION_FACTOR, IndrasError::InvalidScore);
    require!(partner_address != Pubkey::default(), IndrasError::InvalidInput);
    
    vote.vote_id = vote_id;
    vote.proposal_id = proposal_id;
    vote.partnership_id = partnership_id;
    vote.partner_address = partner_address;
    vote.vote_choice = vote_choice;
    vote.vote_weight = vote_weight;
    vote.voted_at = current_time;
    vote.vote_hash = vote_hash;
    
    msg!("Governance vote {} recorded for proposal {} (partnership: {}, choice: {:?}, weight: {}%)", 
         vote_id, proposal_id, partnership_id, vote_choice, vote_weight * 100 / PRECISION_FACTOR);
    
    Ok(())
}

/// Update participation score
pub fn update_participation_score(
    participation: &mut GovernanceParticipation,
    total_votes: u64,
    total_proposals: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    update_participation_score_with_time(participation, total_votes, total_proposals, current_time)
}

/// Update participation score with explicit timestamp (for testing)
pub fn update_participation_score_with_time(
    participation: &mut GovernanceParticipation,
    total_votes: u64,
    total_proposals: u64,
    current_time: i64,
) -> Result<()> {
    require!(participation.is_active, IndrasError::InvalidState);
    
    participation.total_votes_cast = total_votes;
    participation.total_proposals_participated = total_proposals;
    participation.last_participation = current_time;
    
    // Calculate participation score (weighted: votes 60%, proposals 40%)
    let vote_score = if total_votes > 0 {
        // Normalize to 0-PRECISION_FACTOR (assuming max 100 votes = 100%)
        let normalized_votes = total_votes.min(100)
            .checked_mul(PRECISION_FACTOR)
            .ok_or(IndrasError::Overflow)?
            .checked_div(100)
            .ok_or(IndrasError::DivisionByZero)?;
        
        normalized_votes
            .checked_mul(600_000) // 60% weight
            .ok_or(IndrasError::Overflow)?
            .checked_div(PRECISION_FACTOR)
            .ok_or(IndrasError::DivisionByZero)?
    } else {
        0
    };
    
    let proposal_score = if total_proposals > 0 {
        // Normalize to 0-PRECISION_FACTOR (assuming max 50 proposals = 100%)
        let normalized_proposals = total_proposals.min(50)
            .checked_mul(PRECISION_FACTOR)
            .ok_or(IndrasError::Overflow)?
            .checked_div(50)
            .ok_or(IndrasError::DivisionByZero)?;
        
        normalized_proposals
            .checked_mul(400_000) // 40% weight
            .ok_or(IndrasError::Overflow)?
            .checked_div(PRECISION_FACTOR)
            .ok_or(IndrasError::DivisionByZero)?
    } else {
        0
    };
    
    participation.participation_score = vote_score
        .checked_add(proposal_score)
        .ok_or(IndrasError::Overflow)?
        .min(PRECISION_FACTOR);
    
    Ok(())
}

/// Upgrade governance level
pub fn upgrade_governance_level(
    participation: &mut GovernanceParticipation,
    new_level: GovernanceLevel,
) -> Result<()> {
    require!(participation.is_active, IndrasError::InvalidState);
    
    // Validate level upgrade (can only upgrade, not downgrade)
    let current_level_value = match participation.governance_level {
        GovernanceLevel::Observer => 0,
        GovernanceLevel::Contributor => 1,
        GovernanceLevel::Member => 2,
        GovernanceLevel::Strategic => 3,
    };
    
    let new_level_value = match new_level {
        GovernanceLevel::Observer => 0,
        GovernanceLevel::Contributor => 1,
        GovernanceLevel::Member => 2,
        GovernanceLevel::Strategic => 3,
    };
    
    require!(new_level_value >= current_level_value, IndrasError::InvalidInput);
    
    // Update voting power based on new level
    let new_voting_power = match new_level {
        GovernanceLevel::Observer => 0,
        GovernanceLevel::Contributor => PRECISION_FACTOR / 4, // 25%
        GovernanceLevel::Member => PRECISION_FACTOR / 2, // 50%
        GovernanceLevel::Strategic => PRECISION_FACTOR * 3 / 4, // 75%
    };
    
    participation.governance_level = new_level;
    participation.voting_power = new_voting_power.max(participation.voting_power);
    
    msg!("Governance level upgraded for partnership {} (new level: {:?}, voting power: {}%)", 
         participation.partnership_id, new_level, participation.voting_power * 100 / PRECISION_FACTOR);
    
    Ok(())
}

// ============================================================================
// OFF-CHAIN FUNCTIONS (Regular Rust - Not Anchor)
// ============================================================================

/// OFF-CHAIN: Analyze governance participation
/// 
/// This function should be called off-chain to analyze governance participation patterns.
pub fn analyze_governance_participation_offchain(
    _participation: &GovernanceParticipation,
    _votes: &[GovernanceVote],
) -> Result<Vec<String>> {
    // OFF-CHAIN IMPLEMENTATION:
    // 1. Analyze voting patterns
    // 2. Identify participation trends
    // 3. Generate recommendations
    // 4. Return analysis results
    
    // This is a placeholder - actual implementation happens off-chain
    Err(IndrasError::NotImplemented.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_governance_participation() {
        let mut participation = GovernanceParticipation {
            participation_id: 0,
            partnership_id: 0,
            partner_address: Pubkey::default(),
            governance_level: GovernanceLevel::Observer,
            voting_power: 0,
            total_votes_cast: 0,
            total_proposals_participated: 0,
            participation_score: 0,
            last_participation: 0,
            participation_start: 0,
            is_active: false,
            bump: 0,
        };

        let partner = Pubkey::from([1u8; 32]);

        let result = register_governance_participation_with_time(
            &mut participation,
            1,
            100,
            partner,
            GovernanceLevel::Member,
            PRECISION_FACTOR / 2, // 50%
            1000, // Mock timestamp
        );

        assert!(result.is_ok());
        assert_eq!(participation.participation_id, 1);
        assert_eq!(participation.partnership_id, 100);
        assert_eq!(participation.governance_level, GovernanceLevel::Member);
        assert_eq!(participation.voting_power, PRECISION_FACTOR / 2);
        assert!(participation.is_active);
    }

    #[test]
    fn test_register_participation_observer_level() {
        let mut participation = GovernanceParticipation {
            participation_id: 0,
            partnership_id: 0,
            partner_address: Pubkey::default(),
            governance_level: GovernanceLevel::Observer,
            voting_power: 0,
            total_votes_cast: 0,
            total_proposals_participated: 0,
            participation_score: 0,
            last_participation: 0,
            participation_start: 0,
            is_active: false,
            bump: 0,
        };

        let partner = Pubkey::from([1u8; 32]);

        let result = register_governance_participation_with_time(
            &mut participation,
            1,
            100,
            partner,
            GovernanceLevel::Observer,
            0,
            1000, // Mock timestamp
        );

        assert!(result.is_ok());
        assert_eq!(participation.governance_level, GovernanceLevel::Observer);
        assert_eq!(participation.voting_power, 0);
    }

    #[test]
    fn test_record_governance_vote() {
        let mut vote = GovernanceVote {
            vote_id: 0,
            proposal_id: 0,
            partnership_id: 0,
            partner_address: Pubkey::default(),
            vote_choice: VoteChoice::Yes,
            vote_weight: 0,
            voted_at: 0,
            vote_hash: [0u8; 32],
            bump: 0,
        };

        let partner = Pubkey::from([1u8; 32]);
        let vote_hash = [1u8; 32];

        let result = record_governance_vote_with_time(
            &mut vote,
            1,
            200,
            100,
            partner,
            VoteChoice::Yes,
            PRECISION_FACTOR / 2, // 50% weight
            vote_hash,
            1000, // Mock timestamp
        );

        assert!(result.is_ok());
        assert_eq!(vote.vote_id, 1);
        assert_eq!(vote.proposal_id, 200);
        assert_eq!(vote.partnership_id, 100);
        assert_eq!(vote.vote_choice, VoteChoice::Yes);
        assert_eq!(vote.vote_weight, PRECISION_FACTOR / 2);
    }

    #[test]
    fn test_update_participation_score() {
        let mut participation = GovernanceParticipation {
            participation_id: 1,
            partnership_id: 100,
            partner_address: Pubkey::from([1u8; 32]),
            governance_level: GovernanceLevel::Member,
            voting_power: PRECISION_FACTOR / 2,
            total_votes_cast: 0,
            total_proposals_participated: 0,
            participation_score: 0,
            last_participation: 0,
            participation_start: 0,
            is_active: true,
            bump: 0,
        };

        let result = update_participation_score_with_time(&mut participation, 50, 10, 1000);

        assert!(result.is_ok());
        assert_eq!(participation.total_votes_cast, 50);
        assert_eq!(participation.total_proposals_participated, 10);
        assert!(participation.participation_score > 0);
    }

    #[test]
    fn test_upgrade_governance_level() {
        let mut participation = GovernanceParticipation {
            participation_id: 1,
            partnership_id: 100,
            partner_address: Pubkey::from([1u8; 32]),
            governance_level: GovernanceLevel::Contributor,
            voting_power: PRECISION_FACTOR / 4, // 25%
            total_votes_cast: 0,
            total_proposals_participated: 0,
            participation_score: 0,
            last_participation: 0,
            participation_start: 0,
            is_active: true,
            bump: 0,
        };

        let result = upgrade_governance_level(&mut participation, GovernanceLevel::Member);

        assert!(result.is_ok());
        assert_eq!(participation.governance_level, GovernanceLevel::Member);
        assert_eq!(participation.voting_power, PRECISION_FACTOR / 2); // 50%
    }

    #[test]
    fn test_upgrade_level_fails_on_downgrade() {
        let mut participation = GovernanceParticipation {
            participation_id: 1,
            partnership_id: 100,
            partner_address: Pubkey::from([1u8; 32]),
            governance_level: GovernanceLevel::Member,
            voting_power: PRECISION_FACTOR / 2,
            total_votes_cast: 0,
            total_proposals_participated: 0,
            participation_score: 0,
            last_participation: 0,
            participation_start: 0,
            is_active: true,
            bump: 0,
        };

        // Try to downgrade from Member to Contributor - should fail
        let result = upgrade_governance_level(&mut participation, GovernanceLevel::Contributor);

        assert!(result.is_err());
    }
}
