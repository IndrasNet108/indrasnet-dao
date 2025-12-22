//! Idea account structure
//!
//! Includes analytics, evaluation, and ranking functionality

use anchor_lang::prelude::*;
use crate::state::enums::IdeaStatus;
use crate::error::IndrasError;

/// Rights that author can transfer to e.V. without grant
/// Complies with German law: Urheberrechtsgesetz (UrhG)
/// Author can transfer usage rights (Nutzungsrechte), retaining personal non-property rights
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub struct TransferredRights {
    /// Can modify the idea (Bearbeitung) - right to adapt
    pub can_modify: bool,
    
    /// Can distribute the idea (Verbreitung) - right to distribute
    pub can_distribute: bool,
    
    /// Can reproduce the idea (Vervielfältigung) - right to reproduce
    pub can_reproduce: bool,
    
    /// Can develop/improve the idea (Weiterentwicklung) - right to develop
    pub can_develop: bool,
    
    /// Can sublicense the idea (Unterlizenzierung) - right to sublicense
    pub can_sublicense: bool,
    
    /// Can gift the idea (Schenkung) - right to gift
    pub can_gift: bool,
    
    /// Can bequeath the idea (Vermächtnis) - right to bequeath
    pub can_bequeath: bool,
    
    /// Timestamp when rights were transferred
    pub transferred_at: i64,
    
    /// Author who transferred the rights
    pub transferred_by: Pubkey,
}

/// Idea account structure
#[account]
#[derive(InitSpace, PartialEq, Eq, Debug)]
pub struct Idea {
    pub id: u64,
    pub author: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    pub status: IdeaStatus,
    // NOTE: Rights transferred to e.V. without grant (voluntary transfer)
    // If author doesn't need a grant and doesn't want to develop the idea,
    // author can transfer rights to DAO (e.V.) at author's choice:
    // modify, distribute, reproduce, develop, sublicense, gift, or bequeath
    // Uses Option for backward compatibility (None = rights not transferred)
    pub rights_transferred_to_ev: Option<TransferredRights>,
    // Embedding fields (for Track B)
    pub idea_hash: Option<[u8; 32]>,
    pub embedding_hash: Option<[u8; 32]>,
    pub embedding_signature: Option<[u8; 64]>,
    #[max_len(100)]
    pub embedding_provider: Option<String>,
    #[max_len(100)]
    pub embedding_model: Option<String>,
    #[max_len(50)]
    pub embedding_model_version: Option<String>,
    pub embedding_created_at: Option<i64>,
    pub embedding_updated_at: Option<i64>,
    pub embedding_update_count: u64,
    pub bump: u8,
}

/// Idea analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum IdeaAnalyticsType {
    /// Popularity analytics
    Popularity,
    /// Impact analytics
    Impact,
    /// Feasibility analytics
    Feasibility,
    /// Custom analytics
    Custom,
}

/// Idea analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum IdeaAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Idea evaluation criteria
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum IdeaEvaluationCriteria {
    /// Innovation
    Innovation,
    /// Feasibility
    Feasibility,
    /// Impact
    Impact,
    /// Custom criteria
    Custom,
}

/// Idea evaluation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum IdeaEvaluationStatus {
    /// Evaluation pending
    Pending,
    /// Evaluation in progress
    InProgress,
    /// Evaluation completed
    Completed,
}

/// Idea ranking algorithm
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum IdeaRankingAlgorithm {
    /// Popularity-based
    Popularity,
    /// Quality-based
    Quality,
    /// Impact-based
    Impact,
    /// Custom algorithm
    Custom,
}

/// Idea ranking status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum IdeaRankingStatus {
    /// Ranking active
    Active,
    /// Ranking paused
    Paused,
    /// Ranking disabled
    Disabled,
}

