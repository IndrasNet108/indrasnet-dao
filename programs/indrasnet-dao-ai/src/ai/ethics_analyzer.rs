//! Unified Ethics Analyzer
//! 
//! This module combines the best features from both complex and simple ethics analyzers
//! to provide a comprehensive yet efficient ethics analysis system.
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-ai/src/ai/ethics_analyzer.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Maximum length for idea title (bytes)
pub const MAX_IDEA_TITLE_LENGTH: usize = 200;

/// Maximum length for idea description (bytes)
pub const MAX_IDEA_DESCRIPTION_LENGTH: usize = 3000;

/// Maximum combined length for title + description analysis (bytes)
/// This prevents excessive compute units for text processing
pub const MAX_COMBINED_TEXT_LENGTH: usize = 3200;

/// Comprehensive ethics analysis for ideas
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct EthicsAnalysis {
    /// Overall ethical score (0-100)
    pub overall_score: u8,
    /// Detailed category scores
    pub category_scores: Vec<(String, u8)>,
    /// Identified ethical concerns
    pub ethical_concerns: Vec<EthicalConcern>,
    /// Improvement recommendations
    pub recommendations: Vec<String>,
    /// Risk level
    pub risk_level: EthicsRiskLevel,
    /// Whether additional review is required
    pub requires_review: bool,
    /// Number of concerns (for compatibility)
    pub concerns_count: u8,
    /// Number of recommendations (for compatibility)
    pub recommendations_count: u8,
}

/// Ethical concerns
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct EthicalConcern {
    /// Type of concern
    pub concern_type: EthicalConcernType,
    /// Description of the concern
    pub description: String,
    /// Severity level (1-5)
    pub severity: u8,
    /// Mitigation recommendations
    pub mitigation: String,
}

/// Types of ethical concerns
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum EthicalConcernType {
    /// Bias and discrimination
    Bias,
    /// Privacy violations
    Privacy,
    /// Safety concerns
    Safety,
    /// Fairness issues
    Fairness,
    /// Transparency problems
    Transparency,
    /// Accountability issues
    Accountability,
    /// Environmental harm
    EnvironmentalHarm,
    /// Economic inequality
    EconomicInequality,
    /// Manipulation
    Manipulation,
}

/// Ethics risk level
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum EthicsRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Ethics Analyzer
pub struct EthicsAnalyzer;

impl EthicsAnalyzer {
    /// Analyze idea for ethical concerns
    /// 
    /// # Arguments
    /// * `idea_title` - Idea title (max MAX_IDEA_TITLE_LENGTH bytes)
    /// * `idea_description` - Idea description (max MAX_IDEA_DESCRIPTION_LENGTH bytes)
    /// * `_idea_context` - Additional context (currently unused)
    /// 
    /// # Errors
    /// * `IndrasError::InvalidInput` - If title or description is empty
    /// * `IndrasError::DataTooLarge` - If title or description exceeds maximum length
    pub fn analyze_idea(
        idea_title: &str,
        idea_description: &str,
        _idea_context: &str,
    ) -> Result<EthicsAnalysis> {
        // SECURITY: Validate input text lengths early (fail-fast pattern)
        // This prevents excessive compute units for processing large texts
        require!(!idea_title.is_empty(), IndrasError::InvalidInput);
        require!(!idea_description.is_empty(), IndrasError::InvalidInput);
        require!(
            idea_title.len() <= MAX_IDEA_TITLE_LENGTH,
            IndrasError::DataTooLarge
        );
        require!(
            idea_description.len() <= MAX_IDEA_DESCRIPTION_LENGTH,
            IndrasError::DataTooLarge
        );
        
        msg!("Starting ethics analysis for idea: {}", idea_title);
        
        // Initialize analysis
        let mut analysis = EthicsAnalysis {
            overall_score: 0,
            category_scores: Vec::new(),
            ethical_concerns: Vec::new(),
            recommendations: Vec::new(),
            risk_level: EthicsRiskLevel::Low,
            requires_review: false,
            concerns_count: 0,
            recommendations_count: 0,
        };
        
        // Analyze different aspects
        Self::analyze_bias(&mut analysis, idea_title, idea_description)?;
        Self::analyze_privacy(&mut analysis, idea_description)?;
        Self::analyze_safety(&mut analysis, idea_description)?;
        Self::analyze_fairness(&mut analysis, idea_description)?;
        Self::analyze_transparency(&mut analysis, idea_description)?;
        Self::analyze_accountability(&mut analysis, idea_description)?;
        
        // Calculate overall score
        analysis.overall_score = Self::calculate_overall_score(&analysis.category_scores);
        
        // Determine risk level
        analysis.risk_level = Self::determine_risk_level(analysis.overall_score, analysis.ethical_concerns.len());
        
        // Set counts for compatibility
        analysis.concerns_count = analysis.ethical_concerns.len() as u8;
        analysis.recommendations_count = analysis.recommendations.len() as u8;
        
        // Determine if review is required
        analysis.requires_review = analysis.risk_level == EthicsRiskLevel::High || 
                                  analysis.risk_level == EthicsRiskLevel::Critical;
        
        msg!("Ethics analysis completed. Score: {}, Risk: {:?}", 
             analysis.overall_score, analysis.risk_level);
        
        Ok(analysis)
    }
    
