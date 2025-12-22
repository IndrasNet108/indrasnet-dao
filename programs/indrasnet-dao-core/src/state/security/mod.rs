//! Advanced Security Systems Module
//!
//! INNOVATIONS: Advanced security systems
//! 1. Adaptive Security Scoring - dynamic risk assessment
//! 2. Progressive Security Unlocking - progressive unlocking
//! 3. Behavioral Pattern Analysis - behavioral pattern analysis

pub mod adaptive_scoring;
pub mod progressive_unlock;
pub mod behavioral_analysis;
pub mod compliance_proof;

// AI security modules (integrated from v2)
pub mod ai;

// Re-export types
pub use adaptive_scoring::{
    SecurityRiskFactors,
    OperationSecurityProfile,
    MIN_SECURITY_SCORE,
    HIGH_SECURITY_SCORE,
    CRITICAL_SECURITY_SCORE,
};
pub use progressive_unlock::{
    UnlockStage,
    ProgressiveUnlock,
};
pub use behavioral_analysis::{
    BehavioralPattern,
    LOW_ANOMALY_THRESHOLD,
    MEDIUM_ANOMALY_THRESHOLD,
    HIGH_ANOMALY_THRESHOLD,
};
pub use compliance_proof::{
    ComplianceProof,
    MIN_COMPLIANCE_SCORE,
};
