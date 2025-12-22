//! Security Policies module
//!
//! Security policies management for governance
//!
//! On-chain: Metadata for security policies
//! Off-chain: Actual policy enforcement, analysis

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Security policy status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum SecurityPolicyStatus {
    /// Policy active
    Active,
    /// Policy inactive
    Inactive,
    /// Policy draft
    Draft,
}

/// Security policy metadata (on-chain)
///
/// Stores metadata for security policies
#[account]
#[derive(InitSpace)]
pub struct SecurityPolicyMetadata {
    /// Policy ID
    pub policy_id: u64,
    /// Policy name
    #[max_len(100)]
    pub name: String,
    /// Status
    pub status: SecurityPolicyStatus,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// Policy data hash
    pub policy_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for security policies
pub mod onchain {
    use super::*;

    /// Initialize security policy
    pub fn initialize_policy(
        policy: &mut SecurityPolicyMetadata,
        policy_id: u64,
        name: String,
        policy_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(policy_id > 0, IndrasError::InvalidInput);
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= 100, IndrasError::InvalidInput);
        
        policy.policy_id = policy_id;
        policy.name = name;
        policy.status = SecurityPolicyStatus::Draft;
        policy.created_at = current_time;
        policy.updated_at = current_time;
        policy.policy_data_hash = policy_data_hash;
        policy.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for security policies
pub mod offchain {
    /// Enforce security policy
    pub fn enforce_policy(_policy_id: u64) -> bool {
        // Implementation in off-chain service
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_policy() -> SecurityPolicyMetadata {
        SecurityPolicyMetadata {
            policy_id: 1,
            name: "Test Policy".to_string(),
            status: SecurityPolicyStatus::Draft,
            created_at: 1000,
            updated_at: 1000,
            policy_data_hash: [0u8; 32],
            bump: 255,
        }
    }

    #[test]
    fn test_security_policy_status_variants() {
        assert_eq!(SecurityPolicyStatus::Active, SecurityPolicyStatus::Active);
        assert_eq!(SecurityPolicyStatus::Inactive, SecurityPolicyStatus::Inactive);
        assert_eq!(SecurityPolicyStatus::Draft, SecurityPolicyStatus::Draft);
    }

    #[test]
    fn test_security_policy_metadata_structure() {
        let policy = create_test_policy();
        assert_eq!(policy.policy_id, 1);
        assert_eq!(policy.name, "Test Policy");
        assert_eq!(policy.status, SecurityPolicyStatus::Draft);
        assert_eq!(policy.created_at, 1000);
        assert_eq!(policy.updated_at, 1000);
        assert_eq!(policy.bump, 255);
    }

    #[test]
    fn test_initialize_policy() {
        let mut policy = SecurityPolicyMetadata {
            policy_id: 0,
            name: String::new(),
            status: SecurityPolicyStatus::Draft,
            created_at: 0,
            updated_at: 0,
            policy_data_hash: [0u8; 32],
            bump: 0,
        };

        let data_hash = [4u8; 32];
        let result = onchain::initialize_policy(
            &mut policy,
            500,
            "New Security Policy".to_string(),
            data_hash,
            10000,
            64,
        );

        assert!(result.is_ok());
        assert_eq!(policy.policy_id, 500);
        assert_eq!(policy.name, "New Security Policy");
        assert_eq!(policy.status, SecurityPolicyStatus::Draft);
        assert_eq!(policy.created_at, 10000);
        assert_eq!(policy.updated_at, 10000);
        assert_eq!(policy.policy_data_hash, data_hash);
        assert_eq!(policy.bump, 64);
    }

    #[test]
    fn test_initialize_policy_invalid_id() {
        let mut policy = create_test_policy();

        let result = onchain::initialize_policy(
            &mut policy,
            0, // Invalid: policy_id must be > 0
            "Policy Name".to_string(),
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_policy_empty_name() {
        let mut policy = create_test_policy();

        let result = onchain::initialize_policy(
            &mut policy,
            1,
            String::new(), // Invalid: name must not be empty
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_policy_name_too_long() {
        let mut policy = create_test_policy();
        let long_name = "a".repeat(101); // 101 chars, max is 100

        let result = onchain::initialize_policy(
            &mut policy,
            1,
            long_name,
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_policy_name_max_length() {
        let mut policy = create_test_policy();
        let max_name = "a".repeat(100); // Exactly 100 chars

        let result = onchain::initialize_policy(
            &mut policy,
            1,
            max_name.clone(),
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(policy.name.len(), 100);
    }

    #[test]
    fn test_initialize_policy_status_always_draft_on_init() {
        let mut policy = create_test_policy();
        policy.status = SecurityPolicyStatus::Active;

        let result = onchain::initialize_policy(
            &mut policy,
            1,
            "New Policy".to_string(),
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        // Status should always be set to Draft on initialization
        assert_eq!(policy.status, SecurityPolicyStatus::Draft);
    }

    #[test]
    fn test_initialize_policy_data_hash() {
        let mut policy = create_test_policy();
        let custom_hash = [88u8; 32];

        let result = onchain::initialize_policy(
            &mut policy,
            1,
            "Policy Name".to_string(),
            custom_hash,
            2000,
            128,
        );

        assert!(result.is_ok());
        assert_eq!(policy.policy_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_policy_timestamps() {
        let mut policy = create_test_policy();

        let result = onchain::initialize_policy(
            &mut policy,
            1,
            "Policy Name".to_string(),
            [0u8; 32],
            12345,
            200,
        );

        assert!(result.is_ok());
        // Both created_at and updated_at should be set to current_time
        assert_eq!(policy.created_at, 12345);
        assert_eq!(policy.updated_at, 12345);
    }

    #[test]
    fn test_security_policy_status_enum_equality() {
        // Test that enum variants can be compared
        let status1 = SecurityPolicyStatus::Draft;
        let status2 = SecurityPolicyStatus::Draft;
        let status3 = SecurityPolicyStatus::Active;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_initialize_policy_name_exact_max_length() {
        let mut policy = create_test_policy();

        let max_name = "a".repeat(100); // Exactly max_len(100)
        let result = onchain::initialize_policy(
            &mut policy,
            1,
            max_name.clone(),
            [0u8; 32],
            1000,
            255,
        );

        assert!(result.is_ok());
        assert_eq!(policy.name.len(), 100);
    }

    #[test]
    fn test_initialize_policy_all_statuses() {
        let mut policy = create_test_policy();
        
        // Initialize should always set status to Draft
        let result = onchain::initialize_policy(
            &mut policy,
            1,
            "Policy".to_string(),
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(policy.status, SecurityPolicyStatus::Draft);
    }

    #[test]
    fn test_initialize_policy_custom_hash() {
        let mut policy = create_test_policy();
        let custom_hash = [255u8; 32];
        
        let result = onchain::initialize_policy(
            &mut policy,
            1,
            "Policy".to_string(),
            custom_hash,
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(policy.policy_data_hash, custom_hash);
    }

    #[test]
    fn test_initialize_policy_different_timestamps() {
        let mut policy = create_test_policy();
        
        let result = onchain::initialize_policy(
            &mut policy,
            1,
            "Policy".to_string(),
            [0u8; 32],
            5000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(policy.created_at, 5000);
        assert_eq!(policy.updated_at, 5000);
    }

    #[test]
    fn test_initialize_policy_different_bump_seeds() {
        let mut policy = create_test_policy();
        
        for bump in [0u8, 128u8, 255u8] {
            let result = onchain::initialize_policy(
                &mut policy,
                1,
                "Policy".to_string(),
                [0u8; 32],
                1000,
                bump,
            );
            
            assert!(result.is_ok());
            assert_eq!(policy.bump, bump);
        }
    }

    #[test]
    fn test_initialize_policy_large_policy_id() {
        let mut policy = create_test_policy();
        
        let result = onchain::initialize_policy(
            &mut policy,
            u64::MAX,
            "Policy".to_string(),
            [0u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(policy.policy_id, u64::MAX);
    }

    #[test]
    fn test_security_policy_status_equality() {
        assert_eq!(SecurityPolicyStatus::Active, SecurityPolicyStatus::Active);
        assert_ne!(SecurityPolicyStatus::Active, SecurityPolicyStatus::Inactive);
        assert_ne!(SecurityPolicyStatus::Active, SecurityPolicyStatus::Draft);
        assert_eq!(SecurityPolicyStatus::Inactive, SecurityPolicyStatus::Inactive);
        assert_ne!(SecurityPolicyStatus::Inactive, SecurityPolicyStatus::Draft);
        assert_eq!(SecurityPolicyStatus::Draft, SecurityPolicyStatus::Draft);
    }

    #[test]
    fn test_security_policy_metadata_name_variations() {
        let mut policy = create_test_policy();
        
        let names = vec![
            "A".to_string(),
            "Policy Name".to_string(),
            "Very Long Policy Name That Is Still Valid".to_string(),
            "a".repeat(50),
        ];
        
        for name in names {
            let result = onchain::initialize_policy(
                &mut policy,
                1,
                name.clone(),
                [0u8; 32],
                1000,
                255,
            );
            
            assert!(result.is_ok());
            assert_eq!(policy.name, name);
        }
    }

    #[test]
    fn test_security_policy_metadata_structure_all_fields() {
        let policy = SecurityPolicyMetadata {
            policy_id: 123,
            name: "Test Policy Name".to_string(),
            status: SecurityPolicyStatus::Active,
            created_at: 5000,
            updated_at: 6000,
            policy_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(policy.policy_id, 123);
        assert_eq!(policy.name, "Test Policy Name");
        assert_eq!(policy.status, SecurityPolicyStatus::Active);
        assert_eq!(policy.created_at, 5000);
        assert_eq!(policy.updated_at, 6000);
        assert_eq!(policy.policy_data_hash, [42u8; 32]);
        assert_eq!(policy.bump, 128);
    }

    #[test]
    fn test_initialize_policy_preserves_other_fields() {
        let mut policy = SecurityPolicyMetadata {
            policy_id: 999,
            name: "Old Name".to_string(),
            status: SecurityPolicyStatus::Active,
            created_at: 1000,
            updated_at: 2000,
            policy_data_hash: [1u8; 32],
            bump: 50,
        };
        
        let result = onchain::initialize_policy(
            &mut policy,
            1,
            "New Name".to_string(),
            [2u8; 32],
            3000,
            100,
        );
        
        assert!(result.is_ok());
        // All fields should be updated
        assert_eq!(policy.policy_id, 1);
        assert_eq!(policy.name, "New Name");
        assert_eq!(policy.status, SecurityPolicyStatus::Draft);
        assert_eq!(policy.created_at, 3000);
        assert_eq!(policy.updated_at, 3000);
        assert_eq!(policy.policy_data_hash, [2u8; 32]);
        assert_eq!(policy.bump, 100);
    }

    #[test]
    fn test_security_policy_status_all_variants_unique() {
        let statuses = vec![
            SecurityPolicyStatus::Active,
            SecurityPolicyStatus::Inactive,
            SecurityPolicyStatus::Draft,
        ];
        
        for i in 0..statuses.len() {
            for j in (i + 1)..statuses.len() {
                assert_ne!(statuses[i], statuses[j], "Duplicate status found");
            }
        }
    }

    #[test]
    fn test_security_policy_status_copy() {
        let status1 = SecurityPolicyStatus::Active;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_security_policy_status_space() {
        assert_eq!(<SecurityPolicyStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_offchain_enforce_policy() {
        // Test that offchain function exists and returns false (default)
        let result = offchain::enforce_policy(1);
        assert_eq!(result, false);
    }

    #[test]
    fn test_offchain_enforce_policy_different_ids() {
        // Test with different IDs
        let result1 = offchain::enforce_policy(1);
        let result2 = offchain::enforce_policy(999);
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }
}