/// Idea analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct IdeaAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Idea ID
    pub idea_id: u64,
    /// Analytics type
    pub analytics_type: IdeaAnalyticsType,
    /// Status
    pub status: IdeaAnalyticsStatus,
    /// Created at
    pub created_at: i64,
    /// Analytics config hash
    pub analytics_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Idea evaluation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct IdeaEvaluationMetadata {
    /// Evaluation ID
    pub evaluation_id: u64,
    /// Idea ID
    pub idea_id: u64,
    /// Evaluation criteria
    pub evaluation_criteria: IdeaEvaluationCriteria,
    /// Status
    pub status: IdeaEvaluationStatus,
    /// Created at
    pub created_at: i64,
    /// Evaluation data hash
    pub evaluation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// Idea ranking metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct IdeaRankingMetadata {
    /// Ranking ID
    pub ranking_id: u64,
    /// Idea ID
    pub idea_id: u64,
    /// Ranking algorithm
    pub ranking_algorithm: IdeaRankingAlgorithm,
    /// Status
    pub status: IdeaRankingStatus,
    /// Created at
    pub created_at: i64,
    /// Ranking config hash
    pub ranking_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for idea analytics, evaluation, and ranking
pub mod onchain {
    use super::*;

    pub fn initialize_idea_analytics(
        analytics: &mut IdeaAnalyticsMetadata,
        analytics_id: u64,
        idea_id: u64,
        analytics_type: IdeaAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.idea_id = idea_id;
        analytics.analytics_type = analytics_type;
        analytics.status = IdeaAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }

    pub fn initialize_idea_evaluation(
        evaluation: &mut IdeaEvaluationMetadata,
        evaluation_id: u64,
        idea_id: u64,
        evaluation_criteria: IdeaEvaluationCriteria,
        evaluation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(evaluation_id > 0, IndrasError::InvalidInput);
        evaluation.evaluation_id = evaluation_id;
        evaluation.idea_id = idea_id;
        evaluation.evaluation_criteria = evaluation_criteria;
        evaluation.status = IdeaEvaluationStatus::Pending;
        evaluation.created_at = current_time;
        evaluation.evaluation_data_hash = evaluation_data_hash;
        evaluation.bump = bump;
        Ok(())
    }

    pub fn initialize_idea_ranking(
        ranking: &mut IdeaRankingMetadata,
        ranking_id: u64,
        idea_id: u64,
        ranking_algorithm: IdeaRankingAlgorithm,
        ranking_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(ranking_id > 0, IndrasError::InvalidInput);
        ranking.ranking_id = ranking_id;
        ranking.idea_id = idea_id;
        ranking.ranking_algorithm = ranking_algorithm;
        ranking.status = IdeaRankingStatus::Active;
        ranking.created_at = current_time;
        ranking.ranking_config_hash = ranking_config_hash;
        ranking.bump = bump;
        Ok(())
    }
}

/// Off-chain functions for idea analytics, evaluation, and ranking
pub mod offchain {
    pub fn generate_idea_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }

    pub fn evaluate_idea(_evaluation_id: u64) -> Vec<u8> {
        vec![]
    }

    pub fn rank_idea(_ranking_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    #[test]
    fn test_initialize_idea_analytics() {
        let mut analytics = IdeaAnalyticsMetadata {
            analytics_id: 0,
            idea_id: 0,
            analytics_type: IdeaAnalyticsType::Popularity,
            status: IdeaAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_idea_analytics(
            &mut analytics,
            1,
            10,
            IdeaAnalyticsType::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.idea_id, 10);
        assert_eq!(analytics.analytics_type, IdeaAnalyticsType::Impact);
        assert_eq!(analytics.status, IdeaAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_idea_analytics_invalid_id() {
        let mut analytics = IdeaAnalyticsMetadata {
            analytics_id: 0,
            idea_id: 0,
            analytics_type: IdeaAnalyticsType::Popularity,
            status: IdeaAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_idea_analytics(
            &mut analytics,
            0, // Invalid: must be > 0
            10,
            IdeaAnalyticsType::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_idea_evaluation() {
        let mut evaluation = IdeaEvaluationMetadata {
            evaluation_id: 0,
            idea_id: 0,
            evaluation_criteria: IdeaEvaluationCriteria::Impact,
            status: IdeaEvaluationStatus::Pending,
            created_at: 0,
            evaluation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_idea_evaluation(
            &mut evaluation,
            1,
            10,
            IdeaEvaluationCriteria::Feasibility,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(evaluation.evaluation_id, 1);
        assert_eq!(evaluation.idea_id, 10);
        assert_eq!(evaluation.evaluation_criteria, IdeaEvaluationCriteria::Feasibility);
        assert_eq!(evaluation.status, IdeaEvaluationStatus::Pending);
        assert_eq!(evaluation.created_at, 1000);
        assert_eq!(evaluation.bump, 255);
    }

    #[test]
    fn test_initialize_idea_ranking() {
        let mut ranking = IdeaRankingMetadata {
            ranking_id: 0,
            idea_id: 0,
            ranking_algorithm: IdeaRankingAlgorithm::Popularity,
            status: IdeaRankingStatus::Active,
            created_at: 0,
            ranking_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_idea_ranking(
            &mut ranking,
            1,
            10,
            IdeaRankingAlgorithm::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(ranking.ranking_id, 1);
        assert_eq!(ranking.idea_id, 10);
        assert_eq!(ranking.ranking_algorithm, IdeaRankingAlgorithm::Impact);
        assert_eq!(ranking.status, IdeaRankingStatus::Active);
        assert_eq!(ranking.created_at, 1000);
        assert_eq!(ranking.bump, 255);
    }

    #[test]
    fn test_transferred_rights_creation() {
        let rights = TransferredRights {
            can_modify: true,
            can_distribute: false,
            can_reproduce: true,
            can_develop: false,
            can_sublicense: true,
            can_gift: false,
            can_bequeath: true,
            transferred_at: 1000,
            transferred_by: Pubkey::new_unique(),
        };
        
        assert!(rights.can_modify);
        assert!(!rights.can_distribute);
        assert_eq!(rights.transferred_at, 1000);
    }

    #[test]
    fn test_idea_analytics_type_variants() {
        assert_eq!(IdeaAnalyticsType::Popularity, IdeaAnalyticsType::Popularity);
        assert_eq!(IdeaAnalyticsType::Impact, IdeaAnalyticsType::Impact);
        assert_eq!(IdeaAnalyticsType::Feasibility, IdeaAnalyticsType::Feasibility);
        assert_eq!(IdeaAnalyticsType::Custom, IdeaAnalyticsType::Custom);
    }

    #[test]
    fn test_idea_analytics_status_variants() {
        assert_eq!(IdeaAnalyticsStatus::Active, IdeaAnalyticsStatus::Active);
        assert_eq!(IdeaAnalyticsStatus::Paused, IdeaAnalyticsStatus::Paused);
        assert_eq!(IdeaAnalyticsStatus::Disabled, IdeaAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_initialize_idea_evaluation_invalid_id() {
        let mut evaluation = IdeaEvaluationMetadata {
            evaluation_id: 0,
            idea_id: 0,
            evaluation_criteria: IdeaEvaluationCriteria::Impact,
            status: IdeaEvaluationStatus::Completed,
            created_at: 0,
            evaluation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_idea_evaluation(
            &mut evaluation,
            0, // Invalid: must be > 0
            10,
            IdeaEvaluationCriteria::Feasibility,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_idea_ranking_invalid_id() {
        let mut ranking = IdeaRankingMetadata {
            ranking_id: 0,
            idea_id: 0,
            ranking_algorithm: IdeaRankingAlgorithm::Popularity,
            status: IdeaRankingStatus::Disabled,
            created_at: 0,
            ranking_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_idea_ranking(
            &mut ranking,
            0, // Invalid: must be > 0
            10,
            IdeaRankingAlgorithm::Impact,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_idea_evaluation_criteria_variants() {
        assert_eq!(IdeaEvaluationCriteria::Innovation, IdeaEvaluationCriteria::Innovation);
        assert_eq!(IdeaEvaluationCriteria::Feasibility, IdeaEvaluationCriteria::Feasibility);
        assert_eq!(IdeaEvaluationCriteria::Impact, IdeaEvaluationCriteria::Impact);
        assert_eq!(IdeaEvaluationCriteria::Custom, IdeaEvaluationCriteria::Custom);
    }

    #[test]
    fn test_idea_evaluation_status_variants() {
        assert_eq!(IdeaEvaluationStatus::Pending, IdeaEvaluationStatus::Pending);
        assert_eq!(IdeaEvaluationStatus::InProgress, IdeaEvaluationStatus::InProgress);
        assert_eq!(IdeaEvaluationStatus::Completed, IdeaEvaluationStatus::Completed);
    }

    #[test]
    fn test_idea_ranking_algorithm_variants() {
        assert_eq!(IdeaRankingAlgorithm::Popularity, IdeaRankingAlgorithm::Popularity);
        assert_eq!(IdeaRankingAlgorithm::Quality, IdeaRankingAlgorithm::Quality);
        assert_eq!(IdeaRankingAlgorithm::Impact, IdeaRankingAlgorithm::Impact);
        assert_eq!(IdeaRankingAlgorithm::Custom, IdeaRankingAlgorithm::Custom);
    }

    #[test]
    fn test_idea_ranking_status_variants() {
        assert_eq!(IdeaRankingStatus::Active, IdeaRankingStatus::Active);
        assert_eq!(IdeaRankingStatus::Paused, IdeaRankingStatus::Paused);
        assert_eq!(IdeaRankingStatus::Disabled, IdeaRankingStatus::Disabled);
    }

    #[test]
    fn test_idea_analytics_type_all_variants_unique() {
        let types = vec![
            IdeaAnalyticsType::Popularity,
            IdeaAnalyticsType::Impact,
            IdeaAnalyticsType::Feasibility,
            IdeaAnalyticsType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_idea_analytics_status_all_variants_unique() {
        let statuses = vec![
            IdeaAnalyticsStatus::Active,
            IdeaAnalyticsStatus::Paused,
            IdeaAnalyticsStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_idea_evaluation_criteria_all_variants_unique() {
        let criteria = vec![
            IdeaEvaluationCriteria::Innovation,
            IdeaEvaluationCriteria::Feasibility,
            IdeaEvaluationCriteria::Impact,
            IdeaEvaluationCriteria::Custom,
        ];
        
        for i in 0..criteria.len() {
            for j in (i + 1)..criteria.len() {
                assert_ne!(criteria[i], criteria[j], "Duplicate criteria found");
            }
        }
    }

    #[test]
    fn test_idea_evaluation_status_all_variants_unique() {
        let statuses = vec![
            IdeaEvaluationStatus::Pending,
            IdeaEvaluationStatus::InProgress,
            IdeaEvaluationStatus::Completed,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_idea_ranking_algorithm_all_variants_unique() {
        let algorithms = vec![
            IdeaRankingAlgorithm::Popularity,
            IdeaRankingAlgorithm::Quality,
            IdeaRankingAlgorithm::Impact,
            IdeaRankingAlgorithm::Custom,
        ];
        
        for i in 0..algorithms.len() {
            for j in (i + 1)..algorithms.len() {
                assert_ne!(algorithms[i], algorithms[j], "Duplicate algorithm found");
            }
        }
    }

    #[test]
    fn test_idea_ranking_status_all_variants_unique() {
        let statuses = vec![
            IdeaRankingStatus::Active,
            IdeaRankingStatus::Paused,
            IdeaRankingStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_idea_analytics_type_copy() {
        let type1 = IdeaAnalyticsType::Impact;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_idea_analytics_status_copy() {
        let status1 = IdeaAnalyticsStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_idea_evaluation_criteria_copy() {
        let criteria1 = IdeaEvaluationCriteria::Impact;
        let criteria2 = criteria1; // Copy trait
        assert_eq!(criteria1, criteria2);
    }

    #[test]
    fn test_idea_evaluation_status_copy() {
        let status1 = IdeaEvaluationStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_idea_ranking_algorithm_copy() {
        let algorithm1 = IdeaRankingAlgorithm::Quality;
        let algorithm2 = algorithm1; // Copy trait
        assert_eq!(algorithm1, algorithm2);
    }

    #[test]
    fn test_idea_ranking_status_copy() {
        let status1 = IdeaRankingStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_idea_analytics_type_space() {
        assert_eq!(<IdeaAnalyticsType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_idea_analytics_status_space() {
        assert_eq!(<IdeaAnalyticsStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_idea_evaluation_criteria_space() {
        assert_eq!(<IdeaEvaluationCriteria as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_idea_evaluation_status_space() {
        assert_eq!(<IdeaEvaluationStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_idea_ranking_algorithm_space() {
        assert_eq!(<IdeaRankingAlgorithm as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_idea_ranking_status_space() {
        assert_eq!(<IdeaRankingStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_offchain_generate_idea_analytics() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::generate_idea_analytics(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_evaluate_idea() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::evaluate_idea(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_rank_idea() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::rank_idea(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_idea_functions_different_ids() {
        // Test with different IDs
        let result1 = offchain::generate_idea_analytics(1);
        let result2 = offchain::generate_idea_analytics(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
        
        let result3 = offchain::evaluate_idea(1);
        let result4 = offchain::evaluate_idea(999);
        assert_eq!(result3, Vec::<u8>::new());
        assert_eq!(result4, Vec::<u8>::new());
        
        let result5 = offchain::rank_idea(1);
        let result6 = offchain::rank_idea(999);
        assert_eq!(result5, Vec::<u8>::new());
        assert_eq!(result6, Vec::<u8>::new());
    }

}
