//! Financial Benchmarking module
//!
//! Financial benchmarking
//!
//! On-chain: Metadata for financial benchmarking
//! Off-chain: Actual benchmarking, comparison

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Benchmark type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBenchmarkType {
    /// Industry benchmark
    Industry,
    /// Peer benchmark
    Peer,
    /// Historical benchmark
    Historical,
    /// Custom benchmark
    Custom,
}

/// Benchmark status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum FinancialBenchmarkStatus {
    /// Benchmark active
    Active,
    /// Benchmark paused
    Paused,
    /// Benchmark disabled
    Disabled,
}

/// Financial benchmarking metadata (on-chain)
#[account]
#[derive(InitSpace)]
pub struct FinancialBenchmarkingMetadata {
    /// Benchmark ID
    pub benchmark_id: u64,
    /// Entity ID
    pub entity_id: u64,
    /// Benchmark type
    pub benchmark_type: FinancialBenchmarkType,
    /// Status
    pub status: FinancialBenchmarkStatus,
    /// Created at
    pub created_at: i64,
    /// Benchmark config hash
    pub benchmark_config_hash: [u8; 32],
    /// Bump seed
    pub bump: u8,
}

/// On-chain functions
pub mod onchain {
    use super::*;
    pub fn initialize_financial_benchmarking(
        benchmark: &mut FinancialBenchmarkingMetadata,
        benchmark_id: u64,
        entity_id: u64,
        benchmark_type: FinancialBenchmarkType,
        benchmark_config_hash: [u8; 32],
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        require!(benchmark_id > 0, IndrasError::InvalidInput);
        benchmark.benchmark_id = benchmark_id;
        benchmark.entity_id = entity_id;
        benchmark.benchmark_type = benchmark_type;
        benchmark.status = FinancialBenchmarkStatus::Active;
        benchmark.created_at = current_time;
        benchmark.benchmark_config_hash = benchmark_config_hash;
        benchmark.bump = bump;
        Ok(())
    }
}

/// Off-chain functions
pub mod offchain {
    pub fn benchmark_financials(_benchmark_id: u64) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_financial_benchmarking() {
        let mut benchmark = FinancialBenchmarkingMetadata {
            benchmark_id: 0,
            entity_id: 0,
            benchmark_type: FinancialBenchmarkType::Industry,
            status: FinancialBenchmarkStatus::Disabled,
            created_at: 0,
            benchmark_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_benchmarking(
            &mut benchmark,
            1,
            10,
            FinancialBenchmarkType::Peer,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_ok());
        assert_eq!(benchmark.benchmark_id, 1);
        assert_eq!(benchmark.entity_id, 10);
        assert_eq!(benchmark.benchmark_type, FinancialBenchmarkType::Peer);
        assert_eq!(benchmark.status, FinancialBenchmarkStatus::Active);
        assert_eq!(benchmark.created_at, 1000);
        assert_eq!(benchmark.benchmark_config_hash, [1u8; 32]);
        assert_eq!(benchmark.bump, 255);
    }

    #[test]
    fn test_initialize_financial_benchmarking_invalid_id() {
        let mut benchmark = FinancialBenchmarkingMetadata {
            benchmark_id: 0,
            entity_id: 0,
            benchmark_type: FinancialBenchmarkType::Industry,
            status: FinancialBenchmarkStatus::Active,
            created_at: 0,
            benchmark_config_hash: [0u8; 32],
            bump: 0,
        };
        
        let result = onchain::initialize_financial_benchmarking(
            &mut benchmark,
            0, // Invalid: must be > 0
            10,
            FinancialBenchmarkType::Peer,
            [1u8; 32],
            1000,
            255,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_initialize_financial_benchmarking_all_types() {
        let types = vec![
            FinancialBenchmarkType::Industry,
            FinancialBenchmarkType::Peer,
            FinancialBenchmarkType::Historical,
            FinancialBenchmarkType::Custom,
        ];

        for benchmark_type in types {
            let mut benchmark = FinancialBenchmarkingMetadata {
                benchmark_id: 0,
                entity_id: 0,
                benchmark_type: FinancialBenchmarkType::Industry,
                status: FinancialBenchmarkStatus::Active,
                created_at: 0,
                benchmark_config_hash: [0u8; 32],
                bump: 0,
            };

            let result = onchain::initialize_financial_benchmarking(
                &mut benchmark,
                1,
                10,
                benchmark_type,
                [0u8; 32],
                1000,
                255,
            );

            assert!(result.is_ok());
            assert_eq!(benchmark.benchmark_type, benchmark_type);
        }
    }

    #[test]
    fn test_financial_benchmark_type_variants() {
        assert_eq!(FinancialBenchmarkType::Industry, FinancialBenchmarkType::Industry);
        assert_eq!(FinancialBenchmarkType::Peer, FinancialBenchmarkType::Peer);
        assert_eq!(FinancialBenchmarkType::Historical, FinancialBenchmarkType::Historical);
        assert_eq!(FinancialBenchmarkType::Custom, FinancialBenchmarkType::Custom);
    }

    #[test]
    fn test_financial_benchmark_status_variants() {
        assert_eq!(FinancialBenchmarkStatus::Active, FinancialBenchmarkStatus::Active);
        assert_eq!(FinancialBenchmarkStatus::Paused, FinancialBenchmarkStatus::Paused);
        assert_eq!(FinancialBenchmarkStatus::Disabled, FinancialBenchmarkStatus::Disabled);
    }

    #[test]
    fn test_financial_benchmark_type_all_variants_unique() {
        let variants = vec![
            FinancialBenchmarkType::Industry,
            FinancialBenchmarkType::Peer,
            FinancialBenchmarkType::Historical,
            FinancialBenchmarkType::Custom,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_financial_benchmark_status_all_variants_unique() {
        let variants = vec![
            FinancialBenchmarkStatus::Active,
            FinancialBenchmarkStatus::Paused,
            FinancialBenchmarkStatus::Disabled,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_offchain_benchmark_financials() {
        let result = offchain::benchmark_financials(1);
        assert_eq!(result, Vec::<u8>::new());
    }
}
