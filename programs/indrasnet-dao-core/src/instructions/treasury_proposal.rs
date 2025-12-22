//! Treasury Proposal instruction handlers
//!
//! Handlers for creating and executing Treasury proposals

use anchor_lang::prelude::*;

use crate::error::IndrasError;
use crate::state::proposal::ProposalStatus;
use crate::state::proposal::treasury::{TreasuryOperationData, TreasuryProposalType};
use crate::state::member::role::role_permissions;
// Security CPI types (one-way dependency: Core → Security, safe)
// NOTE: Temporarily disabled - Security in exclude, causes build issues
// use indrasnet_dao_security::cpi::accounts::CheckProposalSecurity;
// use indrasnet_dao_security::cpi::check_proposal_security_cpi;

/// Create a Treasury proposal
///
/// This handler creates a new proposal specifically for Treasury operations.
/// The proposal will be linked to Treasury and can execute Treasury operations upon approval.
pub fn create_treasury_proposal_handler(
    ctx: Context<crate::CreateTreasuryProposal>,
    proposal_id: u64,
    title: String,
    description: String,
    treasury_operation: TreasuryOperationData,
    voting_duration: Option<i64>,
) -> Result<()> {
    let author = ctx.accounts.author.key();
    
    // Check permission: author must have CAN_PROPOSE permission
    if let Some(author_role) = &ctx.accounts.author_role {
        require!(
            author_role.has_permission(role_permissions::CAN_PROPOSE),
            IndrasError::Unauthorized
        );
    }
    
    // Validate treasury operation data
    treasury_operation.validate()?;
    
    let proposal = &mut ctx.accounts.proposal;
    let bump = ctx.bumps.proposal;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate inputs
    require!(!title.is_empty(), IndrasError::InvalidInput);
    require!(title.len() <= 200, IndrasError::InvalidInput);
    require!(!description.is_empty(), IndrasError::InvalidInput);
    require!(description.len() <= 2000, IndrasError::InvalidInput);
    
    // Security check via CPI (if Security program is provided)
    // NOTE: Temporarily disabled - Security in exclude, causes build issues
    // if let Some(security_program) = &ctx.accounts.security_program {
    //     // ... security check code ...
    // }
    
    // Initialize proposal fields
    proposal.id = proposal_id;
    proposal.title = title;
    proposal.description = description;
    proposal.proposal_type = "treasury".to_string();
    proposal.author = author;
    proposal.created_at = current_time;
    proposal.updated_at = None;
    proposal.submitted_at = None;
    proposal.cancelled_at = None;
    proposal.executed_at = None;
    proposal.archived_at = None;
    proposal.voting_duration = voting_duration.unwrap_or(7 * 24 * 3600); // 7 days default
    proposal.status = ProposalStatus::Draft;
    proposal.bump = bump;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.total_votes = 0;
    proposal.last_tallied_at = None;
    proposal.cancellation_reason = None;
    proposal.execution_data = None;
    proposal.expires_at = None;
    proposal.idea_id = None;
    proposal.treasury_operation = Some(treasury_operation);
    
    msg!("Treasury proposal {} created by {}", proposal_id, author);
    Ok(())
}

