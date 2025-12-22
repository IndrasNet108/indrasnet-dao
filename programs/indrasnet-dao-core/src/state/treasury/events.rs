//! Treasury events
//!
//! Event definitions for treasury operations
//!
//! On-chain: Event emission for treasury operations
//! Off-chain: Event processing and analytics

use anchor_lang::prelude::*;

/// Treasury deposit event
#[event]
pub struct TreasuryDepositEvent {
    pub treasury_id: u64,
    pub amount: u64,
    pub depositor: Pubkey,
    pub timestamp: i64,
}

/// Treasury withdrawal event
#[event]
pub struct TreasuryWithdrawalEvent {
    pub treasury_id: u64,
    pub amount: u64,
    pub withdrawer: Pubkey,
    pub capability_id: Option<u64>,
    pub timestamp: i64,
}

/// Treasury transfer event
#[event]
pub struct TreasuryTransferEvent {
    pub from_treasury_id: u64,
    pub to_treasury_id: u64,
    pub amount: u64,
    pub transferrer: Pubkey,
    pub timestamp: i64,
}

/// Capability grant event
#[event]
pub struct CapabilityGrantEvent {
    pub capability_id: u64,
    pub grantee: Pubkey,
    pub granter: Pubkey,
    pub capability_type: String,
    pub expires_at: i64,
    pub timestamp: i64,
}

/// Capability revoke event
#[event]
pub struct CapabilityRevokeEvent {
    pub capability_id: u64,
    pub revoker: Pubkey,
    pub timestamp: i64,
}

/// Treasury balance update event
#[event]
pub struct TreasuryBalanceUpdateEvent {
    pub treasury_id: u64,
    pub old_balance: u64,
    pub new_balance: u64,
    pub operation_type: String,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_treasury_deposit_event_creation() {
        let treasury_id = 1u64;
        let amount = 1_000_000_000u64;
        let depositor = create_test_pubkey(1);
        let timestamp = 1234567890i64;

        let event = TreasuryDepositEvent {
            treasury_id,
            amount,
            depositor,
            timestamp,
        };

        assert_eq!(event.treasury_id, treasury_id);
        assert_eq!(event.amount, amount);
        assert_eq!(event.depositor, depositor);
        assert_eq!(event.timestamp, timestamp);
    }

    #[test]
    fn test_treasury_deposit_event_all_fields() {
        let event = TreasuryDepositEvent {
            treasury_id: 42,
            amount: 5_000_000_000,
            depositor: create_test_pubkey(5),
            timestamp: 9876543210,
        };

        assert_eq!(event.treasury_id, 42);
        assert_eq!(event.amount, 5_000_000_000);
        assert_eq!(event.depositor, create_test_pubkey(5));
        assert_eq!(event.timestamp, 9876543210);
    }

    #[test]
    fn test_treasury_deposit_event_zero_values() {
        let event = TreasuryDepositEvent {
            treasury_id: 0,
            amount: 0,
            depositor: Pubkey::default(),
            timestamp: 0,
        };

        assert_eq!(event.treasury_id, 0);
        assert_eq!(event.amount, 0);
        assert_eq!(event.depositor, Pubkey::default());
        assert_eq!(event.timestamp, 0);
    }

    #[test]
    fn test_treasury_deposit_event_max_values() {
        let event = TreasuryDepositEvent {
            treasury_id: u64::MAX,
            amount: u64::MAX,
            depositor: create_test_pubkey(255),
            timestamp: i64::MAX,
        };

        assert_eq!(event.treasury_id, u64::MAX);
        assert_eq!(event.amount, u64::MAX);
        assert_eq!(event.timestamp, i64::MAX);
    }

