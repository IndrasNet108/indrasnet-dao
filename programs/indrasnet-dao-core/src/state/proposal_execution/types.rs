//! Execution status types for proposal execution

use anchor_lang::prelude::*;

/// Execution status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, InitSpace)]
pub enum ExecutionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_status_variants() {
        let variants = vec![
            ExecutionStatus::Pending,
            ExecutionStatus::InProgress,
            ExecutionStatus::Completed,
            ExecutionStatus::Failed,
            ExecutionStatus::Cancelled,
        ];
        
        // Check all variants are unique
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_execution_status_equality() {
        assert_eq!(ExecutionStatus::Pending, ExecutionStatus::Pending);
        assert_eq!(ExecutionStatus::InProgress, ExecutionStatus::InProgress);
        assert_eq!(ExecutionStatus::Completed, ExecutionStatus::Completed);
        assert_eq!(ExecutionStatus::Failed, ExecutionStatus::Failed);
        assert_eq!(ExecutionStatus::Cancelled, ExecutionStatus::Cancelled);
    }

    #[test]
    fn test_execution_status_inequality() {
        assert_ne!(ExecutionStatus::Pending, ExecutionStatus::InProgress);
        assert_ne!(ExecutionStatus::Pending, ExecutionStatus::Completed);
        assert_ne!(ExecutionStatus::Pending, ExecutionStatus::Failed);
        assert_ne!(ExecutionStatus::Pending, ExecutionStatus::Cancelled);
        assert_ne!(ExecutionStatus::InProgress, ExecutionStatus::Completed);
        assert_ne!(ExecutionStatus::InProgress, ExecutionStatus::Failed);
        assert_ne!(ExecutionStatus::InProgress, ExecutionStatus::Cancelled);
        assert_ne!(ExecutionStatus::Completed, ExecutionStatus::Failed);
        assert_ne!(ExecutionStatus::Completed, ExecutionStatus::Cancelled);
        assert_ne!(ExecutionStatus::Failed, ExecutionStatus::Cancelled);
    }

    #[test]
    fn test_execution_status_clone() {
        let status1 = ExecutionStatus::Pending;
        let status2 = status1.clone();
        assert_eq!(status1, status2);
        
        let status3 = ExecutionStatus::InProgress;
        let status4 = status3.clone();
        assert_eq!(status3, status4);
    }

    #[test]
    fn test_execution_status_all_variants() {
        let all_variants = vec![
            ExecutionStatus::Pending,
            ExecutionStatus::InProgress,
            ExecutionStatus::Completed,
            ExecutionStatus::Failed,
            ExecutionStatus::Cancelled,
        ];
        
        assert_eq!(all_variants.len(), 5);
    }

    #[test]
    fn test_execution_status_debug() {
        let status = ExecutionStatus::Pending;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Pending") || debug_str.contains("ExecutionStatus"));
    }

    #[test]
    fn test_execution_status_space() {
        // Test that InitSpace is implemented
        assert_eq!(<ExecutionStatus as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_execution_status_serialize_deserialize() {
        let statuses = vec![
            ExecutionStatus::Pending,
            ExecutionStatus::InProgress,
            ExecutionStatus::Completed,
            ExecutionStatus::Failed,
            ExecutionStatus::Cancelled,
        ];
        
        for status in &statuses {
            let mut buf = Vec::new();
            status.serialize(&mut buf).unwrap();
            let deserialized = ExecutionStatus::deserialize(&mut &buf[..]).unwrap();
            assert_eq!(*status, deserialized);
        }
    }

    #[test]
    fn test_execution_status_pending() {
        let status = ExecutionStatus::Pending;
        assert_eq!(status, ExecutionStatus::Pending);
    }

    #[test]
    fn test_execution_status_in_progress() {
        let status = ExecutionStatus::InProgress;
        assert_eq!(status, ExecutionStatus::InProgress);
    }

    #[test]
    fn test_execution_status_completed() {
        let status = ExecutionStatus::Completed;
        assert_eq!(status, ExecutionStatus::Completed);
    }

    #[test]
    fn test_execution_status_failed() {
        let status = ExecutionStatus::Failed;
        assert_eq!(status, ExecutionStatus::Failed);
    }

    #[test]
    fn test_execution_status_cancelled() {
        let status = ExecutionStatus::Cancelled;
        assert_eq!(status, ExecutionStatus::Cancelled);
    }

    #[test]
    fn test_execution_status_all_variants_unique() {
        let variants = vec![
            ExecutionStatus::Pending,
            ExecutionStatus::InProgress,
            ExecutionStatus::Completed,
            ExecutionStatus::Failed,
            ExecutionStatus::Cancelled,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }
}
