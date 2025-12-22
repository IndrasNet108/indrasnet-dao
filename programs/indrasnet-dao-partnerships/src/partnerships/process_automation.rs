//! Partnership Process Automation Module
//! 
//! Comprehensive automation system for partnership lifecycle management.
//! Automates proposal creation, approval workflows, milestone tracking, and revenue distribution.
//! 
//! Hybrid model: Off-chain complex automation and analysis, on-chain state management
//! 
//! Principle: "Blockchain = Proof, not Storage"
//! - ON-CHAIN: Workflow state, approval records, automation triggers
//! - OFF-CHAIN: Complex workflow execution, decision logic, detailed analytics
//!
//! Improved version with enhanced automation capabilities and comprehensive testing

use anchor_lang::prelude::*;
use crate::error::IndrasError;
use super::types::*;

// ============================================================================
// ON-CHAIN STRUCTURES (Anchor Account Types)
// ============================================================================

/// Automation workflow status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum WorkflowStatus {
    /// Workflow not started
    NotStarted,
    /// Workflow in progress
    InProgress,
    /// Workflow paused
    Paused,
    /// Workflow completed
    Completed,
    /// Workflow failed
    Failed,
    /// Workflow cancelled
    Cancelled,
}

impl Space for WorkflowStatus {
    const INIT_SPACE: usize = 1;
}

/// Automation trigger type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum AutomationTrigger {
    /// Time-based trigger
    TimeBased,
    /// Milestone completion trigger
    MilestoneCompletion,
    /// Revenue threshold trigger
    RevenueThreshold,
    /// Performance threshold trigger
    PerformanceThreshold,
    /// Manual trigger
    Manual,
    /// Event-based trigger
    EventBased,
}

impl Space for AutomationTrigger {
    const INIT_SPACE: usize = 1;
}

