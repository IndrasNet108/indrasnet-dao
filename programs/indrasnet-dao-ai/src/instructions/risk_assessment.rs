//! Risk Assessment handlers
//!
//! Handlers for Risk Assessment instructions
//!
//! On-chain: Validation and state management
//! Off-chain: Actual risk assessment and analysis (in separate service)

use anchor_lang::prelude::*;
use crate::ai::risk_assessment::*;

/// Create risk assessment
///
/// Creates a risk assessment record for an entity (idea, proposal, etc.)
///
/// # Compute Units
/// Recommended: 40,000 CU
/// - Validation: ~10,000 CU
/// - Account initialization: ~30,000 CU
pub fn create_risk_assessment_handler(
    ctx: Context<crate::CreateRiskAssessment>,
    assessment_id: u64,
    entity_id: u64,
    entity_type: String,
    risk_scores: RiskScores,
    metadata_uri: String,
    assessment_hash: [u8; 32],
) -> Result<()> {
    let assessment = &mut ctx.accounts.assessment;
    let assessor = ctx.accounts.authority.key();
    
    create_risk_assessment(
        assessment,
        assessment_id,
        entity_id,
        entity_type,
        risk_scores,
        metadata_uri,
        assessment_hash,
        assessor,
    )
}
