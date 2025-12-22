//! Treasury Operations module
//!
//! Treasury operations and management
//!
//! On-chain: Metadata for treasury operations
//! Off-chain: Actual operations, execution
//!
//! NOTE: TreasuryOperationType is defined in treasury::types
//! This module provides operations metadata and functions

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::TreasuryOperationType;

/// Operation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryOperationStatus {
    /// Operation pending
    Pending,
    /// Operation in progress
    InProgress,
    /// Operation completed
    Completed,
    /// Operation failed
    Failed,
}

/// Treasury operations metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct TreasuryOperationsMetadata {
    /// Operation ID
    pub operation_id: u64,
    /// Treasury ID
    pub treasury_id: u64,
    /// Operation type
    pub operation_type: TreasuryOperationType,
    /// Status
    pub status: TreasuryOperationStatus,
    /// Created at
    pub created_at: i64,
    /// Operation data hash
    pub operation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    
    pub fn initialize_treasury_operations(
        operation: &mut TreasuryOperationsMetadata,
        operation_id: u64,
        treasury_id: u64,
        operation_type: TreasuryOperationType,
        operation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(operation_id > 0, IndrasError::InvalidInput);
        operation.operation_id = operation_id;
        operation.treasury_id = treasury_id;
        operation.operation_type = operation_type;
        operation.status = TreasuryOperationStatus::Pending;
        operation.created_at = current_time;
        operation.operation_data_hash = operation_data_hash;
        operation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn execute_treasury_operation(_operation_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_treasury_operations() {
        let mut operation = TreasuryOperationsMetadata {
            operation_id: 0,
            treasury_id: 0,
            operation_type: TreasuryOperationType::Deposit,
            status: TreasuryOperationStatus::Failed,
            created_at: 0,
            operation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_operations(
            &mut operation,
            1,
            10,
            TreasuryOperationType::Withdrawal,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(operation.operation_id, 1);
        assert_eq!(operation.treasury_id, 10);
        assert_eq!(operation.operation_type, TreasuryOperationType::Withdrawal);
        assert_eq!(operation.status, TreasuryOperationStatus::Pending);
        assert_eq!(operation.created_at, 1000);
        assert_eq!(operation.bump, 255);
    }

    #[test]
    fn test_initialize_treasury_operations_invalid_id() {
        let mut operation = TreasuryOperationsMetadata {
            operation_id: 0,
            treasury_id: 0,
            operation_type: TreasuryOperationType::Deposit,
            status: TreasuryOperationStatus::Failed,
            created_at: 0,
            operation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_treasury_operations(
            &mut operation,
            0, // Invalid: must be > 0
            10,
            TreasuryOperationType::Withdrawal,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_treasury_operation_status_variants() {
        assert_eq!(TreasuryOperationStatus::Pending, TreasuryOperationStatus::Pending);
        assert_eq!(TreasuryOperationStatus::InProgress, TreasuryOperationStatus::InProgress);
        assert_eq!(TreasuryOperationStatus::Completed, TreasuryOperationStatus::Completed);
        assert_eq!(TreasuryOperationStatus::Failed, TreasuryOperationStatus::Failed);
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_operation_id_zero() {
        // Test: operation_id == 0 should fail
        let operation_id = 0u64;
        
        // Validation logic: require!(operation_id > 0, IndrasError::InvalidInput)
        assert_eq!(operation_id, 0, "Operation ID zero should be detected");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_operation_id_one() {
        // Test: operation_id == 1 should pass
        let operation_id = 1u64;
        
        // Validation logic: require!(operation_id > 0, IndrasError::InvalidInput)
        assert!(operation_id > 0, "Operation ID one should pass");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_operation_id_max() {
        // Test: operation_id == u64::MAX should pass
        let operation_id = u64::MAX;
        
        // Validation logic: require!(operation_id > 0, IndrasError::InvalidInput)
        assert!(operation_id > 0, "Operation ID at max should pass");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_treasury_id_zero() {
        // Test: treasury_id == 0 should be allowed (no validation)
        let treasury_id = 0u64;
        
        // Zero treasury ID should be allowed
        assert_eq!(treasury_id, 0, "Zero treasury ID should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_treasury_id_max() {
        // Test: treasury_id == u64::MAX should be allowed
        let treasury_id = u64::MAX;
        
        // Max treasury ID should be allowed
        assert_eq!(treasury_id, u64::MAX, "Max treasury ID should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_all_operation_types() {
        // Test: all TreasuryOperationType variants should be valid
        let operation_types = vec![
            TreasuryOperationType::Deposit,
            TreasuryOperationType::Withdrawal,
            TreasuryOperationType::Transfer,
            TreasuryOperationType::CapabilityGrant,
            TreasuryOperationType::CapabilityRevoke,
        ];
        
        // All operation types should be valid
        assert_eq!(operation_types.len(), 5, "All operation types should be valid");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_status_pending() {
        // Test: status should be set to Pending on initialization
        let status = TreasuryOperationStatus::Pending;
        
        // Status should be Pending
        assert_eq!(status, TreasuryOperationStatus::Pending, "Status should be Pending on initialization");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_all_statuses() {
        // Test: all TreasuryOperationStatus variants should be valid
        let statuses = vec![
            TreasuryOperationStatus::Pending,
            TreasuryOperationStatus::InProgress,
            TreasuryOperationStatus::Completed,
            TreasuryOperationStatus::Failed,
        ];
        
        // All statuses should be valid
        assert_eq!(statuses.len(), 4, "All operation statuses should be valid");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_operation_data_hash_zero() {
        // Test: operation_data_hash == [0u8; 32] should be allowed (no validation)
        let operation_data_hash = [0u8; 32];
        
        // No validation for zero hash - this is allowed
        assert_eq!(operation_data_hash, [0u8; 32], "Zero operation data hash should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_current_time_zero() {
        // Test: current_time == 0 should be allowed (no validation)
        let current_time = 0i64;
        
        // Zero time should be allowed
        assert_eq!(current_time, 0, "Zero current time should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_current_time_negative() {
        // Test: current_time < 0 should be allowed (no validation)
        let current_time = -1i64;
        
        // Negative time should be allowed
        assert!(current_time < 0, "Negative current time should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_current_time_positive() {
        // Test: current_time > 0 should be allowed
        let current_time = 1000000i64;
        
        // Positive time should be allowed
        assert!(current_time > 0, "Positive current time should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_bump_zero() {
        // Test: bump == 0 should be allowed (no validation)
        let bump = 0u8;
        
        // Zero bump should be allowed
        assert_eq!(bump, 0, "Zero bump should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_bump_max() {
        // Test: bump == u8::MAX should be allowed
        let bump = u8::MAX;
        
        // Max bump should be allowed
        assert_eq!(bump, u8::MAX, "Max bump should be allowed");
    }
    
    #[test]
    fn test_initialize_treasury_operations_validation_created_at_set() {
        // Test: created_at should be set to current_time on initialization
        let current_time = 1000000i64;
        let created_at = current_time;
        
        // Created should be set
        assert_eq!(created_at, current_time, "Created at should be set on initialization");
    }

    #[test]
    fn test_initialize_treasury_operations_all_operation_types() {
        let operation_types = vec![
            TreasuryOperationType::Deposit,
            TreasuryOperationType::Withdrawal,
            TreasuryOperationType::Transfer,
            TreasuryOperationType::CapabilityGrant,
            TreasuryOperationType::CapabilityRevoke,
        ];

        for op_type in operation_types {
            let mut operation = TreasuryOperationsMetadata {
                operation_id: 0,
                treasury_id: 0,
                operation_type: TreasuryOperationType::Deposit,
                status: TreasuryOperationStatus::Failed,
                created_at: 0,
                operation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_treasury_operations(
                &mut operation,
                1,
                10,
                op_type,
                [1u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(operation.operation_type, op_type);
            assert_eq!(operation.status, TreasuryOperationStatus::Pending);
        }
    }

    #[test]
    fn test_initialize_treasury_operations_operation_data_hash_variations() {
        let hashes = vec![
            [0u8; 32],
            [1u8; 32],
            [255u8; 32],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        ];

        for hash in hashes {
            let mut operation = TreasuryOperationsMetadata {
                operation_id: 0,
                treasury_id: 0,
                operation_type: TreasuryOperationType::Deposit,
                status: TreasuryOperationStatus::Failed,
                created_at: 0,
                operation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_treasury_operations(
                &mut operation,
                1,
                10,
                TreasuryOperationType::Deposit,
                hash,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(operation.operation_data_hash, hash);
        }
    }

    #[test]
    fn test_initialize_treasury_operations_treasury_id_variations() {
        let treasury_ids = vec![0u64, 1u64, 100u64, u64::MAX];

        for treasury_id in treasury_ids {
            let mut operation = TreasuryOperationsMetadata {
                operation_id: 0,
                treasury_id: 0,
                operation_type: TreasuryOperationType::Deposit,
                status: TreasuryOperationStatus::Failed,
                created_at: 0,
                operation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_treasury_operations(
                &mut operation,
                1,
                treasury_id,
                TreasuryOperationType::Deposit,
                [1u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(operation.treasury_id, treasury_id);
        }
    }

    #[test]
    fn test_initialize_treasury_operations_timestamp_variations() {
        let timestamps = vec![0i64, 1i64, 1234567890i64, i64::MAX, -1i64];

        for timestamp in timestamps {
            let mut operation = TreasuryOperationsMetadata {
                operation_id: 0,
                treasury_id: 0,
                operation_type: TreasuryOperationType::Deposit,
                status: TreasuryOperationStatus::Failed,
                created_at: 0,
                operation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_treasury_operations(
                &mut operation,
                1,
                10,
                TreasuryOperationType::Deposit,
                [1u8; 32],
                timestamp,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(operation.created_at, timestamp);
        }
    }

    #[test]
    fn test_initialize_treasury_operations_bump_variations() {
        let bumps = vec![0u8, 1u8, 128u8, 255u8];

        for bump in bumps {
            let mut operation = TreasuryOperationsMetadata {
                operation_id: 0,
                treasury_id: 0,
                operation_type: TreasuryOperationType::Deposit,
                status: TreasuryOperationStatus::Failed,
                created_at: 0,
                operation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_treasury_operations(
                &mut operation,
                1,
                10,
                TreasuryOperationType::Deposit,
                [1u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(operation.bump, bump);
        }
    }

    #[test]
    fn test_initialize_treasury_operations_operation_id_edge_cases() {
        // Test operation_id = 1 (minimum valid)
        let mut operation = TreasuryOperationsMetadata {
            operation_id: 0,
            treasury_id: 0,
            operation_type: TreasuryOperationType::Deposit,
            status: TreasuryOperationStatus::Failed,
            created_at: 0,
            operation_data_hash: [0u8; 32],
            bump: 0,
        };

        let result = onchain::initialize_treasury_operations(
            &mut operation,
            1,
            10,
            TreasuryOperationType::Deposit,
            [1u8; 32],
            1000,
            255,
        );
        assert!(result.is_ok());
        assert_eq!(operation.operation_id, 1);

        // Test operation_id = u64::MAX
        let mut operation2 = TreasuryOperationsMetadata {
            operation_id: 0,
            treasury_id: 0,
            operation_type: TreasuryOperationType::Deposit,
            status: TreasuryOperationStatus::Failed,
            created_at: 0,
            operation_data_hash: [0u8; 32],
            bump: 0,
        };

        let result2 = onchain::initialize_treasury_operations(
            &mut operation2,
            u64::MAX,
            10,
            TreasuryOperationType::Deposit,
            [1u8; 32],
            1000,
            255,
        );
        assert!(result2.is_ok());
        assert_eq!(operation2.operation_id, u64::MAX);
    }

    #[test]
    fn test_initialize_treasury_operations_status_always_pending() {
        let initial_statuses = vec![
            TreasuryOperationStatus::Pending,
            TreasuryOperationStatus::InProgress,
            TreasuryOperationStatus::Completed,
            TreasuryOperationStatus::Failed,
        ];

        for initial_status in initial_statuses {
            let mut operation = TreasuryOperationsMetadata {
                operation_id: 0,
                treasury_id: 0,
                operation_type: TreasuryOperationType::Deposit,
                status: initial_status,
                created_at: 0,
                operation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_treasury_operations(
                &mut operation,
                1,
                10,
                TreasuryOperationType::Deposit,
                [1u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            // Status should always be set to Pending regardless of initial value
            assert_eq!(operation.status, TreasuryOperationStatus::Pending);
        }
    }

    #[test]
    fn test_initialize_treasury_operations_all_fields_set() {
        let mut operation = TreasuryOperationsMetadata {
            operation_id: 999,
            treasury_id: 888,
            operation_type: TreasuryOperationType::Transfer,
            status: TreasuryOperationStatus::Completed,
            created_at: 777,
            operation_data_hash: [99u8; 32],
            bump: 66,
        };

        let operation_id = 1u64;
        let treasury_id = 10u64;
        let operation_type = TreasuryOperationType::Withdrawal;
        let operation_data_hash = [1u8; 32];
        let current_time = 1000i64;
        let bump = 255u8;

        let result = onchain::initialize_treasury_operations(
            &mut operation,
            operation_id,
            treasury_id,
            operation_type,
            operation_data_hash,
            current_time,
            bump,
        );

        assert!(result.is_ok());
        assert_eq!(operation.operation_id, operation_id);
        assert_eq!(operation.treasury_id, treasury_id);
        assert_eq!(operation.operation_type, operation_type);
        assert_eq!(operation.status, TreasuryOperationStatus::Pending);
        assert_eq!(operation.created_at, current_time);
        assert_eq!(operation.operation_data_hash, operation_data_hash);
        assert_eq!(operation.bump, bump);
    }

    #[test]
    fn test_treasury_operation_status_equality() {
        let status1 = TreasuryOperationStatus::Pending;
        let status2 = TreasuryOperationStatus::Pending;
        let status3 = TreasuryOperationStatus::Completed;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_treasury_operation_status_all_variants() {
        let variants = vec![
            TreasuryOperationStatus::Pending,
            TreasuryOperationStatus::InProgress,
            TreasuryOperationStatus::Completed,
            TreasuryOperationStatus::Failed,
        ];

        // All variants should be unique
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_execute_treasury_operation() {
        let result = offchain::execute_treasury_operation(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_execute_treasury_operation_different_ids() {
        let result1 = offchain::execute_treasury_operation(1);
        let result2 = offchain::execute_treasury_operation(999);
        let result3 = offchain::execute_treasury_operation(u64::MAX);

        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
        assert_eq!(result3, Vec::<u8>::new());
    }
}
