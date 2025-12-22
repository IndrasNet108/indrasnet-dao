//! AI Analysis state structure
//!
//! AI analysis of idea for compliance with DAO norms:
//! - Ethical requirements (ethics_compliance_score)
//! - Legal requirements (legal_compliance_score)
//! - Innovation (impact_score + feasibility_score)
//! - Uniqueness (uniqueness_score) - CRITICAL!
//! - Feasibility (feasibility_score with artifacts)

use anchor_lang::prelude::*;

/// AI analysis decision
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug)]
pub enum AIReviewDecision {
    Approve,    // Approve
    Reject,     // Reject
    Appeal,     // Requires appeal
}

impl anchor_lang::Space for AIReviewDecision {
    const INIT_SPACE: usize = 1;
}

/// Rejection reason
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug)]
pub enum RejectionReason {
    NotCompliantWithCharter,      // Does not comply with Charter
    NotCompliantWithGovernance,   // Does not comply with governance rules
    NotCompliantWithCodeOfConduct, // Does not comply with code of conduct
    NotCompliantWithEthics,       // Does not comply with ethical principles
    NotCompliantWithLegal,        // Does not comply with legislation
    NotCompliantWithTechnical,    // Does not comply with technical standards
    Duplicate,                    // Duplicate (low uniqueness)
    NotFeasible,                  // Technically not feasible
    LowImpact,                    // Low impact potential
    NotInnovative,                // Not innovative
    NotUnique,                    // Not unique
    FeasibilityNotProven,         // Feasibility not proven (no artifacts)
    Incomplete,                   // Incomplete information
    Other,                        // Other
}

impl anchor_lang::Space for RejectionReason {
    const INIT_SPACE: usize = 1;
}

/// Approval reason
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, Debug)]
pub enum ApprovalReason {
    UniqueIdea,                   // Unique idea
    HighImpact,                   // High impact potential
    CompliantWithCharter,         // Complies with Charter
    CompliantWithGovernance,      // Complies with governance rules
    CompliantWithCodeOfConduct,   // Complies with code of conduct
    CompliantWithEthics,          // Complies with ethical principles
    CompliantWithLegal,           // Complies with legislation
    CompliantWithTechnical,       // Complies with technical standards
    TechnicallyFeasible,          // Technically feasible
    Innovative,                   // Innovative
    FeasibilityProven,            // Feasibility proven (has artifacts)
    WellDocumented,               // Well documented
    Other,                        // Other
}

impl anchor_lang::Space for ApprovalReason {
    const INIT_SPACE: usize = 1;
}

/// AI analysis of idea
///
/// Checks idea for compliance with DAO norms:
/// - Ethical requirements (ethics_compliance_score >= 50)
/// - Legal requirements (legal_compliance_score >= 50)
/// - Innovation (impact_score >= 70, feasibility_score >= 70)
/// - Uniqueness (uniqueness_score >= 70) - CRITICAL!
/// - Feasibility (feasibility_score >= 70 with artifacts)
#[account]
#[derive(InitSpace)]
pub struct AIAnalysis {
    pub idea_id: u64,
    pub reviewer: Pubkey,                    // AI or human
    
    // Main decision
    /// OPTIONAL: Contract can also determine itself
    pub decision: Option<AIReviewDecision>,
    
    // Detailed scores by criteria
    pub charter_compliance_score: u8,        // Charter compliance (0-100)
    pub governance_compliance_score: u8,     // Governance rules compliance (0-100)
    pub ethics_compliance_score: u8,         // Ethics compliance (0-100) - CRITICAL!
    pub legal_compliance_score: u8,          // Legal compliance (0-100) - CRITICAL!
    pub technical_feasibility_score: u8,     // Technical feasibility (0-100)
    pub uniqueness_score: u8,                // Uniqueness (0-100) - CRITICAL!
    pub impact_score: u8,                    // Impact potential / social significance (0-100)
    pub feasibility_score: u8,               // Feasibility with evidence (0-100) - CRITICAL!
    
    // Overall score
    /// OPTIONAL: Contract can compute itself
    pub overall_score: Option<u8>,           // Overall score (0-100)
    
    // Decision reasons
    #[max_len(10)]
    pub rejection_reasons: Vec<RejectionReason>,
    #[max_len(10)]
    pub approval_reasons: Vec<ApprovalReason>,
    
    // References to verified documents
    #[max_len(20)]
    pub checked_governance_docs: Vec<u64>,   // IDs of verified governance documents
    #[max_len(20)]
    pub checked_legal_frameworks: Vec<u64>,  // IDs of verified legal frameworks
    #[max_len(20)]
    pub checked_ethics_frameworks: Vec<u64>, // IDs of verified ethics frameworks
    
