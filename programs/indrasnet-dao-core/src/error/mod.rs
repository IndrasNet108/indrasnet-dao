//! Error types for the IndrasNet DAO Core program

use anchor_lang::prelude::*;

#[error_code]
pub enum IndrasError {
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid input")]
    InvalidInput,
    
    #[msg("Account discriminator mismatch")]
    AccountDiscriminatorMismatch,
    
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    #[msg("Account not found")]
    AccountNotFound,
    
    #[msg("Invalid state")]
    InvalidState,
    
    #[msg("DAO is inactive")]
    DaoInactive,
    
    #[msg("Proposal already executed")]
    AlreadyExecuted,
    
    #[msg("Execution delay not met")]
    ExecutionDelayNotMet,
    
    #[msg("Operation not allowed")]
    OperationNotAllowed,
    
    #[msg("Voting not active")]
    VotingNotActive,
    
    #[msg("Proposal not found")]
    ProposalNotFound,
    
    #[msg("Member not found")]
    MemberNotFound,
    
    #[msg("Member is banned")]
    MemberBanned,
    
    #[msg("Member has already voted")]
    AlreadyVoted,
    
    #[msg("Invalid vote")]
    InvalidVote,
    
    #[msg("Idea not found")]
    IdeaNotFound,
    
    #[msg("Invalid idea status")]
    InvalidIdeaStatus,
    
    #[msg("Treasury operation failed")]
    TreasuryOperationFailed,
    
    #[msg("Capability not found")]
    CapabilityNotFound,
    
    #[msg("Capability expired or revoked")]
    CapabilityExpired,
    
    #[msg("Integration failed")]
    IntegrationFailed,
    
    #[msg("AI operation failed")]
    AIOperationFailed,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Arithmetic underflow")]
    Underflow,
    
    #[msg("Invalid operation")]
    InvalidOperation,
    
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    
    #[msg("Data too large")]
    DataTooLarge,
    
    #[msg("URI too long")]
    UriTooLong,
    
    #[msg("Division by zero")]
    DivisionByZero,
    
    #[msg("Invalid score")]
    InvalidScore,
    
    #[msg("Account not initialized")]
    AccountNotInitialized,
    
    #[msg("Group is full")]
    GroupFull,
    
    #[msg("Already a member")]
    AlreadyMember,
    
    #[msg("Cannot remove leader")]
    CannotRemoveLeader,
    
    #[msg("Insufficient members")]
    InsufficientMembers,
    
    #[msg("Insufficient progress - group needs more contributions before requesting grant")]
    InsufficientProgress,
    
    #[msg("Mesh group is not part of a phenomenon - groups must be in AI-formed phenomena to request grants")]
    NotInPhenomenon,
    
    #[msg("Idea is not part of the phenomenon - the phenomenon must contain the group's idea")]
    IdeaNotInPhenomenon,
    
    #[msg("Already exists")]
    AlreadyExists,
    
    #[msg("Suspension not expired")]
    SuspensionNotExpired,
    
    #[msg("Resource not found")]
    NotFound,
    
    #[msg("Amount too small")]
    AmountTooSmall,
    
    #[msg("Amount too large")]
    AmountTooLarge,
    
    #[msg("String too long")]
    StringTooLong,
    
    #[msg("Compliance violation")]
    ComplianceViolation,
    
    #[msg("Idea is not in the mesh group")]
    IdeaNotInMeshGroup,
    
    #[msg("Too many supporting groups")]
    TooManySupportingGroups,
    
    #[msg("Idea is not innovative - innovation requires impact_score >= 70 and feasibility_score >= 70")]
    IdeaNotInnovative,
    
    #[msg("Idea is not unique - uniqueness_score must be >= 70 for authorship")]
    IdeaNotUnique,
    
    #[msg("Feasibility not proven - feasibility_score must be >= 70 with verified artifacts")]
    FeasibilityNotProven,
    
    #[msg("No artifacts provided - artifacts are required to prove feasibility")]
    NoArtifacts,
    
    #[msg("Artifacts not verified - artifacts must be verified before idea can enter mesh group")]
    ArtifactsNotVerified,
    
    #[msg("Invalid program - account owner does not match expected program")]
    InvalidProgram,
    
    #[msg("Idea not approved - idea must be approved by AI analysis before grant creation")]
    IdeaNotApproved,
    
    #[msg("Input hash mismatch - analysis input_hash does not match idea.idea_hash")]
    InputHashMismatch,
    
