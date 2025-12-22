//! Expert Registry
//!
//! Centralized catalog of experts by semantic domain for grant voting.
//! Allows efficient lookup of experts for a given semantic domain.

use anchor_lang::prelude::*;
use crate::state::grant::semantic::{CompetencyLevel, CompetencySource};

/// Expert Registry Account
///
/// Centralized catalog storing experts organized by semantic domain.
/// PDA seeds: [b"expert_registry"]
#[account]
#[derive(InitSpace)]
pub struct ExpertRegistry {
    /// Total number of expert entries across all domains
    pub total_experts: u32,
    /// Total number of unique domains with experts
    pub total_domains: u32,
    /// Created timestamp
    pub created_at: i64,
    /// Last updated timestamp
    pub updated_at: i64,
    /// Bump seed
    pub bump: u8,
}

/// Expert Entry Account
///
/// Represents a single expert's competency in a semantic domain.
/// PDA seeds: [b"expert", expert_pubkey.as_ref(), domain_id.as_ref()]
#[account]
#[derive(InitSpace)]
pub struct ExpertEntry {
    /// Expert's public key
    pub expert: Pubkey,
    /// Semantic domain ID (e.g., "mathematics", "film-production")
    #[max_len(50)]
    pub domain_id: String,
    /// Competency level (must be Expert for entry in registry)
    pub competency_level: CompetencyLevel,
    /// Confidence score (0-100)
    pub confidence: u8,
    /// Source of competency (how it was derived)
    pub source: CompetencySource,
    /// Reputation score (minimum required for expert status)
    pub reputation_score: u64,
    /// Created timestamp
    pub created_at: i64,
    /// Last updated timestamp
    pub updated_at: i64,
    /// Whether this expert entry is active
    pub is_active: bool,
    /// Who added/verified this expert (governance authority)
    pub verified_by: Pubkey,
    /// Bump seed
    pub bump: u8,
}

/// Domain Expert Index Account
///
/// Index for fast lookup of all experts in a specific domain.
/// PDA seeds: [b"domain_experts", domain_id.as_ref()]
#[account]
#[derive(InitSpace)]
pub struct DomainExpertIndex {
    /// Semantic domain ID
    #[max_len(50)]
    pub domain_id: String,
    /// List of expert public keys in this domain
    /// NOTE: Limited to 100 experts per domain for gas efficiency
    #[max_len(100)]
    pub experts: Vec<Pubkey>,
    /// Total number of experts (for quick lookup)
    pub expert_count: u32,
    /// Created timestamp
    pub created_at: i64,
    /// Last updated timestamp
    pub updated_at: i64,
    /// Bump seed
    pub bump: u8,
}

impl ExpertRegistry {
    /// Minimum reputation score required for expert status
    pub const MIN_EXPERT_REPUTATION: u64 = 100;
    
    /// Maximum experts per domain (gas efficiency)
    pub const MAX_EXPERTS_PER_DOMAIN: usize = 100;
}

impl ExpertEntry {
    /// Minimum reputation score required for expert status
    pub const MIN_EXPERT_REPUTATION: u64 = 100;
    
    /// Minimum confidence score required
    pub const MIN_CONFIDENCE: u8 = 70;
    
    /// Check if entry is valid expert (active + Expert level + sufficient reputation)
    pub fn is_valid_expert(&self) -> bool {
        self.is_active
            && self.competency_level == CompetencyLevel::Expert
            && self.reputation_score >= Self::MIN_EXPERT_REPUTATION
            && self.confidence >= Self::MIN_CONFIDENCE
    }
}

impl DomainExpertIndex {
    /// Maximum experts per domain
    pub const MAX_EXPERTS: usize = 100;
    
    /// Add expert to domain index
    pub fn add_expert(&mut self, expert: Pubkey) -> Result<()> {
        require!(
            !self.experts.contains(&expert),
            crate::error::IndrasError::ExpertAlreadyExists
        );
        require!(
            self.experts.len() < Self::MAX_EXPERTS,
            crate::error::IndrasError::DomainExpertLimitReached
        );
        self.experts.push(expert);
        self.expert_count = self.experts.len() as u32;
        Ok(())
    }
    
