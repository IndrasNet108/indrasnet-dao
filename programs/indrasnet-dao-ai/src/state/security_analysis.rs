//! Security Analysis Enhancement State
//!
//! Types for AI-enhanced security analysis results

use anchor_lang::prelude::*;

/// Enhanced security analysis result from AI
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct SecurityAnalysisEnhancement {
    /// Enhanced threat interpretation from AI
    pub enhanced_threat_interpretation: String,
    /// AI confidence score (0-100)
    pub ai_confidence: u8,
    /// AI recommendations for risk assessment
    pub recommendations: String,
    /// Additional context provided by AI
    pub context_insights: String,
    /// Risk level suggested by AI
    pub suggested_risk_level: Option<RiskLevel>,
}

impl SecurityAnalysisEnhancement {
    pub const INIT_SPACE: usize = 4 + // discriminator
        4 + 500 + // enhanced_threat_interpretation (String)
        1 + // ai_confidence (u8)
        4 + 200 + // recommendations (String)
        4 + 300 + // context_insights (String)
        1 + 1; // suggested_risk_level (Option<RiskLevel>)
}

/// Risk level enum (matches Security module)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum RiskLevel {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}
