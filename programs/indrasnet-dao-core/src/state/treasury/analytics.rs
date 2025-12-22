//! Treasury Analytics module
//!
//! Treasury analytics and reporting
//!
//! On-chain: Metadata for treasury analytics
//! Off-chain: Actual analytics, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Analytics type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryAnalyticsType {
    /// Balance analytics
    Balance,
    /// Flow analytics
    Flow,
    /// Performance analytics
    Performance,
    /// Custom analytics
    Custom,
}

/// Analytics status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryAnalyticsStatus {
    /// Analytics active
    Active,
    /// Analytics paused
    Paused,
    /// Analytics disabled
    Disabled,
}

/// Treasury analytics metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct TreasuryAnalyticsMetadata {
    /// Analytics ID
    pub analytics_id: u64,
    /// Treasury ID
    pub treasury_id: u64,
    /// Analytics type
    pub analytics_type: TreasuryAnalyticsType,
    /// Status
    pub status: TreasuryAnalyticsStatus,
    /// Created at
    pub created_at: i64,
    /// Analytics config hash
    pub analytics_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_treasury_analytics(
        analytics: &mut TreasuryAnalyticsMetadata,
        analytics_id: u64,
        treasury_id: u64,
        analytics_type: TreasuryAnalyticsType,
        analytics_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(analytics_id > 0, IndrasError::InvalidInput);
        analytics.analytics_id = analytics_id;
        analytics.treasury_id = treasury_id;
        analytics.analytics_type = analytics_type;
        analytics.status = TreasuryAnalyticsStatus::Active;
        analytics.created_at = current_time;
        analytics.analytics_config_hash = analytics_config_hash;
        analytics.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn generate_treasury_analytics(_analytics_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_treasury_analytics() {
        let mut analytics = TreasuryAnalyticsMetadata {
            analytics_id: 0,
            treasury_id: 0,
            analytics_type: TreasuryAnalyticsType::Balance,
            status: TreasuryAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_analytics(
            &mut analytics,
            1,
            10,
            TreasuryAnalyticsType::Performance,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, 1);
        assert_eq!(analytics.treasury_id, 10);
        assert_eq!(analytics.analytics_type, TreasuryAnalyticsType::Performance);
        assert_eq!(analytics.status, TreasuryAnalyticsStatus::Active);
        assert_eq!(analytics.created_at, 1000);
        assert_eq!(analytics.bump, 255);
    }

    #[test]
    fn test_initialize_treasury_analytics_invalid_id() {
        let mut analytics = TreasuryAnalyticsMetadata {
            analytics_id: 0,
            treasury_id: 0,
            analytics_type: TreasuryAnalyticsType::Balance,
            status: TreasuryAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_analytics(
            &mut analytics,
            0, // Invalid: must be > 0
            10,
            TreasuryAnalyticsType::Performance,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_treasury_analytics_type_variants() {
        assert_eq!(TreasuryAnalyticsType::Balance, TreasuryAnalyticsType::Balance);
        assert_eq!(TreasuryAnalyticsType::Flow, TreasuryAnalyticsType::Flow);
        assert_eq!(TreasuryAnalyticsType::Performance, TreasuryAnalyticsType::Performance);
        assert_eq!(TreasuryAnalyticsType::Custom, TreasuryAnalyticsType::Custom);
    }

    #[test]
    fn test_treasury_analytics_status_variants() {
        assert_eq!(TreasuryAnalyticsStatus::Active, TreasuryAnalyticsStatus::Active);
        assert_eq!(TreasuryAnalyticsStatus::Paused, TreasuryAnalyticsStatus::Paused);
        assert_eq!(TreasuryAnalyticsStatus::Disabled, TreasuryAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_treasury_analytics_type_all_variants_unique() {
        let types = vec![
            TreasuryAnalyticsType::Balance,
            TreasuryAnalyticsType::Flow,
            TreasuryAnalyticsType::Performance,
            TreasuryAnalyticsType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_treasury_analytics_status_all_variants_unique() {
        let statuses = vec![
            TreasuryAnalyticsStatus::Active,
            TreasuryAnalyticsStatus::Paused,
            TreasuryAnalyticsStatus::Disabled,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_treasury_analytics_type_equality() {
        assert_eq!(TreasuryAnalyticsType::Balance, TreasuryAnalyticsType::Balance);
        assert_ne!(TreasuryAnalyticsType::Balance, TreasuryAnalyticsType::Flow);
        assert_eq!(TreasuryAnalyticsType::Flow, TreasuryAnalyticsType::Flow);
        assert_ne!(TreasuryAnalyticsType::Flow, TreasuryAnalyticsType::Performance);
        assert_eq!(TreasuryAnalyticsType::Performance, TreasuryAnalyticsType::Performance);
        assert_ne!(TreasuryAnalyticsType::Performance, TreasuryAnalyticsType::Custom);
        assert_eq!(TreasuryAnalyticsType::Custom, TreasuryAnalyticsType::Custom);
    }

    #[test]
    fn test_treasury_analytics_status_equality() {
        assert_eq!(TreasuryAnalyticsStatus::Active, TreasuryAnalyticsStatus::Active);
        assert_ne!(TreasuryAnalyticsStatus::Active, TreasuryAnalyticsStatus::Paused);
        assert_eq!(TreasuryAnalyticsStatus::Paused, TreasuryAnalyticsStatus::Paused);
        assert_ne!(TreasuryAnalyticsStatus::Paused, TreasuryAnalyticsStatus::Disabled);
        assert_eq!(TreasuryAnalyticsStatus::Disabled, TreasuryAnalyticsStatus::Disabled);
    }

    #[test]
    fn test_treasury_analytics_type_copy() {
        let type1 = TreasuryAnalyticsType::Balance;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_treasury_analytics_status_copy() {
        let status1 = TreasuryAnalyticsStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_treasury_analytics_type_space() {
        assert_eq!(<TreasuryAnalyticsType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_treasury_analytics_status_space() {
        assert_eq!(<TreasuryAnalyticsStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_treasury_analytics_all_types() {
        let types = vec![
            TreasuryAnalyticsType::Balance,
            TreasuryAnalyticsType::Flow,
            TreasuryAnalyticsType::Performance,
            TreasuryAnalyticsType::Custom,
        ];
        
        for analytics_type in types {
            let mut analytics = TreasuryAnalyticsMetadata {
                analytics_id: 0,
                treasury_id: 0,
                analytics_type: TreasuryAnalyticsType::Balance,
                status: TreasuryAnalyticsStatus::Disabled,
                created_at: 0,
                analytics_config_hash: [0u8; 32],
                bump: 0,
            };
            
            let result = onchain::initialize_treasury_analytics(
                &mut analytics,
                1,
                10,
                analytics_type,
                [0u8; 32],
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(analytics.analytics_type, analytics_type);
        }
    }

    #[test]
    fn test_initialize_treasury_analytics_large_ids() {
        let mut analytics = TreasuryAnalyticsMetadata {
            analytics_id: 0,
            treasury_id: 0,
            analytics_type: TreasuryAnalyticsType::Balance,
            status: TreasuryAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_analytics(
            &mut analytics,
            u64::MAX,
            u64::MAX,
            TreasuryAnalyticsType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_id, u64::MAX);
        assert_eq!(analytics.treasury_id, u64::MAX);
    }

    #[test]
    fn test_initialize_treasury_analytics_custom_hash() {
        let mut analytics = TreasuryAnalyticsMetadata {
            analytics_id: 0,
            treasury_id: 0,
            analytics_type: TreasuryAnalyticsType::Balance,
            status: TreasuryAnalyticsStatus::Disabled,
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let custom_hash = [255u8; 32];
        let result = onchain::initialize_treasury_analytics(
            &mut analytics,
            1,
            10,
            TreasuryAnalyticsType::Custom,
            custom_hash,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(analytics.analytics_config_hash, custom_hash);
    }

    #[test]
    fn test_initialize_treasury_analytics_always_active_on_init() {
        let mut analytics = TreasuryAnalyticsMetadata {
            analytics_id: 0,
            treasury_id: 0,
            analytics_type: TreasuryAnalyticsType::Balance,
            status: TreasuryAnalyticsStatus::Disabled, // Will be reset
            created_at: 0,
            analytics_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_analytics(
            &mut analytics,
            1,
            10,
            TreasuryAnalyticsType::Balance,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Active on initialization
        assert_eq!(analytics.status, TreasuryAnalyticsStatus::Active);
    }

    #[test]
    fn test_treasury_analytics_metadata_all_fields() {
        let analytics = TreasuryAnalyticsMetadata {
            analytics_id: 123,
            treasury_id: 456,
            analytics_type: TreasuryAnalyticsType::Flow,
            status: TreasuryAnalyticsStatus::Paused,
            created_at: 5000,
            analytics_config_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(analytics.analytics_id, 123);
        assert_eq!(analytics.treasury_id, 456);
        assert_eq!(analytics.analytics_type, TreasuryAnalyticsType::Flow);
        assert_eq!(analytics.status, TreasuryAnalyticsStatus::Paused);
        assert_eq!(analytics.created_at, 5000);
        assert_eq!(analytics.analytics_config_hash, [42u8; 32]);
        assert_eq!(analytics.bump, 128);
    }

    #[test]
    fn test_offchain_generate_treasury_analytics() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::generate_treasury_analytics(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_generate_treasury_analytics_different_ids() {
        // Test with different IDs
        let result1 = offchain::generate_treasury_analytics(1);
        let result2 = offchain::generate_treasury_analytics(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
