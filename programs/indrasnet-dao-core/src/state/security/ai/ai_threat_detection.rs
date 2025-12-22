//! Advanced AI Threat Detection System
//! 
//! Comprehensive AI-specific threat detection system.
//! Detects adversarial attacks, model poisoning, data poisoning, model evasion,
//! AI-generated threats, and other AI-specific security risks.
//! Integrates with EDR, Security Analytics, SIEM, and Emergency Response.
//! 
//! Hybrid model: Off-chain AI threat analysis and detection, on-chain metadata storage
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Only detection rules, threat metadata, and configuration
//! - OFF-CHAIN: All actual AI threat detection, ML-based analysis, pattern matching
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-security/src/security/ai_threat_detection/
//! Combined from: part_1.rs, part_2.rs, part_3.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;
// Note: Audit module not available in github-mvp, using simplified version
// TODO: Implement audit module or remove audit dependencies

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// AI threat type classification
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AIThreatType {
    /// Adversarial attack on AI model
    AdversarialAttack,
    /// Model poisoning attack
    ModelPoisoning,
    /// Data poisoning attack
    DataPoisoning,
    /// Model evasion attempt
    ModelEvasion,
    /// AI-generated malicious content
    AIGeneratedThreat,
    /// Model inversion attack
    ModelInversion,
    /// Membership inference attack
    MembershipInference,
    /// Model extraction attack
    ModelExtraction,
    /// Training data extraction
    TrainingDataExtraction,
    /// Unknown AI threat
    Unknown,
}

impl Space for AIThreatType {
    const INIT_SPACE: usize = 1;
}

/// AI threat severity
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AIThreatSeverity {
    /// Critical - immediate response required
    Critical,
    /// High - response within minutes
    High,
    /// Medium - response within hours
    Medium,
    /// Low - response within days
    Low,
}

impl Space for AIThreatSeverity {
    const INIT_SPACE: usize = 1;
}

/// AI threat detection status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AIThreatDetectionStatus {
    /// Threat detected
    Detected,
    /// Threat being analyzed
    Analyzing,
    /// Threat confirmed
    Confirmed,
    /// Threat requires response
    RequiresResponse,
    /// Threat mitigated
    Mitigated,
    /// False positive
    FalsePositive,
    /// Threat resolved
    Resolved,
}

impl Space for AIThreatDetectionStatus {
    const INIT_SPACE: usize = 1;
}

/// AI model context (Non-account struct)
/// 
/// ON-CHAIN: Stores model metadata
/// OFF-CHAIN: Actual model analysis happens off-chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct AIModelContext {
    /// Model identifier
    pub model_id: String,
    /// Model type/version
    pub model_type: String,
    /// Model owner/authority
    pub model_owner: Pubkey,
}

impl Space for AIModelContext {
    const INIT_SPACE: usize = 
        4 + 200 + // model_id: String
        4 + 100 + // model_type: String
        32; // model_owner: Pubkey
}

/// AI threat detection pattern (Non-account struct)
/// 
/// ON-CHAIN: Stores pattern definition
/// OFF-CHAIN: Actual pattern matching and ML analysis happens off-chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct AIThreatDetectionPattern {
    /// Threat type pattern
    pub threat_type: AIThreatType,
    /// Model pattern (wildcard supported)
    pub model_pattern: String,
    /// Input pattern (wildcard supported)
    pub input_pattern: String,
    /// Confidence threshold (0-10000 basis points)
    pub confidence_threshold: u64,
    /// Time window in seconds
    pub time_window: i64,
}

impl Space for AIThreatDetectionPattern {
    const INIT_SPACE: usize = 
        1 + // threat_type: AIThreatType
        4 + 200 + // model_pattern: String
        4 + 200 + // input_pattern: String
        8 + // confidence_threshold: u64
        8; // time_window: i64
}