    /// Analyze for bias concerns
    fn analyze_bias(
        analysis: &mut EthicsAnalysis,
        title: &str,
        description: &str,
    ) -> Result<()> {
        let bias_score = Self::calculate_bias_score(title, description);
        analysis.category_scores.push(("Bias".to_string(), bias_score));
        
        if bias_score < 70 {
            analysis.ethical_concerns.push(EthicalConcern {
                concern_type: EthicalConcernType::Bias,
                description: "Potential bias detected in idea content".to_string(),
                severity: (100 - bias_score) / 20 + 1,
                mitigation: "Review content for inclusive language and diverse perspectives".to_string(),
            });
            
            analysis.recommendations.push("Ensure diverse representation in idea development".to_string());
        }
        
        Ok(())
    }
    
    /// Analyze for privacy concerns
    fn analyze_privacy(
        analysis: &mut EthicsAnalysis,
        description: &str,
    ) -> Result<()> {
        let privacy_score = Self::calculate_privacy_score(description);
        analysis.category_scores.push(("Privacy".to_string(), privacy_score));
        
        if privacy_score < 70 {
            analysis.ethical_concerns.push(EthicalConcern {
                concern_type: EthicalConcernType::Privacy,
                description: "Privacy concerns identified in idea".to_string(),
                severity: (100 - privacy_score) / 20 + 1,
                mitigation: "Implement privacy-by-design principles".to_string(),
            });
            
            analysis.recommendations.push("Add privacy protection measures".to_string());
        }
        
        Ok(())
    }
    
    /// Analyze for safety concerns
    fn analyze_safety(
        analysis: &mut EthicsAnalysis,
        description: &str,
    ) -> Result<()> {
        let safety_score = Self::calculate_safety_score(description);
        analysis.category_scores.push(("Safety".to_string(), safety_score));
        
        if safety_score < 70 {
            analysis.ethical_concerns.push(EthicalConcern {
                concern_type: EthicalConcernType::Safety,
                description: "Safety concerns identified in idea".to_string(),
                severity: (100 - safety_score) / 20 + 1,
                mitigation: "Implement safety measures and risk assessment".to_string(),
            });
            
            analysis.recommendations.push("Add safety protocols and monitoring".to_string());
        }
        
        Ok(())
    }
    
    /// Analyze for fairness concerns
    fn analyze_fairness(
        analysis: &mut EthicsAnalysis,
        description: &str,
    ) -> Result<()> {
        let fairness_score = Self::calculate_fairness_score(description);
        analysis.category_scores.push(("Fairness".to_string(), fairness_score));
        
        if fairness_score < 70 {
            analysis.ethical_concerns.push(EthicalConcern {
                concern_type: EthicalConcernType::Fairness,
                description: "Fairness concerns identified in idea".to_string(),
                severity: (100 - fairness_score) / 20 + 1,
                mitigation: "Ensure equal access and treatment for all users".to_string(),
            });
            
            analysis.recommendations.push("Implement fairness metrics and monitoring".to_string());
        }
        
        Ok(())
    }
    