    #[msg("Analysis expired - analysis has passed its expiration timestamp")]
    AnalysisExpired,
    
    #[msg("Unauthorized AI service - service is not registered or inactive")]
    UnauthorizedAIService,
    
    #[msg("Invalid model version - model version is not verified or deprecated")]
    InvalidModelVersion,
    
    // SEC-INV-8: Rate limiting errors
    #[msg("Rate limit exceeded - too many operations in time window")]
    RateLimitExceeded,
    
    // SEC-INV-15: Sybil protection errors
    #[msg("Insufficient reputation - member reputation score is below required threshold")]
    InsufficientReputation,
    
    #[msg("Cooldown period active - former member must wait before rejoining group")]
    CooldownPeriodActive,
    
    // SEC-INV-16: Critical moment errors
    #[msg("Critical moment active - member changes blocked during active grants/proposals")]
    CriticalMomentActive,
    
    // Track B: Embedding signature errors (SEC-INV-10)
    #[msg("Embedding signature invalid - signature verification failed")]
    EmbeddingSignatureInvalid,
    
    #[msg("Embedding provider not authorized - provider not in whitelist")]
    EmbeddingProviderNotAuthorized,
    
    #[msg("Embedding hash mismatch - computed hash does not match provided hash")]
    EmbeddingHashMismatch,
    
    #[msg("Invalid embedding provider - provider not registered in AIServiceRegistry")]
    InvalidEmbeddingProvider,

    #[msg("AI Service already exists in the registry")]
    ServiceAlreadyExists,

    #[msg("AI Service Registry is full")]
    RegistryFull,

    #[msg("AI Service not found in the registry")]
    ServiceNotFound,
    
    #[msg("Embedding vector size invalid - vector must be normalized and within valid size range")]
    EmbeddingVectorSizeInvalid,
    
    // Track B: DBSCAN validation errors (B3)
    #[msg("Semantic distance exceeded - distance between ideas exceeds eps threshold")]
    SemanticDistanceExceeded,
    
    #[msg("Insufficient cluster size - cluster has fewer ideas than min_samples requirement")]
    InsufficientClusterSize,
    
    #[msg("No author overlap - cluster ideas must have at least one shared author")]
    NoAuthorOverlap,
    
    #[msg("Temporal window exceeded - ideas in cluster span more than allowed time window")]
    TemporalWindowExceeded,
    
    #[msg("Cluster signature invalid - cluster signature verification failed")]
    ClusterSignatureInvalid,
    
    // Track B: Semantic Grant Voting errors (B4)
    #[msg("Invalid semantic domain - semantic domain account not found or invalid")]
    InvalidSemanticDomain,
    
    #[msg("Semantic distance mismatch - idea semantic distance does not match provided distance")]
    SemanticDistanceMismatch,
    
    // Expert Registry errors
    #[msg("Expert not found in registry")]
    ExpertNotFound,
    
    #[msg("Expert already exists in domain")]
    ExpertAlreadyExists,
    
    #[msg("Domain expert limit reached - maximum experts per domain exceeded")]
    DomainExpertLimitReached,
    
    #[msg("Insufficient reputation for expert status - minimum reputation score not met")]
    InsufficientReputationForExpert,
    
    #[msg("Invalid competency level - must be Expert level for registry entry")]
    InvalidCompetencyLevel,
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indras_error_unauthorized() {
        let err = IndrasError::Unauthorized;
        // Test that error can be created
        assert!(matches!(err, IndrasError::Unauthorized));
    }

    #[test]
    fn test_indras_error_invalid_input() {
        let err = IndrasError::InvalidInput;
        assert!(matches!(err, IndrasError::InvalidInput));
    }

    #[test]
    fn test_indras_error_insufficient_funds() {
        let err = IndrasError::InsufficientFunds;
        assert!(matches!(err, IndrasError::InsufficientFunds));
    }

    #[test]
    fn test_indras_error_overflow() {
        let err = IndrasError::Overflow;
        assert!(matches!(err, IndrasError::Overflow));
    }

    #[test]
    fn test_indras_error_underflow() {
        let err = IndrasError::Underflow;
        assert!(matches!(err, IndrasError::Underflow));
    }

    #[test]
    fn test_indras_error_division_by_zero() {
        let err = IndrasError::DivisionByZero;
        assert!(matches!(err, IndrasError::DivisionByZero));
    }

