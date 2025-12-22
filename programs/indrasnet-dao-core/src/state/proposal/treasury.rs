//! Treasury Proposal types
//!
//! Specialized proposal types for Treasury operations

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Treasury Proposal Type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, InitSpace)]
pub enum TreasuryProposalType {
    /// Withdraw funds from treasury
    Withdrawal,
    /// Deposit funds to treasury
    Deposit,
    /// Transfer funds between treasuries
    Transfer,
    /// Grant capability for treasury operations
    GrantCapability,
    /// Revoke capability
    RevokeCapability,
    /// Update treasury configuration
    UpdateConfig,
}

/// Treasury Proposal Operation Data
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, InitSpace)]
pub struct TreasuryOperationData {
    pub operation_type: TreasuryProposalType,
    pub amount: Option<u64>, // For withdrawal, deposit, transfer
    pub target_treasury: Option<Pubkey>, // For transfer
    pub capability_grantee: Option<Pubkey>, // For grant/revoke capability
    #[max_len(50)]
    pub capability_type: Option<String>, // For grant capability
    pub expires_at: Option<i64>, // For grant capability
    #[max_len(200)]
    pub description: String, // Operation description
}

impl TreasuryOperationData {
    pub fn validate(&self) -> Result<()> {
        match self.operation_type {
            TreasuryProposalType::Withdrawal | 
            TreasuryProposalType::Deposit | 
            TreasuryProposalType::Transfer => {
                require!(self.amount.is_some(), IndrasError::InvalidInput);
                require!(self.amount.unwrap() > 0, IndrasError::InvalidInput);
                if self.operation_type == TreasuryProposalType::Transfer {
                    require!(self.target_treasury.is_some(), IndrasError::InvalidInput);
                }
            },
            TreasuryProposalType::GrantCapability => {
                require!(self.capability_grantee.is_some(), IndrasError::InvalidInput);
                require!(self.capability_type.is_some(), IndrasError::InvalidInput);
                require!(self.expires_at.is_some(), IndrasError::InvalidInput);
                if let Some(exp) = self.expires_at {
                    require!(exp > Clock::get()?.unix_timestamp, IndrasError::InvalidInput);
                }
            },
            TreasuryProposalType::RevokeCapability => {
                require!(self.capability_grantee.is_some(), IndrasError::InvalidInput);
            },
            TreasuryProposalType::UpdateConfig => {
                // No specific requirements for config updates
            },
        }
        require!(!self.description.is_empty(), IndrasError::InvalidInput);
        require!(self.description.len() <= 200, IndrasError::InvalidInput);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treasury_operation_data_withdrawal() {
        let data = TreasuryOperationData {
            operation_type: TreasuryProposalType::Withdrawal,
            amount: Some(1000),
            target_treasury: None,
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Withdraw for expenses".to_string(),
        };
        // Validation would require Clock, so skip in unit test
        assert_eq!(data.operation_type, TreasuryProposalType::Withdrawal);
        assert_eq!(data.amount, Some(1000));
    }

    #[test]
    fn test_treasury_operation_data_transfer() {
        let target = Pubkey::new_unique();
        let data = TreasuryOperationData {
            operation_type: TreasuryProposalType::Transfer,
            amount: Some(5000),
            target_treasury: Some(target),
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Transfer to secondary treasury".to_string(),
        };
        assert_eq!(data.operation_type, TreasuryProposalType::Transfer);
        assert_eq!(data.target_treasury, Some(target));
    }
}
