//! Accounting Period module
//!
//! Accounting period management
//!
//! On-chain: Metadata for accounting periods
//! Off-chain: Actual period management, closing

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Period type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AccountingPeriodType {
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Annually
    Annually,
    /// Custom period
    Custom,
}

/// Period status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AccountingPeriodStatus {
    /// Period open
    Open,
    /// Period closed
    Closed,
    /// Period locked
    Locked,
}

/// Accounting period metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct AccountingPeriodMetadata {
    /// Period ID
    pub period_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Period type
    pub period_type: AccountingPeriodType,
    /// Status
    pub status: AccountingPeriodStatus,
    /// Created at
    pub created_at: i64,
    /// Period start
    pub period_start: i64,
    /// Period end
    pub period_end: i64,
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_accounting_period(
        period: &mut AccountingPeriodMetadata,
        period_id: u64,
        entity_id: u64,
        period_type: AccountingPeriodType,
        period_start: i64,
        period_end: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(period_id > 0, IndrasError::InvalidInput);
        require!(period_end > period_start, IndrasError::InvalidInput);
        period.period_id = period_id;
        period.entity_id = entity_id;
        period.period_type = period_type;
        period.status = AccountingPeriodStatus::Open;
        period.created_at = current_time;
        period.period_start = period_start;
        period.period_end = period_end;
        period.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn close_accounting_period(_period_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_accounting_period() {
        let mut period = AccountingPeriodMetadata {
            period_id: 0,
            entity_id: 0,
            period_type: AccountingPeriodType::Monthly,
            status: AccountingPeriodStatus::Closed,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_period(
            &mut period,
            1,
            10,
            AccountingPeriodType::Quarterly,
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(period.period_id, 1);
        assert_eq!(period.entity_id, 10);
        assert_eq!(period.period_type, AccountingPeriodType::Quarterly);
        assert_eq!(period.status, AccountingPeriodStatus::Open);
        assert_eq!(period.created_at, 1500);
        assert_eq!(period.period_start, 1000);
        assert_eq!(period.period_end, 2000);
        assert_eq!(period.bump, 255);
    }

    #[test]
    fn test_initialize_accounting_period_invalid_id() {
        let mut period = AccountingPeriodMetadata {
            period_id: 0,
            entity_id: 0,
            period_type: AccountingPeriodType::Monthly,
            status: AccountingPeriodStatus::Closed,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_period(
            &mut period,
            0, // Invalid: must be > 0
            10,
            AccountingPeriodType::Quarterly,
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_accounting_period_invalid_period_range() {
        let mut period = AccountingPeriodMetadata {
            period_id: 0,
            entity_id: 0,
            period_type: AccountingPeriodType::Monthly,
            status: AccountingPeriodStatus::Closed,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            bump: 0,
        };
        
        // period_end <= period_start is invalid
        let result = onchain::initialize_accounting_period(
            &mut period,
            1,
            10,
            AccountingPeriodType::Quarterly,
            2000,
            1000, // Invalid: end <= start
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_accounting_period_equal_start_end() {
        let mut period = AccountingPeriodMetadata {
            period_id: 0,
            entity_id: 0,
            period_type: AccountingPeriodType::Monthly,
            status: AccountingPeriodStatus::Closed,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            bump: 0,
        };
        
        // period_end == period_start is invalid (must be >)
        let result = onchain::initialize_accounting_period(
            &mut period,
            1,
            10,
            AccountingPeriodType::Quarterly,
            1000,
            1000, // Invalid: end == start
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_accounting_period_all_period_types() {
        let period_types = vec![
            AccountingPeriodType::Monthly,
            AccountingPeriodType::Quarterly,
            AccountingPeriodType::Annually,
            AccountingPeriodType::Custom,
        ];

        for period_type in period_types {
            let mut period = AccountingPeriodMetadata {
                period_id: 0,
                entity_id: 0,
                period_type: AccountingPeriodType::Monthly,
                status: AccountingPeriodStatus::Closed,
                created_at: 0,
                period_start: 0,
                period_end: 0,
                bump: 0,
            };

            let result = onchain::initialize_accounting_period(
                &mut period,
                1,
                10,
                period_type,
                1000,
                2000,
                1500,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(period.period_type, period_type);
        }
    }

    #[test]
    fn test_initialize_accounting_period_always_open_on_init() {
        let mut period = AccountingPeriodMetadata {
            period_id: 0,
            entity_id: 0,
            period_type: AccountingPeriodType::Monthly,
            status: AccountingPeriodStatus::Closed, // Will be reset
            created_at: 0,
            period_start: 0,
            period_end: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_period(
            &mut period,
            1,
            10,
            AccountingPeriodType::Monthly,
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        // Status should always be set to Open on initialization
        assert_eq!(period.status, AccountingPeriodStatus::Open);
    }

    #[test]
    fn test_initialize_accounting_period_large_values() {
        let mut period = AccountingPeriodMetadata {
            period_id: 0,
            entity_id: 0,
            period_type: AccountingPeriodType::Monthly,
            status: AccountingPeriodStatus::Closed,
            created_at: 0,
            period_start: 0,
            period_end: 0,
            bump: 0,
        };
        
        let result = onchain::initialize_accounting_period(
            &mut period,
            u64::MAX,
            u64::MAX,
            AccountingPeriodType::Custom,
            i64::MAX - 1,
            i64::MAX,
            i64::MAX,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(period.period_id, u64::MAX);
        assert_eq!(period.entity_id, u64::MAX);
        assert_eq!(period.period_start, i64::MAX - 1);
        assert_eq!(period.period_end, i64::MAX);
        assert_eq!(period.created_at, i64::MAX);
    }

    #[test]
    fn test_accounting_period_type_variants() {
        assert_eq!(AccountingPeriodType::Monthly, AccountingPeriodType::Monthly);
        assert_eq!(AccountingPeriodType::Quarterly, AccountingPeriodType::Quarterly);
        assert_eq!(AccountingPeriodType::Annually, AccountingPeriodType::Annually);
        assert_eq!(AccountingPeriodType::Custom, AccountingPeriodType::Custom);
    }

    #[test]
    fn test_accounting_period_status_variants() {
        assert_eq!(AccountingPeriodStatus::Open, AccountingPeriodStatus::Open);
        assert_eq!(AccountingPeriodStatus::Closed, AccountingPeriodStatus::Closed);
        assert_eq!(AccountingPeriodStatus::Locked, AccountingPeriodStatus::Locked);
    }

    #[test]
    fn test_accounting_period_type_all_variants_unique() {
        let variants = vec![
            AccountingPeriodType::Monthly,
            AccountingPeriodType::Quarterly,
            AccountingPeriodType::Annually,
            AccountingPeriodType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_accounting_period_status_all_variants_unique() {
        let variants = vec![
            AccountingPeriodStatus::Open,
            AccountingPeriodStatus::Closed,
            AccountingPeriodStatus::Locked,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_accounting_period_metadata_all_fields() {
        let period = AccountingPeriodMetadata {
            period_id: 123,
            entity_id: 456,
            period_type: AccountingPeriodType::Annually,
            status: AccountingPeriodStatus::Locked,
            created_at: 5000,
            period_start: 1000,
            period_end: 2000,
            bump: 128,
        };
        
        assert_eq!(period.period_id, 123);
        assert_eq!(period.entity_id, 456);
        assert_eq!(period.period_type, AccountingPeriodType::Annually);
        assert_eq!(period.status, AccountingPeriodStatus::Locked);
        assert_eq!(period.created_at, 5000);
        assert_eq!(period.period_start, 1000);
        assert_eq!(period.period_end, 2000);
        assert_eq!(period.bump, 128);
    }

    #[test]
    fn test_offchain_close_accounting_period() {
        let result = offchain::close_accounting_period(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_close_accounting_period_different_ids() {
        let result1 = offchain::close_accounting_period(1);
        let result2 = offchain::close_accounting_period(999);
        let result3 = offchain::close_accounting_period(u64::MAX);

        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
        assert_eq!(result3, Vec::<u8>::new());
    }
}
