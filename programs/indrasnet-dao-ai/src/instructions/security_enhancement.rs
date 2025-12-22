//! Security Enhancement Instructions
//!
//! AI module instructions for enhancing Security module analysis

use anchor_lang::prelude::*;
use crate::state::security_analysis::{SecurityAnalysisEnhancement, RiskLevel};

/// Enhance security analysis with AI insights
///
/// Called by Security module via CPI to get AI-enhanced threat interpretation
///
/// # Parameters
/// - `threat_data`: Raw threat data from ML models
/// - `threat_score`: Threat score from ML (0-100)
/// - `anomaly_score`: Anomaly score from ML (0-100)
/// - `context`: Additional context for AI analysis
///
/// # Returns
/// Enhanced security analysis with AI insights
///
/// # Compute Units
/// Recommended: 50,000 CU
pub fn enhance_security_analysis_handler(
    _ctx: Context<crate::EnhanceSecurityAnalysis>,
    threat_data: String,
    threat_score: u8,
    anomaly_score: u8,
    context: String,
) -> Result<SecurityAnalysisEnhancement> {
    // Validate inputs
    require!(threat_score <= 100, crate::error::IndrasError::InvalidScore);
    require!(anomaly_score <= 100, crate::error::IndrasError::InvalidScore);
    require!(threat_data.len() <= 1000, crate::error::IndrasError::StringTooLong);
    require!(context.len() <= 500, crate::error::IndrasError::StringTooLong);
    
    // AI analysis logic
    // In production, this would use off-chain AI service (Gemini API)
    // For now, we provide on-chain analysis based on scores and context
    
    // Calculate combined risk score
    let combined_score = ((threat_score as u16 + anomaly_score as u16) / 2) as u8;
    
    // Determine suggested risk level
    let suggested_risk_level = if combined_score >= 80 {
        Some(RiskLevel::Critical)
    } else if combined_score >= 60 {
        Some(RiskLevel::High)
    } else if combined_score >= 40 {
        Some(RiskLevel::Medium)
    } else {
        Some(RiskLevel::Low)
    };
    
    // Generate enhanced threat interpretation
    let enhanced_threat_interpretation = format!(
        "AI Analysis: Threat score {}%, Anomaly score {}%. Combined risk: {}%. Context: {}. \
        The AI analysis suggests {} risk level based on the provided data and context.",
        threat_score,
        anomaly_score,
        combined_score,
        if context.is_empty() { "no additional context" } else { &context },
        match suggested_risk_level {
            Some(RiskLevel::Critical) => "CRITICAL",
            Some(RiskLevel::High) => "HIGH",
            Some(RiskLevel::Medium) => "MEDIUM",
            Some(RiskLevel::Low) => "LOW",
            None => "UNKNOWN",
        }
    );
    
    // Generate AI confidence (based on data quality)
    let ai_confidence = if !threat_data.is_empty() && !context.is_empty() {
        85u8 // High confidence with good data
    } else if !threat_data.is_empty() || !context.is_empty() {
        70u8 // Medium confidence with partial data
    } else {
        50u8 // Low confidence with minimal data
    };
    
    // Generate recommendations
    let recommendations = match suggested_risk_level {
        Some(RiskLevel::Critical) => {
            "CRITICAL: Immediate action required. Block transaction and require DAO vote. \
            Conduct detailed security review before any execution."
        }
        Some(RiskLevel::High) => {
            "HIGH: Requires DAO vote and security review. Do not auto-execute. \
            Review all transaction details carefully."
        }
        Some(RiskLevel::Medium) => {
            "MEDIUM: Review recommended before execution. Monitor for suspicious patterns. \
            Consider additional verification steps."
        }
        Some(RiskLevel::Low) => {
            "LOW: Generally safe to proceed. Standard security checks should be sufficient. \
            Continue monitoring for any changes."
        }
        None => {
            "UNKNOWN: Unable to determine risk level. Recommend manual review."
        }
    }.to_string();
    
    // Generate context insights
    let context_insights = if context.contains("proposal") {
        "Context indicates this is a proposal. AI recommends checking proposal content against \
        DAO governance rules and ensuring all required approvals are in place."
    } else if context.contains("transfer") || context.contains("treasury") {
        "Context indicates treasury operation. AI recommends verifying authorization levels \
        and checking against treasury management policies."
    } else if context.contains("grant") {
        "Context indicates grant operation. AI recommends verifying grant eligibility \
        and checking against grant program requirements."
    } else {
        "Standard security analysis. AI recommends following standard security protocols."
    }.to_string();
    
    msg!(
        "AI Security Enhancement: threat={}%, anomaly={}%, risk={:?}, confidence={}%",
        threat_score,
        anomaly_score,
        suggested_risk_level,
        ai_confidence
    );
    
    Ok(SecurityAnalysisEnhancement {
        enhanced_threat_interpretation,
        ai_confidence,
        recommendations,
        context_insights,
        suggested_risk_level,
    })
}