/// Automation workflow configuration
#[account]
#[derive(InitSpace)]
pub struct AutomationWorkflow {
    /// Workflow ID
    pub workflow_id: u64,
    /// Partnership ID
    pub partnership_id: u64,
    /// Workflow name
    #[max_len(100)]
    pub name: String,
    /// Workflow description
    #[max_len(500)]
    pub description: String,
    /// Workflow status
    pub status: WorkflowStatus,
    /// Automation trigger
    pub trigger: AutomationTrigger,
    /// Trigger parameters (JSON string, max 1000 chars)
    #[max_len(1000)]
    pub trigger_params: String,
    /// Current step index
    pub current_step: u64,
    /// Total steps
    pub total_steps: u64,
    /// Workflow creator
    pub creator: Pubkey,
    /// Workflow creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Whether workflow is active
    pub is_active: bool,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Automation step record
#[account]
#[derive(InitSpace)]
pub struct AutomationStep {
    /// Step ID
    pub step_id: u64,
    /// Workflow ID
    pub workflow_id: u64,
    /// Step index
    pub step_index: u64,
    /// Step name
    #[max_len(100)]
    pub name: String,
    /// Step description
    #[max_len(500)]
    pub description: String,
    /// Step status
    pub status: StepStatus,
    /// Step execution timestamp
    pub executed_at: Option<i64>,
    /// Step executor (automation system)
    pub executor: Pubkey,
    /// Step result hash (for verification)
    pub result_hash: [u8; 32],
    /// Bump seed for PDA
    pub bump: u8,
}

/// Step status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy)]
pub enum StepStatus {
    /// Step pending
    Pending,
    /// Step in progress
    InProgress,
    /// Step completed
    Completed,
    /// Step failed
    Failed,
    /// Step skipped
    Skipped,
}

impl Space for StepStatus {
    const INIT_SPACE: usize = 1;
}

// ============================================================================
// ON-CHAIN FUNCTIONS (Anchor Handlers)
// ============================================================================

/// Create automation workflow
pub fn create_automation_workflow(
    workflow: &mut AutomationWorkflow,
    workflow_id: u64,
    partnership_id: u64,
    name: String,
    description: String,
    trigger: AutomationTrigger,
    trigger_params: String,
    total_steps: u64,
    creator: Pubkey,
) -> Result<()> {
    require!(!name.is_empty(), IndrasError::InvalidInput);
    require!(name.len() <= 100, IndrasError::StringTooLong);
    require!(description.len() <= 500, IndrasError::StringTooLong);
    require!(trigger_params.len() <= 1000, IndrasError::StringTooLong);
    require!(total_steps > 0, IndrasError::InvalidInput);
    require!(total_steps <= 100, IndrasError::InvalidInput); // Max 100 steps
    
    let current_time = Clock::get()?.unix_timestamp;
    
    create_automation_workflow_with_time(
        workflow,
        workflow_id,
        partnership_id,
        name,
        description,
        trigger,
        trigger_params,
        total_steps,
        creator,
        current_time,
    )
}

/// Create automation workflow with explicit timestamp (for testing)
pub fn create_automation_workflow_with_time(
    workflow: &mut AutomationWorkflow,
    workflow_id: u64,
    partnership_id: u64,
    name: String,
    description: String,
    trigger: AutomationTrigger,
    trigger_params: String,
    total_steps: u64,
    creator: Pubkey,
    current_time: i64,
) -> Result<()> {
    workflow.workflow_id = workflow_id;
    workflow.partnership_id = partnership_id;
    workflow.name = name;
    workflow.description = description;
    workflow.status = WorkflowStatus::NotStarted;
    workflow.trigger = trigger;
    workflow.trigger_params = trigger_params;
    workflow.current_step = 0;
    workflow.total_steps = total_steps;
    workflow.creator = creator;
    workflow.created_at = current_time;
    workflow.updated_at = current_time;
    workflow.is_active = true;
    
    msg!("Automation workflow {} created for partnership {} (trigger: {:?}, steps: {})", 
         workflow_id, partnership_id, trigger, total_steps);
    
    Ok(())
}

/// Start automation workflow
pub fn start_automation_workflow(
    workflow: &mut AutomationWorkflow,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    start_automation_workflow_with_time(workflow, current_time)
}

/// Start automation workflow with explicit timestamp (for testing)
pub fn start_automation_workflow_with_time(
    workflow: &mut AutomationWorkflow,
    current_time: i64,
) -> Result<()> {
    require!(workflow.is_active, IndrasError::InvalidState);
    require!(
        workflow.status == WorkflowStatus::NotStarted || 
        workflow.status == WorkflowStatus::Paused,
        IndrasError::InvalidState
    );
    
    workflow.status = WorkflowStatus::InProgress;
    workflow.updated_at = current_time;
    
    msg!("Automation workflow {} started", workflow.workflow_id);
    
    Ok(())
}

/// Execute automation step
pub fn execute_automation_step(
    step: &mut AutomationStep,
    result_hash: [u8; 32],
    executor: Pubkey,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    execute_automation_step_with_time(step, result_hash, executor, current_time)
}

/// Execute automation step with explicit timestamp (for testing)
pub fn execute_automation_step_with_time(
    step: &mut AutomationStep,
    result_hash: [u8; 32],
    executor: Pubkey,
    current_time: i64,
) -> Result<()> {
    require!(
        step.status == StepStatus::Pending || 
        step.status == StepStatus::InProgress,
        IndrasError::InvalidState
    );
    
    step.status = StepStatus::Completed;
    step.executed_at = Some(current_time);
    step.executor = executor;
    step.result_hash = result_hash;
    
    msg!("Automation step {} executed (workflow: {})", step.step_id, step.workflow_id);
    
    Ok(())
}

/// Update workflow progress
pub fn update_workflow_progress(
    workflow: &mut AutomationWorkflow,
    current_step: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    update_workflow_progress_with_time(workflow, current_step, current_time)
}

/// Update workflow progress with explicit timestamp (for testing)
pub fn update_workflow_progress_with_time(
    workflow: &mut AutomationWorkflow,
    current_step: u64,
    current_time: i64,
) -> Result<()> {
    require!(workflow.is_active, IndrasError::InvalidState);
    require!(workflow.status == WorkflowStatus::InProgress, IndrasError::InvalidState);
    require!(current_step <= workflow.total_steps, IndrasError::InvalidInput);
    
    workflow.current_step = current_step;
    workflow.updated_at = current_time;
    
    // Check if workflow is completed
    if current_step >= workflow.total_steps {
        workflow.status = WorkflowStatus::Completed;
        msg!("Automation workflow {} completed", workflow.workflow_id);
    }
    
    Ok(())
}

/// Complete automation workflow
pub fn complete_automation_workflow(
    workflow: &mut AutomationWorkflow,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    complete_automation_workflow_with_time(workflow, current_time)
}

/// Complete automation workflow with explicit timestamp (for testing)
pub fn complete_automation_workflow_with_time(
    workflow: &mut AutomationWorkflow,
    current_time: i64,
) -> Result<()> {
    require!(workflow.status == WorkflowStatus::InProgress, IndrasError::InvalidState);
    require!(workflow.current_step >= workflow.total_steps, IndrasError::InvalidState);
    
    workflow.status = WorkflowStatus::Completed;
    workflow.updated_at = current_time;
    
    msg!("Automation workflow {} completed successfully", workflow.workflow_id);
    
    Ok(())
}

// ============================================================================
// OFF-CHAIN FUNCTIONS (Regular Rust - Not Anchor)
// ============================================================================

/// OFF-CHAIN: Evaluate automation trigger
/// 
/// This function should be called off-chain to evaluate if trigger conditions are met.
pub fn evaluate_trigger_offchain(
    _trigger: AutomationTrigger,
    _trigger_params: &str,
    _partnership: &Partnership,
    _metrics: &PartnershipMetrics,
) -> Result<bool> {
    // OFF-CHAIN IMPLEMENTATION:
    // 1. Parse trigger parameters
    // 2. Evaluate trigger conditions
    // 3. Return true if trigger should fire
    
    // This is a placeholder - actual implementation happens off-chain
    Ok(false)
}

/// OFF-CHAIN: Execute workflow step
/// 
/// This function should be called off-chain to execute actual workflow logic.
pub fn execute_workflow_step_offchain(
    _step: &AutomationStep,
    _workflow: &AutomationWorkflow,
    _partnership: &Partnership,
) -> Result<[u8; 32]> {
    // OFF-CHAIN IMPLEMENTATION:
    // 1. Execute step logic
    // 2. Generate result hash
    // 3. Return hash to be stored on-chain
    
    // This is a placeholder - actual implementation happens off-chain
    Ok([0u8; 32])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_automation_workflow() {
        let mut workflow = AutomationWorkflow {
            workflow_id: 0,
            partnership_id: 0,
            name: String::new(),
            description: String::new(),
            status: WorkflowStatus::NotStarted,
            trigger: AutomationTrigger::Manual,
            trigger_params: String::new(),
            current_step: 0,
            total_steps: 0,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: false,
            bump: 0,
        };

        let creator = Pubkey::from([1u8; 32]);

        let result = create_automation_workflow_with_time(
            &mut workflow,
            1,
            100,
            "Test Workflow".to_string(),
            "Test Description".to_string(),
            AutomationTrigger::MilestoneCompletion,
            r#"{"milestone_id": 1}"#.to_string(),
            5,
            creator,
            1000, // Mock timestamp
        );

        assert!(result.is_ok());
        assert_eq!(workflow.workflow_id, 1);
        assert_eq!(workflow.partnership_id, 100);
        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.status, WorkflowStatus::NotStarted);
        assert_eq!(workflow.total_steps, 5);
        assert!(workflow.is_active);
    }