    /// Analyze for transparency concerns
    fn analyze_transparency(
        analysis: &mut EthicsAnalysis,
        description: &str,
    ) -> Result<()> {
        let transparency_score = Self::calculate_transparency_score(description);
        analysis.category_scores.push(("Transparency".to_string(), transparency_score));
        
        if transparency_score < 70 {
            analysis.ethical_concerns.push(EthicalConcern {
                concern_type: EthicalConcernType::Transparency,
                description: "Transparency concerns identified in idea".to_string(),
                severity: (100 - transparency_score) / 20 + 1,
                mitigation: "Increase transparency in decision-making processes".to_string(),
            });
            
            analysis.recommendations.push("Add transparency reporting mechanisms".to_string());
        }
        
        Ok(())
    }
    
    /// Analyze for accountability concerns
    fn analyze_accountability(
        analysis: &mut EthicsAnalysis,
        description: &str,
    ) -> Result<()> {
        let accountability_score = Self::calculate_accountability_score(description);
        analysis.category_scores.push(("Accountability".to_string(), accountability_score));
        
        if accountability_score < 70 {
            analysis.ethical_concerns.push(EthicalConcern {
                concern_type: EthicalConcernType::Accountability,
                description: "Accountability concerns identified in idea".to_string(),
                severity: (100 - accountability_score) / 20 + 1,
                mitigation: "Implement clear accountability mechanisms".to_string(),
            });
            
            analysis.recommendations.push("Add accountability frameworks and oversight".to_string());
        }
        
        Ok(())
    }
    
    /// Calculate bias score
    fn calculate_bias_score(title: &str, description: &str) -> u8 {
        // Simple heuristic-based scoring
        let mut score = 100u8;
        
        // Check for potentially biased terms
        let biased_terms = ["exclusive", "elite", "premium", "superior"];
        let text = format!("{} {}", title, description).to_lowercase();
        
        for term in &biased_terms {
            if text.contains(term) {
                score = score.saturating_sub(20);
            }
        }
        
        score
    }
    
    /// Calculate privacy score
    /// 
    /// # Arguments
    /// * `description` - Idea description (must be <= MAX_IDEA_DESCRIPTION_LENGTH)
    fn calculate_privacy_score(description: &str) -> u8 {
        // SECURITY: Early return if description exceeds limit (defensive check)
        if description.len() > MAX_IDEA_DESCRIPTION_LENGTH {
            return 50; // Neutral score
        }
        
        let mut score = 100u8;
        
        // Check for privacy-related terms
        let privacy_terms = ["personal data", "private", "confidential", "sensitive"];
        let text = description.to_lowercase();
        
        for term in &privacy_terms {
            if text.contains(term) {
                score = score.saturating_sub(15);
            }
        }
        
        score
    }
    
    /// Calculate safety score
    /// 
    /// # Arguments
    /// * `description` - Idea description (must be <= MAX_IDEA_DESCRIPTION_LENGTH)
    fn calculate_safety_score(description: &str) -> u8 {
        // SECURITY: Early return if description exceeds limit (defensive check)
        if description.len() > MAX_IDEA_DESCRIPTION_LENGTH {
            return 50; // Neutral score
        }
        
        let mut score = 100u8;
        
        // Check for safety-related terms
        let safety_terms = ["dangerous", "risky", "hazardous", "unsafe"];
        let text = description.to_lowercase();
        
        for term in &safety_terms {
            if text.contains(term) {
                score = score.saturating_sub(25);
            }
        }
        
        score
    }
    
    /// Calculate fairness score
    /// 
    /// # Arguments
    /// * `description` - Idea description (must be <= MAX_IDEA_DESCRIPTION_LENGTH)
    fn calculate_fairness_score(description: &str) -> u8 {
        // SECURITY: Early return if description exceeds limit (defensive check)
        if description.len() > MAX_IDEA_DESCRIPTION_LENGTH {
            return 50; // Neutral score
        }
        
        let mut score = 100u8;
        
        // Check for fairness-related terms
        let fairness_terms = ["unfair", "discriminatory", "biased", "unequal"];
        let text = description.to_lowercase();
        
        for term in &fairness_terms {
            if text.contains(term) {
                score = score.saturating_sub(20);
            }
        }
        
        score
    }
    