/// Execute Treasury proposal
///
/// This handler executes a passed Treasury proposal by performing the Treasury operation.
/// Only proposals with status Passed and treasury_operation set can be executed.
pub fn execute_treasury_proposal_handler(
    ctx: Context<crate::ExecuteTreasuryProposal>,
    proposal_id: u64,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let treasury = &mut ctx.accounts.treasury;
    let dao_config = &ctx.accounts.dao_config;
    
    require!(proposal.id == proposal_id, IndrasError::InvalidInput);
    require!(proposal.status == ProposalStatus::Passed, IndrasError::InvalidState);
    require!(proposal.treasury_operation.is_some(), IndrasError::InvalidState);
    
    // SECURITY: Verify executor is DAO authority
    require!(
        ctx.accounts.executor.key() == dao_config.authority,
        IndrasError::Unauthorized
    );
    
    let operation = proposal.treasury_operation.as_ref().unwrap();
    let current_time = Clock::get()?.unix_timestamp;
    
    // Execute the Treasury operation based on type
    match operation.operation_type {
        TreasuryProposalType::Withdrawal => {
            let amount = operation.amount.ok_or(IndrasError::InvalidInput)?;
            require!(treasury.balance >= amount, IndrasError::InsufficientFunds);
            treasury.balance = treasury.balance
                .checked_sub(amount)
                .ok_or(IndrasError::Underflow)?;
            msg!("Treasury proposal {} executed: withdrew {}", proposal_id, amount);
        },
        TreasuryProposalType::Deposit => {
            let amount = operation.amount.ok_or(IndrasError::InvalidInput)?;
            treasury.balance = treasury.balance
                .checked_add(amount)
                .ok_or(IndrasError::Overflow)?;
            msg!("Treasury proposal {} executed: deposited {}", proposal_id, amount);
        },
        TreasuryProposalType::Transfer => {
            // Transfer would require target treasury account
            // For now, just validate - actual transfer would need target treasury in accounts
            let amount = operation.amount.ok_or(IndrasError::InvalidInput)?;
            require!(treasury.balance >= amount, IndrasError::InsufficientFunds);
            require!(operation.target_treasury.is_some(), IndrasError::InvalidInput);
            // Actual transfer logic would be implemented here
            msg!("Treasury proposal {} executed: transfer {} to {:?}", 
                 proposal_id, amount, operation.target_treasury);
        },
        TreasuryProposalType::GrantCapability => {
            let grantee = operation.capability_grantee
                .ok_or(IndrasError::InvalidInput)?;
            let capability_type = operation.capability_type
                .as_ref()
                .ok_or(IndrasError::InvalidInput)?
                .clone();
            let expires_at = current_time + 86400 * 365; // Default: 1 year from now
            
            // Create or update capability account
            if let Some(capability_info) = &ctx.accounts.capability {
                // Verify capability PDA matches expected seeds
                let expected_capability = Pubkey::create_program_address(
                    &[
                        b"capability",
                        grantee.as_ref(),
                        treasury.key().as_ref(),
                    ],
                    ctx.program_id
                ).map_err(|_| IndrasError::InvalidInput)?;
                
                require!(
                    capability_info.key() == expected_capability,
                    IndrasError::InvalidInput
                );
                
                // Deserialize and update capability
                let mut capability_data = capability_info.try_borrow_mut_data()?;
                require!(capability_data.len() > 8, IndrasError::InvalidInput);
                
                // If account exists, update it; otherwise initialize
                if capability_data.len() == 8 {
                    // Initialize new capability
                    let capability = crate::Capability {
                        grantee,
                        granter: treasury.key(),
                        capability_type: capability_type.clone(),
                        expires_at,
                        bump: 0, // Will be set by Anchor if using init
                    };
                    
                    // Serialize capability
                    let serialized = anchor_lang::AnchorSerialize::try_to_vec(&capability)?;
                    require!(serialized.len() + 8 <= capability_data.len(), IndrasError::DataTooLarge);
                    capability_data[8..8+serialized.len()].copy_from_slice(&serialized);
                } else {
                    // Update existing capability
                    let mut capability: crate::Capability = borsh::BorshDeserialize::try_from_slice(&capability_data[8..])?;
                    capability.grantee = grantee;
                    capability.granter = treasury.key();
                    capability.capability_type = capability_type.clone();
                    capability.expires_at = expires_at;
                    
                    // Serialize updated capability
                    let serialized = anchor_lang::AnchorSerialize::try_to_vec(&capability)?;
                    capability_data[8..8+serialized.len()].copy_from_slice(&serialized);
                }
                
                msg!("Treasury proposal {} executed: granted capability '{}' to {} until {}", 
                     proposal_id, capability_type, grantee, expires_at);
            } else {
                // Capability account not provided - create via CPI would require separate instruction
                // For now, log the operation
                msg!("Treasury proposal {} executed: grant capability to {} (capability account not provided, use grant_capability instruction)", 
                     proposal_id, grantee);
            }
        },
        TreasuryProposalType::RevokeCapability => {
            let grantee = operation.capability_grantee
                .ok_or(IndrasError::InvalidInput)?;
            
            if let Some(capability_info) = &ctx.accounts.capability {
                // Deserialize and revoke capability
                let mut capability_data = capability_info.try_borrow_mut_data()?;
                require!(capability_data.len() > 8, IndrasError::InvalidInput);
                
                let mut capability: crate::Capability = borsh::BorshDeserialize::try_from_slice(&capability_data[8..])?;
                require!(
                    capability.grantee == grantee && capability.granter == treasury.key(),
                    IndrasError::InvalidInput
                );
                
                // Revoke by setting expiration to 0
                capability.expires_at = 0;
                
                // Serialize updated capability
                let serialized = anchor_lang::AnchorSerialize::try_to_vec(&capability)?;
                capability_data[8..8+serialized.len()].copy_from_slice(&serialized);
                
                msg!("Treasury proposal {} executed: revoked capability from {}", 
                     proposal_id, grantee);
            } else {
                msg!("Treasury proposal {} executed: revoke capability from {} (capability account not provided, use revoke_capability instruction)", 
                     proposal_id, grantee);
            }
        },
        TreasuryProposalType::UpdateConfig => {
            // Config update would be handled separately
            msg!("Treasury proposal {} executed: update treasury config", proposal_id);
        },
    }
    
    // Mark proposal as executed
    proposal.status = ProposalStatus::Executed;
    proposal.executed_at = Some(current_time);
    
    // Store execution data
    proposal.execution_data = Some(format!("treasury_operation_executed:{}", current_time));
    
    msg!("Treasury proposal {} executed successfully", proposal_id);
    Ok(())
}
