//! Treasury Manager
//!
//! Core treasury management functionality
//!
//! On-chain: Basic treasury operations, balance management
//! Off-chain: Advanced analytics, reporting (in separate service)

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Treasury account (on-chain)
///
/// Core treasury account structure
#[account]
#[derive(InitSpace)]
pub struct Treasury {
    #[max_len(100)]
    pub name: String,
    pub balance: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

impl Treasury {
    /// Deposit funds to treasury
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.balance = self.balance
            .checked_add(amount)
            .ok_or(error!(IndrasError::Overflow))?;
        Ok(())
    }

    /// Withdraw funds from treasury
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(
            self.balance >= amount,
            IndrasError::InsufficientFunds
        );
        self.balance = self.balance
            .checked_sub(amount)
            .ok_or(error!(IndrasError::Underflow))?;
        Ok(())
    }
}

/// On-chain functions for treasury management
pub mod onchain {
    use super::*;

    /// Initialize treasury
    pub fn initialize_treasury(
        treasury: &mut Treasury,
        name: String,
        authority: Pubkey,
        bump: u8,
    ) -> Result<()> {
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= 100, IndrasError::InvalidInput);
        
        treasury.name = name;
        treasury.balance = 0;
        treasury.authority = authority;
        treasury.bump = bump;
        
        Ok(())
    }

    /// Deposit to treasury
    pub fn deposit_to_treasury(
        treasury: &mut Treasury,
        amount: u64,
    ) -> Result<()> {
        treasury.deposit(amount)
    }

    /// Withdraw from treasury
    pub fn withdraw_from_treasury(
        treasury: &mut Treasury,
        amount: u64,
    ) -> Result<()> {
        treasury.withdraw(amount)
    }
}

/// Off-chain functions for treasury management
///
/// These functions should be implemented in off-chain service
/// for advanced analytics and reporting.
pub mod offchain {
    // Off-chain functions will be implemented in separate service
    
    /// Generate treasury report
    pub fn generate_treasury_report(_treasury_id: u64) -> Vec<u8> {
        // Implementation in off-chain service
        // Generates detailed treasury report
        vec![]
    }