    /// Calculate transparency score
    /// 
    /// # Arguments
    /// * `description` - Idea description (must be <= MAX_IDEA_DESCRIPTION_LENGTH)
    fn calculate_transparency_score(description: &str) -> u8 {
        // SECURITY: Early return if description exceeds limit (defensive check)
        if description.len() > MAX_IDEA_DESCRIPTION_LENGTH {
            return 50; // Neutral score
        }
        
        let mut score = 100u8;
        
        // Check for transparency-related terms
        let transparency_terms = ["transparent", "open", "clear", "public"];
        let text = description.to_lowercase();
        
        for term in &transparency_terms {
            if text.contains(term) {
                score = score.saturating_add(10);
            }
        }
        
        score.min(100)
    }
    
    /// Calculate accountability score
    /// 
    /// # Arguments
    /// * `description` - Idea description (must be <= MAX_IDEA_DESCRIPTION_LENGTH)
    fn calculate_accountability_score(description: &str) -> u8 {
        // SECURITY: Early return if description exceeds limit (defensive check)
        if description.len() > MAX_IDEA_DESCRIPTION_LENGTH {
            return 50; // Neutral score
        }
        
        let mut score = 100u8;
        
        // Check for accountability-related terms
        let accountability_terms = ["accountable", "responsible", "oversight", "governance"];
        let text = description.to_lowercase();
        
        for term in &accountability_terms {
            if text.contains(term) {
                score = score.saturating_add(10);
            }
        }
        
        score.min(100)
    }
    
    /// Calculate overall score from category scores
    fn calculate_overall_score(category_scores: &[(String, u8)]) -> u8 {
        if category_scores.is_empty() {
            return 0;
        }
        
        let total: u32 = category_scores.iter().map(|(_, score)| *score as u32).sum();
        (total / category_scores.len() as u32) as u8
    }
    