    // Detailed feedback
    #[max_len(2000)]
    pub detailed_feedback: String,
    
    // Artifacts hash (feasibility proof)
    // Stored off-chain (IPFS, Arweave), hash on-chain
    pub artifacts_hash: Option<[u8; 32]>,    // Artifacts hash
    pub artifacts_verified: bool,            // Artifacts verified
    
    // Hashing and verification (MVP v1.0.0 specification)
    /// Hash of input data (idea_text): sha256(idea_text)
    /// REQUIRED: Critical for verification and audit
    pub input_hash: [u8; 32],
    /// Hash of output data (json_response): sha256(json_response)
    /// REQUIRED: Critical for verification and audit
    pub output_hash: [u8; 32],
    /// AI model identifier (e.g., "gemini-2.5-flash")
    /// REQUIRED: Critical for verification and audit
    #[max_len(100)]
    pub model_id: String,
    /// AI model version (e.g., "1.0.0")
    /// REQUIRED: Ensures determinism
    #[max_len(50)]
    pub model_version: String,
    /// Model hash: SHA256(model_id || model_version || model_weights_hash)
    /// SEC-INV-7: Optional for MVP, required if ModelRegistry provided
    pub model_hash: Option<[u8; 32]>,
    /// Analyzer public key (analyzer_pubkey)
    /// REQUIRED: Governance requirement
    pub analyzer_pubkey: Pubkey,
    /// Analyzer signature: ed25519(bundle_hash, analyzer_key)
    /// bundle_hash = sha256(input_hash || output_hash || model_id || model_version)
    /// REQUIRED: Without signature anchor is empty
    pub analyzer_signature: [u8; 64],
    /// Threshold profile for decision making (enum for different profiles)
    /// OPTIONAL: Can be defaulted
    pub thresholds_profile: Option<u8>,
    
    // Metadata
    pub created_at: i64,
    pub analysis_duration_ms: u64,          // Analysis duration in milliseconds
    
    // Replay attack protection (REQUIRED)
    /// Unique nonce per analysis (replay attack protection)
    pub nonce: u64,
    /// Optional expiration timestamp
    pub expires_at: Option<i64>,
    
    pub bump: u8,
}

impl AIAnalysis {
    /// Checks if idea meets ethical requirements
    pub fn is_ethics_compliant(&self) -> bool {
        self.ethics_compliance_score >= 50
    }
    
    /// Checks if idea meets legal requirements
    pub fn is_legal_compliant(&self) -> bool {
        self.legal_compliance_score >= 50
    }
    
    /// Checks if idea is innovative
    /// Innovation = (impact_score + feasibility_score) / 2 >= 70
    pub fn is_innovative(&self) -> bool {
        let innovation_score = (self.impact_score as u16 + self.feasibility_score as u16) / 2;
        innovation_score >= 70
    }
    
    /// Checks if idea is unique
    /// CRITICAL: uniqueness_score >= 70 for authorship
    pub fn is_unique(&self) -> bool {
        self.uniqueness_score >= 70
    }
    
    /// Checks if feasibility is proven
    /// Feasibility = feasibility_score >= 70 AND artifacts_verified = true
    pub fn is_feasibility_proven(&self) -> bool {
        self.feasibility_score >= 70 && self.artifacts_verified
    }
    
    /// Checks if idea can get authorship
    /// Requirements for authorship:
    /// - Innovation (impact_score >= 70, feasibility_score >= 70)
    /// - Uniqueness (uniqueness_score >= 70)
    /// - Feasibility proven (feasibility_score >= 70, artifacts_verified = true)
    pub fn can_get_authorship(&self) -> bool {
        self.is_innovative() && self.is_unique() && self.is_feasibility_proven()
    }
    
    /// Checks if idea can enter mesh group
    /// Requirements:
    /// - Ethical requirements (ethics_compliance_score >= 50)
    /// - Legal requirements (legal_compliance_score >= 50)
    /// - Innovation (impact_score >= 70, feasibility_score >= 70)
    /// - Uniqueness (uniqueness_score >= 70)
    /// - Feasibility proven (feasibility_score >= 70, artifacts_verified = true)
    pub fn can_enter_mesh_group(&self) -> bool {
        self.is_ethics_compliant() 
            && self.is_legal_compliant() 
            && self.is_innovative() 
            && self.is_unique() 
            && self.is_feasibility_proven()
    }
    