/// AI Threat Detection Rule
/// 
/// Defines rules for detecting AI-specific threats
/// 
/// ON-CHAIN: Stores rule definition
/// OFF-CHAIN: Actual threat detection and ML analysis happens off-chain
#[account]
#[derive(InitSpace)]
pub struct AIThreatDetectionRule {
    /// Rule ID
    pub rule_id: u64,
    /// Rule name
    #[max_len(100)]
    pub name: String,
    /// Rule description
    #[max_len(500)]
    pub description: String,
    /// Detection patterns
    #[max_len(10)]
    pub patterns: Vec<AIThreatDetectionPattern>,
    /// Threat severity when triggered
    pub threat_severity: AIThreatSeverity,
    /// Whether rule is active
    pub is_active: bool,
    /// Created timestamp
    pub created_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl AIThreatDetectionRule {
    /// Maximum rule name length
    pub const MAX_NAME_LENGTH: usize = 100;
    /// Maximum description length
    pub const MAX_DESCRIPTION_LENGTH: usize = 500;
    /// Maximum patterns per rule
    pub const MAX_PATTERNS: usize = 10;
    /// Maximum time window (24 hours)
    pub const MAX_TIME_WINDOW: i64 = 86400;
    /// Maximum confidence score (100%)
    pub const MAX_CONFIDENCE_SCORE: u64 = 10000;
}

/// AI Threat Detection
/// 
/// Represents a detected AI-specific threat
/// 
/// ON-CHAIN: Stores detection metadata
/// OFF-CHAIN: Actual threat detection and analysis happens off-chain
#[account]
#[derive(InitSpace)]
pub struct AIThreatDetection {
    /// Detection ID
    pub detection_id: u64,
    /// Threat type
    pub threat_type: AIThreatType,
    /// Threat severity
    pub threat_severity: AIThreatSeverity,
    /// Detection rule that triggered this detection
    pub rule_id: u64,
    /// AI model context
    pub model_context: AIModelContext,
    /// Detection description
    #[max_len(1000)]
    pub description: String,
    /// Related event IDs
    #[max_len(50)]
    pub related_event_ids: Vec<u64>,
    /// Detection confidence score (0-10000 basis points)
    pub confidence_score: u64,
    /// Status
    pub status: AIThreatDetectionStatus,
    /// Associated SIEM incident ID (if created)
    pub siem_incident_id: Option<u64>,
    /// Associated crisis event ID (if created)
    pub crisis_event_id: Option<u64>,
    /// Detected timestamp
    pub detected_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
    /// Resolved timestamp (if resolved)
    pub resolved_at: Option<i64>,
    /// Resolution notes
    #[max_len(500)]
    pub resolution_notes: Option<String>,
    /// Bump seed for PDA
    pub bump: u8,
}

impl AIThreatDetection {
    /// Maximum description length
    pub const MAX_DESCRIPTION_LENGTH: usize = 1000;
    /// Maximum related events
    pub const MAX_RELATED_EVENTS: usize = 50;
    /// Maximum resolution notes length
    pub const MAX_RESOLUTION_NOTES_LENGTH: usize = 500;
    /// Maximum confidence score (100%)
    pub const MAX_CONFIDENCE_SCORE: u64 = 10000;
}

/// AI Threat Detection Configuration
/// 
/// Global AI threat detection system configuration
#[account]
#[derive(InitSpace)]
pub struct AIThreatDetectionConfig {
    /// Configuration ID
    pub config_id: u64,
    /// Authority who can update configuration
    pub authority: Pubkey,
    /// Whether AI threat detection is enabled
    pub is_enabled: bool,
    /// Minimum confidence score to alert (0-10000 basis points)
    pub min_confidence_score: u64,
    /// Minimum severity to process
    pub min_severity: AIThreatSeverity,
    /// Auto-escalate to crisis management
    pub auto_escalate_to_crisis: bool,
    /// Created timestamp
    pub created_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl AIThreatDetectionConfig {
    /// Maximum confidence score (100%)
    pub const MAX_CONFIDENCE_SCORE: u64 = 10000;
}

// ============================================================================
// ON-CHAIN MANAGER (Validation and State Management)
// ============================================================================

/// AI Threat Detection Manager for handling AI threat detection operations
/// 
/// ON-CHAIN: Validates inputs, manages state, records detections
/// OFF-CHAIN: Actual AI threat detection, ML analysis, pattern matching (see OFF-CHAIN section below)
pub struct AIThreatDetectionManager;

impl AIThreatDetectionManager {
    /// Simple hash function for BPF compatibility
    /// 
    /// WARNING: This is NOT a cryptographic hash function!
    /// For production, use proper cryptographic hashing OFF-CHAIN.
    /// This is only for on-chain storage and indexing.
    pub fn simple_hash(data: &[u8]) -> [u8; 32] {
        // Simple hash implementation for BPF (no std)
        // This is NOT cryptographically secure - only for indexing/storage
        let mut result = [0u8; 32];
        
        // Simple XOR-based hash (not secure, only for indexing)
        for (i, &byte) in data.iter().enumerate() {
            result[i % 32] ^= byte;
        }
        
        // Add some mixing
        for i in 0..32 {
            result[i] = result[i].wrapping_add((i as u8).wrapping_mul(7));
        }
        
        result
    }
    
