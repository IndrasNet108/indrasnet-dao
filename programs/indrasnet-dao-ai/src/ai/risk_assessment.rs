//! Risk Assessment Module
//! 
//! Comprehensive risk assessment and analysis system for ideas, proposals, and operations.
//! Integrates with AI analysis and security modules for comprehensive risk evaluation.
//! 
//! Hybrid model: Off-chain risk assessment and analysis, on-chain metadata storage
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Only risk assessment metadata, scores, and summary records
//! - OFF-CHAIN: All actual risk assessment, analysis, and detailed evaluation
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-ai/src/ai/risk_assessment.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Risk category
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum RiskCategory {
    /// Technical risk
    Technical,
    /// Financial risk
    Financial,
    /// Legal risk
    Legal,
    /// Operational risk
    Operational,
    /// Reputational risk
    Reputational,
    /// Security risk
    Security,
    /// Compliance risk
    Compliance,
}

impl Space for RiskCategory {
    const INIT_SPACE: usize = 1;
}

/// Risk level
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

impl Space for RiskLevel {
    const INIT_SPACE: usize = 1;
}

/// Risk assessment record
#[account]
#[derive(InitSpace)]
pub struct RiskAssessment {
    /// Assessment ID
    pub assessment_id: u64,
    /// Entity ID (idea, proposal, etc.)
    pub entity_id: u64,
    /// Entity type
    #[max_len(50)]
    pub entity_type: String,
    /// Overall risk level
    pub overall_risk: RiskLevel,
    /// Risk scores by category (0-100)
    pub risk_scores: RiskScores,
    /// Assessment timestamp
    pub assessed_at: i64,
    /// Assessor (AI system or authority)
    pub assessor: Pubkey,
    /// Assessment metadata URI (IPFS or similar)
    #[max_len(500)]
    pub metadata_uri: String,
    /// Assessment hash (for verification)
    pub assessment_hash: [u8; 32],
    /// Bump seed for PDA
    pub bump: u8,
}

/// Risk scores by category
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct RiskScores {
    /// Technical risk score (0-100)
    pub technical: u8,
    /// Financial risk score (0-100)
    pub financial: u8,
    /// Legal risk score (0-100)
    pub legal: u8,
    /// Operational risk score (0-100)
    pub operational: u8,
    /// Reputational risk score (0-100)
    pub reputational: u8,
    /// Security risk score (0-100)
    pub security: u8,
    /// Compliance risk score (0-100)
    pub compliance: u8,
}

impl Space for RiskScores {
    const INIT_SPACE: usize = 7; // 7 * u8
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Create risk assessment
pub fn create_risk_assessment(
    assessment: &mut RiskAssessment,
    assessment_id: u64,
    entity_id: u64,
    entity_type: String,
    risk_scores: RiskScores,
    metadata_uri: String,
    assessment_hash: [u8; 32],
    assessor: Pubkey,
) -> Result<()> {
    require!(entity_type.len() <= 50, IndrasError::StringTooLong);
    require!(metadata_uri.len() <= 500, IndrasError::StringTooLong);
    
    // Validate risk scores (0-100)
    let scores = [
        risk_scores.technical,
        risk_scores.financial,
        risk_scores.legal,
        risk_scores.operational,
        risk_scores.reputational,
        risk_scores.security,
        risk_scores.compliance,
    ];
    
    for score in scores.iter() {
        require!(*score <= 100, IndrasError::InvalidScore);
    }
    
    // Calculate overall risk level
    let avg_score = (scores.iter().map(|&s| s as u32).sum::<u32>() / scores.len() as u32) as u8;
    let overall_risk = match avg_score {
        0..=25 => RiskLevel::Low,
        26..=50 => RiskLevel::Medium,
        51..=75 => RiskLevel::High,
        _ => RiskLevel::Critical,
    };
    
    assessment.assessment_id = assessment_id;
    assessment.entity_id = entity_id;
    assessment.entity_type = entity_type;
    assessment.overall_risk = overall_risk;
    assessment.risk_scores = risk_scores;
    assessment.assessed_at = Clock::get()?.unix_timestamp;
    assessment.assessor = assessor;
    assessment.metadata_uri = metadata_uri;
    assessment.assessment_hash = assessment_hash;
    
    msg!("Risk assessment {} created for entity {} (overall risk: {:?}, avg score: {})", 
         assessment_id, entity_id, overall_risk, avg_score);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_risk_assessment() {
        let mut assessment = RiskAssessment {
            assessment_id: 0,
            entity_id: 0,
            entity_type: String::new(),
            overall_risk: RiskLevel::Low,
            risk_scores: RiskScores {
                technical: 0,
                financial: 0,
                legal: 0,
                operational: 0,
                reputational: 0,
                security: 0,
                compliance: 0,
            },
            assessed_at: 0,
            assessor: Pubkey::default(),
            metadata_uri: String::new(),
            assessment_hash: [0u8; 32],
            bump: 0,
        };

        let assessor = Pubkey::from([1u8; 32]);
        let assessment_hash = [1u8; 32];
        let risk_scores = RiskScores {
            technical: 30,
            financial: 40,
            legal: 20,
            operational: 35,
            reputational: 25,
            security: 30,
            compliance: 28,
        };

        let result = create_risk_assessment(
            &mut assessment,
            1,
            100,
            "idea".to_string(),
            risk_scores.clone(),
            "ipfs://test".to_string(),
            assessment_hash,
            assessor,
        );

        assert!(result.is_ok());
        assert_eq!(assessment.assessment_id, 1);
        assert_eq!(assessment.entity_id, 100);
        assert_eq!(assessment.overall_risk, RiskLevel::Medium); // Average ~30
    }
}