    #[test]
    fn test_indras_error_all_variants_unique() {
        // Test that all error variants are unique by checking they don't match each other
        let errors = vec![
            IndrasError::Unauthorized,
            IndrasError::InvalidInput,
            IndrasError::AccountDiscriminatorMismatch,
            IndrasError::InsufficientFunds,
            IndrasError::AccountNotFound,
            IndrasError::InvalidState,
            IndrasError::DaoInactive,
            IndrasError::AlreadyExecuted,
            IndrasError::ExecutionDelayNotMet,
            IndrasError::OperationNotAllowed,
            IndrasError::VotingNotActive,
            IndrasError::ProposalNotFound,
            IndrasError::MemberNotFound,
            IndrasError::MemberBanned,
            IndrasError::AlreadyVoted,
            IndrasError::InvalidVote,
            IndrasError::IdeaNotFound,
            IndrasError::InvalidIdeaStatus,
            IndrasError::TreasuryOperationFailed,
            IndrasError::CapabilityNotFound,
            IndrasError::CapabilityExpired,
            IndrasError::IntegrationFailed,
            IndrasError::AIOperationFailed,
            IndrasError::Overflow,
            IndrasError::Underflow,
            IndrasError::InvalidOperation,
            IndrasError::InvalidTimestamp,
            IndrasError::DataTooLarge,
            IndrasError::UriTooLong,
            IndrasError::DivisionByZero,
            IndrasError::InvalidScore,
            IndrasError::AccountNotInitialized,
            IndrasError::GroupFull,
            IndrasError::AlreadyMember,
            IndrasError::CannotRemoveLeader,
            IndrasError::InsufficientMembers,
            IndrasError::InsufficientProgress,
            IndrasError::NotInPhenomenon,
            IndrasError::IdeaNotInPhenomenon,
            IndrasError::AlreadyExists,
            IndrasError::SuspensionNotExpired,
            IndrasError::NotFound,
            IndrasError::AmountTooSmall,
            IndrasError::AmountTooLarge,
            IndrasError::StringTooLong,
            IndrasError::ComplianceViolation,
            IndrasError::IdeaNotInMeshGroup,
            IndrasError::TooManySupportingGroups,
            IndrasError::IdeaNotInnovative,
            IndrasError::IdeaNotUnique,
            IndrasError::FeasibilityNotProven,
            IndrasError::NoArtifacts,
            IndrasError::ArtifactsNotVerified,
            IndrasError::InvalidProgram,
            IndrasError::IdeaNotApproved,
            IndrasError::InputHashMismatch,
            IndrasError::AnalysisExpired,
            IndrasError::UnauthorizedAIService,
            IndrasError::InvalidModelVersion,
            IndrasError::RateLimitExceeded,
            IndrasError::InsufficientReputation,
            IndrasError::CooldownPeriodActive,
            IndrasError::CriticalMomentActive,
            IndrasError::EmbeddingSignatureInvalid,
            IndrasError::EmbeddingProviderNotAuthorized,
            IndrasError::EmbeddingHashMismatch,
            IndrasError::InvalidEmbeddingProvider,
            IndrasError::ServiceAlreadyExists,
            IndrasError::RegistryFull,
            IndrasError::ServiceNotFound,
            IndrasError::EmbeddingVectorSizeInvalid,
            IndrasError::SemanticDistanceExceeded,
            IndrasError::InsufficientClusterSize,
            IndrasError::NoAuthorOverlap,
            IndrasError::TemporalWindowExceeded,
            IndrasError::ClusterSignatureInvalid,
            IndrasError::InvalidSemanticDomain,
            IndrasError::SemanticDistanceMismatch,
            IndrasError::ExpertNotFound,
            IndrasError::ExpertAlreadyExists,
            IndrasError::DomainExpertLimitReached,
            IndrasError::InsufficientReputationForExpert,
            IndrasError::InvalidCompetencyLevel,
        ];

        // Check all errors are unique (they should all be different)
        for i in 0..errors.len() {
            for j in (i + 1)..errors.len() {
                // Use format! to compare error messages as a way to verify uniqueness
                // Note: This is a basic check - in practice, error codes would be used
                assert_ne!(
                    format!("{:?}", errors[i]),
                    format!("{:?}", errors[j]),
                    "Duplicate error found at indices {} and {}", i, j
                );
            }
        }
    }

    #[test]
    fn test_indras_error_treasury_errors() {
        assert!(matches!(IndrasError::TreasuryOperationFailed, IndrasError::TreasuryOperationFailed));
        assert!(matches!(IndrasError::InsufficientFunds, IndrasError::InsufficientFunds));
        assert!(matches!(IndrasError::AmountTooSmall, IndrasError::AmountTooSmall));
        assert!(matches!(IndrasError::AmountTooLarge, IndrasError::AmountTooLarge));
    }

