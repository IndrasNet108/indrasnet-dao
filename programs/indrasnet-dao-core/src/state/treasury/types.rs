//! Treasury types
//!
//! Type definitions for treasury operations

use anchor_lang::prelude::*;

/// Treasury operation type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryOperationType {
    /// Deposit operation
    Deposit,
    /// Withdrawal operation
    Withdrawal,
    /// Transfer operation
    Transfer,
    /// Capability grant
    CapabilityGrant,
    /// Capability revoke
    CapabilityRevoke,
}

/// Treasury transaction status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryTransactionStatus {
    /// Transaction pending
    Pending,
    /// Transaction approved
    Approved,
    /// Transaction executed
    Executed,
    /// Transaction rejected
    Rejected,
    /// Transaction cancelled
    Cancelled,
}

/// Treasury balance type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum TreasuryBalanceType {
    /// Available balance
    Available,
    /// Reserved balance
    Reserved,
    /// Locked balance
    Locked,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treasury_operation_type_variants() {
        assert_eq!(TreasuryOperationType::Deposit, TreasuryOperationType::Deposit);
        assert_eq!(TreasuryOperationType::Withdrawal, TreasuryOperationType::Withdrawal);
        assert_eq!(TreasuryOperationType::Transfer, TreasuryOperationType::Transfer);
        assert_eq!(TreasuryOperationType::CapabilityGrant, TreasuryOperationType::CapabilityGrant);
        assert_eq!(TreasuryOperationType::CapabilityRevoke, TreasuryOperationType::CapabilityRevoke);
    }

    #[test]
    fn test_treasury_transaction_status_variants() {
        let variants = vec![
            TreasuryTransactionStatus::Pending,
            TreasuryTransactionStatus::Approved,
            TreasuryTransactionStatus::Executed,
            TreasuryTransactionStatus::Rejected,
            TreasuryTransactionStatus::Cancelled,
        ];
        
        // Check all variants are unique
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_treasury_balance_type_variants() {
        assert_eq!(TreasuryBalanceType::Available, TreasuryBalanceType::Available);
        assert_eq!(TreasuryBalanceType::Reserved, TreasuryBalanceType::Reserved);
        assert_eq!(TreasuryBalanceType::Locked, TreasuryBalanceType::Locked);
    }

    #[test]
    fn test_treasury_operation_type_all_variants_unique() {
        let variants = vec![
            TreasuryOperationType::Deposit,
            TreasuryOperationType::Withdrawal,
            TreasuryOperationType::Transfer,
            TreasuryOperationType::CapabilityGrant,
            TreasuryOperationType::CapabilityRevoke,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_treasury_operation_type_equality() {
        assert_eq!(TreasuryOperationType::Deposit, TreasuryOperationType::Deposit);
        assert_eq!(TreasuryOperationType::Withdrawal, TreasuryOperationType::Withdrawal);
        assert_eq!(TreasuryOperationType::Transfer, TreasuryOperationType::Transfer);
        assert_eq!(TreasuryOperationType::CapabilityGrant, TreasuryOperationType::CapabilityGrant);
        assert_eq!(TreasuryOperationType::CapabilityRevoke, TreasuryOperationType::CapabilityRevoke);
    }

    #[test]
    fn test_treasury_operation_type_copy() {
        let op1 = TreasuryOperationType::Deposit;
        let op2 = op1; // Copy trait
        assert_eq!(op1, op2);
    }

    #[test]
    fn test_treasury_operation_type_space() {
        assert_eq!(<TreasuryOperationType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_treasury_transaction_status_equality() {
        assert_eq!(TreasuryTransactionStatus::Pending, TreasuryTransactionStatus::Pending);
        assert_eq!(TreasuryTransactionStatus::Approved, TreasuryTransactionStatus::Approved);
        assert_eq!(TreasuryTransactionStatus::Executed, TreasuryTransactionStatus::Executed);
        assert_eq!(TreasuryTransactionStatus::Rejected, TreasuryTransactionStatus::Rejected);
        assert_eq!(TreasuryTransactionStatus::Cancelled, TreasuryTransactionStatus::Cancelled);
    }

    #[test]
    fn test_treasury_transaction_status_copy() {
        let status1 = TreasuryTransactionStatus::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_treasury_transaction_status_space() {
        assert_eq!(<TreasuryTransactionStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_treasury_balance_type_all_variants_unique() {
        let variants = vec![
            TreasuryBalanceType::Available,
            TreasuryBalanceType::Reserved,
            TreasuryBalanceType::Locked,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_treasury_balance_type_equality() {
        assert_eq!(TreasuryBalanceType::Available, TreasuryBalanceType::Available);
        assert_eq!(TreasuryBalanceType::Reserved, TreasuryBalanceType::Reserved);
        assert_eq!(TreasuryBalanceType::Locked, TreasuryBalanceType::Locked);
    }

    #[test]
    fn test_treasury_balance_type_copy() {
        let balance1 = TreasuryBalanceType::Available;
        let balance2 = balance1; // Copy trait
        assert_eq!(balance1, balance2);
    }

    #[test]
    fn test_treasury_balance_type_space() {
        assert_eq!(<TreasuryBalanceType as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_treasury_operation_type_inequality() {
        assert_ne!(TreasuryOperationType::Deposit, TreasuryOperationType::Withdrawal);
        assert_ne!(TreasuryOperationType::Withdrawal, TreasuryOperationType::Transfer);
        assert_ne!(TreasuryOperationType::Transfer, TreasuryOperationType::CapabilityGrant);
        assert_ne!(TreasuryOperationType::CapabilityGrant, TreasuryOperationType::CapabilityRevoke);
    }

    #[test]
    fn test_treasury_transaction_status_inequality() {
        assert_ne!(TreasuryTransactionStatus::Pending, TreasuryTransactionStatus::Approved);
        assert_ne!(TreasuryTransactionStatus::Approved, TreasuryTransactionStatus::Executed);
        assert_ne!(TreasuryTransactionStatus::Executed, TreasuryTransactionStatus::Rejected);
        assert_ne!(TreasuryTransactionStatus::Rejected, TreasuryTransactionStatus::Cancelled);
    }

    #[test]
    fn test_treasury_balance_type_inequality() {
        assert_ne!(TreasuryBalanceType::Available, TreasuryBalanceType::Reserved);
        assert_ne!(TreasuryBalanceType::Reserved, TreasuryBalanceType::Locked);
        assert_ne!(TreasuryBalanceType::Locked, TreasuryBalanceType::Available);
    }

    #[test]
    fn test_treasury_operation_type_clone() {
        let op1 = TreasuryOperationType::Deposit;
        let op2 = op1.clone();
        assert_eq!(op1, op2);
    }

    #[test]
    fn test_treasury_transaction_status_clone() {
        let status1 = TreasuryTransactionStatus::Pending;
        let status2 = status1.clone();
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_treasury_balance_type_clone() {
        let balance1 = TreasuryBalanceType::Available;
        let balance2 = balance1.clone();
        assert_eq!(balance1, balance2);
    }

    #[test]
    fn test_treasury_operation_type_debug() {
        let op = TreasuryOperationType::Deposit;
        let debug_str = format!("{:?}", op);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_treasury_transaction_status_debug() {
        let status = TreasuryTransactionStatus::Pending;
        let debug_str = format!("{:?}", status);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_treasury_balance_type_debug() {
        let balance = TreasuryBalanceType::Available;
        let debug_str = format!("{:?}", balance);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_treasury_operation_type_all_variants() {
        let all_variants = vec![
            TreasuryOperationType::Deposit,
            TreasuryOperationType::Withdrawal,
            TreasuryOperationType::Transfer,
            TreasuryOperationType::CapabilityGrant,
            TreasuryOperationType::CapabilityRevoke,
        ];

        assert_eq!(all_variants.len(), 5);
        for variant in &all_variants {
            match variant {
                TreasuryOperationType::Deposit => {},
                TreasuryOperationType::Withdrawal => {},
                TreasuryOperationType::Transfer => {},
                TreasuryOperationType::CapabilityGrant => {},
                TreasuryOperationType::CapabilityRevoke => {},
            }
        }
    }

    #[test]
    fn test_treasury_transaction_status_all_variants() {
        let all_variants = vec![
            TreasuryTransactionStatus::Pending,
            TreasuryTransactionStatus::Approved,
            TreasuryTransactionStatus::Executed,
            TreasuryTransactionStatus::Rejected,
            TreasuryTransactionStatus::Cancelled,
        ];

        assert_eq!(all_variants.len(), 5);
        for variant in &all_variants {
            match variant {
                TreasuryTransactionStatus::Pending => {},
                TreasuryTransactionStatus::Approved => {},
                TreasuryTransactionStatus::Executed => {},
                TreasuryTransactionStatus::Rejected => {},
                TreasuryTransactionStatus::Cancelled => {},
            }
        }
    }

    #[test]
    fn test_treasury_balance_type_all_variants() {
        let all_variants = vec![
            TreasuryBalanceType::Available,
            TreasuryBalanceType::Reserved,
            TreasuryBalanceType::Locked,
        ];

        assert_eq!(all_variants.len(), 3);
        for variant in &all_variants {
            match variant {
                TreasuryBalanceType::Available => {},
                TreasuryBalanceType::Reserved => {},
                TreasuryBalanceType::Locked => {},
            }
        }
    }
}
