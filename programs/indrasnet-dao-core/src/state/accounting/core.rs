//! Accounting module
//!
//! Accounting management
//!
//! On-chain: Metadata for accounting entries
//! Off-chain: Actual accounting calculations, reporting

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Accounting entry type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AccountingEntryType {
    /// Debit entry
    Debit,
    /// Credit entry
    Credit,
}

/// Accounting entry metadata (on-chain)
///
/// Stores metadata for accounting entries
#[account]
#[derive(InitSpace)]
pub struct AccountingEntryMetadata {
    /// Entry ID
    pub entry_id: u64,
    /// Entry type
    pub entry_type: AccountingEntryType,
    /// Amount (in smallest unit)
    pub amount: u64,
    /// Created at
    pub created_at: i64,
    /// Entry data hash
    pub entry_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for accounting
pub mod onchain {
    use super::*;

    /// Initialize accounting entry
    pub fn initialize_accounting_entry(
        entry: &mut AccountingEntryMetadata,
        entry_id: u64,
        entry_type: AccountingEntryType,
        amount: u64,
        entry_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(entry_id > 0, IndrasError::InvalidInput);
        require!(amount > 0, IndrasError::InvalidInput);
        
        entry.entry_id = entry_id;
        entry.entry_type = entry_type;
        entry.amount = amount;
        entry.created_at = current_time;
        entry.entry_data_hash = entry_data_hash;
        entry.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for accounting
pub mod offchain {
    /// Generate financial report
    pub fn generate_financial_report(_period_start: i64, _period_end: i64) -> Vec<u8> {
        // Implementation in off-chain service
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_accounting_entry() {
        let mut entry = AccountingEntryMetadata {
            entry_id: 0,
            entry_type: AccountingEntryType::Debit,
            amount: 0,
            created_at: 0,
            entry_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_entry(
            &mut entry,
            1,
            AccountingEntryType::Credit,
            1000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(entry.entry_id, 1);
        assert_eq!(entry.entry_type, AccountingEntryType::Credit);
        assert_eq!(entry.amount, 1000);
        assert_eq!(entry.created_at, 1000);
        assert_eq!(entry.entry_data_hash, [1u8; 32]);
        assert_eq!(entry.bump, 255);
    }

    #[test]
    fn test_initialize_accounting_entry_invalid_id() {
        let mut entry = AccountingEntryMetadata {
            entry_id: 0,
            entry_type: AccountingEntryType::Debit,
            amount: 0,
            created_at: 0,
            entry_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_entry(
            &mut entry,
            0, // Invalid: must be > 0
            AccountingEntryType::Debit,
            1000,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_accounting_entry_invalid_amount() {
        let mut entry = AccountingEntryMetadata {
            entry_id: 0,
            entry_type: AccountingEntryType::Debit,
            amount: 0,
            created_at: 0,
            entry_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_entry(
            &mut entry,
            1,
            AccountingEntryType::Debit,
            0, // Invalid: must be > 0
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_accounting_entry_all_entry_types() {
        let entry_types = vec![
            AccountingEntryType::Debit,
            AccountingEntryType::Credit,
        ];

        for entry_type in entry_types {
            let mut entry = AccountingEntryMetadata {
                entry_id: 0,
                entry_type: AccountingEntryType::Debit,
                amount: 0,
                created_at: 0,
                entry_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_accounting_entry(
                &mut entry,
                1,
                entry_type,
                1000,
                [1u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(entry.entry_type, entry_type);
        }
    }

    #[test]
    fn test_initialize_accounting_entry_large_values() {
        let mut entry = AccountingEntryMetadata {
            entry_id: 0,
            entry_type: AccountingEntryType::Debit,
            amount: 0,
            created_at: 0,
            entry_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_entry(
            &mut entry,
            u64::MAX,
            AccountingEntryType::Debit,
            u64::MAX,
            [255u8; 32],
            i64::MAX,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(entry.entry_id, u64::MAX);
        assert_eq!(entry.amount, u64::MAX);
        assert_eq!(entry.created_at, i64::MAX);
    }

    #[test]
    fn test_initialize_accounting_entry_data_hash_variations() {
        let hashes = vec![
            [0u8; 32],
            [1u8; 32],
            [255u8; 32],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        ];

        for hash in hashes {
            let mut entry = AccountingEntryMetadata {
                entry_id: 0,
                entry_type: AccountingEntryType::Debit,
                amount: 0,
                created_at: 0,
                entry_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_accounting_entry(
                &mut entry,
                1,
                AccountingEntryType::Debit,
                1000,
                hash,
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(entry.entry_data_hash, hash);
        }
    }

    #[test]
    fn test_initialize_accounting_entry_timestamp_variations() {
        let timestamps = vec![0i64, 1i64, 1234567890i64, i64::MAX, -1i64];

        for timestamp in timestamps {
            let mut entry = AccountingEntryMetadata {
                entry_id: 0,
                entry_type: AccountingEntryType::Debit,
                amount: 0,
                created_at: 0,
                entry_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_accounting_entry(
                &mut entry,
                1,
                AccountingEntryType::Debit,
                1000,
                [1u8; 32],
                timestamp,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(entry.created_at, timestamp);
        }
    }

    #[test]
    fn test_initialize_accounting_entry_bump_variations() {
        let bumps = vec![0u8, 1u8, 128u8, 255u8];

        for bump in bumps {
            let mut entry = AccountingEntryMetadata {
                entry_id: 0,
                entry_type: AccountingEntryType::Debit,
                amount: 0,
                created_at: 0,
                entry_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_accounting_entry(
                &mut entry,
                1,
                AccountingEntryType::Debit,
                1000,
                [1u8; 32],
                1000,
                bump,
            );

            assert!(result.is_ok());
            assert_eq!(entry.bump, bump);
        }
    }

    #[test]
    fn test_accounting_entry_type_variants() {
        assert_eq!(AccountingEntryType::Debit, AccountingEntryType::Debit);
        assert_eq!(AccountingEntryType::Credit, AccountingEntryType::Credit);
    }

    #[test]
    fn test_accounting_entry_type_all_variants_unique() {
        let variants = vec![
            AccountingEntryType::Debit,
            AccountingEntryType::Credit,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_accounting_entry_metadata_all_fields() {
        let entry = AccountingEntryMetadata {
            entry_id: 123,
            entry_type: AccountingEntryType::Credit,
            amount: 5000,
            created_at: 2000,
            entry_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(entry.entry_id, 123);
        assert_eq!(entry.entry_type, AccountingEntryType::Credit);
        assert_eq!(entry.amount, 5000);
        assert_eq!(entry.created_at, 2000);
        assert_eq!(entry.entry_data_hash, [42u8; 32]);
        assert_eq!(entry.bump, 128);
    }

    #[test]
    fn test_offchain_generate_financial_report() {
        let result = offchain::generate_financial_report(1000, 2000);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_generate_financial_report_different_periods() {
        let result1 = offchain::generate_financial_report(0, 1000);
        let result2 = offchain::generate_financial_report(1000, 2000);
        let result3 = offchain::generate_financial_report(i64::MIN, i64::MAX);

        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
        assert_eq!(result3, Vec::<u8>::new());
    }
}