    #[test]
    fn test_indras_error_capability_errors() {
        assert!(matches!(IndrasError::CapabilityNotFound, IndrasError::CapabilityNotFound));
        assert!(matches!(IndrasError::CapabilityExpired, IndrasError::CapabilityExpired));
    }

    #[test]
    fn test_indras_error_ai_errors() {
        assert!(matches!(IndrasError::AIOperationFailed, IndrasError::AIOperationFailed));
        assert!(matches!(IndrasError::UnauthorizedAIService, IndrasError::UnauthorizedAIService));
        assert!(matches!(IndrasError::InvalidModelVersion, IndrasError::InvalidModelVersion));
        assert!(matches!(IndrasError::ServiceAlreadyExists, IndrasError::ServiceAlreadyExists));
        assert!(matches!(IndrasError::RegistryFull, IndrasError::RegistryFull));
        assert!(matches!(IndrasError::ServiceNotFound, IndrasError::ServiceNotFound));
    }

    #[test]
    fn test_indras_error_embedding_errors() {
        assert!(matches!(IndrasError::EmbeddingSignatureInvalid, IndrasError::EmbeddingSignatureInvalid));
        assert!(matches!(IndrasError::EmbeddingProviderNotAuthorized, IndrasError::EmbeddingProviderNotAuthorized));
        assert!(matches!(IndrasError::EmbeddingHashMismatch, IndrasError::EmbeddingHashMismatch));
        assert!(matches!(IndrasError::InvalidEmbeddingProvider, IndrasError::InvalidEmbeddingProvider));
        assert!(matches!(IndrasError::EmbeddingVectorSizeInvalid, IndrasError::EmbeddingVectorSizeInvalid));
    }

    #[test]
    fn test_indras_error_dbscan_errors() {
        assert!(matches!(IndrasError::SemanticDistanceExceeded, IndrasError::SemanticDistanceExceeded));
        assert!(matches!(IndrasError::InsufficientClusterSize, IndrasError::InsufficientClusterSize));
        assert!(matches!(IndrasError::NoAuthorOverlap, IndrasError::NoAuthorOverlap));
        assert!(matches!(IndrasError::TemporalWindowExceeded, IndrasError::TemporalWindowExceeded));
        assert!(matches!(IndrasError::ClusterSignatureInvalid, IndrasError::ClusterSignatureInvalid));
    }

    #[test]
    fn test_indras_error_expert_registry_errors() {
        assert!(matches!(IndrasError::ExpertNotFound, IndrasError::ExpertNotFound));
        assert!(matches!(IndrasError::ExpertAlreadyExists, IndrasError::ExpertAlreadyExists));
        assert!(matches!(IndrasError::DomainExpertLimitReached, IndrasError::DomainExpertLimitReached));
        assert!(matches!(IndrasError::InsufficientReputationForExpert, IndrasError::InsufficientReputationForExpert));
        assert!(matches!(IndrasError::InvalidCompetencyLevel, IndrasError::InvalidCompetencyLevel));
    }

    #[test]
    fn test_indras_error_idea_errors() {
        assert!(matches!(IndrasError::IdeaNotFound, IndrasError::IdeaNotFound));
        assert!(matches!(IndrasError::InvalidIdeaStatus, IndrasError::InvalidIdeaStatus));
        assert!(matches!(IndrasError::IdeaNotInPhenomenon, IndrasError::IdeaNotInPhenomenon));
        assert!(matches!(IndrasError::IdeaNotInMeshGroup, IndrasError::IdeaNotInMeshGroup));
        assert!(matches!(IndrasError::IdeaNotInnovative, IndrasError::IdeaNotInnovative));
        assert!(matches!(IndrasError::IdeaNotUnique, IndrasError::IdeaNotUnique));
        assert!(matches!(IndrasError::IdeaNotApproved, IndrasError::IdeaNotApproved));
    }

    #[test]
    fn test_indras_error_security_errors() {
        assert!(matches!(IndrasError::RateLimitExceeded, IndrasError::RateLimitExceeded));
        assert!(matches!(IndrasError::InsufficientReputation, IndrasError::InsufficientReputation));
        assert!(matches!(IndrasError::CooldownPeriodActive, IndrasError::CooldownPeriodActive));
        assert!(matches!(IndrasError::CriticalMomentActive, IndrasError::CriticalMomentActive));
    }
}
