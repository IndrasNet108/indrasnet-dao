//! State structures for the IndrasNet DAO AI program

pub mod ai_analysis;
pub mod metaphenomenon;
pub mod security_analysis;

// Re-export types
pub use ai_analysis::{
    AIAnalysis, AIReviewDecision, RejectionReason, ApprovalReason
};
pub use metaphenomenon::Metaphenomenon;
pub use security_analysis::{SecurityAnalysisEnhancement, RiskLevel};
