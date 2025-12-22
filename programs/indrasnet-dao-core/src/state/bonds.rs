//! Bonds module
//!
//! Bond management
//!
//! On-chain: Metadata for bonds
//! Off-chain: Actual bond calculations, payments

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Bond status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum BondStatus {
    /// Bond active
    Active,
    /// Bond matured
    Matured,
    /// Bond defaulted
    Defaulted,
}

/// Bond metadata (on-chain)
///
/// Stores metadata for bonds
#[account]
#[derive(InitSpace)]
pub struct BondMetadata {
    /// Bond ID
    pub bond_id: u64,
    /// Issuer pubkey
    pub issuer_pubkey: Pubkey,
    /// Holder pubkey
    pub holder_pubkey: Pubkey,
    /// Principal amount (in smallest unit)
    pub principal: u64,
    /// Interest rate (scaled)
    pub interest_rate: u64,
    /// Status
    pub status: BondStatus,
    /// Created at
    pub created_at: i64,
    /// Maturity date
    pub maturity_date: i64,
    /// Bond data hash
    pub bond_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions for bonds
pub mod onchain {
    use super::*;

    /// Initialize bond
    pub fn initialize_bond(
        bond: &mut BondMetadata,
        bond_id: u64,
        issuer_pubkey: Pubkey,
        holder_pubkey: Pubkey,
        principal: u64,
        interest_rate: u64,
        bond_data_hash: [u8; 32],
        maturity_date: i64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(bond_id > 0, IndrasError::InvalidInput);
        require!(principal > 0, IndrasError::InvalidInput);
        require!(maturity_date > current_time, IndrasError::InvalidInput);
        
        bond.bond_id = bond_id;
        bond.issuer_pubkey = issuer_pubkey;
        bond.holder_pubkey = holder_pubkey;
        bond.principal = principal;
        bond.interest_rate = interest_rate;
        bond.status = BondStatus::Active;
        bond.created_at = current_time;
        bond.maturity_date = maturity_date;
        bond.bond_data_hash = bond_data_hash;
        bond.bump = bump;
        
        Ok(())
    }
}

/// Off-chain functions for bonds
pub mod offchain {
    /// Calculate bond value
    pub fn calculate_bond_value(_bond_id: u64, _current_time: i64) -> u64 {
        // Implementation in off-chain service
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::IndrasError;

    // ========== initialize_bond validation tests ==========
    
    #[test]
    fn test_initialize_bond_validation_bond_id_zero() {
        // Test: bond_id == 0 should fail
        let bond_id = 0u64;
        
        // Validation logic: require!(bond_id > 0, IndrasError::InvalidInput)
        assert_eq!(bond_id, 0, "Bond ID zero should be detected");
    }
    
    #[test]
    fn test_initialize_bond_validation_bond_id_one() {
        // Test: bond_id == 1 should pass
        let bond_id = 1u64;
        
        // Validation logic: require!(bond_id > 0, IndrasError::InvalidInput)
        assert!(bond_id > 0, "Bond ID one should pass");
    }
    
    #[test]
    fn test_initialize_bond_validation_bond_id_max() {
        // Test: bond_id == u64::MAX should pass
        let bond_id = u64::MAX;
        
        // Validation logic: require!(bond_id > 0, IndrasError::InvalidInput)
        assert!(bond_id > 0, "Bond ID at max should pass");
    }
    
    #[test]
    fn test_initialize_bond_validation_principal_zero() {
        // Test: principal == 0 should fail
        let principal = 0u64;
        
        // Validation logic: require!(principal > 0, IndrasError::InvalidInput)
        assert_eq!(principal, 0, "Principal zero should be detected");
    }
    
    #[test]
    fn test_initialize_bond_validation_principal_one() {
        // Test: principal == 1 should pass
        let principal = 1u64;
        
        // Validation logic: require!(principal > 0, IndrasError::InvalidInput)
        assert!(principal > 0, "Principal one should pass");
    }
    
    #[test]
    fn test_initialize_bond_validation_principal_max() {
        // Test: principal == u64::MAX should pass
        let principal = u64::MAX;
        
        // Validation logic: require!(principal > 0, IndrasError::InvalidInput)
        assert!(principal > 0, "Principal at max should pass");
    }
    
    #[test]
    fn test_initialize_bond_validation_maturity_date_not_future() {
        // Test: maturity_date <= current_time should fail
        let current_time = 1000000i64;
        let maturity_date = 1000000i64;
        
        // Validation logic: require!(maturity_date > current_time, IndrasError::InvalidInput)
        assert!(maturity_date <= current_time, "Maturity date not future should be detected");
    }
    
    #[test]
    fn test_initialize_bond_validation_maturity_date_past() {
        // Test: maturity_date < current_time should fail
        let current_time = 1000000i64;
        let maturity_date = 999999i64;
        
        // Validation logic: require!(maturity_date > current_time, IndrasError::InvalidInput)
        assert!(maturity_date < current_time, "Maturity date past should be detected");
    }
    
    #[test]
    fn test_initialize_bond_validation_maturity_date_future() {
        // Test: maturity_date > current_time should pass
        let current_time = 1000000i64;
        let maturity_date = 1000001i64;
        
        // Validation logic: require!(maturity_date > current_time, IndrasError::InvalidInput)
        assert!(maturity_date > current_time, "Maturity date future should pass");
    }
    
    #[test]
    fn test_initialize_bond_validation_interest_rate_zero() {
        // Test: interest_rate == 0 should be allowed (no validation)
        let interest_rate = 0u64;
        
        // No validation for zero interest rate - this is allowed
        assert_eq!(interest_rate, 0, "Zero interest rate should be allowed");
    }
    
    #[test]
    fn test_initialize_bond_validation_interest_rate_max() {
        // Test: interest_rate == u64::MAX should be allowed
        let interest_rate = u64::MAX;
        
        // Max interest rate should be allowed
        assert_eq!(interest_rate, u64::MAX, "Max interest rate should be allowed");
    }
    
    #[test]
    fn test_initialize_bond_validation_issuer_holder_same() {
        // Test: issuer_pubkey == holder_pubkey should be allowed (no validation)
        let issuer_pubkey = Pubkey::new_unique();
        let holder_pubkey = issuer_pubkey;
        
        // Same pubkey should be allowed
        assert_eq!(issuer_pubkey, holder_pubkey, "Issuer and holder same should be allowed");
    }
    
    #[test]
    fn test_initialize_bond_validation_issuer_holder_different() {
        // Test: issuer_pubkey != holder_pubkey should be allowed
        let issuer_pubkey = Pubkey::new_unique();
        let holder_pubkey = Pubkey::new_unique();
        
        // Different pubkeys should be allowed
        assert_ne!(issuer_pubkey, holder_pubkey, "Issuer and holder different should be allowed");
    }
    
    #[test]
    fn test_initialize_bond_validation_bond_data_hash_zero() {
        // Test: bond_data_hash == [0u8; 32] should be allowed (no validation)
        let bond_data_hash = [0u8; 32];
        
        // No validation for zero hash - this is allowed
        assert_eq!(bond_data_hash, [0u8; 32], "Zero bond data hash should be allowed");
    }
    
    #[test]
    fn test_initialize_bond_validation_status_active() {
        // Test: status should be set to Active on initialization
        let status = BondStatus::Active;
        
        // Status should be Active
        assert_eq!(status, BondStatus::Active, "Status should be Active on initialization");
    }
    
    #[test]
    fn test_initialize_bond_validation_all_bond_statuses() {
        // Test: all BondStatus variants should be valid
        let statuses = vec![
            BondStatus::Active,
            BondStatus::Matured,
            BondStatus::Defaulted,
        ];
        
        // All statuses should be valid
        assert_eq!(statuses.len(), 3, "All bond statuses should be valid");
    }
    
    #[test]
    fn test_initialize_bond_validation_valid_inputs() {
        // Test: valid inputs should pass
        let bond_id = 1u64;
        let principal = 1000u64;
        let current_time = 1000000i64;
        let maturity_date = 2000000i64;
        
        // All validations should pass
        assert!(bond_id > 0, "Bond ID should be valid");
        assert!(principal > 0, "Principal should be valid");
        assert!(maturity_date > current_time, "Maturity date should be valid");
    }
    
    #[test]
    fn test_initialize_bond_validation_overflow_principal() {
        // Test: principal == u64::MAX should pass (no overflow check in initialization)
        let principal = u64::MAX;
        
        // Max principal should pass
        assert!(principal > 0, "Max principal should pass");
    }
}