    #[test]
    fn test_treasury_withdrawal_event_creation() {
        let treasury_id = 1u64;
        let amount = 500_000_000u64;
        let withdrawer = create_test_pubkey(2);
        let capability_id = Some(10u64);
        let timestamp = 1234567890i64;

        let event = TreasuryWithdrawalEvent {
            treasury_id,
            amount,
            withdrawer,
            capability_id,
            timestamp,
        };

        assert_eq!(event.treasury_id, treasury_id);
        assert_eq!(event.amount, amount);
        assert_eq!(event.withdrawer, withdrawer);
        assert_eq!(event.capability_id, capability_id);
        assert_eq!(event.timestamp, timestamp);
    }

    #[test]
    fn test_treasury_withdrawal_event_without_capability() {
        let event = TreasuryWithdrawalEvent {
            treasury_id: 1,
            amount: 1_000_000_000,
            withdrawer: create_test_pubkey(3),
            capability_id: None,
            timestamp: 1234567890,
        };

        assert_eq!(event.capability_id, None);
    }

    #[test]
    fn test_treasury_withdrawal_event_with_capability() {
        let capability_id = Some(42u64);
        let event = TreasuryWithdrawalEvent {
            treasury_id: 1,
            amount: 2_000_000_000,
            withdrawer: create_test_pubkey(4),
            capability_id,
            timestamp: 1234567890,
        };

        assert_eq!(event.capability_id, capability_id);
        assert_eq!(event.capability_id.unwrap(), 42);
    }

    #[test]
    fn test_treasury_transfer_event_creation() {
        let from_treasury_id = 1u64;
        let to_treasury_id = 2u64;
        let amount = 3_000_000_000u64;
        let transferrer = create_test_pubkey(5);
        let timestamp = 1234567890i64;

        let event = TreasuryTransferEvent {
            from_treasury_id,
            to_treasury_id,
            amount,
            transferrer,
            timestamp,
        };

        assert_eq!(event.from_treasury_id, from_treasury_id);
        assert_eq!(event.to_treasury_id, to_treasury_id);
        assert_eq!(event.amount, amount);
        assert_eq!(event.transferrer, transferrer);
        assert_eq!(event.timestamp, timestamp);
    }

    #[test]
    fn test_treasury_transfer_event_same_treasury() {
        let treasury_id = 1u64;
        let event = TreasuryTransferEvent {
            from_treasury_id: treasury_id,
            to_treasury_id: treasury_id,
            amount: 1_000_000_000,
            transferrer: create_test_pubkey(6),
            timestamp: 1234567890,
        };

        assert_eq!(event.from_treasury_id, event.to_treasury_id);
    }

    #[test]
    fn test_treasury_transfer_event_different_treasuries() {
        let event = TreasuryTransferEvent {
            from_treasury_id: 1,
            to_treasury_id: 2,
            amount: 5_000_000_000,
            transferrer: create_test_pubkey(7),
            timestamp: 1234567890,
        };

        assert_ne!(event.from_treasury_id, event.to_treasury_id);
    }

    #[test]
    fn test_capability_grant_event_creation() {
        let capability_id = 1u64;
        let grantee = create_test_pubkey(8);
        let granter = create_test_pubkey(9);
        let capability_type = "Withdrawal".to_string();
        let expires_at = 1234657890i64;
        let timestamp = 1234567890i64;

        let event = CapabilityGrantEvent {
            capability_id,
            grantee,
            granter,
            capability_type: capability_type.clone(),
            expires_at,
            timestamp,
        };

        assert_eq!(event.capability_id, capability_id);
        assert_eq!(event.grantee, grantee);
        assert_eq!(event.granter, granter);
        assert_eq!(event.capability_type, capability_type);
        assert_eq!(event.expires_at, expires_at);
        assert_eq!(event.timestamp, timestamp);
    }

    #[test]
    fn test_capability_grant_event_different_grantee_granter() {
        let grantee = create_test_pubkey(10);
        let granter = create_test_pubkey(11);
        let event = CapabilityGrantEvent {
            capability_id: 1,
            grantee,
            granter,
            capability_type: "Deposit".to_string(),
            expires_at: 1234657890,
            timestamp: 1234567890,
        };

        assert_ne!(event.grantee, event.granter);
    }