    /// Remove expert from domain index
    pub fn remove_expert(&mut self, expert: Pubkey) -> Result<()> {
        let index = self.experts.iter()
            .position(|&e| e == expert)
            .ok_or(crate::error::IndrasError::ExpertNotFound)?;
        self.experts.remove(index);
        self.expert_count = self.experts.len() as u32;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_expert_entry() -> ExpertEntry {
        ExpertEntry {
            expert: Pubkey::new_unique(),
            domain_id: "mathematics".to_string(),
            competency_level: CompetencyLevel::Expert,
            confidence: 80,
            source: CompetencySource::ManualAssignment,
            reputation_score: 150,
            created_at: 1000,
            updated_at: 1000,
            is_active: true,
            verified_by: Pubkey::new_unique(),
            bump: 255,
        }
    }

    fn create_test_domain_index() -> DomainExpertIndex {
        DomainExpertIndex {
            domain_id: "mathematics".to_string(),
            experts: Vec::new(),
            expert_count: 0,
            created_at: 1000,
            updated_at: 1000,
            bump: 255,
        }
    }

    #[test]
    fn test_expert_registry_constants() {
        assert_eq!(ExpertRegistry::MIN_EXPERT_REPUTATION, 100);
        assert_eq!(ExpertRegistry::MAX_EXPERTS_PER_DOMAIN, 100);
        assert_eq!(ExpertEntry::MIN_EXPERT_REPUTATION, 100);
        assert_eq!(ExpertEntry::MIN_CONFIDENCE, 70);
        assert_eq!(DomainExpertIndex::MAX_EXPERTS, 100);
    }

    #[test]
    fn test_expert_entry_is_valid_expert() {
        let entry = create_test_expert_entry();
        assert!(entry.is_valid_expert());
    }

    #[test]
    fn test_expert_entry_is_valid_expert_inactive() {
        let mut entry = create_test_expert_entry();
        entry.is_active = false;
        assert!(!entry.is_valid_expert());
    }

    #[test]
    fn test_expert_entry_is_valid_expert_not_expert_level() {
        let mut entry = create_test_expert_entry();
        entry.competency_level = CompetencyLevel::Basic;
        assert!(!entry.is_valid_expert());
    }

    #[test]
    fn test_expert_entry_is_valid_expert_low_reputation() {
        let mut entry = create_test_expert_entry();
        entry.reputation_score = 50; // Below minimum
        assert!(!entry.is_valid_expert());
    }

    #[test]
    fn test_expert_entry_is_valid_expert_low_confidence() {
        let mut entry = create_test_expert_entry();
        entry.confidence = 50; // Below minimum
        assert!(!entry.is_valid_expert());
    }

    #[test]
    fn test_expert_entry_is_valid_expert_minimum_values() {
        let mut entry = create_test_expert_entry();
        entry.reputation_score = ExpertEntry::MIN_EXPERT_REPUTATION;
        entry.confidence = ExpertEntry::MIN_CONFIDENCE;
        assert!(entry.is_valid_expert());
    }

    #[test]
    fn test_domain_expert_index_add_expert() {
        let mut index = create_test_domain_index();
        let expert = Pubkey::new_unique();
        
        assert!(index.add_expert(expert).is_ok());
        assert_eq!(index.expert_count, 1);
        assert_eq!(index.experts.len(), 1);
        assert!(index.experts.contains(&expert));
    }

    #[test]
    fn test_domain_expert_index_add_duplicate_expert() {
        let mut index = create_test_domain_index();
        let expert = Pubkey::new_unique();
        
        assert!(index.add_expert(expert).is_ok());
        assert!(index.add_expert(expert).is_err()); // Duplicate
    }

    #[test]
    fn test_domain_expert_index_add_multiple_experts() {
        let mut index = create_test_domain_index();
        
        for i in 0..10u8 {
            let mut pubkey_bytes = [i; 32];
            pubkey_bytes[0] = i;
            let expert = Pubkey::from(pubkey_bytes);
            assert!(index.add_expert(expert).is_ok());
        }
        
        assert_eq!(index.expert_count, 10);
        assert_eq!(index.experts.len(), 10);
    }

    #[test]
    fn test_domain_expert_index_remove_expert() {
        let mut index = create_test_domain_index();
        let expert = Pubkey::new_unique();
        
        assert!(index.add_expert(expert).is_ok());
        assert!(index.remove_expert(expert).is_ok());
        assert_eq!(index.expert_count, 0);
        assert_eq!(index.experts.len(), 0);
    }

    #[test]
    fn test_domain_expert_index_remove_nonexistent_expert() {
        let mut index = create_test_domain_index();
        let expert = Pubkey::new_unique();
        
        assert!(index.remove_expert(expert).is_err());
    }

    #[test]
    fn test_domain_expert_index_add_remove_cycle() {
        let mut index = create_test_domain_index();
        let expert1 = Pubkey::new_unique();
        let expert2 = Pubkey::new_unique();
        
        assert!(index.add_expert(expert1).is_ok());
        assert!(index.add_expert(expert2).is_ok());
        assert_eq!(index.expert_count, 2);
        
        assert!(index.remove_expert(expert1).is_ok());
        assert_eq!(index.expert_count, 1);
        assert!(!index.experts.contains(&expert1));
        assert!(index.experts.contains(&expert2));
    }

    #[test]
    fn test_domain_expert_index_add_expert_max_limit() {
        let mut index = create_test_domain_index();
        
        // Add 100 experts (max)
        for i in 0..100u8 {
            let mut pubkey_bytes = [i; 32];
            pubkey_bytes[0] = i;
            let expert = Pubkey::from(pubkey_bytes);
            assert!(index.add_expert(expert).is_ok());
        }
        
        assert_eq!(index.expert_count, 100);
        assert_eq!(index.experts.len(), 100);
        
        // Try to add 101st expert - should fail
        let expert = Pubkey::new_unique();
        assert!(index.add_expert(expert).is_err());
    }

    #[test]
    fn test_domain_expert_index_remove_updates_count() {
        let mut index = create_test_domain_index();
        let expert1 = Pubkey::new_unique();
        let expert2 = Pubkey::new_unique();
        let expert3 = Pubkey::new_unique();
        
        assert!(index.add_expert(expert1).is_ok());
        assert!(index.add_expert(expert2).is_ok());
        assert!(index.add_expert(expert3).is_ok());
        assert_eq!(index.expert_count, 3);
        
        assert!(index.remove_expert(expert2).is_ok());
        assert_eq!(index.expert_count, 2);
        assert_eq!(index.experts.len(), 2);
    }

    #[test]
    fn test_expert_entry_all_fields() {
        let expert = Pubkey::new_unique();
        let verified_by = Pubkey::new_unique();
        let entry = ExpertEntry {
            expert,
            domain_id: "quantum-computing".to_string(),
            competency_level: CompetencyLevel::Expert,
            confidence: 95,
            source: CompetencySource::AIAnalysis,
            reputation_score: 200,
            created_at: 5000,
            updated_at: 6000,
            is_active: true,
            verified_by,
            bump: 128,
        };
        
        assert_eq!(entry.expert, expert);
        assert_eq!(entry.domain_id, "quantum-computing");
        assert_eq!(entry.competency_level, CompetencyLevel::Expert);
        assert_eq!(entry.confidence, 95);
        assert_eq!(entry.source, CompetencySource::AIAnalysis);
        assert_eq!(entry.reputation_score, 200);
        assert_eq!(entry.created_at, 5000);
        assert_eq!(entry.updated_at, 6000);
        assert!(entry.is_active);
        assert_eq!(entry.verified_by, verified_by);
        assert_eq!(entry.bump, 128);
    }

    #[test]
    fn test_expert_registry_all_fields() {
        let registry = ExpertRegistry {
            total_experts: 50,
            total_domains: 10,
            created_at: 1000,
            updated_at: 2000,
            bump: 255,
        };
        
        assert_eq!(registry.total_experts, 50);
        assert_eq!(registry.total_domains, 10);
        assert_eq!(registry.created_at, 1000);
        assert_eq!(registry.updated_at, 2000);
        assert_eq!(registry.bump, 255);
    }

    #[test]
    fn test_domain_expert_index_all_fields() {
        let expert1 = Pubkey::new_unique();
        let expert2 = Pubkey::new_unique();
        let index = DomainExpertIndex {
            domain_id: "film-production".to_string(),
            experts: vec![expert1, expert2],
            expert_count: 2,
            created_at: 3000,
            updated_at: 4000,
            bump: 200,
        };
        
        assert_eq!(index.domain_id, "film-production");
        assert_eq!(index.experts.len(), 2);
        assert_eq!(index.expert_count, 2);
        assert_eq!(index.created_at, 3000);
        assert_eq!(index.updated_at, 4000);
        assert_eq!(index.bump, 200);
    }

    #[test]
    fn test_expert_entry_is_valid_expert_all_conditions() {
        // Test all conditions must be met
        let mut entry = create_test_expert_entry();
        
        // All conditions met - should be valid
        assert!(entry.is_valid_expert());
        
        // Test each condition individually
        entry.is_active = false;
        assert!(!entry.is_valid_expert());
        
        entry.is_active = true;
        entry.competency_level = CompetencyLevel::Standard;
        assert!(!entry.is_valid_expert());
        
        entry.competency_level = CompetencyLevel::Expert;
        entry.reputation_score = 50;
        assert!(!entry.is_valid_expert());
        
        entry.reputation_score = 150;
        entry.confidence = 50;
        assert!(!entry.is_valid_expert());
        
        // All conditions met again
        entry.confidence = 80;
        assert!(entry.is_valid_expert());
    }

    #[test]
    fn test_expert_entry_is_valid_expert_exact_thresholds_comprehensive() {
        let mut entry = create_test_expert_entry();
        
        // Test exact minimum reputation (100)
        entry.reputation_score = 100;
        assert!(entry.is_valid_expert());
        
        // Test exact minimum confidence (70)
        entry.confidence = 70;
        assert!(entry.is_valid_expert());
    }

    #[test]
    fn test_expert_entry_is_valid_expert_below_thresholds_comprehensive() {
        let mut entry = create_test_expert_entry();
        
        // Test below minimum reputation (99)
        entry.reputation_score = 99;
        assert!(!entry.is_valid_expert());
        
        // Test below minimum confidence (69)
        entry.reputation_score = 150;
        entry.confidence = 69;
        assert!(!entry.is_valid_expert());
    }
}