    /// Determine risk level based on score and concerns
    fn determine_risk_level(score: u8, concerns_count: usize) -> EthicsRiskLevel {
        match (score, concerns_count) {
            (s, c) if s >= 90 && c == 0 => EthicsRiskLevel::Low,
            (s, c) if s >= 70 && c <= 2 => EthicsRiskLevel::Medium,
            (s, c) if s >= 50 && c <= 5 => EthicsRiskLevel::High,
            _ => EthicsRiskLevel::Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethics_risk_level_variants() {
        assert_eq!(EthicsRiskLevel::Low, EthicsRiskLevel::Low);
        assert_eq!(EthicsRiskLevel::Medium, EthicsRiskLevel::Medium);
        assert_eq!(EthicsRiskLevel::High, EthicsRiskLevel::High);
        assert_eq!(EthicsRiskLevel::Critical, EthicsRiskLevel::Critical);
        
        assert_ne!(EthicsRiskLevel::Low, EthicsRiskLevel::High);
        assert_ne!(EthicsRiskLevel::Medium, EthicsRiskLevel::Critical);
    }

    #[test]
    fn test_ethical_concern_type_variants() {
        let types = vec![
            EthicalConcernType::Bias,
            EthicalConcernType::Privacy,
            EthicalConcernType::Safety,
            EthicalConcernType::Fairness,
            EthicalConcernType::Transparency,
            EthicalConcernType::Accountability,
            EthicalConcernType::EnvironmentalHarm,
            EthicalConcernType::EconomicInequality,
            EthicalConcernType::Manipulation,
        ];

        for (i, type1) in types.iter().enumerate() {
            for (j, type2) in types.iter().enumerate() {
                if i == j {
                    assert_eq!(type1, type2);
                } else {
                    assert_ne!(type1, type2);
                }
            }
        }
    }

    #[test]
    fn test_ethical_concern_creation() {
        let concern = EthicalConcern {
            concern_type: EthicalConcernType::Bias,
            description: "Test concern".to_string(),
            severity: 3,
            mitigation: "Test mitigation".to_string(),
        };

        assert_eq!(concern.concern_type, EthicalConcernType::Bias);
        assert_eq!(concern.description, "Test concern");
        assert_eq!(concern.severity, 3);
        assert_eq!(concern.mitigation, "Test mitigation");
    }

    #[test]
    fn test_ethics_analysis_creation() {
        let analysis = EthicsAnalysis {
            overall_score: 85,
            category_scores: vec![("Bias".to_string(), 90), ("Privacy".to_string(), 80)],
            ethical_concerns: vec![],
            recommendations: vec!["Recommendation 1".to_string()],
            risk_level: EthicsRiskLevel::Low,
            requires_review: false,
            concerns_count: 0,
            recommendations_count: 1,
        };

        assert_eq!(analysis.overall_score, 85);
        assert_eq!(analysis.category_scores.len(), 2);
        assert_eq!(analysis.ethical_concerns.len(), 0);
        assert_eq!(analysis.recommendations.len(), 1);
        assert_eq!(analysis.risk_level, EthicsRiskLevel::Low);
        assert!(!analysis.requires_review);
        assert_eq!(analysis.concerns_count, 0);
        assert_eq!(analysis.recommendations_count, 1);
    }

    #[test]
    fn test_ethics_analysis_with_concerns() {
        let concern = EthicalConcern {
            concern_type: EthicalConcernType::Privacy,
            description: "Privacy issue".to_string(),
            severity: 4,
            mitigation: "Fix privacy".to_string(),
        };

        let analysis = EthicsAnalysis {
            overall_score: 60,
            category_scores: vec![("Privacy".to_string(), 50)],
            ethical_concerns: vec![concern.clone()],
            recommendations: vec!["Add privacy controls".to_string()],
            risk_level: EthicsRiskLevel::High,
            requires_review: true,
            concerns_count: 1,
            recommendations_count: 1,
        };

        assert_eq!(analysis.overall_score, 60);
        assert_eq!(analysis.ethical_concerns.len(), 1);
        assert_eq!(analysis.ethical_concerns[0].concern_type, EthicalConcernType::Privacy);
        assert_eq!(analysis.risk_level, EthicsRiskLevel::High);
        assert!(analysis.requires_review);
    }

    #[test]
    fn test_ethics_analysis_risk_levels() {
        let low_risk = EthicsAnalysis {
            overall_score: 90,
            category_scores: vec![],
            ethical_concerns: vec![],
            recommendations: vec![],
            risk_level: EthicsRiskLevel::Low,
            requires_review: false,
            concerns_count: 0,
            recommendations_count: 0,
        };

        let high_risk = EthicsAnalysis {
            overall_score: 50,
            category_scores: vec![],
            ethical_concerns: vec![],
            recommendations: vec![],
            risk_level: EthicsRiskLevel::High,
            requires_review: true,
            concerns_count: 0,
            recommendations_count: 0,
        };

        assert_eq!(low_risk.risk_level, EthicsRiskLevel::Low);
        assert_eq!(high_risk.risk_level, EthicsRiskLevel::High);
        assert_ne!(low_risk.risk_level, high_risk.risk_level);
    }

    #[test]
    fn test_ethical_concern_severity_ranges() {
        let severities = vec![1u8, 2u8, 3u8, 4u8, 5u8];

        for severity in severities {
            let concern = EthicalConcern {
                concern_type: EthicalConcernType::Safety,
                description: "Test".to_string(),
                severity,
                mitigation: "Test".to_string(),
            };

            assert_eq!(concern.severity, severity);
            assert!(concern.severity >= 1 && concern.severity <= 5);
        }
    }

    #[test]
    fn test_ethics_analysis_category_scores() {
        let analysis = EthicsAnalysis {
            overall_score: 75,
            category_scores: vec![
                ("Bias".to_string(), 80),
                ("Privacy".to_string(), 70),
                ("Safety".to_string(), 75),
            ],
            ethical_concerns: vec![],
            recommendations: vec![],
            risk_level: EthicsRiskLevel::Medium,
            requires_review: false,
            concerns_count: 0,
            recommendations_count: 0,
        };

        assert_eq!(analysis.category_scores.len(), 3);
        assert_eq!(analysis.category_scores[0].1, 80);
        assert_eq!(analysis.category_scores[1].1, 70);
        assert_eq!(analysis.category_scores[2].1, 75);
    }

    #[test]
    fn test_ethics_analysis_requires_review_logic() {
        let critical_risk = EthicsAnalysis {
            overall_score: 30,
            category_scores: vec![],
            ethical_concerns: vec![],
            recommendations: vec![],
            risk_level: EthicsRiskLevel::Critical,
            requires_review: true,
            concerns_count: 0,
            recommendations_count: 0,
        };

        assert!(critical_risk.requires_review);
        assert_eq!(critical_risk.risk_level, EthicsRiskLevel::Critical);
    }
}
