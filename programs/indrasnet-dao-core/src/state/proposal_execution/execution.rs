//! Proposal execution lifecycle methods

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::ExecutionStatus;
use super::ProposalExecution;

impl ProposalExecution {
    /// Create a new proposal execution
    pub fn new(
        id: u64,
        proposal_id: u64,
        executor: Pubkey,
        execution_data: String,
        bump: u8,
    ) -> Result<Self> {
        Self::new_with_time(id, proposal_id, executor, execution_data, bump, Clock::get()?.unix_timestamp)
    }

    /// Create a new proposal execution with explicit timestamp
    pub fn new_with_time(
        id: u64,
        proposal_id: u64,
        executor: Pubkey,
        execution_data: String,
        bump: u8,
        executed_at: i64,
    ) -> Result<Self> {
        require!(!execution_data.is_empty(), IndrasError::InvalidInput);
        require!(execution_data.len() <= 1000, IndrasError::InvalidInput);

        Ok(Self {
            id,
            proposal_id,
            executor,
            executed_at,
            passed_at: None,
            execution_allowed_at: None,
            compliance_proof: None,
            execution_data,
            status: ExecutionStatus::Pending,
            bump,
        })
    }

    /// Start execution (Pending -> InProgress)
    pub fn start_execution(&mut self) -> Result<()> {
        require!(self.status == ExecutionStatus::Pending, IndrasError::InvalidInput);
        self.status = ExecutionStatus::InProgress;
        Ok(())
    }

    /// Complete execution (InProgress -> Completed)
    pub fn complete_execution(&mut self) -> Result<()> {
        require!(self.status == ExecutionStatus::InProgress, IndrasError::InvalidInput);
        self.status = ExecutionStatus::Completed;
        Ok(())
    }

    /// Fail execution (InProgress -> Failed)
    pub fn fail_execution(&mut self) -> Result<()> {
        require!(self.status == ExecutionStatus::InProgress, IndrasError::InvalidInput);
        self.status = ExecutionStatus::Failed;
        Ok(())
    }

    /// Cancel execution (Pending | InProgress -> Cancelled)
    pub fn cancel_execution(&mut self) -> Result<()> {
        require!(
            self.status == ExecutionStatus::Pending || self.status == ExecutionStatus::InProgress,
            IndrasError::InvalidInput
        );
        self.status = ExecutionStatus::Cancelled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    #[test]
    fn test_proposal_execution_new_with_time() {
        let executor = create_test_pubkey(1);
        let execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        assert_eq!(execution.id, 1);
        assert_eq!(execution.proposal_id, 10);
        assert_eq!(execution.executor, executor);
        assert_eq!(execution.execution_data, "exec_data");
        assert_eq!(execution.status, ExecutionStatus::Pending);
        assert_eq!(execution.executed_at, 1000);
    }

    #[test]
    fn test_proposal_execution_new_validation_empty_data() {
        let executor = create_test_pubkey(1);
        let result = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            String::new(), // Invalid: empty
            255,
            1000,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_execution_start_execution() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        assert!(execution.start_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::InProgress);
    }

    #[test]
    fn test_proposal_execution_start_execution_invalid_state() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        execution.status = ExecutionStatus::InProgress;
        // Try to start already started execution - should fail
        assert!(execution.start_execution().is_err());
    }

    #[test]
    fn test_proposal_execution_complete_execution() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        execution.start_execution().unwrap();
        assert!(execution.complete_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::Completed);
    }

    #[test]
    fn test_proposal_execution_complete_execution_invalid_state() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        // Try to complete non-in-progress execution - should fail
        assert!(execution.complete_execution().is_err());
    }

    #[test]
    fn test_proposal_execution_fail_execution() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        execution.start_execution().unwrap();
        assert!(execution.fail_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::Failed);
    }

    #[test]
    fn test_proposal_execution_cancel_execution() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        assert!(execution.cancel_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::Cancelled);
    }

    #[test]
    fn test_proposal_execution_cancel_execution_in_progress() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        execution.start_execution().unwrap();
        assert!(execution.cancel_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::Cancelled);
    }

    #[test]
    fn test_proposal_execution_cancel_execution_invalid_state() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        execution.status = ExecutionStatus::Completed;
        // Try to cancel completed execution - should fail
        assert!(execution.cancel_execution().is_err());
    }

    #[test]
    fn test_proposal_execution_new_validation_too_long_data() {
        let executor = create_test_pubkey(1);
        let long_data = "x".repeat(1001); // 1001 chars > 1000 limit
        let result = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            long_data,
            255,
            1000,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_execution_new_validation_exact_limit() {
        let executor = create_test_pubkey(1);
        let exact_data = "x".repeat(1000); // Exactly 1000 chars
        let result = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            exact_data,
            255,
            1000,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_proposal_execution_all_fields() {
        let executor = create_test_pubkey(5);
        let execution = ProposalExecution::new_with_time(
            999,
            888,
            executor,
            "data".to_string(),
            128,
            5000,
        ).unwrap();
        
        assert_eq!(execution.id, 999);
        assert_eq!(execution.proposal_id, 888);
        assert_eq!(execution.executor, executor);
        assert_eq!(execution.execution_data, "data");
        assert_eq!(execution.status, ExecutionStatus::Pending);
        assert_eq!(execution.executed_at, 5000);
        assert_eq!(execution.bump, 128);
        assert_eq!(execution.passed_at, None);
        assert_eq!(execution.execution_allowed_at, None);
        assert_eq!(execution.compliance_proof, None);
    }

    #[test]
    fn test_proposal_execution_fail_execution_invalid_state() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        // Try to fail non-in-progress execution - should fail
        assert!(execution.fail_execution().is_err());
    }

    #[test]
    fn test_proposal_execution_full_lifecycle() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        assert_eq!(execution.status, ExecutionStatus::Pending);
        assert!(execution.start_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::InProgress);
        assert!(execution.complete_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::Completed);
    }

    #[test]
    fn test_proposal_execution_cancel_from_pending() {
        let executor = create_test_pubkey(1);
        let mut execution = ProposalExecution::new_with_time(
            1,
            10,
            executor,
            "exec_data".to_string(),
            255,
            1000,
        ).unwrap();
        
        assert_eq!(execution.status, ExecutionStatus::Pending);
        assert!(execution.cancel_execution().is_ok());
        assert_eq!(execution.status, ExecutionStatus::Cancelled);
    }
}
