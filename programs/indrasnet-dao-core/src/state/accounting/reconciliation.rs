//! Accounting Reconciliation module
//!
//! Accounting reconciliation
//!
//! On-chain: Metadata for accounting reconciliation
//! Off-chain: Actual reconciliation, matching

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Reconciliation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AccountingReconciliationStatus {
    /// Reconciliation pending
    Pending,
    /// Reconciliation in progress
    InProgress,
    /// Reconciliation completed
    Completed,
    /// Reconciliation failed
    Failed,
}

/// Accounting reconciliation metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct AccountingReconciliationMetadata {
    /// Reconciliation ID
    pub reconciliation_id: u64,
    /// Period ID
    pub period_id: u64,
    /// Status
    pub status: AccountingReconciliationStatus,
    /// Created at
    pub created_at: i64,
    /// Completed at
    pub completed_at: Option<i64>,
    /// Reconciliation data hash
    pub reconciliation_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_accounting_reconciliation(
        reconciliation: &mut AccountingReconciliationMetadata,
        reconciliation_id: u64,
        period_id: u64,
        reconciliation_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(reconciliation_id > 0, IndrasError::InvalidInput);
        reconciliation.reconciliation_id = reconciliation_id;
        reconciliation.period_id = period_id;
        reconciliation.status = AccountingReconciliationStatus::Pending;
        reconciliation.created_at = current_time;
        reconciliation.completed_at = None;
        reconciliation.reconciliation_data_hash = reconciliation_data_hash;
        reconciliation.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn reconcile_accounting(_reconciliation_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_accounting_reconciliation() {
        let mut reconciliation = AccountingReconciliationMetadata {
            reconciliation_id: 0,
            period_id: 0,
            status: AccountingReconciliationStatus::Completed,
            created_at: 0,
            completed_at: Some(1000),
            reconciliation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_reconciliation(
            &mut reconciliation,
            1,
            10,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(reconciliation.reconciliation_id, 1);
        assert_eq!(reconciliation.period_id, 10);
        assert_eq!(reconciliation.status, AccountingReconciliationStatus::Pending);
        assert_eq!(reconciliation.created_at, 1000);
        assert_eq!(reconciliation.completed_at, None);
        assert_eq!(reconciliation.reconciliation_data_hash, [1u8; 32]);
        assert_eq!(reconciliation.bump, 255);
    }

    #[test]
    fn test_initialize_accounting_reconciliation_invalid_id() {
        let mut reconciliation = AccountingReconciliationMetadata {
            reconciliation_id: 0,
            period_id: 0,
            status: AccountingReconciliationStatus::Completed,
            created_at: 0,
            completed_at: None,
            reconciliation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_reconciliation(
            &mut reconciliation,
            0, // Invalid: must be > 0
            10,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_accounting_reconciliation_always_pending_on_init() {
        let mut reconciliation = AccountingReconciliationMetadata {
            reconciliation_id: 0,
            period_id: 0,
            status: AccountingReconciliationStatus::Completed, // Will be reset
            created_at: 0,
            completed_at: Some(2000), // Will be cleared
            reconciliation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_reconciliation(
            &mut reconciliation,
            1,
            10,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Pending on initialization
        assert_eq!(reconciliation.status, AccountingReconciliationStatus::Pending);
        // completed_at should be cleared
        assert_eq!(reconciliation.completed_at, None);
    }

    #[test]
    fn test_initialize_accounting_reconciliation_large_values() {
        let mut reconciliation = AccountingReconciliationMetadata {
            reconciliation_id: 0,
            period_id: 0,
            status: AccountingReconciliationStatus::Pending,
            created_at: 0,
            completed_at: None,
            reconciliation_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_reconciliation(
            &mut reconciliation,
            u64::MAX,
            u64::MAX,
            [255u8; 32],
            i64::MAX,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(reconciliation.reconciliation_id, u64::MAX);
        assert_eq!(reconciliation.period_id, u64::MAX);
        assert_eq!(reconciliation.created_at, i64::MAX);
        assert_eq!(reconciliation.reconciliation_data_hash, [255u8; 32]);
    }

    #[test]
    fn test_initialize_accounting_reconciliation_data_hash_variations() {
        let hashes = vec![
            [0u8; 32],
            [1u8; 32],
            [255u8; 32],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        ];

        for hash in hashes {
            let mut reconciliation = AccountingReconciliationMetadata {
                reconciliation_id: 0,
                period_id: 0,
                status: AccountingReconciliationStatus::Pending,
                created_at: 0,
                completed_at: None,
                reconciliation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_accounting_reconciliation(
                &mut reconciliation,
                1,
                10,
                hash,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(reconciliation.reconciliation_data_hash, hash);
        }
    }

    #[test]
    fn test_initialize_accounting_reconciliation_timestamp_variations() {
        let timestamps = vec![0i64, 1i64, 1234567890i64, i64::MAX, -1i64];

        for timestamp in timestamps {
            let mut reconciliation = AccountingReconciliationMetadata {
                reconciliation_id: 0,
                period_id: 0,
                status: AccountingReconciliationStatus::Pending,
                created_at: 0,
                completed_at: None,
                reconciliation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_accounting_reconciliation(
                &mut reconciliation,
                1,
                10,
                [1u8; 32],
                timestamp,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(reconciliation.created_at, timestamp);
        }
    }

    #[test]
    fn test_initialize_accounting_reconciliation_bump_variations() {
        let bumps = vec![0u8, 1u8, 128u8, 255u8];

        for bump in bumps {
            let mut reconciliation = AccountingReconciliationMetadata {
                reconciliation_id: 0,
                period_id: 0,
                status: AccountingReconciliationStatus::Pending,
                created_at: 0,
                completed_at: None,
                reconciliation_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_accounting_reconciliation(
                &mut reconciliation,
                1,
                10,
                [1u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(reconciliation.bump, bump);
        }
    }

    #[test]
    fn test_accounting_reconciliation_status_variants() {
        assert_eq!(AccountingReconciliationStatus::Pending, AccountingReconciliationStatus::Pending);
        assert_eq!(AccountingReconciliationStatus::InProgress, AccountingReconciliationStatus::InProgress);
        assert_eq!(AccountingReconciliationStatus::Completed, AccountingReconciliationStatus::Completed);
        assert_eq!(AccountingReconciliationStatus::Failed, AccountingReconciliationStatus::Failed);
    }

    #[test]
    fn test_accounting_reconciliation_status_all_variants_unique() {
        let variants = vec![
            AccountingReconciliationStatus::Pending,
            AccountingReconciliationStatus::InProgress,
            AccountingReconciliationStatus::Completed,
            AccountingReconciliationStatus::Failed,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_accounting_reconciliation_metadata_all_fields() {
        let reconciliation = AccountingReconciliationMetadata {
            reconciliation_id: 123,
            period_id: 456,
            status: AccountingReconciliationStatus::InProgress,
            created_at: 2000,
            completed_at: Some(3000),
            reconciliation_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(reconciliation.reconciliation_id, 123);
        assert_eq!(reconciliation.period_id, 456);
        assert_eq!(reconciliation.status, AccountingReconciliationStatus::InProgress);
        assert_eq!(reconciliation.created_at, 2000);
        assert_eq!(reconciliation.completed_at, Some(3000));
        assert_eq!(reconciliation.reconciliation_data_hash, [42u8; 32]);
        assert_eq!(reconciliation.bump, 128);
    }

    #[test]
    fn test_offchain_reconcile_accounting() {
        let result = offchain::reconcile_accounting(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_reconcile_accounting_different_ids() {
        let result1 = offchain::reconcile_accounting(1);
        let result2 = offchain::reconcile_accounting(999);
        let result3 = offchain::reconcile_accounting(u64::MAX);

        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
        assert_eq!(result3, Vec::<u8>::new());
    }
}