    #[test]
    fn test_start_automation_workflow() {
        let mut workflow = AutomationWorkflow {
            workflow_id: 1,
            partnership_id: 100,
            name: "Test".to_string(),
            description: "Test".to_string(),
            status: WorkflowStatus::NotStarted,
            trigger: AutomationTrigger::Manual,
            trigger_params: String::new(),
            current_step: 0,
            total_steps: 5,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: true,
            bump: 0,
        };

        let result = start_automation_workflow_with_time(&mut workflow, 1000);

        assert!(result.is_ok());
        assert_eq!(workflow.status, WorkflowStatus::InProgress);
    }

    #[test]
    fn test_start_workflow_fails_when_not_active() {
        let mut workflow = AutomationWorkflow {
            workflow_id: 1,
            partnership_id: 100,
            name: "Test".to_string(),
            description: "Test".to_string(),
            status: WorkflowStatus::NotStarted,
            trigger: AutomationTrigger::Manual,
            trigger_params: String::new(),
            current_step: 0,
            total_steps: 5,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: false, // Not active
            bump: 0,
        };

        let result = start_automation_workflow(&mut workflow);

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_automation_step() {
        let mut step = AutomationStep {
            step_id: 1,
            workflow_id: 1,
            step_index: 0,
            name: "Test Step".to_string(),
            description: "Test".to_string(),
            status: StepStatus::Pending,
            executed_at: None,
            executor: Pubkey::default(),
            result_hash: [0u8; 32],
            bump: 0,
        };

        let executor = Pubkey::from([1u8; 32]);
        let result_hash = [1u8; 32];

        let result = execute_automation_step_with_time(&mut step, result_hash, executor, 1000);

        assert!(result.is_ok());
        assert_eq!(step.status, StepStatus::Completed);
        assert!(step.executed_at.is_some());
        assert_eq!(step.executor, executor);
    }

    #[test]
    fn test_update_workflow_progress() {
        let mut workflow = AutomationWorkflow {
            workflow_id: 1,
            partnership_id: 100,
            name: "Test".to_string(),
            description: "Test".to_string(),
            status: WorkflowStatus::InProgress,
            trigger: AutomationTrigger::Manual,
            trigger_params: String::new(),
            current_step: 0,
            total_steps: 5,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: true,
            bump: 0,
        };

        let result = update_workflow_progress_with_time(&mut workflow, 3, 1000);

        assert!(result.is_ok());
        assert_eq!(workflow.current_step, 3);
        assert_eq!(workflow.status, WorkflowStatus::InProgress); // Not completed yet
    }

    #[test]
    fn test_complete_workflow_when_all_steps_done() {
        let mut workflow = AutomationWorkflow {
            workflow_id: 1,
            partnership_id: 100,
            name: "Test".to_string(),
            description: "Test".to_string(),
            status: WorkflowStatus::InProgress,
            trigger: AutomationTrigger::Manual,
            trigger_params: String::new(),
            current_step: 5, // All steps done
            total_steps: 5,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: true,
            bump: 0,
        };

        let result = complete_automation_workflow_with_time(&mut workflow, 1000);

        assert!(result.is_ok());
        assert_eq!(workflow.status, WorkflowStatus::Completed);
    }

    #[test]
    fn test_workflow_auto_completes_on_progress_update() {
        let mut workflow = AutomationWorkflow {
            workflow_id: 1,
            partnership_id: 100,
            name: "Test".to_string(),
            description: "Test".to_string(),
            status: WorkflowStatus::InProgress,
            trigger: AutomationTrigger::Manual,
            trigger_params: String::new(),
            current_step: 4,
            total_steps: 5,
            creator: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            is_active: true,
            bump: 0,
        };

        let result = update_workflow_progress_with_time(&mut workflow, 5, 1000); // Final step

        assert!(result.is_ok());
        assert_eq!(workflow.current_step, 5);
        assert_eq!(workflow.status, WorkflowStatus::Completed);
    }
}