    #[test]
    fn test_capability_grant_event_capability_type() {
        let types = vec!["Withdrawal", "Deposit", "Transfer"];
        for cap_type in types {
            let event = CapabilityGrantEvent {
                capability_id: 1,
                grantee: create_test_pubkey(12),
                granter: create_test_pubkey(13),
                capability_type: cap_type.to_string(),
                expires_at: 1234657890,
                timestamp: 1234567890,
            };
            assert_eq!(event.capability_type, cap_type);
        }
    }

    #[test]
    fn test_capability_revoke_event_creation() {
        let capability_id = 1u64;
        let revoker = create_test_pubkey(14);
        let timestamp = 1234567890i64;

        let event = CapabilityRevokeEvent {
            capability_id,
            revoker,
            timestamp,
        };

        assert_eq!(event.capability_id, capability_id);
        assert_eq!(event.revoker, revoker);
        assert_eq!(event.timestamp, timestamp);
    }

    #[test]
    fn test_capability_revoke_event_all_values() {
        let event = CapabilityRevokeEvent {
            capability_id: 42,
            revoker: create_test_pubkey(15),
            timestamp: 9876543210,
        };

        assert_eq!(event.capability_id, 42);
        assert_eq!(event.revoker, create_test_pubkey(15));
        assert_eq!(event.timestamp, 9876543210);
    }

    #[test]
    fn test_treasury_balance_update_event_creation() {
        let treasury_id = 1u64;
        let old_balance = 10_000_000_000u64;
        let new_balance = 12_000_000_000u64;
        let operation_type = "Deposit".to_string();
        let timestamp = 1234567890i64;

        let event = TreasuryBalanceUpdateEvent {
            treasury_id,
            old_balance,
            new_balance,
            operation_type: operation_type.clone(),
            timestamp,
        };

        assert_eq!(event.treasury_id, treasury_id);
        assert_eq!(event.old_balance, old_balance);
        assert_eq!(event.new_balance, new_balance);
        assert_eq!(event.operation_type, operation_type);
        assert_eq!(event.timestamp, timestamp);
    }

    #[test]
    fn test_treasury_balance_update_event_increase() {
        let event = TreasuryBalanceUpdateEvent {
            treasury_id: 1,
            old_balance: 5_000_000_000,
            new_balance: 7_000_000_000,
            operation_type: "Deposit".to_string(),
            timestamp: 1234567890,
        };

        assert!(event.new_balance > event.old_balance);
        assert_eq!(event.new_balance - event.old_balance, 2_000_000_000);
    }

    #[test]
    fn test_treasury_balance_update_event_decrease() {
        let event = TreasuryBalanceUpdateEvent {
            treasury_id: 1,
            old_balance: 10_000_000_000,
            new_balance: 8_000_000_000,
            operation_type: "Withdrawal".to_string(),
            timestamp: 1234567890,
        };

        assert!(event.new_balance < event.old_balance);
        assert_eq!(event.old_balance - event.new_balance, 2_000_000_000);
    }

    #[test]
    fn test_treasury_balance_update_event_no_change() {
        let balance = 5_000_000_000u64;
        let event = TreasuryBalanceUpdateEvent {
            treasury_id: 1,
            old_balance: balance,
            new_balance: balance,
            operation_type: "Transfer".to_string(),
            timestamp: 1234567890,
        };

        assert_eq!(event.old_balance, event.new_balance);
    }

    #[test]
    fn test_treasury_balance_update_event_operation_types() {
        let operation_types = vec!["Deposit", "Withdrawal", "Transfer", "Grant"];
        for op_type in operation_types {
            let event = TreasuryBalanceUpdateEvent {
                treasury_id: 1,
                old_balance: 1_000_000_000,
                new_balance: 2_000_000_000,
                operation_type: op_type.to_string(),
                timestamp: 1234567890,
            };
            assert_eq!(event.operation_type, op_type);
        }
    }