    /// Create AI threat detection rule
    /// 
    /// ON-CHAIN: Validates and stores rule definition
    /// OFF-CHAIN: Rule matching and ML-based threat detection happens off-chain
    pub fn create_detection_rule(
        rule_id: u64,
        name: String,
        description: String,
        patterns: Vec<AIThreatDetectionPattern>,
        threat_severity: AIThreatSeverity,
        timestamp: i64,
        bump: u8,
    ) -> Result<AIThreatDetectionRule> {
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= AIThreatDetectionRule::MAX_NAME_LENGTH, IndrasError::InvalidInput);
        require!(description.len() <= AIThreatDetectionRule::MAX_DESCRIPTION_LENGTH, IndrasError::InvalidInput);
        require!(!patterns.is_empty(), IndrasError::InvalidInput);
        require!(patterns.len() <= AIThreatDetectionRule::MAX_PATTERNS, IndrasError::InvalidInput);
        
        // Validate patterns
        for pattern in &patterns {
            require!(pattern.time_window > 0, IndrasError::InvalidInput);
            require!(pattern.time_window <= AIThreatDetectionRule::MAX_TIME_WINDOW, IndrasError::InvalidInput);
            require!(pattern.confidence_threshold <= AIThreatDetectionRule::MAX_CONFIDENCE_SCORE, IndrasError::InvalidInput);
        }
        
        Ok(AIThreatDetectionRule {
            rule_id,
            name,
            description,
            patterns,
            threat_severity,
            is_active: true,
            created_at: timestamp,
            updated_at: timestamp,
            bump,
        })
    }
    
    /// Create AI threat detection
    /// 
    /// NOTE: Actual AI threat detection happens OFF-CHAIN.
    /// This function records the detection result on-chain.
    /// 
    /// ON-CHAIN: Validates and stores detection metadata
    /// OFF-CHAIN: Must detect threat using ML models before calling this
    pub fn create_threat_detection(
        detection_id: u64,
        threat_type: AIThreatType,
        threat_severity: AIThreatSeverity,
        rule_id: u64,
        model_context: AIModelContext,
        description: String,
        related_event_ids: Vec<u64>,
        confidence_score: u64,
        timestamp: i64,
        bump: u8,
    ) -> Result<AIThreatDetection> {
        require!(!description.is_empty(), IndrasError::InvalidInput);
        require!(description.len() <= AIThreatDetection::MAX_DESCRIPTION_LENGTH, IndrasError::InvalidInput);
        require!(related_event_ids.len() <= AIThreatDetection::MAX_RELATED_EVENTS, IndrasError::InvalidInput);
        require!(confidence_score <= AIThreatDetection::MAX_CONFIDENCE_SCORE, IndrasError::InvalidInput);
        
        Ok(AIThreatDetection {
            detection_id,
            threat_type,
            threat_severity,
            rule_id,
            model_context,
            description,
            related_event_ids,
            confidence_score,
            status: AIThreatDetectionStatus::Detected,
            siem_incident_id: None,
            crisis_event_id: None,
            detected_at: timestamp,
            updated_at: timestamp,
            resolved_at: None,
            resolution_notes: None,
            bump,
        })
    }
    
