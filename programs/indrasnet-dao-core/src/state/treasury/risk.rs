//! Treasury Risk Management module
//!
//! Treasury risk management
//!
//! On-chain: Metadata for treasury risk management
//! Off-chain: Actual risk management, monitoring

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Risk type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryRiskType {
    /// Market risk
    Market,
    /// Credit risk
    Credit,
    /// Liquidity risk
    Liquidity,
    /// Custom risk
    Custom,
}

/// Risk status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryRiskStatus {
    /// Risk assessed
    Assessed,
    /// Risk mitigated
    Mitigated,
    /// Risk active
    Active,
}

/// Treasury risk management metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct TreasuryRiskManagementMetadata {
    /// Risk ID
    pub risk_id: u64,
    /// Treasury ID
    pub treasury_id: u64,
    /// Risk type
    pub risk_type: TreasuryRiskType,
    /// Status
    pub status: TreasuryRiskStatus,
    /// Created at
    pub created_at: i64,
    /// Risk data hash
    pub risk_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_treasury_risk_management(
        risk: &mut TreasuryRiskManagementMetadata,
        risk_id: u64,
        treasury_id: u64,
        risk_type: TreasuryRiskType,
        risk_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(risk_id > 0, IndrasError::InvalidInput);
        risk.risk_id = risk_id;
        risk.treasury_id = treasury_id;
        risk.risk_type = risk_type;
        risk.status = TreasuryRiskStatus::Assessed;
        risk.created_at = current_time;
        risk.risk_data_hash = risk_data_hash;
        risk.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn manage_treasury_risk(_risk_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_treasury_risk_management() {
        let mut risk = TreasuryRiskManagementMetadata {
            risk_id: 0,
            treasury_id: 0,
            risk_type: TreasuryRiskType::Market,
            status: TreasuryRiskStatus::Active,
            created_at: 0,
            risk_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_risk_management(
            &mut risk,
            1,
            10,
            TreasuryRiskType::Liquidity,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(risk.risk_id, 1);
        assert_eq!(risk.treasury_id, 10);
        assert_eq!(risk.risk_type, TreasuryRiskType::Liquidity);
        assert_eq!(risk.status, TreasuryRiskStatus::Assessed);
        assert_eq!(risk.created_at, 1000);
        assert_eq!(risk.bump, 255);
    }

    #[test]
    fn test_initialize_treasury_risk_management_invalid_id() {
        let mut risk = TreasuryRiskManagementMetadata {
            risk_id: 0,
            treasury_id: 0,
            risk_type: TreasuryRiskType::Market,
            status: TreasuryRiskStatus::Active,
            created_at: 0,
            risk_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_risk_management(
            &mut risk,
            0, // Invalid: must be > 0
            10,
            TreasuryRiskType::Liquidity,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_treasury_risk_type_variants() {
        assert_eq!(TreasuryRiskType::Market, TreasuryRiskType::Market);
        assert_eq!(TreasuryRiskType::Credit, TreasuryRiskType::Credit);
        assert_eq!(TreasuryRiskType::Liquidity, TreasuryRiskType::Liquidity);
        assert_eq!(TreasuryRiskType::Custom, TreasuryRiskType::Custom);
    }

    #[test]
    fn test_treasury_risk_status_variants() {
        assert_eq!(TreasuryRiskStatus::Assessed, TreasuryRiskStatus::Assessed);
        assert_eq!(TreasuryRiskStatus::Mitigated, TreasuryRiskStatus::Mitigated);
        assert_eq!(TreasuryRiskStatus::Active, TreasuryRiskStatus::Active);
    }

    #[test]
    fn test_treasury_risk_type_all_variants_unique() {
        let types = vec![
            TreasuryRiskType::Market,
            TreasuryRiskType::Credit,
            TreasuryRiskType::Liquidity,
            TreasuryRiskType::Custom,
        ];
        
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j], "Duplicate type found");
            }
        }
    }

    #[test]
    fn test_treasury_risk_status_all_variants_unique() {
        let statuses = vec![
            TreasuryRiskStatus::Assessed,
            TreasuryRiskStatus::Mitigated,
            TreasuryRiskStatus::Active,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_treasury_risk_type_equality() {
        assert_eq!(TreasuryRiskType::Market, TreasuryRiskType::Market);
        assert_ne!(TreasuryRiskType::Market, TreasuryRiskType::Credit);
        assert_eq!(TreasuryRiskType::Credit, TreasuryRiskType::Credit);
        assert_ne!(TreasuryRiskType::Credit, TreasuryRiskType::Liquidity);
        assert_eq!(TreasuryRiskType::Liquidity, TreasuryRiskType::Liquidity);
        assert_ne!(TreasuryRiskType::Liquidity, TreasuryRiskType::Custom);
        assert_eq!(TreasuryRiskType::Custom, TreasuryRiskType::Custom);
    }

    #[test]
    fn test_treasury_risk_status_equality() {
        assert_eq!(TreasuryRiskStatus::Assessed, TreasuryRiskStatus::Assessed);
        assert_ne!(TreasuryRiskStatus::Assessed, TreasuryRiskStatus::Mitigated);
        assert_eq!(TreasuryRiskStatus::Mitigated, TreasuryRiskStatus::Mitigated);
        assert_ne!(TreasuryRiskStatus::Mitigated, TreasuryRiskStatus::Active);
        assert_eq!(TreasuryRiskStatus::Active, TreasuryRiskStatus::Active);
    }

    #[test]
    fn test_treasury_risk_type_copy() {
        let type1 = TreasuryRiskType::Market;
        let type2 = type1; // Copy trait
        assert_eq!(type1, type2);
    }

    #[test]
    fn test_treasury_risk_status_copy() {
        let status1 = TreasuryRiskStatus::Assessed;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_treasury_risk_type_space() {
        assert_eq!(<TreasuryRiskType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_treasury_risk_status_space() {
        assert_eq!(<TreasuryRiskStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_initialize_treasury_risk_management_all_types() {
        let types = vec![
            TreasuryRiskType::Market,
            TreasuryRiskType::Credit,
            TreasuryRiskType::Liquidity,
            TreasuryRiskType::Custom,
        ];
        
        for risk_type in types {
            let mut risk = TreasuryRiskManagementMetadata {
                risk_id: 0,
                treasury_id: 0,
                risk_type: TreasuryRiskType::Market,
                status: TreasuryRiskStatus::Active,
                created_at: 0,
                risk_data_hash: [0u8; 32],
                bump: 0,
            };
            
            let result = onchain::initialize_treasury_risk_management(
                &mut risk,
                1,
                10,
                risk_type,
                [0u8; 32],
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(risk.risk_type, risk_type);
        }
    }

    #[test]
    fn test_initialize_treasury_risk_management_large_ids() {
        let mut risk = TreasuryRiskManagementMetadata {
            risk_id: 0,
            treasury_id: 0,
            risk_type: TreasuryRiskType::Market,
            status: TreasuryRiskStatus::Active,
            created_at: 0,
            risk_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_risk_management(
            &mut risk,
            u64::MAX,
            u64::MAX,
            TreasuryRiskType::Custom,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(risk.risk_id, u64::MAX);
        assert_eq!(risk.treasury_id, u64::MAX);
    }

    #[test]
    fn test_initialize_treasury_risk_management_custom_hash() {
        let mut risk = TreasuryRiskManagementMetadata {
            risk_id: 0,
            treasury_id: 0,
            risk_type: TreasuryRiskType::Market,
            status: TreasuryRiskStatus::Active,
            created_at: 0,
            risk_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let custom_hash = [255u8; 32];
        let result = onchain::initialize_treasury_risk_management(
            &mut risk,
            1,
            10,
            TreasuryRiskType::Custom,
            custom_hash,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(risk.risk_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_treasury_risk_management_always_assessed_on_init() {
        let mut risk = TreasuryRiskManagementMetadata {
            risk_id: 0,
            treasury_id: 0,
            risk_type: TreasuryRiskType::Market,
            status: TreasuryRiskStatus::Active, // Will be reset
            created_at: 0,
            risk_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_risk_management(
            &mut risk,
            1,
            10,
            TreasuryRiskType::Market,
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Assessed on initialization
        assert_eq!(risk.status, TreasuryRiskStatus::Assessed);
    }

    #[test]
    fn test_treasury_risk_management_metadata_all_fields() {
        let risk = TreasuryRiskManagementMetadata {
            risk_id: 123,
            treasury_id: 456,
            risk_type: TreasuryRiskType::Credit,
            status: TreasuryRiskStatus::Mitigated,
            created_at: 5000,
            risk_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(risk.risk_id, 123);
        assert_eq!(risk.treasury_id, 456);
        assert_eq!(risk.risk_type, TreasuryRiskType::Credit);
        assert_eq!(risk.status, TreasuryRiskStatus::Mitigated);
        assert_eq!(risk.created_at, 5000);
        assert_eq!(risk.risk_data_hash, [42u8; 32]);
        assert_eq!(risk.bump, 128);
    }

    #[test]
    fn test_offchain_manage_treasury_risk() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::manage_treasury_risk(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_manage_treasury_risk_different_ids() {
        // Test with different IDs
        let result1 = offchain::manage_treasury_risk(1);
        let result2 = offchain::manage_treasury_risk(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }
}
