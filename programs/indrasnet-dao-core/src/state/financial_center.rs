//! Financial Center module
//!
//! Financial center management (cost centers and profit centers)
//!
//! On-chain: Metadata for financial centers
//! Off-chain: Actual management, tracking

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Financial center type (cost or profit)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCenterType {
    /// Cost center
    Cost,
    /// Profit center
    Profit,
}

/// Cost center category
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum CostCenterCategory {
    /// Department
    Department,
    /// Project
    Project,
    /// Product
    Product,
    /// Custom type
    Custom,
}

/// Profit center category
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ProfitCenterCategory {
    /// Business unit
    BusinessUnit,
    /// Division
    Division,
    /// Product line
    ProductLine,
    /// Custom type
    Custom,
}

/// Financial center status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialCenterStatus {
    /// Center active
    Active,
    /// Center paused
    Paused,
    /// Center closed
    Closed,
}

/// Financial center metadata (on-chain)
///
/// Unified structure for both cost centers and profit centers
#[account]
#[derive(InitSpace)]
pub struct FinancialCenterMetadata {
    /// Center ID
    pub center_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Center type (Cost or Profit)
    pub center_type: FinancialCenterType,
    /// Cost center category (if center_type is Cost)
    pub cost_center_category: Option<CostCenterCategory>,
    /// Profit center category (if center_type is Profit)
    pub profit_center_category: Option<ProfitCenterCategory>,
    /// Status
    pub status: FinancialCenterStatus,
    /// Created at
    pub created_at: i64,
    /// Center config hash
    pub center_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;

    /// Initialize financial cost center
    pub fn initialize_financial_cost_center(
        center: &mut FinancialCenterMetadata,
        center_id: u64,
        entity_id: u64,
        cost_center_category: CostCenterCategory,
        center_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(center_id > 0, IndrasError::InvalidInput);
        center.center_id = center_id;
        center.entity_id = entity_id;
        center.center_type = FinancialCenterType::Cost;
        center.cost_center_category = Some(cost_center_category);
        center.profit_center_category = None;
        center.status = FinancialCenterStatus::Active;
        center.created_at = current_time;
        center.center_config_hash = center_config_hash;
        center.bump = bump;
        Ok(())
    }

    /// Initialize financial profit center
    pub fn initialize_financial_profit_center(
        center: &mut FinancialCenterMetadata,
        center_id: u64,
        entity_id: u64,
        profit_center_category: ProfitCenterCategory,
        center_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(center_id > 0, IndrasError::InvalidInput);
        center.center_id = center_id;
        center.entity_id = entity_id;
        center.center_type = FinancialCenterType::Profit;
        center.cost_center_category = None;
        center.profit_center_category = Some(profit_center_category);
        center.status = FinancialCenterStatus::Active;
        center.created_at = current_time;
        center.center_config_hash = center_config_hash;
        center.bump = bump;
        Ok(())
    }

    /// Update center status
    pub fn update_center_status(
        center: &mut FinancialCenterMetadata,
        new_status: FinancialCenterStatus,
    ) -> Result<()> {
        center.status = new_status;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    /// Manage financial center
    pub fn manage_financial_center(_center_id: u64) -> Vec<u8> {
        vec![]
    }

    /// Calculate center performance metrics
    pub fn calculate_center_performance(_center_id: u64) -> Vec<u8> {
        vec![]
    }
}

// Re-export legacy types for backward compatibility
pub use CostCenterCategory as FinancialCostCenterType;
pub use ProfitCenterCategory as FinancialProfitCenterType;
pub use FinancialCenterStatus as FinancialCostCenterStatus;
pub use FinancialCenterStatus as FinancialProfitCenterStatus;

// Legacy type aliases for backward compatibility
pub type FinancialCostCenterMetadata = FinancialCenterMetadata;
pub type FinancialProfitCenterMetadata = FinancialCenterMetadata;

// Legacy module aliases for backward compatibility
pub mod financial_cost_center_onchain {
    pub use super::onchain::initialize_financial_cost_center;
}

pub mod financial_profit_center_onchain {
    pub use super::onchain::initialize_financial_profit_center;
}
