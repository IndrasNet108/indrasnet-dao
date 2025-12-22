//! Asset Management state modules
//!
//! Asset management for the DAO:
//! - On-chain: Metadata for assets, tracking, allocation, performance, valuation
//! - Off-chain: Actual asset management, tracking, allocation, performance analysis, valuation
//!
//! Includes: core, tracking, allocation, performance, valuation

pub mod core;
pub mod tracking;
pub mod allocation;
pub mod performance;
pub mod valuation;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use core::{
    AssetMetadata, AssetType, AssetStatus,
    onchain::initialize_asset,
};
pub use tracking::{
    AssetTrackingMetadata, AssetTrackingStatus,
    onchain::initialize_asset_tracking,
};
pub use allocation::{
    AssetAllocationMetadata, AllocationStrategy, AssetAllocationStatus,
    onchain::initialize_asset_allocation,
};
pub use performance::{
    AssetPerformanceMetadata, AssetPerformanceMetric, AssetPerformanceStatus,
    onchain::initialize_asset_performance,
};
pub use valuation::{
    AssetValuationMetadata, ValuationMethod, AssetValuationStatus,
    onchain::initialize_asset_valuation,
    onchain::initialize_advanced_asset_valuation,
};
