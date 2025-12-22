//! Partnership Analytics Benchmarking module
//!
//! Partnership analytics benchmarking
//!
//! On-chain: Metadata for benchmarking
//! Off-chain: Actual benchmarking, comparison

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Benchmark type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipBenchmarkType {
    /// Industry benchmark
    Industry,
    /// Competitive benchmark
    Competitive,
    /// Historical benchmark
    Historical,
    /// Custom benchmark
    Custom,
}

/// Benchmark status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PartnershipBenchmarkStatus {
    /// Benchmark pending
    Pending,
    /// Benchmark in progress
    InProgress,
    /// Benchmark completed
    Completed,
}

/// Partnership analytics benchmarking metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct PartnershipAnalyticsBenchmarkingMetadata {
    /// Benchmark ID
    pub benchmark_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Benchmark type
    pub benchmark_type: PartnershipBenchmarkType,
    /// Status
    pub status: PartnershipBenchmarkStatus,
    /// Created at
    pub created_at: i64,
    /// Benchmark data hash
    pub benchmark_data_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_partnership_analytics_benchmarking(
        benchmark: &mut PartnershipAnalyticsBenchmarkingMetadata,
        benchmark_id: u64,
        partnership_id: u64,
        benchmark_type: PartnershipBenchmarkType,
        benchmark_data_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(benchmark_id > 0, IndrasError::InvalidInput);
        benchmark.benchmark_id = benchmark_id;
        benchmark.partnership_id = partnership_id;
        benchmark.benchmark_type = benchmark_type;
        benchmark.status = PartnershipBenchmarkStatus::Pending;
        benchmark.created_at = current_time;
        benchmark.benchmark_data_hash = benchmark_data_hash;
        benchmark.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn perform_benchmarking(_benchmark_id: u64) -> Vec<u8> {
        vec![]
    }
}
