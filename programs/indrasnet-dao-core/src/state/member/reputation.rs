//! Member Reputation module
//!
//! Member reputation system
//!
//! On-chain: Metadata for member reputation
//! Off-chain: Actual reputation calculation, tracking

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Reputation factor
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MemberReputationFactor {
    /// Contribution factor
    Contribution,
    /// Quality factor
    Quality,
    /// Engagement factor
    Engagement,
    /// Custom factor
    Custom,
}

/// Reputation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MemberReputationStatus {
    /// Reputation active
    Active,
    /// Reputation paused
    Paused,
    /// Reputation disabled
    Disabled,
}

/// Member reputation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct MemberReputationMetadata {
    /// Reputation ID
    pub reputation_id: u64,
    /// Member ID
    pub member_id: u64,
    /// Reputation factor
    pub reputation_factor: MemberReputationFactor,
    /// Status
    pub status: MemberReputationStatus,
    /// Created at
    pub created_at: i64,
    /// Reputation config hash
    pub reputation_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_member_reputation(
        reputation: &mut MemberReputationMetadata,
        reputation_id: u64,
        member_id: u64,
        reputation_factor: MemberReputationFactor,
        reputation_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(reputation_id > 0, IndrasError::InvalidInput);
        reputation.reputation_id = reputation_id;
        reputation.member_id = member_id;
        reputation.reputation_factor = reputation_factor;
        reputation.status = MemberReputationStatus::Active;
        reputation.created_at = current_time;
        reputation.reputation_config_hash = reputation_config_hash;
        reputation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn calculate_reputation(_reputation_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_member_reputation() {
        let mut reputation = MemberReputationMetadata {
            reputation_id: 0,
            member_id: 0,
            reputation_factor: MemberReputationFactor::Contribution,
            status: MemberReputationStatus::Disabled,
            created_at: 0,
            reputation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_reputation(
            &mut reputation,
            1,
            10,
            MemberReputationFactor::Quality,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(reputation.reputation_id, 1);
        assert_eq!(reputation.member_id, 10);
        assert_eq!(reputation.reputation_factor, MemberReputationFactor::Quality);
        assert_eq!(reputation.status, MemberReputationStatus::Active);
        assert_eq!(reputation.created_at, 1000);
        assert_eq!(reputation.bump, 255);
    }

    #[test]
    fn test_initialize_member_reputation_invalid_id() {
        let mut reputation = MemberReputationMetadata {
            reputation_id: 0,
            member_id: 0,
            reputation_factor: MemberReputationFactor::Contribution,
            status: MemberReputationStatus::Disabled,
            created_at: 0,
            reputation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_reputation(
            &mut reputation,
            0, // Invalid: must be > 0
            10,
            MemberReputationFactor::Quality,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_member_reputation_factor_variants() {
        assert_eq!(MemberReputationFactor::Contribution, MemberReputationFactor::Contribution);
        assert_eq!(MemberReputationFactor::Quality, MemberReputationFactor::Quality);
        assert_eq!(MemberReputationFactor::Engagement, MemberReputationFactor::Engagement);
        assert_eq!(MemberReputationFactor::Custom, MemberReputationFactor::Custom);
    }

    #[test]
    fn test_member_reputation_status_variants() {
        assert_eq!(MemberReputationStatus::Active, MemberReputationStatus::Active);
        assert_eq!(MemberReputationStatus::Paused, MemberReputationStatus::Paused);
        assert_eq!(MemberReputationStatus::Disabled, MemberReputationStatus::Disabled);
    }

    fn create_test_reputation() -> MemberReputationMetadata {
        MemberReputationMetadata {
            reputation_id: 1,
            member_id: 100,
            reputation_factor: MemberReputationFactor::Contribution,
            status: MemberReputationStatus::Active,
            created_at: 1000,
            reputation_config_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_member_reputation_metadata_structure() {
        let reputation = create_test_reputation();
        assert_eq!(reputation.reputation_id, 1);
        assert_eq!(reputation.member_id, 100);
        assert_eq!(reputation.reputation_factor, MemberReputationFactor::Contribution);
        assert_eq!(reputation.status, MemberReputationStatus::Active);
        assert_eq!(reputation.created_at, 1000);
        assert_eq!(reputation.bump, 255);
    }

    #[test]
    fn test_initialize_member_reputation_all_factors() {
        let factors = vec![
            MemberReputationFactor::Contribution,
            MemberReputationFactor::Quality,
            MemberReputationFactor::Engagement,
            MemberReputationFactor::Custom,
        ];

        for factor in factors {
            let mut reputation = MemberReputationMetadata {
                reputation_id: 0,
                member_id: 0,
                reputation_factor: MemberReputationFactor::Contribution,
                status: MemberReputationStatus::Disabled,
                created_at: 0,
                reputation_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_member_reputation(
                &mut reputation,
                1,
                1,
                factor,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(reputation.reputation_factor, factor);
        }
    }

    #[test]
    fn test_initialize_member_reputation_status_always_active_on_init() {
        let mut reputation = create_test_reputation();
        reputation.status = MemberReputationStatus::Paused;

        let result = onchain::initialize_member_reputation(
            &mut reputation,
            1,
            1,
            MemberReputationFactor::Contribution,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(reputation.status, MemberReputationStatus::Active);
    }

    #[test]
    fn test_initialize_member_reputation_config_hash() {
        let mut reputation = create_test_reputation();
        let custom_hash = [123u8; 32];

        let result = onchain::initialize_member_reputation(
            &mut reputation,
            1,
            1,
            MemberReputationFactor::Quality,
            custom_hash,
            2000,
            128,
        );

        assert!(result.is_ok());
        assert_eq!(reputation.reputation_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_member_reputation_member_id() {
        let mut reputation = create_test_reputation();

        let result = onchain::initialize_member_reputation(
            &mut reputation,
            1,
            99999,
            MemberReputationFactor::Engagement,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(reputation.member_id, 99999);
    }

    #[test]
    fn test_initialize_member_reputation_timestamp() {
        let mut reputation = create_test_reputation();

        let result = onchain::initialize_member_reputation(
            &mut reputation,
            1,
            1,
            MemberReputationFactor::Custom,
            [0u8; 32],
            98765,
            200,
        );

        assert!(result.is_ok());
        assert_eq!(reputation.created_at, 98765);
    }

    #[test]
    fn test_initialize_member_reputation_bump_seed() {
        let mut reputation = create_test_reputation();

        for bump in [0u8, 100u8, 255u8] {
            let result = onchain::initialize_member_reputation(
                &mut reputation,
                1,
                1,
                MemberReputationFactor::Contribution,
                [0u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(reputation.bump, bump);
        }
    }

    #[test]
    fn test_member_reputation_factor_enum_equality() {
        // Test that enum variants can be compared
        let factor1 = MemberReputationFactor::Contribution;
        let factor2 = MemberReputationFactor::Contribution;
        let factor3 = MemberReputationFactor::Quality;

        assert_eq!(factor1, factor2);
        assert_ne!(factor1, factor3);
    }

    #[test]
    fn test_member_reputation_status_enum_equality() {
        // Test that enum variants can be compared
        let status1 = MemberReputationStatus::Active;
        let status2 = MemberReputationStatus::Active;
        let status3 = MemberReputationStatus::Paused;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_member_reputation_factor_all_variants_unique() {
        let factors = vec![
            MemberReputationFactor::Contribution,
            MemberReputationFactor::Quality,
            MemberReputationFactor::Engagement,
            MemberReputationFactor::Custom,
        ];
        
        for i in 0..factors.len() {
            for j in (i + 1)..factors.len() {
                assert_ne!(factors[i], factors[j], "Duplicate factor found");
            }
        }
    }

    #[test]
    fn test_member_reputation_status_all_variants_unique() {
        let statuses = vec![
            MemberReputationStatus::Active,
            MemberReputationStatus::Paused,
            MemberReputationStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_member_reputation_factor_equality() {
        assert_eq!(MemberReputationFactor::Contribution, MemberReputationFactor::Contribution);
        assert_ne!(MemberReputationFactor::Contribution, MemberReputationFactor::Quality);
        assert_eq!(MemberReputationFactor::Quality, MemberReputationFactor::Quality);
        assert_ne!(MemberReputationFactor::Quality, MemberReputationFactor::Engagement);
        assert_eq!(MemberReputationFactor::Engagement, MemberReputationFactor::Engagement);
        assert_ne!(MemberReputationFactor::Engagement, MemberReputationFactor::Custom);
        assert_eq!(MemberReputationFactor::Custom, MemberReputationFactor::Custom);
    }

    #[test]
    fn test_member_reputation_status_equality() {
        assert_eq!(MemberReputationStatus::Active, MemberReputationStatus::Active);
        assert_ne!(MemberReputationStatus::Active, MemberReputationStatus::Paused);
        assert_eq!(MemberReputationStatus::Paused, MemberReputationStatus::Paused);
        assert_ne!(MemberReputationStatus::Paused, MemberReputationStatus::Disabled);
        assert_eq!(MemberReputationStatus::Disabled, MemberReputationStatus::Disabled);
    }

    #[test]
    fn test_member_reputation_factor_copy() {
        let factor1 = MemberReputationFactor::Contribution;
        let factor2 = factor1; // Copy trait
        assert_eq!(factor1, factor2);
    }

    #[test]
    fn test_member_reputation_status_copy() {
        let status1 = MemberReputationStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_member_reputation_factor_space() {
        assert_eq!(<MemberReputationFactor as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_member_reputation_status_space() {
        assert_eq!(<MemberReputationStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_member_reputation_large_ids() {
        let mut reputation = MemberReputationMetadata {
            reputation_id: 0,
            member_id: 0,
            reputation_factor: MemberReputationFactor::Contribution,
            status: MemberReputationStatus::Disabled,
            created_at: 0,
            reputation_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_member_reputation(
            &mut reputation,
            u64::MAX,
            u64::MAX,
            MemberReputationFactor::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(reputation.reputation_id, u64::MAX);
        assert_eq!(reputation.member_id, u64::MAX);
    }

    #[test]
    fn test_member_reputation_metadata_all_fields() {
        let reputation = MemberReputationMetadata {
            reputation_id: 123,
            member_id: 456,
            reputation_factor: MemberReputationFactor::Engagement,
            status: MemberReputationStatus::Paused,
            created_at: 5000,
            reputation_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(reputation.reputation_id, 123);
        assert_eq!(reputation.member_id, 456);
        assert_eq!(reputation.reputation_factor, MemberReputationFactor::Engagement);
        assert_eq!(reputation.status, MemberReputationStatus::Paused);
        assert_eq!(reputation.created_at, 5000);
        assert_eq!(reputation.reputation_config_hash, [42u8; 32]);
        assert_eq!(reputation.bump, 128);
    }

}
