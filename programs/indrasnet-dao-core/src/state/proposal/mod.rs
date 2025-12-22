//! Proposal module
//! 
//! Provides proposal management functionality:
//! - types: ProposalStatus enum and Proposal struct
//! - lifecycle: Proposal lifecycle methods (new, activate, pass, reject, execute, cancel, archive)
//! - analytics: Proposal analytics and metrics
//! - amendment: Proposal amendment support
//! - template: Proposal template system

pub mod types;
pub mod lifecycle;
pub mod analytics;
pub mod amendment;
pub mod template;
pub mod treasury;

#[cfg(test)]
mod tests;

// Re-export types
pub use types::{Proposal, ProposalStatus};
pub use amendment::ProposalAmendment;
pub use template::{ProposalTemplate, TemplateField, TemplateFieldType};
pub use treasury::{TreasuryProposalType, TreasuryOperationData};
pub use analytics::{
    ProposalAnalyticsMetadata, ProposalAnalyticsType, ProposalAnalyticsStatus,
    onchain::initialize_proposal_analytics,
};