    /// Update threat detection status
    pub fn update_detection_status(
        detection: &mut AIThreatDetection,
        status: AIThreatDetectionStatus,
        timestamp: i64,
    ) -> Result<()> {
        detection.status = status;
        detection.updated_at = timestamp;
        Ok(())
    }
    
    /// Resolve threat detection
    pub fn resolve_threat_detection(
        detection: &mut AIThreatDetection,
        resolution_notes: String,
        timestamp: i64,
    ) -> Result<()> {
        require!(resolution_notes.len() <= AIThreatDetection::MAX_RESOLUTION_NOTES_LENGTH, IndrasError::InvalidInput);
        detection.status = AIThreatDetectionStatus::Resolved;
        detection.resolved_at = Some(timestamp);
        detection.resolution_notes = Some(resolution_notes);
        detection.updated_at = timestamp;
        Ok(())
    }
    
    /// Create AI threat detection configuration
    pub fn create_config(
        config_id: u64,
        authority: Pubkey,
        min_confidence_score: u64,
        min_severity: AIThreatSeverity,
        auto_escalate_to_crisis: bool,
        timestamp: i64,
        bump: u8,
    ) -> Result<AIThreatDetectionConfig> {
        require!(min_confidence_score <= AIThreatDetectionConfig::MAX_CONFIDENCE_SCORE, IndrasError::InvalidInput);
        
        Ok(AIThreatDetectionConfig {
            config_id,
            authority,
            is_enabled: true,
            min_confidence_score,
            min_severity,
            auto_escalate_to_crisis,
            created_at: timestamp,
            updated_at: timestamp,
            bump,
        })
    }
    
    /// Check if threat matches detection pattern
    /// 
    /// ON-CHAIN: Basic pattern matching (simple wildcard support)
    /// OFF-CHAIN: Complex ML-based pattern matching, adversarial detection happens off-chain
    pub fn matches_pattern(
        threat_type: AIThreatType,
        model_id: &str,
        input_pattern: &str,
        pattern: &AIThreatDetectionPattern,
    ) -> bool {
        // Check threat type
        if threat_type != pattern.threat_type {
            return false;
        }
        
        // Check model pattern
        if !Self::matches_string_pattern(model_id, &pattern.model_pattern) {
            return false;
        }
        
        // Check input pattern
        if !Self::matches_string_pattern(input_pattern, &pattern.input_pattern) {
            return false;
        }
        
        true
    }
    