    /// Analyze treasury operations
    pub fn analyze_treasury_operations(_treasury_id: u64) -> Vec<String> {
        // Implementation in off-chain service
        // Analyzes treasury operations and returns insights
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_treasury() -> Treasury {
        Treasury {
            name: "Test Treasury".to_string(),
            balance: 1000,
            authority: create_test_pubkey(1),
            bump: 255,
        }
    }

    #[test]
    fn test_treasury_deposit() {
        let mut treasury = create_test_treasury();
        let initial_balance = treasury.balance;
        
        assert!(treasury.deposit(500).is_ok());
        assert_eq!(treasury.balance, initial_balance + 500);
    }

    #[test]
    fn test_treasury_deposit_overflow() {
        let mut treasury = create_test_treasury();
        treasury.balance = u64::MAX - 100;
        
        // Should fail on overflow
        assert!(treasury.deposit(200).is_err());
        assert_eq!(treasury.balance, u64::MAX - 100);
    }

    #[test]
    fn test_treasury_withdraw() {
        let mut treasury = create_test_treasury();
        let initial_balance = treasury.balance;
        
        assert!(treasury.withdraw(300).is_ok());
        assert_eq!(treasury.balance, initial_balance - 300);
    }

    #[test]
    fn test_treasury_withdraw_insufficient_funds() {
        let mut treasury = create_test_treasury();
        treasury.balance = 100;
        
        // Should fail - insufficient funds
        assert!(treasury.withdraw(200).is_err());
        assert_eq!(treasury.balance, 100);
    }

    #[test]
    fn test_treasury_initialize() {
        let mut treasury = Treasury {
            name: String::new(),
            balance: 999,
            authority: create_test_pubkey(1),
            bump: 0,
        };
        
        let authority = create_test_pubkey(2);
        assert!(onchain::initialize_treasury(&mut treasury, "New Treasury".to_string(), authority, 255).is_ok());
        assert_eq!(treasury.name, "New Treasury");
        assert_eq!(treasury.balance, 0);
        assert_eq!(treasury.authority, authority);
        assert_eq!(treasury.bump, 255);
    }

    #[test]
    fn test_treasury_initialize_empty_name() {
        let mut treasury = Treasury {
            name: String::new(),
            balance: 0,
            authority: create_test_pubkey(1),
            bump: 0,
        };
        
        assert!(onchain::initialize_treasury(&mut treasury, String::new(), create_test_pubkey(2), 255).is_err());
    }

    #[test]
    fn test_treasury_deposit_to_treasury() {
        let mut treasury = create_test_treasury();
        let initial_balance = treasury.balance;
        
        assert!(onchain::deposit_to_treasury(&mut treasury, 500).is_ok());
        assert_eq!(treasury.balance, initial_balance + 500);
    }

    #[test]
    fn test_treasury_withdraw_from_treasury() {
        let mut treasury = create_test_treasury();
        let initial_balance = treasury.balance;
        
        assert!(onchain::withdraw_from_treasury(&mut treasury, 300).is_ok());
        assert_eq!(treasury.balance, initial_balance - 300);
    }

    #[test]
    fn test_treasury_withdraw_exact_balance() {
        let mut treasury = create_test_treasury();
        treasury.balance = 500;
        
        assert!(treasury.withdraw(500).is_ok());
        assert_eq!(treasury.balance, 0);
    }

    #[test]
    fn test_treasury_withdraw_zero() {
        let mut treasury = create_test_treasury();
        let initial_balance = treasury.balance;
        
        assert!(treasury.withdraw(0).is_ok());
        assert_eq!(treasury.balance, initial_balance);
    }

    #[test]
    fn test_treasury_deposit_zero() {
        let mut treasury = create_test_treasury();
        let initial_balance = treasury.balance;
        
        assert!(treasury.deposit(0).is_ok());
        assert_eq!(treasury.balance, initial_balance);
    }

    #[test]
    fn test_treasury_initialize_max_name_length() {
        let mut treasury = Treasury {
            name: String::new(),
            balance: 0,
            authority: create_test_pubkey(1),
            bump: 0,
        };
        
        let max_name = "a".repeat(100);
        assert!(onchain::initialize_treasury(&mut treasury, max_name.clone(), create_test_pubkey(2), 255).is_ok());
        assert_eq!(treasury.name, max_name);
    }

    #[test]
    fn test_treasury_initialize_name_too_long() {
        let mut treasury = Treasury {
            name: String::new(),
            balance: 0,
            authority: create_test_pubkey(1),
            bump: 0,
        };
        
        let too_long_name = "a".repeat(101);
        assert!(onchain::initialize_treasury(&mut treasury, too_long_name, create_test_pubkey(2), 255).is_err());
    }

    #[test]
    fn test_treasury_structure() {
        let treasury = create_test_treasury();
        assert_eq!(treasury.name, "Test Treasury");
        assert_eq!(treasury.balance, 1000);
        assert_eq!(treasury.bump, 255);
    }

    #[test]
    fn test_treasury_multiple_deposits() {
        let mut treasury = create_test_treasury();
        
        assert!(treasury.deposit(100).is_ok());
        assert_eq!(treasury.balance, 1100);
        
        assert!(treasury.deposit(200).is_ok());
        assert_eq!(treasury.balance, 1300);
        
        assert!(treasury.deposit(300).is_ok());
        assert_eq!(treasury.balance, 1600);
    }

    #[test]
    fn test_treasury_multiple_withdrawals() {
        let mut treasury = create_test_treasury();
        
        assert!(treasury.withdraw(100).is_ok());
        assert_eq!(treasury.balance, 900);
        
        assert!(treasury.withdraw(200).is_ok());
        assert_eq!(treasury.balance, 700);
        
        assert!(treasury.withdraw(300).is_ok());
        assert_eq!(treasury.balance, 400);
    }

    #[test]
    fn test_treasury_initialize_preserves_other_fields() {
        let mut treasury = Treasury {
            name: "Old Name".to_string(),
            balance: 9999,
            authority: create_test_pubkey(1),
            bump: 50,
        };
        
        let new_authority = create_test_pubkey(2);
        assert!(onchain::initialize_treasury(&mut treasury, "New Name".to_string(), new_authority, 100).is_ok());
        assert_eq!(treasury.name, "New Name");
        assert_eq!(treasury.balance, 0); // Always reset to 0
        assert_eq!(treasury.authority, new_authority);
        assert_eq!(treasury.bump, 100);
    }

    #[test]
    fn test_offchain_generate_treasury_report() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::generate_treasury_report(1);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_generate_treasury_report_different_ids() {
        // Test with different IDs
        let result1 = offchain::generate_treasury_report(1);
        let result2 = offchain::generate_treasury_report(999);
        assert_eq!(result1, Vec::<u8>::new());
        assert_eq!(result2, Vec::<u8>::new());
    }

    #[test]
    fn test_offchain_analyze_treasury_operations() {
        // Test that offchain function exists and returns empty vec
        let result = offchain::analyze_treasury_operations(1);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_offchain_analyze_treasury_operations_different_ids() {
        // Test with different IDs
        let result1 = offchain::analyze_treasury_operations(1);
        let result2 = offchain::analyze_treasury_operations(999);
        assert_eq!(result1, Vec::<String>::new());
        assert_eq!(result2, Vec::<String>::new());
    }

    #[test]
    fn test_treasury_all_fields() {
        let authority = create_test_pubkey(5);
        let treasury = Treasury {
            name: "Main Treasury".to_string(),
            balance: 50000,
            authority,
            bump: 128,
        };
        
        assert_eq!(treasury.name, "Main Treasury");
        assert_eq!(treasury.balance, 50000);
        assert_eq!(treasury.authority, authority);
        assert_eq!(treasury.bump, 128);
    }

    #[test]
    fn test_treasury_deposit_preserves_other_fields() {
        let mut treasury = create_test_treasury();
        let original_name = treasury.name.clone();
        let original_authority = treasury.authority;
        let original_bump = treasury.bump;
        
        assert!(treasury.deposit(500).is_ok());
        
        assert_eq!(treasury.name, original_name);
        assert_eq!(treasury.authority, original_authority);
        assert_eq!(treasury.bump, original_bump);
    }

    #[test]
    fn test_treasury_withdraw_preserves_other_fields() {
        let mut treasury = create_test_treasury();
        let original_name = treasury.name.clone();
        let original_authority = treasury.authority;
        let original_bump = treasury.bump;
        
        assert!(treasury.withdraw(300).is_ok());
        
        assert_eq!(treasury.name, original_name);
        assert_eq!(treasury.authority, original_authority);
        assert_eq!(treasury.bump, original_bump);
    }
}