    /// Get decision with default
    /// If decision = None, calculates based on scores
    pub fn get_decision(&self) -> AIReviewDecision {
        self.decision.unwrap_or_else(|| {
            // Calculate decision based on scores if not set
            if self.uniqueness_score < 70 {
                AIReviewDecision::Reject
            } else if self.is_ethics_compliant() 
                && self.is_legal_compliant() 
                && self.is_innovative() 
                && self.is_unique() 
                && self.is_feasibility_proven() {
                AIReviewDecision::Approve
            } else {
                AIReviewDecision::Reject
            }
        })
    }
    
    /// Get overall_score with default
    /// If overall_score = None, calculates based on weighted scores
    pub fn get_overall_score(&self) -> u8 {
        self.overall_score.unwrap_or_else(|| {
            // Calculate weighted score
            let weights = [15u32, 10u32, 10u32, 15u32, 10u32, 30u32, 10u32];
            let score_values = [
                self.charter_compliance_score,
                self.governance_compliance_score,
                self.ethics_compliance_score,
                self.legal_compliance_score,
                self.technical_feasibility_score,
                self.uniqueness_score,
                self.impact_score,
            ];
            
            let mut weighted_sum: u32 = 0;
            for (score, weight) in score_values.iter().zip(weights.iter()) {
                weighted_sum += (*score as u32) * weight;
            }
            
            (weighted_sum / 100) as u8
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_analysis() -> AIAnalysis {
        AIAnalysis {
            idea_id: 1,
            reviewer: create_test_pubkey(1),
            decision: None,
            charter_compliance_score: 80,
            governance_compliance_score: 75,
            ethics_compliance_score: 60,
            legal_compliance_score: 55,
            technical_feasibility_score: 70,
            uniqueness_score: 75,
            impact_score: 80,
            feasibility_score: 75,
            overall_score: None,
            rejection_reasons: Vec::new(),
            approval_reasons: Vec::new(),
            checked_governance_docs: Vec::new(),
            checked_legal_frameworks: Vec::new(),
            checked_ethics_frameworks: Vec::new(),
            detailed_feedback: String::new(),
            artifacts_hash: Some([1u8; 32]),
            artifacts_verified: true,
            input_hash: [2u8; 32],
            output_hash: [3u8; 32],
            model_id: "test-model".to_string(),
            model_version: "1.0.0".to_string(),
            model_hash: Some([4u8; 32]),
            analyzer_pubkey: create_test_pubkey(2),
            analyzer_signature: [5u8; 64],
            thresholds_profile: None,
            created_at: 1000,
            analysis_duration_ms: 500,
            nonce: 123,
            expires_at: None,
            bump: 255,
        }
    }

    #[test]
    fn test_ai_analysis_is_ethics_compliant() {
        let mut analysis = create_test_analysis();
        
        analysis.ethics_compliance_score = 50;
        assert!(analysis.is_ethics_compliant());
        
        analysis.ethics_compliance_score = 49;
        assert!(!analysis.is_ethics_compliant());
    }

    #[test]
    fn test_ai_analysis_is_legal_compliant() {
        let mut analysis = create_test_analysis();
        
        analysis.legal_compliance_score = 50;
        assert!(analysis.is_legal_compliant());
        
        analysis.legal_compliance_score = 49;
        assert!(!analysis.is_legal_compliant());
    }

    #[test]
    fn test_ai_analysis_is_innovative() {
        let mut analysis = create_test_analysis();
        
        // (80 + 75) / 2 = 77.5 >= 70
        analysis.impact_score = 80;
        analysis.feasibility_score = 75;
        assert!(analysis.is_innovative());
        
        // (60 + 70) / 2 = 65 < 70
        analysis.impact_score = 60;
        analysis.feasibility_score = 70;
        assert!(!analysis.is_innovative());
    }

    #[test]
    fn test_ai_analysis_is_unique() {
        let mut analysis = create_test_analysis();
        
        analysis.uniqueness_score = 70;
        assert!(analysis.is_unique());
        
        analysis.uniqueness_score = 69;
        assert!(!analysis.is_unique());
    }

    #[test]
    fn test_ai_analysis_is_feasibility_proven() {
        let mut analysis = create_test_analysis();
        
        analysis.feasibility_score = 70;
        analysis.artifacts_verified = true;
        assert!(analysis.is_feasibility_proven());
        
        analysis.feasibility_score = 69;
        assert!(!analysis.is_feasibility_proven());
        
        analysis.feasibility_score = 70;
        analysis.artifacts_verified = false;
        assert!(!analysis.is_feasibility_proven());
    }

    #[test]
    fn test_ai_analysis_can_get_authorship() {
        let mut analysis = create_test_analysis();
        
        // All requirements met
        analysis.impact_score = 80;
        analysis.feasibility_score = 75;
        analysis.uniqueness_score = 75;
        analysis.artifacts_verified = true;
        assert!(analysis.can_get_authorship());
        
        // Not innovative
        analysis.impact_score = 60;
        assert!(!analysis.can_get_authorship());
    }

    #[test]
    fn test_ai_analysis_can_enter_mesh_group() {
        let mut analysis = create_test_analysis();
        
        // All requirements met
        analysis.ethics_compliance_score = 60;
        analysis.legal_compliance_score = 55;
        analysis.impact_score = 80;
        analysis.feasibility_score = 75;
        analysis.uniqueness_score = 75;
        analysis.artifacts_verified = true;
        assert!(analysis.can_enter_mesh_group());
        
        // Not ethics compliant
        analysis.ethics_compliance_score = 49;
        assert!(!analysis.can_enter_mesh_group());
    }

    #[test]
    fn test_ai_analysis_get_decision_explicit() {
        let mut analysis = create_test_analysis();
        
        analysis.decision = Some(AIReviewDecision::Approve);
        assert_eq!(analysis.get_decision(), AIReviewDecision::Approve);
        
        analysis.decision = Some(AIReviewDecision::Reject);
        assert_eq!(analysis.get_decision(), AIReviewDecision::Reject);
    }

    #[test]
    fn test_ai_analysis_get_decision_computed_approve() {
        let mut analysis = create_test_analysis();
        analysis.decision = None;
        
        // All requirements met
        analysis.ethics_compliance_score = 60;
        analysis.legal_compliance_score = 55;
        analysis.impact_score = 80;
        analysis.feasibility_score = 75;
        analysis.uniqueness_score = 75;
        analysis.artifacts_verified = true;
        assert_eq!(analysis.get_decision(), AIReviewDecision::Approve);
    }

    #[test]
    fn test_ai_analysis_get_decision_computed_reject_low_uniqueness() {
        let mut analysis = create_test_analysis();
        analysis.decision = None;
        
        // Low uniqueness - should reject
        analysis.uniqueness_score = 69;
        assert_eq!(analysis.get_decision(), AIReviewDecision::Reject);
    }

    #[test]
    fn test_ai_analysis_get_decision_computed_reject_other() {
        let mut analysis = create_test_analysis();
        analysis.decision = None;
        
        // High uniqueness but not ethics compliant
        analysis.uniqueness_score = 75;
        analysis.ethics_compliance_score = 49;
        assert_eq!(analysis.get_decision(), AIReviewDecision::Reject);
    }

    #[test]
    fn test_ai_analysis_get_overall_score_explicit() {
        let mut analysis = create_test_analysis();
        
        analysis.overall_score = Some(85);
        assert_eq!(analysis.get_overall_score(), 85);
    }

    #[test]
    fn test_ai_analysis_get_overall_score_computed() {
        let mut analysis = create_test_analysis();
        analysis.overall_score = None;
        
        // Set all scores to 100 for easy calculation
        analysis.charter_compliance_score = 100;
        analysis.governance_compliance_score = 100;
        analysis.ethics_compliance_score = 100;
        analysis.legal_compliance_score = 100;
        analysis.technical_feasibility_score = 100;
        analysis.uniqueness_score = 100;
        analysis.impact_score = 100;
        
        // Weighted: (15+10+10+15+10+30+10) * 100 / 100 = 100
        assert_eq!(analysis.get_overall_score(), 100);
    }

    #[test]
    fn test_ai_review_decision_variants() {
        assert_eq!(AIReviewDecision::Approve, AIReviewDecision::Approve);
        assert_eq!(AIReviewDecision::Reject, AIReviewDecision::Reject);
        assert_eq!(AIReviewDecision::Appeal, AIReviewDecision::Appeal);
    }

    #[test]
    fn test_rejection_reason_variants() {
        assert_eq!(RejectionReason::NotCompliantWithCharter, RejectionReason::NotCompliantWithCharter);
        assert_eq!(RejectionReason::Duplicate, RejectionReason::Duplicate);
        assert_eq!(RejectionReason::NotUnique, RejectionReason::NotUnique);
    }

    #[test]
    fn test_approval_reason_variants() {
        assert_eq!(ApprovalReason::UniqueIdea, ApprovalReason::UniqueIdea);
        assert_eq!(ApprovalReason::HighImpact, ApprovalReason::HighImpact);
        assert_eq!(ApprovalReason::CompliantWithEthics, ApprovalReason::CompliantWithEthics);
    }
}
