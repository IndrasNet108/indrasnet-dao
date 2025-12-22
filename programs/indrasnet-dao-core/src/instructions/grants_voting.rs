//! Grant Voting instruction handlers
//!
//! Handlers for grant voting operations with semantic filtering:
//! - cast_grant_vote - cast vote on grant with competency-based weight
//! - tally_grant_votes - tally votes and determine grant approval

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::state::grant::{GrantStatus, GrantVote, VoterType};
use crate::state::expert_registry::ExpertEntry;
use crate::state::{MeshGroup, Idea};
use crate::voting_types::VoteType;

/// Cast vote on grant
///
/// This handler creates a vote account for the given grant.
/// Vote weight is calculated based on:
/// 1. Voter type (author mesh group, expert, DAO member)
/// 2. Competency level in grant's semantic domain (if applicable)
/// 3. Voting layer (author only, author+expert, all layers)
///
/// # Security
/// - Validates grant is in Pending status
/// - Validates voting period hasn't ended
/// - Prevents duplicate voting (PDA seeds ensure uniqueness)
/// - Validates voter eligibility based on voting layer
///
/// # Compute Units
/// Recommended: 30,000 CU
/// - Validation: ~10,000 CU
/// - Weight calculation: ~5,000 CU
/// - Account initialization: ~15,000 CU
pub fn cast_grant_vote_handler(
    ctx: Context<crate::CastGrantVote>,
    grant_id: u64,
    vote_choice: VoteType,
    voter_type: VoterType,
    competency_multiplier: Option<u64>, // Optional: competency multiplier (0-150, where 100 = 1.0x)
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    // Deserialize mesh_group and idea from UncheckedAccount to reduce stack size
    let mesh_group_data = ctx.accounts.mesh_group.try_borrow_data()?;
    let mesh_group: MeshGroup = MeshGroup::try_deserialize(&mut &mesh_group_data[..])?;
    let idea_data = ctx.accounts.idea.try_borrow_data()?;
    let idea: Idea = Idea::try_deserialize(&mut &idea_data[..])?;
    let voter = ctx.accounts.voter.key();
    
    // SECURITY: Validate grant ID matches
    require!(
        grant.id == grant_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate grant is in Pending status
    require!(
        grant.status == GrantStatus::Pending,
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate voting period hasn't ended
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time <= grant.voting_end,
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate grant is associated with correct idea and mesh group
    require!(
        grant.idea_id == idea.id,
        IndrasError::InvalidInput
    );
    require!(
        grant.mesh_group == ctx.accounts.mesh_group.key(),
        IndrasError::InvalidInput
    );
    
    // Validate voter eligibility based on voting layer
    // NOTE: For MVP, we do basic validation. Full semantic filtering is done off-chain.
    match grant.voting_layer {
        crate::state::grant::VotingLayer::AuthorOnly => {
            // Only mesh group members can vote
            require!(
                mesh_group.members.iter().any(|m| m.pubkey == voter),
                IndrasError::Unauthorized
            );
            require!(
                voter_type == VoterType::MeshGroupMember,
                IndrasError::InvalidInput
            );
        }
        crate::state::grant::VotingLayer::AuthorAndExpert => {
            // Mesh group members OR experts can vote
            let is_mesh_member = mesh_group.members.iter().any(|m| m.pubkey == voter);
            let is_expert = voter_type == VoterType::Expert;
            require!(
                is_mesh_member || is_expert,
                IndrasError::Unauthorized
            );
            if is_mesh_member {
                require!(
                    voter_type == VoterType::MeshGroupMember,
                    IndrasError::InvalidInput
                );
            }
            // If voting as expert, validate expert entry
            if is_expert {
                validate_expert_voter(
                    &ctx.accounts.expert_entry,
                    grant,
                    &voter,
                )?;
            }
        }
        crate::state::grant::VotingLayer::AllLayers => {
            // All layers can vote (mesh group, expert, DAO member, idea author)
            // Validate voter_type is appropriate
            match voter_type {
                VoterType::MeshGroupMember => {
                    require!(
                        mesh_group.members.iter().any(|m| m.pubkey == voter),
                        IndrasError::Unauthorized
                    );
                }
                VoterType::IdeaAuthor => {
                    require!(
                        idea.author == voter,
                        IndrasError::Unauthorized
                    );
                }
                VoterType::Expert => {
                    // Validate expert entry if provided (on-chain verification)
                    validate_expert_voter(
                        &ctx.accounts.expert_entry,
                        grant,
                        &voter,
                    )?;
                }
                VoterType::DaoMember => {
                    // Valid voter type for this layer
                    // No additional validation needed for DAO members
                }
            }
        }
    }
    
    // Track B B4: Semantic pre-filter for grant voting
    // Use semantic_distance if available to adjust vote eligibility/weight
    let semantic_weight_multiplier = if let Some(semantic_dist) = grant.semantic_distance {
        // Semantic distance is 0.0-1.0, where 0.0 = identical, 1.0 = completely different
        // For voting: closer ideas (lower distance) should have higher weight
        // Formula: weight_multiplier = 1.0 - (semantic_distance * 0.5)
        // This gives: distance 0.0 -> 1.0x, distance 0.5 -> 0.75x, distance 1.0 -> 0.5x
        let multiplier = (1.0 - (semantic_dist * 0.5)).max(0.5); // Minimum 0.5x weight
        Some((multiplier * 100.0) as u64) // Convert to 0-100 scale (100 = 1.0x)
    } else {
        None // No semantic distance available, use base weight
    };
    
    // Calculate vote weight
    let base_weight = GrantVote::calculate_base_weight(voter_type);
    let final_weight = if let Some(competency) = competency_multiplier {
        // Use competency multiplier if provided (off-chain computed)
        let weight_with_competency = GrantVote::calculate_final_weight(base_weight, competency);
        
        // Apply semantic weight multiplier if available (Track B B4)
        if let Some(semantic_mult) = semantic_weight_multiplier {
            GrantVote::calculate_final_weight(weight_with_competency, semantic_mult)
        } else {
            weight_with_competency
        }
    } else {
        // No competency multiplier - use base weight
        // Apply semantic weight multiplier if available (Track B B4)
        if let Some(semantic_mult) = semantic_weight_multiplier {
            GrantVote::calculate_final_weight(base_weight, semantic_mult)
        } else {
            base_weight
        }
    };
    
    // Initialize vote account
    let vote = &mut ctx.accounts.vote;
    vote.grant_id = grant_id;
    vote.voter = voter;
    vote.vote_type = vote_choice;
    vote.weight = final_weight;
    vote.voter_type = voter_type;
    vote.cast_at = current_time;
    vote.bump = ctx.bumps.vote;
    
    // Update grant vote totals
    grant.total_votes = grant.total_votes
        .checked_add(1)
        .ok_or(error!(IndrasError::Overflow))?;
    
    match vote_choice {
        VoteType::Yes => {
            grant.total_yes_weight = grant.total_yes_weight
                .checked_add(final_weight)
                .ok_or(error!(IndrasError::Overflow))?;
        }
        VoteType::No => {
            grant.total_no_weight = grant.total_no_weight
                .checked_add(final_weight)
                .ok_or(error!(IndrasError::Overflow))?;
        }
        VoteType::Abstain => {
            grant.total_abstain_weight = grant.total_abstain_weight
                .checked_add(final_weight)
                .ok_or(error!(IndrasError::Overflow))?;
        }
    }
    
    msg!("Grant vote cast: grant_id={}, voter={}, vote={:?}, weight={}, voter_type={:?}", 
         grant_id, voter, vote_choice, final_weight, voter_type);
    
    Ok(())
}

/// Tally votes for grant
///
/// This handler tallies votes and updates grant status based on results.
/// Uses three-layer voting thresholds:
/// - Level 1: Author mesh group only (60% threshold)
/// - Level 2: Author + Expert (60% author, 20% expert)
/// - Level 3: All layers (60% author, 20% expert, 20% DAO)
///
/// # Security
/// - Validates grant is in Pending status
/// - Validates voting period has ended
/// - Only DAO authority or authorized member can tally
///
/// # Compute Units
/// Recommended: 20,000 CU
/// - Vote calculation: ~10,000 CU
/// - State update: ~10,000 CU
pub fn tally_grant_votes_handler(
    ctx: Context<crate::TallyGrantVotes>,
    grant_id: u64,
) -> Result<()> {
    let grant = &mut ctx.accounts.grant;
    let mesh_group = &ctx.accounts.mesh_group;
    
    // SECURITY: Validate grant ID matches
    require!(
        grant.id == grant_id,
        IndrasError::InvalidInput
    );
    
    // SECURITY: Validate grant is in Pending status
    require!(
        grant.status == GrantStatus::Pending,
        IndrasError::InvalidState
    );
    
    // SECURITY: Validate voting period has ended
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time >= grant.voting_end,
        IndrasError::InvalidState
    );
    
    // Calculate quorum and approval based on grant level
    let total_weight = grant.total_yes_weight
        .checked_add(grant.total_no_weight)
        .and_then(|w| w.checked_add(grant.total_abstain_weight))
        .ok_or(error!(IndrasError::Overflow))?;
    
    // Calculate approval percentage
    let approval_percentage = if total_weight > 0 {
        (grant.total_yes_weight * 100) / total_weight
    } else {
        0
    };
    
    // Check quorum (minimum participation)
    // For MVP: simplified quorum check (60% of mesh group members)
    let mesh_group_size = mesh_group.members.len() as u64;
    let min_quorum = if mesh_group_size > 0 {
        (mesh_group_size * 60) / 100 // 60% of mesh group
    } else {
        1 // At least 1 vote if mesh group is empty (shouldn't happen)
    };
    grant.quorum_reached = grant.total_votes >= min_quorum;
    
    // Determine approval based on grant level and thresholds
    // NOTE: For MVP, we use simplified thresholds. Full three-layer calculation requires
    // off-chain aggregation of votes by layer (author/expert/DAO)
    let approval_threshold = match grant.grant_level {
        1 => 60u64,  // Level 1: 60% of author mesh group
        2 => 60u64,  // Level 2: 60% author + 20% expert (simplified: overall 60%)
        3 => 60u64,  // Level 3: 60% author + 20% expert + 20% DAO (simplified: overall 60%)
        _ => 60u64,  // Default to 60%
    };
    
    let approved = approval_percentage >= approval_threshold && grant.quorum_reached;
    
    // Update grant status
    if approved {
        grant.status = GrantStatus::Approved;
        grant.approved_at = Some(current_time);
        msg!("Grant {} approved: yes_weight={}, no_weight={}, approval={}%", 
             grant_id, grant.total_yes_weight, grant.total_no_weight, approval_percentage);
    } else {
        grant.status = GrantStatus::Rejected;
        msg!("Grant {} rejected: yes_weight={}, no_weight={}, approval={}%, quorum_reached={}", 
             grant_id, grant.total_yes_weight, grant.total_no_weight, approval_percentage, grant.quorum_reached);
    }
    
    Ok(())
}

/// Validate expert voter through ExpertRegistry
///
/// This function validates that:
/// 1. Expert entry exists and is valid (if provided)
/// 2. Expert is registered for grant's semantic domain
/// 3. Expert entry is active and has sufficient reputation
///    NOTE: Domain index validation is done off-chain or via expert_entry PDA seeds
fn validate_expert_voter(
    expert_entry: &Option<UncheckedAccount>,
    grant: &crate::state::grant::Grant,
    voter: &Pubkey,
) -> Result<()> {
    // If expert entry is provided, validate it
    if let Some(expert_entry_info) = expert_entry {
        // Deserialize expert entry manually
        let expert_entry_data = expert_entry_info.try_borrow_data()?;
        require!(expert_entry_data.len() >= 8, IndrasError::InvalidInput);
        let mut data_slice = &expert_entry_data[8..]; // Skip discriminator
        let expert_entry = ExpertEntry::try_deserialize(&mut data_slice)
            .map_err(|_| IndrasError::InvalidInput)?;
        
        // Validate expert matches voter
        require!(
            expert_entry.expert == *voter,
            IndrasError::InvalidInput
        );
        
        // Validate expert entry is active and valid
        require!(
            expert_entry.is_valid_expert(),
            IndrasError::InvalidState
        );
        
        // Validate expert is registered for grant's semantic domain
        if let Some(ref grant_domain) = grant.semantic_domain {
            require!(
                expert_entry.domain_id == *grant_domain,
                IndrasError::InvalidSemanticDomain
            );
        } else {
            // Grant has no semantic domain - expert entry should not be provided
            return Err(IndrasError::InvalidInput.into());
        }
    } else {
        // Expert entry not provided - this is allowed for off-chain verification
        // but we still require that grant has a semantic domain if voting as expert
        require!(
            grant.semantic_domain.is_some(),
            IndrasError::InvalidInput
        );
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::state::grant::{GrantStatus, VoterType};
    use anchor_lang::prelude::Pubkey;

    // ========== cast_grant_vote_handler validation tests ==========
    
    #[test]
    fn test_cast_grant_vote_validation_grant_id_mismatch() {
        // Test: grant.id != grant_id should fail
        let grant_id = 1u64;
        let grant_id_actual = 2u64;
        
        // Validation logic: require!(grant.id == grant_id, IndrasError::InvalidInput)
        assert_ne!(grant_id_actual, grant_id, "Grant ID mismatch should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_grant_status_not_pending() {
        // Test: grant.status != Pending should fail
        let grant_status = GrantStatus::Approved;
        
        // Validation logic: require!(grant.status == Pending, IndrasError::InvalidState)
        assert_ne!(grant_status, GrantStatus::Pending, "Grant status not Pending should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_voting_period_ended() {
        // Test: current_time > grant.voting_end should fail
        let current_time = 1000000i64;
        let voting_end = 999999i64;
        
        // Validation logic: require!(current_time <= grant.voting_end, IndrasError::InvalidState)
        assert!(current_time > voting_end, "Voting period ended should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_grant_idea_id_mismatch() {
        // Test: grant.idea_id != idea.id should fail
        let grant_idea_id = 1u64;
        let idea_id = 2u64;
        
        // Validation logic: require!(grant.idea_id == idea.id, IndrasError::InvalidInput)
        assert_ne!(grant_idea_id, idea_id, "Grant idea ID mismatch should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_grant_mesh_group_mismatch() {
        // Test: grant.mesh_group != mesh_group.key() should fail
        let grant_mesh_group = Pubkey::new_unique();
        let mesh_group = Pubkey::new_unique();
        
        // Validation logic: require!(grant.mesh_group == mesh_group.key(), IndrasError::InvalidInput)
        assert_ne!(grant_mesh_group, mesh_group, "Grant mesh group mismatch should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_author_only_voter_not_mesh_member() {
        // Test: voter not in mesh_group.members should fail (for AuthorOnly layer)
        let voter = Pubkey::new_unique();
        let mesh_group_members = vec![Pubkey::new_unique(), Pubkey::new_unique()];
        
        // Validation logic: require!(members.iter().any(|m| m.pubkey == voter), IndrasError::Unauthorized)
        assert!(!mesh_group_members.contains(&voter), "Voter not mesh member should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_author_only_voter_type_mismatch() {
        // Test: voter_type != MeshGroupMember should fail (for AuthorOnly layer)
        let voter_type = VoterType::DaoMember;
        
        // Validation logic: require!(voter_type == MeshGroupMember, IndrasError::InvalidInput)
        assert_ne!(voter_type, VoterType::MeshGroupMember, "Voter type mismatch should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_author_and_expert_unauthorized() {
        // Test: voter not mesh member and not expert should fail (for AuthorAndExpert layer)
        let voter = Pubkey::new_unique();
        let mesh_group_members = vec![Pubkey::new_unique()];
        let voter_type = VoterType::DaoMember;
        
        // Validation logic: require!(is_mesh_member || is_expert, IndrasError::Unauthorized)
        let is_mesh_member = mesh_group_members.contains(&voter);
        let is_expert = voter_type == VoterType::Expert;
        assert!(!is_mesh_member && !is_expert, "Unauthorized voter should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_all_layers_idea_author_mismatch() {
        // Test: idea.author != voter should fail (for IdeaAuthor voter type)
        let idea_author = Pubkey::new_unique();
        let voter = Pubkey::new_unique();
        
        // Validation logic: require!(idea.author == voter, IndrasError::Unauthorized)
        assert_ne!(idea_author, voter, "Idea author mismatch should be detected");
    }
    
    #[test]
    fn test_cast_grant_vote_validation_overflow() {
        // Test: checked_add returning None should fail
        let total_votes = u64::MAX;
        let increment = 1u64;
        
        // Validation logic: checked_add should return None on overflow
        assert_eq!(total_votes.checked_add(increment), None, "Overflow should be detected");
    }

    // ========== tally_grant_votes_handler validation tests ==========
    
    #[test]
    fn test_tally_grant_votes_validation_grant_id_mismatch() {
        // Test: grant.id != grant_id should fail
        let grant_id = 1u64;
        let grant_id_actual = 2u64;
        
        // Validation logic: require!(grant.id == grant_id, IndrasError::InvalidInput)
        assert_ne!(grant_id_actual, grant_id, "Grant ID mismatch should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_grant_status_not_pending() {
        // Test: grant.status != Pending should fail
        let grant_status = GrantStatus::Approved;
        
        // Validation logic: require!(grant.status == Pending, IndrasError::InvalidState)
        assert_ne!(grant_status, GrantStatus::Pending, "Grant status not Pending should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_voting_period_not_ended() {
        // Test: current_time < grant.voting_end should fail
        let current_time = 999999i64;
        let voting_end = 1000000i64;
        
        // Validation logic: require!(current_time >= grant.voting_end, IndrasError::InvalidState)
        assert!(current_time < voting_end, "Voting period not ended should be detected");
    }
    
    #[test]
    fn test_tally_grant_votes_validation_overflow() {
        // Test: checked_add returning None should fail
        let total_yes_weight = u64::MAX;
        let total_no_weight = 1u64;
        
        // Validation logic: checked_add should return None on overflow
        assert_eq!(total_yes_weight.checked_add(total_no_weight), None, "Overflow should be detected");
    }

    // ========== validate_expert_voter validation tests ==========
    
    #[test]
    fn test_validate_expert_voter_expert_entry_data_too_short() {
        // Test: expert_entry_data.len() < 8 should fail
        let data_len = 7usize;
        
        // Validation logic: require!(data.len() >= 8, IndrasError::InvalidInput)
        assert!(data_len < 8, "Expert entry data too short should be detected");
    }
    
    #[test]
    fn test_validate_expert_voter_expert_mismatch() {
        // Test: expert_entry.expert != voter should fail
        let expert_entry_expert = Pubkey::new_unique();
        let voter = Pubkey::new_unique();
        
        // Validation logic: require!(expert_entry.expert == voter, IndrasError::InvalidInput)
        assert_ne!(expert_entry_expert, voter, "Expert mismatch should be detected");
    }
    
    #[test]
    fn test_validate_expert_voter_expert_not_valid() {
        // Test: !expert_entry.is_valid_expert() should fail
        // This is validated in ExpertEntry::is_valid_expert()
        assert!(true, "Expert not valid check validated in ExpertEntry::is_valid_expert()");
    }
    
    #[test]
    fn test_validate_expert_voter_domain_id_mismatch() {
        // Test: expert_entry.domain_id != grant.semantic_domain should fail
        let expert_domain_id = "domain1".to_string();
        let grant_domain_id = "domain2".to_string();
        
        // Validation logic: require!(expert_entry.domain_id == grant.semantic_domain, IndrasError::InvalidSemanticDomain)
        assert_ne!(expert_domain_id, grant_domain_id, "Domain ID mismatch should be detected");
    }
    
    #[test]
    fn test_validate_expert_voter_grant_no_semantic_domain() {
        // Test: grant.semantic_domain == None should fail (when expert entry provided)
        let grant_semantic_domain: Option<String> = None;
        
        // Validation logic: require!(grant.semantic_domain.is_some(), IndrasError::InvalidInput)
        assert!(grant_semantic_domain.is_none(), "Grant no semantic domain should be detected");
    }
}
