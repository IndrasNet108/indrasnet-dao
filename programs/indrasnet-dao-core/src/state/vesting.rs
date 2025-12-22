//! Vesting module
//!
//! Token vesting management
//!
//! On-chain: Metadata for vesting schedules
//! Off-chain: Actual vesting calculations, releases

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Vesting status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum VestingStatus {
    /// Vesting active
    Active,
    /// Vesting completed
    Completed,
    /// Vesting cancelled
    Cancelled,
}

/// Vesting metadata (on-chain)
///
/// Stores metadata for token vesting
#[account]
#[derive(InitSpace)]
pub struct VestingMetadata {
    /// Vesting ID
    pub vesting_id: u64,
    /// Recipient pubkey
    pub recipient_pubkey: Pubkey,
    /// Total amount (in smallest unit)
    pub total_amount: u64,
    /// Vested amount (in smallest unit)
    pub vested_amount: u64,
    /// Status
    pub status: VestingStatus,
    /// Created at
    pub created_at: i64,
    /// Vesting start date
    pub start_date: i64,
    /// Vesting end date
    pub end_date: i64,
    /// Vesting data hash
    pub vesting_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for vesting
pub mod onchain {
    use super::*;

    /// Initialize vesting
    pub fn initialize_vesting(
        vesting: &mut VestingMetadata,
        vesting_id: u64,
        recipient_pubkey: Pubkey,
        total_amount: u64,
        vesting_data_hash: [u8; 32],
        start_date: i64,
        end_date: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(vesting_id > 0, IndrasError::InvalidInput);
        require!(total_amount > 0, IndrasError::InvalidInput);
        require!(end_date > start_date, IndrasError::InvalidInput);
        
        vesting.vesting_id = vesting_id;
        vesting.recipient_pubkey = recipient_pubkey;
        vesting.total_amount = total_amount;
        vesting.vested_amount = 0;
        vesting.status = VestingStatus::Active;
        vesting.created_at = current_time;
        vesting.start_date = start_date;
        vesting.end_date = end_date;
        vesting.vesting_data_hash = vesting_data_hash;
        vesting.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for vesting
pub mod offchain {
    /// Calculate vested amount
    pub fn calculate_vested_amount(_vesting_id: u64, _current_time: i64) -> u64 {
        // Implementation in off-chain service
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_initialize_vesting() {
        let mut vesting = VestingMetadata {
            vesting_id: 0,
            recipient_pubkey: create_test_pubkey(1),
            total_amount: 0,
            vested_amount: 999,
            status: VestingStatus::Cancelled,
            created_at: 0,
            start_date: 0,
            end_date: 0,
            vesting_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let recipient = create_test_pubkey(2);
        let result = onchain::initialize_vesting(
            &mut vesting,
            1,
            recipient,
            100000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(vesting.vesting_id, 1);
        assert_eq!(vesting.recipient_pubkey, recipient);
        assert_eq!(vesting.total_amount, 100000);
        assert_eq!(vesting.vested_amount, 0);
        assert_eq!(vesting.status, VestingStatus::Active);
        assert_eq!(vesting.created_at, 1500);
        assert_eq!(vesting.start_date, 1000);
        assert_eq!(vesting.end_date, 2000);
        assert_eq!(vesting.vesting_data_hash, [1u8; 32]);
        assert_eq!(vesting.bump, 255);
    }

    #[test]
    fn test_initialize_vesting_invalid_id() {
        let mut vesting = VestingMetadata {
            vesting_id: 0,
            recipient_pubkey: create_test_pubkey(1),
            total_amount: 0,
            vested_amount: 0,
            status: VestingStatus::Active,
            created_at: 0,
            start_date: 0,
            end_date: 0,
            vesting_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_vesting(
            &mut vesting,
            0, // Invalid: must be > 0
            create_test_pubkey(2),
            100000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_vesting_invalid_amount() {
        let mut vesting = VestingMetadata {
            vesting_id: 0,
            recipient_pubkey: create_test_pubkey(1),
            total_amount: 0,
            vested_amount: 0,
            status: VestingStatus::Active,
            created_at: 0,
            start_date: 0,
            end_date: 0,
            vesting_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_vesting(
            &mut vesting,
            1,
            create_test_pubkey(2),
            0, // Invalid: must be > 0
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_vesting_invalid_date_range() {
        let mut vesting = VestingMetadata {
            vesting_id: 0,
            recipient_pubkey: create_test_pubkey(1),
            total_amount: 0,
            vested_amount: 0,
            status: VestingStatus::Active,
            created_at: 0,
            start_date: 0,
            end_date: 0,
            vesting_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_vesting(
            &mut vesting,
            1,
            create_test_pubkey(2),
            100000,
            [1u8; 32],
            2000,
            1000, // Invalid: end <= start
            1500,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_vesting_always_active_on_init() {
        let mut vesting = VestingMetadata {
            vesting_id: 0,
            recipient_pubkey: create_test_pubkey(1),
            total_amount: 0,
            vested_amount: 999, // Will be reset
            status: VestingStatus::Cancelled, // Will be reset
            created_at: 0,
            start_date: 0,
            end_date: 0,
            vesting_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_vesting(
            &mut vesting,
            1,
            create_test_pubkey(2),
            100000,
            [1u8; 32],
            1000,
            2000,
            1500,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(vesting.status, VestingStatus::Active);
        assert_eq!(vesting.vested_amount, 0);
    }

    #[test]
    fn test_vesting_status_variants() {
        assert_eq!(VestingStatus::Active, VestingStatus::Active);
        assert_eq!(VestingStatus::Completed, VestingStatus::Completed);
        assert_eq!(VestingStatus::Cancelled, VestingStatus::Cancelled);
    }

    #[test]
    fn test_vesting_status_all_variants_unique() {
        let variants = vec![
            VestingStatus::Active,
            VestingStatus::Completed,
            VestingStatus::Cancelled,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_vesting_metadata_all_fields() {
        let recipient = create_test_pubkey(5);
        let vesting = VestingMetadata {
            vesting_id: 123,
            recipient_pubkey: recipient,
            total_amount: 1000000,
            vested_amount: 500000,
            status: VestingStatus::Active,
            created_at: 2000,
            start_date: 1000,
            end_date: 3000,
            vesting_data_hash: [42u8; 32],
            bump: 128,
        };
        
        assert_eq!(vesting.vesting_id, 123);
        assert_eq!(vesting.recipient_pubkey, recipient);
        assert_eq!(vesting.total_amount, 1000000);
        assert_eq!(vesting.vested_amount, 500000);
        assert_eq!(vesting.status, VestingStatus::Active);
        assert_eq!(vesting.created_at, 2000);
        assert_eq!(vesting.start_date, 1000);
        assert_eq!(vesting.end_date, 3000);
        assert_eq!(vesting.vesting_data_hash, [42u8; 32]);
        assert_eq!(vesting.bump, 128);
    }

    #[test]
    fn test_initialize_vesting_large_values() {
        let mut vesting = VestingMetadata {
            vesting_id: 0,
            recipient_pubkey: create_test_pubkey(1),
            total_amount: 0,
            vested_amount: 0,
            status: VestingStatus::Active,
            created_at: 0,
            start_date: 0,
            end_date: 0,
            vesting_data_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_vesting(
            &mut vesting,
            u64::MAX,
            create_test_pubkey(2),
            u64::MAX,
            [255u8; 32],
            i64::MAX - 1,
            i64::MAX,
            i64::MAX,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(vesting.vesting_id, u64::MAX);
        assert_eq!(vesting.total_amount, u64::MAX);
        assert_eq!(vesting.start_date, i64::MAX - 1);
        assert_eq!(vesting.end_date, i64::MAX);
        assert_eq!(vesting.created_at, i64::MAX);
    }

    #[test]
    fn test_initialize_vesting_recipient_variations() {
        let recipients = vec![
            create_test_pubkey(1),
            create_test_pubkey(2),
            create_test_pubkey(255),
            Pubkey::new_unique(),
        ];

        for recipient in recipients {
            let mut vesting = VestingMetadata {
                vesting_id: 0,
                recipient_pubkey: create_test_pubkey(1),
                total_amount: 0,
                vested_amount: 0,
                status: VestingStatus::Active,
                created_at: 0,
                start_date: 0,
                end_date: 0,
                vesting_data_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_vesting(
                &mut vesting,
                1,
                recipient,
                100000,
                [1u8; 32],
                1000,
                2000,
                1500,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(vesting.recipient_pubkey, recipient);
        }
    }

    #[test]
    fn test_offchain_calculate_vested_amount() {
        let result = offchain::calculate_vested_amount(1, 1000);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_offchain_calculate_vested_amount_different_times() {
        let result1 = offchain::calculate_vested_amount(1, 0);
        let result2 = offchain::calculate_vested_amount(1, 1000);
        let result3 = offchain::calculate_vested_amount(1, i64::MAX);

        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
    }
}
