//! AI Security Modules
//!
//! Advanced AI security modules integrated from indrasnet-dao-v2

pub mod ai_threat_detection;
pub mod security_risk_assessment_advanced;
pub mod predictive_security;

// Re-export types for external use
pub use ai_threat_detection::{
    AIThreatType,
    AIThreatSeverity,
    AIThreatDetectionStatus,
    AIModelContext,
    AIThreatDetection,
    AIThreatDetectionConfig,
    AIThreatDetectionRule,
    AIThreatDetectionManager,
};