    /// Simple string pattern matching (wildcard support)
    /// 
    /// ON-CHAIN: Basic wildcard matching only
    /// OFF-CHAIN: Complex regex, ML-based pattern matching happens off-chain
    fn matches_string_pattern(text: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        
        if pattern.ends_with("*") {
            let prefix = &pattern[..pattern.len() - 1];
            return text.starts_with(prefix);
        }
        
        text == pattern
    }
}

// ============================================================================
// AUDIT INTEGRATION
// ============================================================================

/// Audit event emitted for AI threat detection operations
#[event]
pub struct AIThreatDetectionAuditEvent {
    pub user: Pubkey,
    pub action: String,
    pub success: bool,
    pub resource: String,
    pub details: String,
    pub timestamp: i64,
}

/// Create audit event for AI threat detection operation
///
/// Emits an Anchor event that downstream indexers can persist.
/// This avoids the dependency on the v2 audit module while still
/// providing traceability.
pub fn create_ai_threat_detection_audit_event(
    user: Pubkey,
    action: &str,
    success: bool,
    resource: &str,
    details: &str,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    emit!(AIThreatDetectionAuditEvent {
        user,
        action: action.to_string(),
        success,
        resource: resource.to_string(),
        details: details.to_string(),
        timestamp: now,
    });
    Ok(())
}

// ============================================================================
// OFF-CHAIN IMPLEMENTATION NOTES
// ============================================================================

/*

OFF-CHAIN AI THREAT DETECTION IMPLEMENTATION (Standard Rust, not Anchor)

The following operations MUST be performed off-chain:

1. **AI Threat Detection:**
   ```rust
   // offchain_services/src/ai_threat_detection.rs
   use ml_models::threat_detection;
   
   pub async fn detect_ai_threats_offchain(
       model_inputs: &[ModelInput],
       rules: &[AIThreatDetectionRule],
   ) -> Result<Vec<AIThreatDetection>> {
       // 1. Analyze inputs for adversarial patterns
       // 2. Check for model poisoning indicators
       // 3. Detect data poisoning attempts
       // 4. Use ML models for threat classification
       // 5. Calculate confidence scores
       // 6. Create threat detections
       // 7. Return detections for on-chain storage
   }
   ```

2. **Adversarial Detection:**
   ```rust
   pub async fn detect_adversarial_attack(
       model: &Model,
       input: &[f32],
   ) -> Result<(bool, u64)> {
       // 1. Analyze input for adversarial perturbations
       // 2. Check gradient-based attacks
       // 3. Detect evasion attempts
       // 4. Calculate confidence score
   }
   ```

3. **Pattern Matching:**
   ```rust
   use regex::Regex;
   use ml_models::pattern_recognition;
   
   pub fn match_pattern_advanced(text: &str, pattern: &str) -> bool {
       // Complex regex matching
       // ML-based pattern recognition
       // Behavioral analysis
   }
   ```

Required off-chain dependencies:
- ML models for threat detection (adversarial detection, poisoning detection)
- Pattern recognition libraries
- Statistical analysis tools
- Model security frameworks

*/

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_ai_model_context_direct(
        model_id: String,
        model_type: String,
        model_owner: Pubkey,
    ) -> AIModelContext {
        AIModelContext {
            model_id,
            model_type,
            model_owner,
        }
    }

    fn create_test_ai_threat_detection_pattern_direct(
        threat_type: AIThreatType,
        model_pattern: String,
        input_pattern: String,
        confidence_threshold: u64,
        time_window: i64,
    ) -> AIThreatDetectionPattern {
        AIThreatDetectionPattern {
            threat_type,
            model_pattern,
            input_pattern,
            confidence_threshold,
            time_window,
        }
    }

    fn create_test_ai_threat_detection_direct(
        detection_id: u64,
        threat_type: AIThreatType,
        threat_severity: AIThreatSeverity,
        rule_id: u64,
        model_context: AIModelContext,
        description: String,
        related_event_ids: Vec<u64>,
        confidence_score: u64,
        status: AIThreatDetectionStatus,
        detected_at: i64,
        updated_at: i64,
        bump: u8,
    ) -> AIThreatDetection {
        AIThreatDetection {
            detection_id,
            threat_type,
            threat_severity,
            rule_id,
            model_context,
            description,
            related_event_ids,
            confidence_score,
            status,
            siem_incident_id: None,
            crisis_event_id: None,
            detected_at,
            updated_at,
            resolved_at: None,
            resolution_notes: None,
            bump,
        }
    }

    #[test]
    fn test_ai_threat_type_enum() {
        assert_eq!(AIThreatType::AdversarialAttack, AIThreatType::AdversarialAttack);
        assert_ne!(AIThreatType::AdversarialAttack, AIThreatType::ModelPoisoning);
        assert_eq!(AIThreatType::INIT_SPACE, 1);
    }

    #[test]
    fn test_ai_threat_severity_enum() {
        assert_eq!(AIThreatSeverity::Critical, AIThreatSeverity::Critical);
        assert_ne!(AIThreatSeverity::Critical, AIThreatSeverity::Low);
        assert_eq!(AIThreatSeverity::INIT_SPACE, 1);
    }

    #[test]
    fn test_ai_threat_detection_status_enum() {
        assert_eq!(AIThreatDetectionStatus::Detected, AIThreatDetectionStatus::Detected);
        assert_ne!(AIThreatDetectionStatus::Detected, AIThreatDetectionStatus::Resolved);
        assert_eq!(AIThreatDetectionStatus::INIT_SPACE, 1);
    }

