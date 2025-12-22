//! Security Enhancements Module
//! 
//! Comprehensive security enhancements and recommendations system.
//! Integrates with security modules and AI analysis for security improvements.
//! 
//! Hybrid model: Off-chain security analysis and recommendations, on-chain metadata storage
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Only security enhancement metadata, recommendations, and summary records
//! - OFF-CHAIN: All actual security analysis, vulnerability assessment, and detailed recommendations
//!
//! Migrated from: indrasnet-dao-v3-gitlab/programs/indrasnet-dao-ai/src/ai/security_enhancements.rs

use anchor_lang::prelude::*;
use crate::error::IndrasError;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Security enhancement priority
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum EnhancementPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl Space for EnhancementPriority {
    const INIT_SPACE: usize = 1;
}

/// Security enhancement status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum EnhancementStatus {
    /// Recommendation created
    Recommended,
    /// Enhancement approved
    Approved,
    /// Enhancement in progress
    InProgress,
    /// Enhancement completed
    Completed,
    /// Enhancement rejected
    Rejected,
}

impl Space for EnhancementStatus {
    const INIT_SPACE: usize = 1;
}

/// Security enhancement record
#[account]
#[derive(InitSpace)]
pub struct SecurityEnhancement {
    /// Enhancement ID
    pub enhancement_id: u64,
    /// Entity ID (idea, system, etc.)
    pub entity_id: u64,
    /// Entity type
    #[max_len(50)]
    pub entity_type: String,
    /// Enhancement title
    #[max_len(200)]
    pub title: String,
    /// Enhancement description
    #[max_len(1000)]
    pub description: String,
    /// Enhancement priority
    pub priority: EnhancementPriority,
    /// Enhancement status
    pub status: EnhancementStatus,
    /// Security score improvement (0-100)
    pub score_improvement: u8,
    /// Enhancement metadata URI (IPFS or similar)
    #[max_len(500)]
    pub metadata_uri: String,
    /// Enhancement hash (for verification)
    pub enhancement_hash: [u8; 32],
    /// Recommendation timestamp
    pub recommended_at: i64,
    /// Recommender (AI system or authority)
    pub recommender: Pubkey,
    /// Bump seed for PDA
    pub bump: u8,
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Create security enhancement recommendation
pub fn create_security_enhancement(
    enhancement: &mut SecurityEnhancement,
    enhancement_id: u64,
    entity_id: u64,
    entity_type: String,
    title: String,
    description: String,
    priority: EnhancementPriority,
    score_improvement: u8,
    metadata_uri: String,
    enhancement_hash: [u8; 32],
    recommender: Pubkey,
) -> Result<()> {
    require!(!title.is_empty(), IndrasError::InvalidInput);
    require!(title.len() <= 200, IndrasError::StringTooLong);
    require!(description.len() <= 1000, IndrasError::StringTooLong);
    require!(entity_type.len() <= 50, IndrasError::StringTooLong);
    require!(metadata_uri.len() <= 500, IndrasError::StringTooLong);
    require!(score_improvement <= 100, IndrasError::InvalidScore);
    
    enhancement.enhancement_id = enhancement_id;
    enhancement.entity_id = entity_id;
    enhancement.entity_type = entity_type;
    enhancement.title = title;
    enhancement.description = description;
    enhancement.priority = priority;
    enhancement.status = EnhancementStatus::Recommended;
    enhancement.score_improvement = score_improvement;
    enhancement.metadata_uri = metadata_uri;
    enhancement.enhancement_hash = enhancement_hash;
    enhancement.recommended_at = Clock::get()?.unix_timestamp;
    enhancement.recommender = recommender;
    
    msg!("Security enhancement {} created for entity {} (priority: {:?}, improvement: {}%)", 
         enhancement_id, entity_id, priority, score_improvement);
    
    Ok(())
}

/// Update security enhancement status
pub fn update_security_enhancement_status(
    enhancement: &mut SecurityEnhancement,
    status: EnhancementStatus,
) -> Result<()> {
    enhancement.status = status;
    
    msg!("Security enhancement {} status updated to {:?}", enhancement.enhancement_id, status);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_security_enhancement() {
        let mut enhancement = SecurityEnhancement {
            enhancement_id: 0,
            entity_id: 0,
            entity_type: String::new(),
            title: String::new(),
            description: String::new(),
            priority: EnhancementPriority::Low,
            status: EnhancementStatus::Recommended,
            score_improvement: 0,
            metadata_uri: String::new(),
            enhancement_hash: [0u8; 32],
            recommended_at: 0,
            recommender: Pubkey::default(),
            bump: 0,
        };

        let recommender = Pubkey::from([1u8; 32]);
        let enhancement_hash = [1u8; 32];

        let result = create_security_enhancement(
            &mut enhancement,
            1,
            100,
            "idea".to_string(),
            "Test Enhancement".to_string(),
            "Test Description".to_string(),
            EnhancementPriority::High,
            25,
            "ipfs://test".to_string(),
            enhancement_hash,
            recommender,
        );

        assert!(result.is_ok());
        assert_eq!(enhancement.enhancement_id, 1);
        assert_eq!(enhancement.priority, EnhancementPriority::High);
        assert_eq!(enhancement.status, EnhancementStatus::Recommended);
    }
}
