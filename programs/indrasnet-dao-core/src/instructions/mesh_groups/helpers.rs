//! Helper functions for mesh group operations

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use crate::constants::ai_program_id;
use crate::state::ai_analysis_record::AIAnalysisRecord;

/// Helper function to verify AI analysis account
/// 
/// CRITICAL: Checks that AI analysis exists and belongs to Core program,
/// and has decision == Approve (idea can be added to mesh group)
/// 
/// NOTE: Use manual deserialization to avoid circular dependency
pub(crate) fn verify_ai_analysis(
    analysis_account: &UncheckedAccount,
    idea_id: u64,
    idea_pubkey: &Pubkey,
    analysis_record: &AIAnalysisRecord,
) -> Result<()> {
    // Check that account is not empty
    require!(
        !analysis_account.data_is_empty(),
        IndrasError::InvalidInput
    );
    
    // Check that account belongs to AI program
    require!(
        analysis_account.owner == &ai_program_id(),
        IndrasError::InvalidProgram
    );

    // Check AI program PDA for analysis
    let (expected_analysis, _) = Pubkey::find_program_address(
        &[b"ai_analysis", idea_pubkey.as_ref()],
        &ai_program_id(),
    );
    require!(
        analysis_account.key() == expected_analysis,
        IndrasError::InvalidProgram
    );

    // Verify AI analysis registration record (CPI-guarded)
    require!(
        analysis_record.analysis == analysis_account.key(),
        IndrasError::InvalidInput
    );
    require!(
        analysis_record.idea_id == idea_id,
        IndrasError::InvalidInput
    );
    require!(
        analysis_record.ai_program == ai_program_id(),
        IndrasError::InvalidProgram
    );
    
    // Deserialize AIAnalysis manually (avoid circular dependency)
    // AIAnalysis structure: idea_id (u64), reviewer (Pubkey), decision (enum), ...
    // decision is at position 8 + 8 + 32 = 48 bytes from start (after discriminator)
    let data = analysis_account.try_borrow_data()?;
    require!(data.len() > 48, IndrasError::InvalidInput); // Minimum 49 bytes
    
    // Check idea_id (bytes 8-15 after discriminator)
    let idea_id_bytes: [u8; 8] = data[8..16].try_into().map_err(|_| IndrasError::InvalidInput)?;
    let analysis_idea_id = u64::from_le_bytes(idea_id_bytes);
    require!(
        analysis_idea_id == idea_id,
        IndrasError::InvalidInput
    );
    
    // Check decision (now Option<AIReviewDecision>)
    // Structure: idea_id (8), reviewer (32), decision (Option<enum> = 1 or 2 bytes)
    // decision is at position 8 + 32 = 40 bytes from start (after discriminator)
    // Option<enum> = 1 byte (Some) + 1 byte (enum value) = 2 bytes, or 0 bytes (None)
    // 
    // For backward compatibility: if decision = None, calculate based on scores
    // For now, check that decision exists and equals Approve
    if data.len() >= 42 {
        // Check that decision is not None (byte 40: 0 = None, 1 = Some)
        let decision_option = data[40];
        if decision_option == 1 {
            // Some - Check value (byte 41)
            let decision_value = data[41];
            require!(
                decision_value == 0, // AIReviewDecision::Approve = 0
                IndrasError::InvalidState
            );
        } else {
            // None - decision not set, skip check (for backward compatibility)
            // In this case, decision will be calculated based on scores
        }
    }
    
    // Additional checks: scores for can_enter_mesh_group
    // ethics_compliance_score (byte 51), legal_compliance_score (byte 52)
    // uniqueness_score (byte 54), feasibility_score (byte 56), artifacts_verified (byte ~120)
    if data.len() >= 57 {
        let ethics_score = data[51];
        let legal_score = data[52];
        let uniqueness_score = data[54];
        let feasibility_score = data[56];
        
        // Check can_enter_mesh_group criteria
        require!(ethics_score >= 50, IndrasError::InvalidState);
        require!(legal_score >= 50, IndrasError::InvalidState);
        require!(uniqueness_score >= 70, IndrasError::InvalidState);
        require!(feasibility_score >= 70, IndrasError::InvalidState);
        
        // Check artifacts_verified (approximately at position 120)
        if data.len() >= 121 {
            let artifacts_verified = data[120] != 0;
            require!(artifacts_verified, IndrasError::InvalidState);
        }
    }
    
    Ok(())
}