    #[test]
    fn test_ai_model_context_creation() {
        let owner = Pubkey::from([1u8; 32]);
        let context = create_test_ai_model_context_direct(
            "model-123".to_string(),
            "GPT-4".to_string(),
            owner,
        );

        assert_eq!(context.model_id, "model-123");
        assert_eq!(context.model_type, "GPT-4");
        assert_eq!(context.model_owner, owner);
        assert_eq!(AIModelContext::INIT_SPACE, 4 + 200 + 4 + 100 + 32);
    }

    #[test]
    fn test_ai_threat_detection_pattern_creation() {
        let pattern = create_test_ai_threat_detection_pattern_direct(
            AIThreatType::AdversarialAttack,
            "model-*".to_string(),
            "input-*".to_string(),
            7500,
            3600,
        );

        assert_eq!(pattern.threat_type, AIThreatType::AdversarialAttack);
        assert_eq!(pattern.model_pattern, "model-*");
        assert_eq!(pattern.confidence_threshold, 7500);
        assert_eq!(pattern.time_window, 3600);
    }

    #[test]
    fn test_simple_hash() {
        let data = b"test data";
        let hash = AIThreatDetectionManager::simple_hash(data);
        
        assert_eq!(hash.len(), 32);
        
        let hash2 = AIThreatDetectionManager::simple_hash(data);
        assert_eq!(hash, hash2);
        
        let hash3 = AIThreatDetectionManager::simple_hash(b"different");
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_create_detection_rule() {
        let pattern = create_test_ai_threat_detection_pattern_direct(
            AIThreatType::AdversarialAttack,
            "model-*".to_string(),
            "*".to_string(),
            5000,
            3600,
        );

        let rule = AIThreatDetectionManager::create_detection_rule(
            1,
            "Test Rule".to_string(),
            "Test Description".to_string(),
            vec![pattern],
            AIThreatSeverity::High,
            1234567890,
            255,
        ).unwrap();

        assert_eq!(rule.rule_id, 1);
        assert_eq!(rule.name, "Test Rule");
        assert_eq!(rule.patterns.len(), 1);
        assert!(rule.is_active);
    }

    #[test]
    fn test_create_threat_detection() {
        let owner = Pubkey::from([1u8; 32]);
        let context = create_test_ai_model_context_direct(
            "model-123".to_string(),
            "GPT-4".to_string(),
            owner,
        );

        let detection = AIThreatDetectionManager::create_threat_detection(
            1,
            AIThreatType::AdversarialAttack,
            AIThreatSeverity::High,
            1,
            context.clone(),
            "Test threat detected".to_string(),
            vec![100, 200],
            8500,
            1234567890,
            255,
        ).unwrap();

        assert_eq!(detection.detection_id, 1);
        assert_eq!(detection.threat_type, AIThreatType::AdversarialAttack);
        assert_eq!(detection.confidence_score, 8500);
        assert_eq!(detection.status, AIThreatDetectionStatus::Detected);
        assert_eq!(detection.model_context.model_id, "model-123");
    }

    #[test]
    fn test_update_detection_status() {
        let owner = Pubkey::from([1u8; 32]);
        let context = create_test_ai_model_context_direct(
            "model-123".to_string(),
            "GPT-4".to_string(),
            owner,
        );

        let mut detection = create_test_ai_threat_detection_direct(
            1, AIThreatType::AdversarialAttack, AIThreatSeverity::High,
            1, context, "Test".to_string(), vec![],
            8500, AIThreatDetectionStatus::Detected,
            1234567890, 1234567890, 255,
        );

        AIThreatDetectionManager::update_detection_status(
            &mut detection,
            AIThreatDetectionStatus::Confirmed,
            1234568000,
        ).unwrap();

        assert_eq!(detection.status, AIThreatDetectionStatus::Confirmed);
        assert_eq!(detection.updated_at, 1234568000);
    }

    #[test]
    fn test_resolve_threat_detection() {
        let owner = Pubkey::from([1u8; 32]);
        let context = create_test_ai_model_context_direct(
            "model-123".to_string(),
            "GPT-4".to_string(),
            owner,
        );

        let mut detection = create_test_ai_threat_detection_direct(
            1, AIThreatType::AdversarialAttack, AIThreatSeverity::High,
            1, context, "Test".to_string(), vec![],
            8500, AIThreatDetectionStatus::Confirmed,
            1234567890, 1234567890, 255,
        );

        AIThreatDetectionManager::resolve_threat_detection(
            &mut detection,
            "Resolved as false positive".to_string(),
            1234569000,
        ).unwrap();

        assert_eq!(detection.status, AIThreatDetectionStatus::Resolved);
        assert!(detection.resolved_at.is_some());
        assert!(detection.resolution_notes.is_some());
    }

    #[test]
    fn test_create_config() {
        let authority = Pubkey::from([1u8; 32]);
        let config = AIThreatDetectionManager::create_config(
            1,
            authority,
            5000,
            AIThreatSeverity::Medium,
            true,
            1234567890,
            255,
        ).unwrap();

        assert_eq!(config.config_id, 1);
        assert_eq!(config.authority, authority);
        assert_eq!(config.min_confidence_score, 5000);
        assert!(config.is_enabled);
        assert!(config.auto_escalate_to_crisis);
    }

    #[test]
    fn test_matches_pattern_exact() {
        let pattern = create_test_ai_threat_detection_pattern_direct(
            AIThreatType::AdversarialAttack,
            "model-123".to_string(),
            "input-456".to_string(),
            5000,
            3600,
        );

        assert!(AIThreatDetectionManager::matches_pattern(
            AIThreatType::AdversarialAttack,
            "model-123",
            "input-456",
            &pattern,
        ));
    }

    #[test]
    fn test_matches_pattern_wildcard_all() {
        let pattern = create_test_ai_threat_detection_pattern_direct(
            AIThreatType::AdversarialAttack,
            "*".to_string(),
            "*".to_string(),
            5000,
            3600,
        );

        assert!(AIThreatDetectionManager::matches_pattern(
            AIThreatType::AdversarialAttack,
            "any-model",
            "any-input",
            &pattern,
        ));
    }

    #[test]
    fn test_matches_pattern_prefix_wildcard() {
        let pattern = create_test_ai_threat_detection_pattern_direct(
            AIThreatType::AdversarialAttack,
            "model-*".to_string(),
            "input-*".to_string(),
            5000,
            3600,
        );

        assert!(AIThreatDetectionManager::matches_pattern(
            AIThreatType::AdversarialAttack,
            "model-123",
            "input-456",
            &pattern,
        ));

        assert!(!AIThreatDetectionManager::matches_pattern(
            AIThreatType::AdversarialAttack,
            "other-123",
            "input-456",
            &pattern,
        ));
    }

    #[test]
    fn test_matches_pattern_type_mismatch() {
        let pattern = create_test_ai_threat_detection_pattern_direct(
            AIThreatType::ModelPoisoning,
            "*".to_string(),
            "*".to_string(),
            5000,
            3600,
        );

        assert!(!AIThreatDetectionManager::matches_pattern(
            AIThreatType::AdversarialAttack,
            "model-123",
            "input-456",
            &pattern,
        ));
    }

    #[test]
    fn test_constants() {
        assert_eq!(AIThreatDetectionRule::MAX_NAME_LENGTH, 100);
        assert_eq!(AIThreatDetectionRule::MAX_DESCRIPTION_LENGTH, 500);
        assert_eq!(AIThreatDetectionRule::MAX_PATTERNS, 10);
        assert_eq!(AIThreatDetectionRule::MAX_TIME_WINDOW, 86400);
        assert_eq!(AIThreatDetectionRule::MAX_CONFIDENCE_SCORE, 10000);
        assert_eq!(AIThreatDetection::MAX_DESCRIPTION_LENGTH, 1000);
        assert_eq!(AIThreatDetection::MAX_RELATED_EVENTS, 50);
    }
}
