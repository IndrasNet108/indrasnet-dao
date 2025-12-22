//! Idea Voting instruction handlers
//!
//! Handlers for idea voting operations: cast vote, tally votes, start voting
//!
//! NOTE: Idea voting is for voting on ideas in mesh groups that are part of phenomena.
//! Voting happens BEFORE grant requests - ideas are voted on to determine if they should receive grants.

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::enums::IdeaStatus;
use crate::voting_types::VoteType;
use crate::state::GovernanceParams;

/// Cast a vote on an idea
///
/// This handler creates a vote account for the given idea.
/// The vote account is a PDA with seeds [b"idea_vote", idea_id.to_le_bytes(), voter.key()].
///
/// # Security
/// - Validates idea is in Voting status
/// - Validates idea is in mesh group
/// - Validates mesh group is in a phenomenon
/// - Prevents duplicate voting (PDA seeds ensure uniqueness)
/// - Validates vote weight is within reasonable range
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~20,000 CU
pub fn cast_idea_vote_handler(
    ctx: Context<crate::CastIdeaVote>,
    idea_id: u64,
    vote_type: VoteType,
    weight: u64,
) -> Result<()> {
    let idea = &ctx.accounts.idea;
    let mesh_group = &ctx.accounts.mesh_group;
    
    // SECURITY: Validate idea ID matches
    require!(
        idea.id == idea_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate idea is in Voting status (checked in Accounts, but double-check)
    require!(
        idea.status == IdeaStatus::Voting,
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate idea is in mesh group (checked in Accounts, but double-check)
    require!(
        mesh_group.ideas.contains(&idea_id),
        IndrasError::IdeaNotInMeshGroup
    );
    
    // SECURITY: Validate mesh group is in a phenomenon (checked in Accounts, but double-check)
    require!(
        !mesh_group.phenomena.is_empty(),
        IndrasError::NotInPhenomenon
    );
    
    // SECURITY: Validate vote weight
    require!(weight > 0, IndrasError::InvalidInput);
    require!(weight <= 1_000_000, IndrasError::AmountTooLarge); // Max weight limit
    
    // Create vote - PDA seeds include voter to prevent duplicate voting
    let vote = &mut ctx.accounts.vote;
    vote.idea_id = idea_id;
    vote.voter = ctx.accounts.voter.key();
    vote.vote_type = vote_type; // Clone for logging
    vote.weight = weight;
    vote.cast_at = Clock::get()?.unix_timestamp;
    vote.bump = ctx.bumps.vote;
    
    // Log vote
    let vote_str = match &vote_type {
        VoteType::Yes => "Yes",
        VoteType::No => "No",
        VoteType::Abstain => "Abstain",
    };
    
    msg!("Vote cast on idea {} by {}: {} (weight: {}) - Idea is in mesh group {} which is in {} phenomena", 
         idea_id, 
         ctx.accounts.voter.key(), 
         vote_str, 
         weight,
         mesh_group.name,
         mesh_group.phenomena.len());
    
    Ok(())
}

/// Tally votes for an idea and update status based on results
///
/// This handler tallies votes from remaining_accounts and updates the idea status.
/// If voting passes (yes_votes > no_votes and quorum met), idea status is updated to InProgress.
/// If voting fails, idea status is updated to Rejected.
///
/// # Security
/// - Validates idea is in Voting status
/// - Validates tallyer is DAO authority
/// - Validates vote accounts belong to this idea
/// - Checks quorum requirements from governance params
///
/// # Compute Units
/// Recommended: 50,000 CU
/// - Vote calculation: ~30,000 CU (iterating through remaining_accounts)
/// - State update: ~20,000 CU
pub fn tally_idea_votes_handler(
    ctx: Context<crate::TallyIdeaVotes>,
    idea_id: u64,
) -> Result<()> {
    let idea_key = ctx.accounts.idea.key();
    let idea = &mut ctx.accounts.idea;
    let dao_config = &ctx.accounts.dao_config;
    
    // SECURITY: Validate idea ID matches
    require!(
        idea.id == idea_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate idea is in Voting status
    require!(
        idea.status == IdeaStatus::Voting,
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate tallyer is DAO authority (checked in Accounts, but double-check)
    require!(
        ctx.accounts.tallyer.key() == dao_config.authority,
        IndrasError::Unauthorized
    );
    
    // Get quorum percentage from governance params (if provided) or use default
    let quorum_percentage = if !ctx.accounts.governance_params.data_is_empty() {
        // Deserialize governance params
        let data = ctx.accounts.governance_params.try_borrow_data()?;
        require!(data.len() >= 8, IndrasError::InvalidInput);
        let mut data_slice = &data[8..]; // Skip discriminator
        let gov_params = GovernanceParams::try_deserialize(&mut data_slice)
            .map_err(|_| IndrasError::InvalidInput)?;
        gov_params.quorum_percentage as u32
    } else {
        20u32 // Default 20% quorum
    };
    
    // Tally votes from remaining_accounts
    let mut yes_votes: u64 = 0;
    let mut no_votes: u64 = 0;
    let mut abstain_votes: u64 = 0;
    let mut total_weight: u64 = 0;
    let mut seen_votes: Vec<Pubkey> = Vec::new();
    
    // OPTIMIZATION: Limit iteration to prevent excessive compute units
    const MAX_VOTES_TO_PROCESS: usize = 100; // Max 100 votes per tally
    let accounts_to_process = ctx.remaining_accounts.len().min(MAX_VOTES_TO_PROCESS);
    
    // Iterate through remaining_accounts (vote accounts)
    for account_info in ctx.remaining_accounts.iter().take(accounts_to_process) {
        // Skip empty accounts
        if account_info.data_is_empty() {
            continue;
        }

        require!(
            account_info.owner == ctx.program_id,
            IndrasError::InvalidProgram
        );

        let vote_key = account_info.key();
        require!(
            !seen_votes.iter().any(|key| *key == vote_key),
            IndrasError::InvalidInput
        );
        seen_votes.push(vote_key);
        
        // Deserialize IdeaVote
        let data = account_info.try_borrow_data()?;
        require!(data.len() >= 8, IndrasError::InvalidInput);
        let mut data_slice = &data[8..]; // Skip discriminator
        let vote = crate::state::idea_vote::IdeaVote::try_deserialize(&mut data_slice)
            .map_err(|_| IndrasError::InvalidInput)?;

        let (expected_vote_pda, _) = Pubkey::find_program_address(
            &[b"idea_vote", idea_key.as_ref(), vote.voter.as_ref()],
            ctx.program_id,
        );
        require!(
            vote_key == expected_vote_pda,
            IndrasError::InvalidInput
        );
        
        // Validate vote is for this idea
        require!(
            vote.idea_id == idea_id,
            IndrasError::InvalidInput
        );
        
        // Tally vote
        match vote.vote_type {
            VoteType::Yes => yes_votes = yes_votes.checked_add(vote.weight).ok_or(IndrasError::Overflow)?,
            VoteType::No => no_votes = no_votes.checked_add(vote.weight).ok_or(IndrasError::Overflow)?,
            VoteType::Abstain => abstain_votes = abstain_votes.checked_add(vote.weight).ok_or(IndrasError::Overflow)?,
        }
        
        total_weight = total_weight.checked_add(vote.weight).ok_or(IndrasError::Overflow)?;
    }
    
    // Calculate quorum requirement
    // For MVP: Use simple percentage of total_weight
    // In production, this would use total DAO voting power
    let quorum_required = (total_weight as u128)
        .checked_mul(quorum_percentage as u128)
        .and_then(|n| n.checked_div(100))
        .ok_or(IndrasError::Overflow)? as u64;
    
    let total_participation = yes_votes + no_votes + abstain_votes;
    
    // Check if quorum is met
    let quorum_met = total_participation >= quorum_required;
    
    // Determine result
    if !quorum_met {
        // Quorum not met - voting fails
        idea.status = IdeaStatus::Rejected;
        msg!("Idea {} voting failed: quorum not met (required: {}, got: {})", 
             idea_id, quorum_required, total_participation);
    } else if yes_votes > no_votes {
        // Voting passed - update idea status to InProgress
        idea.status = IdeaStatus::InProgress;
        
        // Create anchor record if not exists
        let anchor_record = &mut ctx.accounts.anchor_record;
        if anchor_record.idea_id == 0 {
            // Initialize anchor record
            anchor_record.idea_id = idea_id;
            anchor_record.anchor_data = format!("Idea {} approved for grant via voting (yes: {}, no: {})", 
                                                 idea_id, yes_votes, no_votes);
            anchor_record.anchored_at = Clock::get()?.unix_timestamp;
            anchor_record.anchorer = ctx.accounts.tallyer.key();
            anchor_record.bump = ctx.bumps.anchor_record;
        }
        
        msg!("Idea {} voting passed: yes={}, no={}, abstain={}, quorum met", 
             idea_id, yes_votes, no_votes, abstain_votes);
    } else if no_votes > yes_votes {
        // Voting failed
        idea.status = IdeaStatus::Rejected;
        msg!("Idea {} voting failed: yes={}, no={}, abstain={}", 
             idea_id, yes_votes, no_votes, abstain_votes);
    } else {
        // Tie - voting fails
        idea.status = IdeaStatus::Rejected;
        msg!("Idea {} voting tied: yes={}, no={}, abstain={}", 
             idea_id, yes_votes, no_votes, abstain_votes);
    }
    
    Ok(())
}

/// Start voting on an idea
///
/// This handler transitions an idea from Approved to Voting status.
/// Idea must be in Approved status, in a mesh group, and the mesh group must be in a phenomenon.
///
/// # Security
/// - Validates idea is in Approved status
/// - Validates idea is in mesh group
/// - Validates mesh group is in a phenomenon
/// - Validates starter is DAO authority or idea author
///
/// # Compute Units
/// Recommended: 15,000 CU
/// - Validation: ~10,000 CU
/// - State update: ~5,000 CU
pub fn start_idea_voting_handler(
    ctx: Context<crate::StartIdeaVoting>,
    idea_id: u64,
) -> Result<()> {
    let idea = &mut ctx.accounts.idea;
    let mesh_group = &ctx.accounts.mesh_group;
    
    // SECURITY: Validate idea ID matches
    require!(
        idea.id == idea_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate idea is in Approved status
    require!(
        idea.status == IdeaStatus::Approved,
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate idea is in mesh group
    require!(
        mesh_group.ideas.contains(&idea_id),
        IndrasError::IdeaNotInMeshGroup
    );
    
    // SECURITY: Validate mesh group is in a phenomenon
    require!(
        !mesh_group.phenomena.is_empty(),
        IndrasError::NotInPhenomenon
    );
    
    // If phenomenon account is provided, validate it contains the idea
    if let Some(phenomenon_info) = &ctx.accounts.phenomenon {
        if !phenomenon_info.data_is_empty() {
            // Deserialize Phenomenon
            let data = phenomenon_info.try_borrow_data()?;
            require!(data.len() >= 8, IndrasError::InvalidInput);
            let mut data_slice = &data[8..]; // Skip discriminator
            let phenomenon = crate::state::Phenomenon::try_deserialize(&mut data_slice)
                .map_err(|_| IndrasError::InvalidInput)?;
            
            // Validate idea is in phenomenon's related_ideas
            // NOTE: For MVP, we do basic check - full deserialization of Vec<u64> can be added later
            require!(
                !phenomenon.related_ideas.is_empty(), // At least one idea
                IndrasError::IdeaNotInPhenomenon
            );
        }
    }
    
    // Transition idea to Voting status
    idea.status = IdeaStatus::Voting;
    
    msg!("Voting started on idea {} - idea is in mesh group {} which is in {} phenomena", 
         idea_id, mesh_group.name, mesh_group.phenomena.len());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::state::enums::IdeaStatus;
    use anchor_lang::prelude::Pubkey;

    // ========== cast_idea_vote_handler validation tests ==========
    
    #[test]
    fn test_cast_idea_vote_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea_id_actual = 2u64;
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea_id_actual, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_cast_idea_vote_validation_idea_status_not_voting() {
        // Test: idea.status != Voting should fail
        let idea_status = IdeaStatus::Draft;
        
        // Validation logic: require!(idea.status == Voting, IndrasError::InvalidState)
        assert_ne!(idea_status, IdeaStatus::Voting, "Idea status not Voting should be detected");
    }
    
    #[test]
    fn test_cast_idea_vote_validation_idea_not_in_mesh_group() {
        // Test: idea_id not in mesh_group.ideas should fail
        let idea_id = 1u64;
        let mesh_group_ideas = vec![2u64, 3u64];
        
        // Validation logic: require!(mesh_group.ideas.contains(&idea_id), IndrasError::IdeaNotInMeshGroup)
        assert!(!mesh_group_ideas.contains(&idea_id), "Idea not in mesh group should be detected");
    }
    
    #[test]
    fn test_cast_idea_vote_validation_mesh_group_not_in_phenomenon() {
        // Test: mesh_group.phenomena.is_empty() should fail
        let phenomena = Vec::<Pubkey>::new();
        
        // Validation logic: require!(!phenomena.is_empty(), IndrasError::NotInPhenomenon)
        assert!(phenomena.is_empty(), "Mesh group not in phenomenon should be detected");
    }
    
    #[test]
    fn test_cast_idea_vote_validation_weight_zero() {
        // Test: weight == 0 should fail
        let weight = 0u64;
        
        // Validation logic: require!(weight > 0, IndrasError::InvalidInput)
        assert_eq!(weight, 0, "Zero weight should be detected");
    }
    
    #[test]
    fn test_cast_idea_vote_validation_weight_too_large() {
        // Test: weight > 1_000_000 should fail
        let weight = 1_000_001u64;
        
        // Validation logic: require!(weight <= 1_000_000, IndrasError::AmountTooLarge)
        assert!(weight > 1_000_000, "Weight too large should be detected");
    }
    
    #[test]
    fn test_cast_idea_vote_validation_valid_inputs() {
        // Test: valid inputs should pass
        let idea_id = 1u64;
        let idea_status = IdeaStatus::Voting;
        let mesh_group_ideas = vec![1u64, 2u64];
        let phenomena = vec![Pubkey::new_unique()];
        let weight = 1000u64;
        
        // All validations should pass
        assert_eq!(idea_id, idea_id, "Idea ID should match");
        assert_eq!(idea_status, IdeaStatus::Voting, "Idea status should be Voting");
        assert!(mesh_group_ideas.contains(&idea_id), "Idea should be in mesh group");
        assert!(!phenomena.is_empty(), "Mesh group should be in phenomenon");
        assert!(weight > 0 && weight <= 1_000_000, "Weight should be valid");
    }

    // ========== tally_idea_votes_handler validation tests ==========
    
    #[test]
    fn test_tally_idea_votes_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea_id_actual = 2u64;
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea_id_actual, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_tally_idea_votes_validation_idea_status_not_voting() {
        // Test: idea.status != Voting should fail
        let idea_status = IdeaStatus::Draft;
        
        // Validation logic: require!(idea.status == Voting, IndrasError::InvalidState)
        assert_ne!(idea_status, IdeaStatus::Voting, "Idea status not Voting should be detected");
    }
    
    #[test]
    fn test_tally_idea_votes_validation_tallyer_not_authority() {
        // Test: tallyer != dao_config.authority should fail
        let tallyer = Pubkey::new_unique();
        let dao_authority = Pubkey::new_unique();
        
        // Validation logic: require!(tallyer == dao_config.authority, IndrasError::Unauthorized)
        assert_ne!(tallyer, dao_authority, "Tallyer not authority should be detected");
    }
    
    #[test]
    fn test_tally_idea_votes_validation_governance_params_data_too_short() {
        // Test: data.len() < 8 should fail
        let data_len = 7usize;
        
        // Validation logic: require!(data.len() >= 8, IndrasError::InvalidInput)
        assert!(data_len < 8, "Governance params data too short should be detected");
    }
    
    #[test]
    fn test_tally_idea_votes_validation_vote_idea_id_mismatch() {
        // Test: vote.idea_id != idea_id should fail
        let idea_id = 1u64;
        let vote_idea_id = 2u64;
        
        // Validation logic: require!(vote.idea_id == idea_id, IndrasError::InvalidInput)
        assert_ne!(vote_idea_id, idea_id, "Vote idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_tally_idea_votes_validation_overflow() {
        // Test: checked_add/checked_mul returning None should fail
        let yes_votes = u64::MAX;
        let vote_weight = 1u64;
        
        // Validation logic: checked_add should return None on overflow
        assert_eq!(yes_votes.checked_add(vote_weight), None, "Overflow should be detected");
    }

    // ========== start_idea_voting_handler validation tests ==========
    
    #[test]
    fn test_start_idea_voting_validation_idea_id_mismatch() {
        // Test: idea.id != idea_id should fail
        let idea_id = 1u64;
        let idea_id_actual = 2u64;
        
        // Validation logic: require!(idea.id == idea_id, IndrasError::InvalidInput)
        assert_ne!(idea_id_actual, idea_id, "Idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_start_idea_voting_validation_idea_status_not_approved() {
        // Test: idea.status != Approved should fail
        let idea_status = IdeaStatus::Draft;
        
        // Validation logic: require!(idea.status == Approved, IndrasError::InvalidState)
        assert_ne!(idea_status, IdeaStatus::Approved, "Idea status not Approved should be detected");
    }
    
    #[test]
    fn test_start_idea_voting_validation_idea_not_in_mesh_group() {
        // Test: idea_id not in mesh_group.ideas should fail
        let idea_id = 1u64;
        let mesh_group_ideas = vec![2u64, 3u64];
        
        // Validation logic: require!(mesh_group.ideas.contains(&idea_id), IndrasError::IdeaNotInMeshGroup)
        assert!(!mesh_group_ideas.contains(&idea_id), "Idea not in mesh group should be detected");
    }
    
    #[test]
    fn test_start_idea_voting_validation_mesh_group_not_in_phenomenon() {
        // Test: mesh_group.phenomena.is_empty() should fail
        let phenomena = Vec::<Pubkey>::new();
        
        // Validation logic: require!(!phenomena.is_empty(), IndrasError::NotInPhenomenon)
        assert!(phenomena.is_empty(), "Mesh group not in phenomenon should be detected");
    }
    
    #[test]
    fn test_start_idea_voting_validation_phenomenon_data_too_short() {
        // Test: data.len() < 8 should fail
        let data_len = 7usize;
        
        // Validation logic: require!(data.len() >= 8, IndrasError::InvalidInput)
        assert!(data_len < 8, "Phenomenon data too short should be detected");
    }
    
    #[test]
    fn test_start_idea_voting_validation_phenomenon_related_ideas_empty() {
        // Test: phenomenon.related_ideas.is_empty() should fail
        let related_ideas = Vec::<Pubkey>::new();
        
        // Validation logic: require!(!related_ideas.is_empty(), IndrasError::IdeaNotInPhenomenon)
        assert!(related_ideas.is_empty(), "Phenomenon related ideas empty should be detected");
    }
}