    #[test]
    fn test_treasury_balance_update_event_zero_balance() {
        let event = TreasuryBalanceUpdateEvent {
            treasury_id: 1,
            old_balance: 0,
            new_balance: 0,
            operation_type: "Initialization".to_string(),
            timestamp: 1234567890,
        };

        assert_eq!(event.old_balance, 0);
        assert_eq!(event.new_balance, 0);
    }

    #[test]
    fn test_treasury_balance_update_event_max_balance() {
        let event = TreasuryBalanceUpdateEvent {
            treasury_id: 1,
            old_balance: u64::MAX - 1_000_000_000,
            new_balance: u64::MAX,
            operation_type: "Deposit".to_string(),
            timestamp: 1234567890,
        };

        assert_eq!(event.new_balance, u64::MAX);
    }

    #[test]
    fn test_all_events_timestamp_consistency() {
        let timestamp = 1234567890i64;
        
        let deposit_event = TreasuryDepositEvent {
            treasury_id: 1,
            amount: 1_000_000_000,
            depositor: create_test_pubkey(16),
            timestamp,
        };
        
        let withdrawal_event = TreasuryWithdrawalEvent {
            treasury_id: 1,
            amount: 500_000_000,
            withdrawer: create_test_pubkey(17),
            capability_id: None,
            timestamp,
        };
        
        let transfer_event = TreasuryTransferEvent {
            from_treasury_id: 1,
            to_treasury_id: 2,
            amount: 300_000_000,
            transferrer: create_test_pubkey(18),
            timestamp,
        };

        assert_eq!(deposit_event.timestamp, timestamp);
        assert_eq!(withdrawal_event.timestamp, timestamp);
        assert_eq!(transfer_event.timestamp, timestamp);
    }

    #[test]
    fn test_events_pubkey_consistency() {
        let pubkey = create_test_pubkey(19);
        
        let deposit_event = TreasuryDepositEvent {
            treasury_id: 1,
            amount: 1_000_000_000,
            depositor: pubkey,
            timestamp: 1234567890,
        };
        
        let withdrawal_event = TreasuryWithdrawalEvent {
            treasury_id: 1,
            amount: 500_000_000,
            withdrawer: pubkey,
            capability_id: None,
            timestamp: 1234567890,
        };

        assert_eq!(deposit_event.depositor, pubkey);
        assert_eq!(withdrawal_event.withdrawer, pubkey);
    }

    #[test]
    fn test_capability_events_consistency() {
        let capability_id = 42u64;
        let timestamp = 1234567890i64;
        
        let grant_event = CapabilityGrantEvent {
            capability_id,
            grantee: create_test_pubkey(20),
            granter: create_test_pubkey(21),
            capability_type: "Withdrawal".to_string(),
            expires_at: 1234657890,
            timestamp,
        };
        
        let revoke_event = CapabilityRevokeEvent {
            capability_id,
            revoker: create_test_pubkey(22),
            timestamp: timestamp + 100,
        };

        assert_eq!(grant_event.capability_id, capability_id);
        assert_eq!(revoke_event.capability_id, capability_id);
        assert_eq!(grant_event.capability_id, revoke_event.capability_id);
    }

    #[test]
    fn test_treasury_events_edge_cases() {
        // Test with minimum values
        let min_deposit = TreasuryDepositEvent {
            treasury_id: 0,
            amount: 0,
            depositor: Pubkey::default(),
            timestamp: i64::MIN,
        };
        assert_eq!(min_deposit.amount, 0);
        assert_eq!(min_deposit.timestamp, i64::MIN);

        // Test with maximum values
        let max_withdrawal = TreasuryWithdrawalEvent {
            treasury_id: u64::MAX,
            amount: u64::MAX,
            withdrawer: create_test_pubkey(255),
            capability_id: Some(u64::MAX),
            timestamp: i64::MAX,
        };
        assert_eq!(max_withdrawal.amount, u64::MAX);
        assert_eq!(max_withdrawal.timestamp, i64::MAX);
    }
}
