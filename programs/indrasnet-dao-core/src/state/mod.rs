//! State structures for the IndrasNet DAO Core program

// Core state modules
pub mod dao_config;
pub mod rate_limit_tracker;
pub mod security;
pub mod security_invariants;
pub mod enums;
pub mod fsm;
pub mod idea;
pub mod idea_vote;
pub mod member;
pub mod proposal;
pub mod proposal_execution;
pub mod vote_delegation;
pub mod grant;
pub mod mesh_group;
pub mod phenomenon;
pub mod commercial_enterprise;
pub mod expert_registry;
pub mod governance_params;
pub mod treasury;
pub mod governance;
pub mod membership;
// AI Analysis module (from indrasnet-dao-ai)
pub mod ai_analysis;
pub mod ai_analysis_record;
pub mod ai_service_registry;
pub mod embedding_deduplication;
pub mod model_registry;
// REMOVED FOR MVP: Move to off-chain services (defi_integration/)
// pub mod rewards;
// pub mod staking;
// pub mod vesting;
// pub mod liquidity;
// pub mod bonds;
// pub mod insurance;
// pub mod derivatives;
// pub mod loans;
// pub mod options;
// pub mod futures;
// pub mod swaps;
// pub mod commodities;
// pub mod forex;
// pub mod cryptocurrency;
// REMOVED FOR MVP: Move to off-chain services (portfolio_management/)
// pub mod portfolio;
// portfolio_* modules merged into portfolio/
// REMOVED FOR MVP: Move to off-chain services (portfolio_management/)
// pub mod asset_management;
// asset_* modules merged into asset_management/
// REMOVED FOR MVP: Move to off-chain services (risk_compliance/)
// pub mod risk_management;
// REMOVED FOR MVP: Move to off-chain services (risk_compliance/)
// pub mod compliance;
// REMOVED FOR MVP: Move to off-chain services (accounting/)
// pub mod accounting;
// accounting_* modules merged into accounting/
// REMOVED FOR MVP: Move to off-chain services (taxation/)
// pub mod taxation;
// tax_* modules merged into taxation/
// REMOVED FOR MVP: Move to off-chain services (reporting_audit/)
// pub mod reporting;
// REMOVED FOR MVP: Move to off-chain services (reporting_audit/)
// pub mod audit;
// REMOVED FOR MVP: Move to off-chain services
// pub mod budgeting;
// REMOVED FOR MVP: Move to off-chain services
// pub mod expense_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod cost_management;
// REMOVED FOR MVP: Move to off-chain services (payment_processing/)
// pub mod payment;
// payment_processing and invoice_* modules merged into payment/
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_planning;
// financial_strategic_planning and financial_scenario_planning merged into financial_planning/
// REMOVED FOR MVP: Move to off-chain services
// pub mod cash_flow;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_reporting;
// financial_reporting modules merged into financial_reporting/
// asset_tracking merged into asset_management/tracking
// asset_valuation merged into asset_management/valuation
// asset_allocation merged into asset_management/allocation
// REMOVED FOR MVP: Move to off-chain services
// pub mod performance_metrics;
// tax_optimization merged into taxation/optimization
// REMOVED FOR MVP: Move to off-chain services
// pub mod rebalancing;
// REMOVED FOR MVP: Move to off-chain services
// pub mod transaction_monitoring;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_forecasting;
// budget_analysis merged into budgeting
// cost_analysis merged into cost_management
// expense_analysis merged into expense_management
// payment_reconciliation merged into payment/reconciliation
// invoice_processing merged into payment/invoice_processing
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_consolidation;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_modeling;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_analysis;
// investment_analysis and other analysis modules merged into financial_analysis/
// portfolio_optimization merged into portfolio/optimization
// REMOVED FOR MVP: Move to off-chain services
// pub mod risk_assessment;
// REMOVED FOR MVP: Move to off-chain services
// pub mod compliance_monitoring;
// accounting_reconciliation merged into accounting/reconciliation
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_controls;
// REMOVED FOR MVP: Move to off-chain services
// pub mod internal_audit;
// financial_planning_advanced merged into financial_planning
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_capital;
// capital_management and financial_capital_* modules merged into financial_capital/
// treasury_operations merged into treasury/operations
// treasury_analytics merged into treasury/analytics
// governance_analytics merged into governance/analytics
// member_analytics merged into member/analytics
// idea_analytics merged into idea
// grant_analytics merged into grant/analytics
// phenomenon_analytics merged into phenomenon
// mesh_group_analytics merged into mesh_group/analytics
// treasury_risk_management merged into treasury/risk
// governance_voting merged into governance/voting
// proposal_analytics merged into proposal/analytics
// member_contribution merged into member/contribution
// idea_evaluation merged into idea
// grant_evaluation merged into grant/evaluation
// phenomenon_tracking merged into phenomenon
// mesh_group_collaboration merged into mesh_group/collaboration
// commercial_enterprise_analytics merged into commercial_enterprise
// asset_valuation_advanced merged into asset_valuation
// portfolio_risk_management merged into portfolio/risk_management
// tax_planning merged into taxation/planning
// expense_forecasting merged into expense_management
// cost_forecasting merged into cost_management
// budget_forecasting merged into budgeting
// financial_budget_control merged into budgeting
// REMOVED FOR MVP: Move to off-chain services
// pub mod revenue_forecasting;
// profitability_analysis merged into financial_analysis/profitability
// liquidity_analysis merged into financial_analysis/liquidity
// solvency_analysis merged into financial_analysis/solvency
// efficiency_analysis merged into financial_analysis/efficiency
// growth_analysis merged into financial_analysis/growth
// valuation_analysis merged into financial_analysis/valuation
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_metrics;
// financial_ratios, financial_benchmarking, financial_kpis, financial_dashboard merged into financial_metrics/
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_alerts;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_workflow;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_automation;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_integration;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_data_management;
// financial_data_* modules merged into financial_data_management/
// treasury_allocation merged into treasury/allocation
// governance_participation merged into governance/participation
// member_reputation merged into member/reputation
// idea_ranking merged into idea
// grant_tracking merged into grant/tracking
// phenomenon_evolution merged into phenomenon
// mesh_group_governance merged into mesh_group/governance
// commercial_enterprise_operations merged into commercial_enterprise
// asset_performance merged into asset_management/performance
// portfolio_diversification merged into portfolio/diversification
// tax_compliance merged into taxation/compliance
// accounting_period merged into accounting/period
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_statement;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_audit_trail;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_approval_workflow;
// financial_budget_control merged into budgeting
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_center;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_currency_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_intercompany;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_consolidation_rules;
// financial_segment_reporting merged into financial_reporting/segment
// financial_management_reporting merged into financial_reporting/management
// financial_regulatory_reporting merged into financial_reporting/regulatory
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_disclosure;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_transparency;
// financial_risk_reporting merged into financial_reporting/risk
// financial_compliance_reporting merged into financial_reporting/compliance
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_performance_management;
// financial_strategic_planning merged into financial_planning/strategic
// financial_scenario_planning merged into financial_planning/scenario
// financial_sensitivity_analysis merged into financial_analysis/sensitivity
// financial_what_if_analysis merged into financial_analysis/what_if
// financial_break_even_analysis merged into financial_analysis/break_even
// financial_capital_* modules merged into financial_capital/
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_asset_liability_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_hedging;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_derivatives_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_valuation;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_due_diligence;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_merger_acquisition;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_ipo;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_restructuring;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_liquidation;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_bankruptcy;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_insolvency;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_recovery;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_crisis_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_continuity_planning;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_disaster_recovery;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_risk_mitigation;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_insurance;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_claims_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_legal_compliance;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_regulatory_compliance;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_contract_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_vendor_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_procurement;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_supply_chain;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_inventory_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_quality_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_project_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_resource_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_capacity_planning;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_workforce_planning;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_technology_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_innovation_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_research_development;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_intellectual_property;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_brand_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_marketing_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_sales_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_customer_relationship;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_stakeholder_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_sustainability;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_corporate_social_responsibility;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_environmental_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_energy_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_waste_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_carbon_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_climate_risk;
// financial_esg_reporting merged into financial_reporting/esg
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_impact_measurement;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_impact_investing;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_philanthropy;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_community_investment;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_social_impact;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_diversity_inclusion;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_ethics_compliance;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_governance_risk_compliance;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_enterprise_risk_management;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_operational_excellence;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_digital_transformation;
// REMOVED FOR MVP: Move to off-chain services
// pub mod financial_agility;

// Re-export commonly used types
pub use dao_config::DaoConfig;
pub use rate_limit_tracker::RateLimitTracker;
pub use enums::{
    IdeaStatus, MemberStatus, MemberAction, DelegationType, 
    CapabilityType, ContributionType, OffchainVoteStatus
};
// Note: MemberRole enum from enums is deprecated, use member::MemberRole struct instead
pub use idea::Idea;
pub use idea_vote::IdeaVote;
pub use member::{Member, MemberRegistry, MemberRole};
pub use proposal::{Proposal, ProposalStatus};
pub use proposal_execution::{ProposalExecution, ExecutionStatus};
pub use vote_delegation::VoteDelegation;
pub use grant::{Grant, GrantStatus, GrantCategory, GrantType, VerificationStatus};
pub use mesh_group::{
    MeshGroup, GroupType, GroupStatus, DevelopmentStage,
    GroupMember, MilestoneStatus, MeshGroupParams,
    GroupMemberHistory, MemberHistoryEntry, MemberLeaveReason
};
pub use phenomenon::{
    Phenomenon,
    DiscoveryMethod,
    PhenomenonStatus,
};
// REMOVED FOR MVP: Can be deferred
// pub use commercial_enterprise::{
//     CommercialEnterprise, EnterpriseType, EnterpriseStatus, ProductionStatus
// };
pub use governance_params::GovernanceParams;
// Treasury re-exports (specific to avoid ambiguous glob re-exports)
pub use treasury::{
    Treasury, TreasuryOperationType, TreasuryTransactionStatus, TreasuryBalanceType,
    TreasuryDepositEvent, TreasuryWithdrawalEvent, TreasuryTransferEvent,
    CapabilityGrantEvent, CapabilityRevokeEvent, TreasuryBalanceUpdateEvent,
    TreasuryAllocationMetadata, TreasuryStrategyMetadata, TreasuryStrategyType,
    TreasuryBatchOperationMetadata, BatchOperationStatus,
    TreasuryReportMetadata, TreasuryReportType,
    TreasuryOperationsMetadata, TreasuryOperationStatus,
    TreasuryAnalyticsMetadata, TreasuryAnalyticsType, TreasuryAnalyticsStatus,
    TreasuryRiskManagementMetadata, TreasuryRiskType, TreasuryRiskStatus,
    TreasuryAllocationStrategyMetadata, TreasuryAllocationStrategy, TreasuryAllocationStatus,
};
// Governance re-exports (specific to avoid ambiguous glob re-exports)
pub use governance::{
    SecurityBoardMemberMetadata, SecurityBoardDecisionMetadata,
    SecurityBoardMemberRole, SecurityBoardDecisionStatus,
    QuorumMetadata, QuorumCalculationMethod,
    SecurityPolicyMetadata, SecurityPolicyStatus,
    SecurityExcellenceMetadata,
    ProposalLifecycleMetadata, ProposalLifecycleStage,
    SecurityCommitteeMetadata, CommitteeMemberRole,
    GovernanceAnalyticsMetadata, GovernanceAnalyticsType, GovernanceAnalyticsStatus,
    GovernanceVotingMetadata, GovernanceVotingType, GovernanceVotingStatus,
    GovernanceParticipationMetadata, GovernanceParticipationType, GovernanceParticipationStatus,
};
pub use membership::{
    MembershipMetadata, MembershipStatus, MembershipTier,
    onchain as membership_onchain,
};
// REMOVED FOR MVP: Move to off-chain services (defi_integration/)
// pub use rewards::{
//     RewardMetadata, RewardType, RewardStatus,
//     onchain as rewards_onchain,
// };
// pub use staking::{
//     StakingMetadata, StakingStatus,
//     onchain as staking_onchain,
// };
// pub use vesting::{
//     VestingMetadata, VestingStatus,
//     onchain as vesting_onchain,
// };
// pub use liquidity::{
//     LiquidityPoolMetadata, PoolStatus,
//     onchain as liquidity_onchain,
// };
// pub use bonds::{
//     BondMetadata, BondStatus,
//     onchain as bonds_onchain,
// };
// pub use insurance::{
//     InsurancePolicyMetadata, InsurancePolicyStatus,
//     onchain as insurance_onchain,
// };
// pub use derivatives::{
//     DerivativeMetadata, DerivativeType, DerivativeStatus,
//     onchain as derivatives_onchain,
// };
// pub use loans::{
//     LoanMetadata, LoanStatus,
//     onchain as loans_onchain,
// };
// pub use options::{
//     OptionMetadata, OptionType, OptionStatus,
//     onchain as options_onchain,
// };
// pub use futures::{
//     FuturesMetadata, FuturesStatus,
//     onchain as futures_onchain,
// };
// pub use swaps::{
//     SwapMetadata, SwapStatus,
//     onchain as swaps_onchain,
// };
// pub use commodities::{
//     CommodityMetadata, CommodityType, CommodityStatus,
//     onchain as commodities_onchain,
// };
// pub use forex::{
//     ForexPairMetadata, ForexPairStatus,
//     onchain as forex_onchain,
// };
// pub use cryptocurrency::{
//     CryptocurrencyMetadata, CryptocurrencyStatus,
//     onchain as cryptocurrency_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services (portfolio_management/)
// Portfolio now includes all portfolio modules
// Type aliases for backward compatibility
// pub use portfolio::{
//     PortfolioMetadata, PortfolioStatus,
//     PortfolioOptimizationMetadata, PortfolioOptimizationMethod, PortfolioOptimizationStatus,
//     PortfolioRiskManagementMetadata, PortfolioRiskManagementStrategy, PortfolioRiskManagementStatus,
//     PortfolioDiversificationMetadata, PortfolioDiversificationStrategy, PortfolioDiversificationStatus,
//     initialize_portfolio,
//     initialize_portfolio_optimization,
//     initialize_portfolio_risk_management,
//     initialize_portfolio_diversification,
// };
// REMOVED FOR MVP: Move to off-chain services (portfolio_management/)
// Asset management now includes all asset modules
// Type aliases for backward compatibility
// pub use asset_management::{
//     AssetMetadata, AssetType, AssetStatus,
//     AssetTrackingMetadata, AssetTrackingStatus,
//     AssetValuationMetadata, ValuationMethod, AssetValuationStatus,
//     AssetAllocationMetadata, AllocationStrategy, AssetAllocationStatus,
//     AssetPerformanceMetadata, AssetPerformanceMetric, AssetPerformanceStatus,
//     initialize_asset,
//     initialize_asset_tracking,
//     initialize_asset_valuation,
//     initialize_advanced_asset_valuation,
//     initialize_asset_allocation,
//     initialize_asset_performance,
// };
// REMOVED FOR MVP: Move to off-chain services (risk_compliance/)
// pub use risk_management::{
//     RiskAssessmentMetadata, RiskLevel,
//     onchain as risk_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services (risk_compliance/)
// pub use compliance::{
//     ComplianceCheckMetadata, ComplianceStatus,
//     onchain as compliance_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services (accounting/)
// Accounting now includes all accounting modules
// Type aliases for backward compatibility
// pub use accounting::{
//     AccountingEntryMetadata, AccountingEntryType,
//     AccountingPeriodMetadata, AccountingPeriodType, AccountingPeriodStatus,
//     AccountingReconciliationMetadata, AccountingReconciliationStatus,
//     initialize_accounting_entry,
//     initialize_accounting_period,
//     initialize_accounting_reconciliation,
// };
// REMOVED FOR MVP: Move to off-chain services (taxation/)
// Taxation now includes all tax modules
// Type aliases for backward compatibility
// pub use taxation::{
//     TaxRecordMetadata, TaxType, TaxRecordStatus,
//     TaxComplianceMetadata, TaxComplianceRequirement, TaxComplianceStatus,
//     TaxPlanningMetadata, TaxPlanningStrategy, TaxPlanningStatus,
//     TaxOptimizationMetadata, TaxOptimizationStrategy, TaxOptimizationStatus,
//     initialize_tax_record,
//     initialize_tax_compliance,
//     initialize_tax_planning,
//     initialize_tax_optimization,
// };
// REMOVED FOR MVP: Move to off-chain services (reporting_audit/)
// pub use reporting::{
//     ReportMetadata, ReportType, ReportStatus,
//     onchain as reporting_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services (reporting_audit/)
// pub use audit::{
//     AuditMetadata, AuditType, AuditStatus,
//     onchain as audit_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use budgeting::{
//     BudgetMetadata, BudgetStatus,
//     onchain as budgeting_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use expense_management::{
//     ExpenseMetadata, ExpenseCategory, ExpenseStatus,
//     onchain as expense_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use cost_management::{
//     CostMetadata, CostType,
//     onchain as cost_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services (payment_processing/)
// Payment now includes all payment and invoice modules
// Type aliases for backward compatibility
// pub use payment::{
//     PaymentMetadata, PaymentMethod, PaymentStatus,
//     PaymentReconciliationMetadata, PaymentReconciliationStatus,
//     InvoiceMetadata, InvoiceStatus,
//     InvoiceProcessingMetadata, InvoiceProcessingStatus,
//     initialize_payment,
//     initialize_payment_reconciliation,
//     initialize_invoice,
//     initialize_invoice_processing,
// };
// Invoice management now part of payment module
// Re-exports already included in payment section above
// Financial planning now includes all planning modules
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_planning::{
//    FinancialPlanMetadata, FinancialPlanStatus, FinancialPlanningType,
//    FinancialStrategicPlanningMetadata, FinancialStrategicPlanningHorizon, FinancialStrategicPlanningStatus,
//    FinancialScenarioPlanningMetadata, FinancialScenarioType, FinancialScenarioPlanningStatus,
//    initialize_financial_plan,
//    initialize_advanced_financial_planning,
//    initialize_financial_strategic_planning,
//    initialize_financial_scenario_planning,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use cash_flow::{
//     CashFlowMetadata, CashFlowType,
//     onchain as cash_flow_onchain,
// };
// Financial reporting now part of financial_reporting module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_reporting::{
//    FinancialReportMetadata, FinancialReportType, FinancialReportStatus,
//    FinancialESGReportingMetadata, FinancialESGDimension, FinancialESGReportStatus,
//    FinancialComplianceReportingMetadata, FinancialComplianceFramework, FinancialComplianceReportStatus,
//    FinancialRiskReportingMetadata, FinancialRiskReportType, FinancialRiskReportStatus,
//    FinancialRegulatoryReportingMetadata, FinancialRegulatoryRequirement, FinancialRegulatoryReportStatus,
//    FinancialManagementReportingMetadata, FinancialManagementReportType, FinancialManagementReportStatus,
//    FinancialSegmentReportingMetadata, FinancialSegmentType, FinancialSegmentReportingStatus,
//    initialize_financial_report,
//    initialize_financial_esg_reporting,
//    initialize_financial_compliance_reporting,
//    initialize_financial_risk_reporting,
//    initialize_financial_regulatory_reporting,
//    initialize_financial_management_reporting,
//    initialize_financial_segment_reporting,
// };
// Asset tracking/valuation/allocation now part of asset_management module
// Re-exports already included in asset_management section above
// REMOVED FOR MVP: Move to off-chain services
// pub use performance_metrics::{
//     PerformanceMetricsMetadata, PerformanceMetricType, PerformanceMetricStatus,
//     onchain as performance_metrics_onchain,
// };
// Tax optimization now part of taxation module
// Re-exports already included in taxation section above
// REMOVED FOR MVP: Move to off-chain services
// pub use rebalancing::{
//     RebalancingMetadata, RebalancingStrategy, RebalancingStatus,
//     onchain as rebalancing_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use transaction_monitoring::{
//     TransactionMonitoringMetadata, TransactionMonitoringType, TransactionMonitoringStatus,
//     onchain as transaction_monitoring_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_forecasting::{
//    FinancialForecastingMetadata, ForecastingMethod, FinancialForecastingStatus,
//    onchain as financial_forecasting_onchain,
// };
// Budget analysis now part of budgeting module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain services
// pub use budgeting::{
//     BudgetAnalysisMetadata, BudgetAnalysisType,
//     onchain::initialize_budget_analysis,
// };
// Cost analysis now part of cost_management module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain services
// pub use cost_management::{
//     CostAnalysisMetadata, CostAnalysisType,
//     onchain::initialize_cost_analysis,
// };
// Expense analysis now part of expense_management module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain services
// pub use expense_management::{
//     ExpenseAnalysisMetadata, ExpenseAnalysisType,
//     onchain::initialize_expense_analysis,
// };
// Payment reconciliation and invoice processing now part of payment module
// Re-exports already included in payment section above
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_consolidation::{
//    FinancialConsolidationMetadata, FinancialConsolidationType, FinancialConsolidationStatus,
//    onchain as financial_consolidation_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_modeling::{
//    FinancialModelingMetadata, FinancialModelType, FinancialModelStatus,
//    onchain as financial_modeling_onchain,
// };
// Investment analysis now part of financial_analysis module
// Re-exports included in financial_analysis section below
// Portfolio optimization now part of portfolio module
// Re-exports already included in portfolio section above
// REMOVED FOR MVP: Move to off-chain services
// pub use risk_assessment::{
//     RiskAssessmentAnalysisMetadata, RiskAssessmentType, RiskAssessmentStatus,
//     onchain as risk_assessment_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use compliance_monitoring::{
//     ComplianceMonitoringMetadata, ComplianceMonitoringType, ComplianceMonitoringStatus,
//     onchain as compliance_monitoring_onchain,
// };
// Accounting reconciliation now part of accounting module
// Re-exports already included in accounting section above
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_controls::{
//    FinancialControlsMetadata, FinancialControlType, FinancialControlStatus,
//    onchain as financial_controls_onchain,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use internal_audit::{
//     InternalAuditMetadata, InternalAuditType, InternalAuditStatus,
//     onchain as internal_audit_onchain,
// };
// Advanced financial planning now part of financial_planning module
// Re-exports already included in financial_planning section above
// Financial capital now includes all capital management modules
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_capital::{
//    CapitalManagementMetadata, CapitalManagementStrategy, CapitalManagementStatus,
//    FinancialCapitalStructureMetadata, FinancialCapitalComponent, FinancialCapitalStructureStatus,
//    FinancialWorkingCapitalMetadata, FinancialWorkingCapitalComponent, FinancialWorkingCapitalStatus,
//    FinancialCashManagementMetadata, FinancialCashManagementStrategy, FinancialCashManagementStatus,
//    FinancialDebtManagementMetadata, FinancialDebtType, FinancialDebtManagementStatus,
//    FinancialEquityManagementMetadata, FinancialEquityType, FinancialEquityManagementStatus,
//    FinancialInvestmentManagementMetadata, FinancialInvestmentStrategy, FinancialInvestmentManagementStatus,
//    initialize_capital_management,
//    initialize_financial_capital_structure,
//    initialize_financial_working_capital,
//    initialize_financial_cash_management,
//    initialize_financial_debt_management,
//    initialize_financial_equity_management,
//    initialize_financial_investment_management,
// };
// Treasury operations and analytics already included in treasury re-exports above
// Governance analytics already included in governance re-exports above
// Member analytics now part of member module
// Type aliases for backward compatibility
pub use member::{
    MemberAnalyticsMetadata, MemberAnalyticsType, MemberAnalyticsStatus,
    initialize_member_analytics,
};
// Idea analytics now part of idea module
// Type aliases for backward compatibility
pub use idea::{
    IdeaAnalyticsMetadata, IdeaAnalyticsType, IdeaAnalyticsStatus,
    onchain::initialize_idea_analytics,
};
// Grant analytics now part of grant module
// Type aliases for backward compatibility
pub use grant::{
    GrantAnalyticsMetadata, GrantAnalyticsType, GrantAnalyticsStatus,
    initialize_grant_analytics,
};
// Phenomenon analytics now part of phenomenon module
// Type aliases for backward compatibility
pub use phenomenon::{
    PhenomenonAnalyticsMetadata, PhenomenonAnalyticsType, PhenomenonAnalyticsStatus,
    onchain::initialize_phenomenon_analytics,
};
// Mesh group analytics now part of mesh_group module
// Type aliases for backward compatibility
pub use mesh_group::{
    MeshGroupAnalyticsMetadata, MeshGroupAnalyticsType, MeshGroupAnalyticsStatus,
    initialize_mesh_group_analytics,
};
// Treasury risk management already included in treasury re-exports above
// Governance voting already included in governance re-exports above
// Proposal analytics now part of proposal module
// Type aliases for backward compatibility
pub use proposal::{
    ProposalAnalyticsMetadata, ProposalAnalyticsType, ProposalAnalyticsStatus,
    initialize_proposal_analytics,
};
// Member contribution now part of member module
// Type aliases for backward compatibility
pub use member::{
    MemberContributionMetadata, MemberContributionType, MemberContributionStatus,
    initialize_member_contribution,
};
// Idea evaluation now part of idea module
// Type aliases for backward compatibility
pub use idea::{
    IdeaEvaluationMetadata, IdeaEvaluationCriteria, IdeaEvaluationStatus,
    onchain::initialize_idea_evaluation,
};
// Grant evaluation now part of grant module
// Type aliases for backward compatibility
pub use grant::{
    GrantEvaluationMetadata, GrantEvaluationCriteria, GrantEvaluationStatus,
    initialize_grant_evaluation,
};
// Phenomenon tracking now part of phenomenon module
// Type aliases for backward compatibility
pub use phenomenon::{
    PhenomenonTrackingMetadata, PhenomenonTrackingType, PhenomenonTrackingStatus,
    onchain::initialize_phenomenon_tracking,
};
// Mesh group collaboration now part of mesh_group module
// Type aliases for backward compatibility
pub use mesh_group::{
    MeshGroupCollaborationMetadata, MeshGroupCollaborationType, MeshGroupCollaborationStatus,
    initialize_mesh_group_collaboration,
};
// REMOVED FOR MVP: Can be deferred
// Commercial enterprise analytics now part of commercial_enterprise module
// Type aliases for backward compatibility
// pub use commercial_enterprise::{
//     CommercialEnterpriseAnalyticsMetadata, CommercialEnterpriseAnalyticsType, CommercialEnterpriseAnalyticsStatus,
//     onchain::initialize_commercial_enterprise_analytics,
// };
// Advanced asset valuation now part of asset_management module
// Re-exports already included in asset_management section above
// Portfolio risk management now part of portfolio module
// Re-exports already included in portfolio section above
// Tax planning now part of taxation module
// Re-exports already included in taxation section above
// Expense forecasting now part of expense_management module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain services
// pub use expense_management::{
//     ExpenseForecastingMetadata, ExpenseForecastingMethod,
//     onchain::initialize_expense_forecasting,
// };
// Cost forecasting now part of cost_management module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain services
// pub use cost_management::{
//     CostForecastingMetadata, CostForecastingMethod,
//     onchain::initialize_cost_forecasting,
// };
// Budget forecasting now part of budgeting module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain services
// pub use budgeting::{
//     BudgetForecastingMetadata, BudgetForecastingMethod,
//     onchain::initialize_budget_forecasting,
// };
// REMOVED FOR MVP: Move to off-chain services
// pub use revenue_forecasting::{
//     RevenueForecastingMetadata, RevenueForecastingMethod, RevenueForecastingStatus,
//     onchain as revenue_forecasting_onchain,
// };
// Financial analysis now includes all analysis modules
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_analysis::{
//    ProfitabilityAnalysisMetadata, ProfitabilityAnalysisType, ProfitabilityAnalysisStatus,
//    LiquidityAnalysisMetadata, LiquidityAnalysisType, LiquidityAnalysisStatus,
//    SolvencyAnalysisMetadata, SolvencyAnalysisType, SolvencyAnalysisStatus,
//    EfficiencyAnalysisMetadata, EfficiencyAnalysisType, EfficiencyAnalysisStatus,
//    GrowthAnalysisMetadata, GrowthAnalysisType, GrowthAnalysisStatus,
//    ValuationAnalysisMetadata, ValuationAnalysisType, ValuationAnalysisStatus,
//    InvestmentAnalysisMetadata, InvestmentAnalysisType, InvestmentAnalysisStatus,
//    FinancialBreakEvenAnalysisMetadata, FinancialBreakEvenAnalysisMethod, FinancialBreakEvenAnalysisStatus,
//    FinancialWhatIfAnalysisMetadata, FinancialWhatIfAnalysisScenario, FinancialWhatIfAnalysisStatus,
//    FinancialSensitivityAnalysisMetadata, FinancialSensitivityAnalysisType, FinancialSensitivityAnalysisStatus,
//    initialize_profitability_analysis,
//    initialize_liquidity_analysis,
//    initialize_solvency_analysis,
//    initialize_efficiency_analysis,
//    initialize_growth_analysis,
//    initialize_valuation_analysis,
//    initialize_investment_analysis,
//    initialize_financial_break_even_analysis,
//    initialize_financial_what_if_analysis,
//    initialize_financial_sensitivity_analysis,
// };
// Financial metrics now includes all metrics modules
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_metrics::{
//    FinancialRatiosMetadata, FinancialRatioType, FinancialRatioStatus,
//    FinancialBenchmarkingMetadata, FinancialBenchmarkType, FinancialBenchmarkStatus,
//    FinancialKPIsMetadata, FinancialKPICategory, FinancialKPIStatus,
//    FinancialDashboardMetadata, FinancialDashboardType, FinancialDashboardStatus,
//    initialize_financial_ratios,
//    initialize_financial_benchmarking,
//    initialize_financial_kpis,
//    initialize_financial_dashboard,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_alerts::{
//    FinancialAlertsMetadata, FinancialAlertType, FinancialAlertStatus,
//    onchain as financial_alerts_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_workflow::{
//    FinancialWorkflowMetadata, FinancialWorkflowType, FinancialWorkflowStatus,
//    onchain as financial_workflow_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_automation::{
//    FinancialAutomationMetadata, FinancialAutomationType, FinancialAutomationStatus,
//    onchain as financial_automation_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_integration::{
//    FinancialIntegrationMetadata, FinancialIntegrationType, FinancialIntegrationStatus,
//    onchain as financial_integration_onchain,
// };
// Financial data management now part of financial_data_management module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_data_management::{
//    FinancialDataQualityMetadata, FinancialDataQualityCheckType, FinancialDataQualityStatus,
//    FinancialDataGovernanceMetadata, FinancialDataGovernancePolicyType, FinancialDataGovernanceStatus,
//    FinancialDataSecurityMetadata, FinancialDataSecurityMeasureType, FinancialDataSecurityStatus,
//    FinancialDataBackupMetadata, FinancialDataBackupType, FinancialDataBackupStatus,
//    FinancialDataArchivalMetadata, FinancialDataArchivalStrategy, FinancialDataArchivalStatus,
//    FinancialDataMigrationMetadata, FinancialDataMigrationType, FinancialDataMigrationStatus,
//    FinancialDataSynchronizationMetadata, FinancialDataSynchronizationType, FinancialDataSynchronizationStatus,
//    FinancialDataTransformationMetadata, FinancialDataTransformationType, FinancialDataTransformationStatus,
//    FinancialDataValidationMetadata, FinancialDataValidationRuleType, FinancialDataValidationStatus,
//    FinancialDataCleansingMetadata, FinancialDataCleansingType, FinancialDataCleansingStatus,
//    FinancialDataEnrichmentMetadata, FinancialDataEnrichmentType, FinancialDataEnrichmentStatus,
//    FinancialDataLineageMetadata, FinancialDataLineageTrackingType, FinancialDataLineageStatus,
//    FinancialDataCatalogMetadata, FinancialDataCatalogType, FinancialDataCatalogStatus,
//    initialize_financial_data_quality,
//    initialize_financial_data_governance,
//    initialize_financial_data_security,
//    initialize_financial_data_backup,
//    initialize_financial_data_archival,
//    initialize_financial_data_migration,
//    initialize_financial_data_synchronization,
//    initialize_financial_data_transformation,
//    initialize_financial_data_validation,
//    initialize_financial_data_cleansing,
//    initialize_financial_data_enrichment,
//    initialize_financial_data_lineage,
//    initialize_financial_data_catalog,
// };
// Treasury allocation now part of treasury module
// Type aliases for backward compatibility
// NOTE: advanced_management::TreasuryAllocationMetadata is for detailed allocation
// allocation::TreasuryAllocationStrategyMetadata is for strategy-based allocation
// Treasury allocation already included in treasury re-exports above
// Governance participation already included in governance re-exports above
// Member reputation now part of member module
// Type aliases for backward compatibility
pub use member::{
    MemberReputationMetadata, MemberReputationFactor, MemberReputationStatus,
    initialize_member_reputation,
};
// Idea ranking now part of idea module
// Type aliases for backward compatibility
pub use idea::{
    IdeaRankingMetadata, IdeaRankingAlgorithm, IdeaRankingStatus,
    onchain::initialize_idea_ranking,
};
// Grant tracking now part of grant module
// Type aliases for backward compatibility
pub use grant::{
    GrantTrackingMetadata, GrantTrackingMetric, GrantTrackingStatus,
    initialize_grant_tracking,
};
// Phenomenon evolution now part of phenomenon module
// Type aliases for backward compatibility
pub use phenomenon::{
    PhenomenonEvolutionMetadata, PhenomenonEvolutionStage, PhenomenonEvolutionStatus,
    onchain::initialize_phenomenon_evolution,
};
// Mesh group governance now part of mesh_group module
// Type aliases for backward compatibility
pub use mesh_group::{
    MeshGroupGovernanceMetadata, MeshGroupGovernanceModel, MeshGroupGovernanceStatus,
    initialize_mesh_group_governance,
};
// REMOVED FOR MVP: Can be deferred
// Commercial enterprise operations now part of commercial_enterprise module
// Type aliases for backward compatibility
// pub use commercial_enterprise::{
//     CommercialEnterpriseOperationsMetadata, CommercialEnterpriseOperationType, CommercialEnterpriseOperationStatus,
//     onchain::initialize_commercial_enterprise_operations,
// };
// Asset performance now part of asset_management module
// Re-exports already included in asset_management section above
// Portfolio diversification now part of portfolio module
// Re-exports already included in portfolio section above
// Tax compliance now part of taxation module
// Re-exports already included in taxation section above
// Accounting period now part of accounting module
// Re-exports already included in accounting section above
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_statement::{
//    FinancialStatementMetadata, FinancialStatementType, FinancialStatementStatus,
//    onchain as financial_statement_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_audit_trail::{
//    FinancialAuditTrailMetadata, FinancialAuditTrailEventType, FinancialAuditTrailStatus,
//    onchain as financial_audit_trail_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_approval_workflow::{
//    FinancialApprovalWorkflowMetadata, FinancialApprovalLevel, FinancialApprovalWorkflowStatus,
//    onchain as financial_approval_workflow_onchain,
// };
// Budget control now part of budgeting module
// Type aliases for backward compatibility
// REMOVED FOR MVP: Move to off-chain services
// pub use budgeting::{
//     BudgetControlMetadata as FinancialBudgetControlMetadata,
//     BudgetControlType as FinancialBudgetControlType,
//     onchain::initialize_budget_control,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_center::{
//    // Unified types
//    FinancialCenterMetadata, FinancialCenterType, FinancialCenterStatus,
//    CostCenterCategory, ProfitCenterCategory,
//    onchain as financial_center_onchain,
//    // Legacy type aliases for backward compatibility
//    FinancialCostCenterMetadata, FinancialCostCenterType, FinancialCostCenterStatus,
//    FinancialProfitCenterMetadata, FinancialProfitCenterType, FinancialProfitCenterStatus,
//    // Legacy module aliases for backward compatibility
//    financial_cost_center_onchain, financial_profit_center_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_currency_management::{
//    FinancialCurrencyManagementMetadata, FinancialCurrencyOperationType, FinancialCurrencyStatus,
//    onchain as financial_currency_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_intercompany::{
//    FinancialIntercompanyMetadata, FinancialIntercompanyTransactionType, FinancialIntercompanyTransactionStatus,
//    onchain as financial_intercompany_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_consolidation_rules::{
//    FinancialConsolidationRulesMetadata, FinancialConsolidationRuleType, FinancialConsolidationRuleStatus,
//    onchain as financial_consolidation_rules_onchain,
// };
// Financial segment/management/regulatory reporting now part of financial_reporting module
// Re-exports already included in financial_reporting section above
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_disclosure::{
//    FinancialDisclosureMetadata, FinancialDisclosureType, FinancialDisclosureStatus,
//    onchain as financial_disclosure_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_transparency::{
//    FinancialTransparencyMetadata, FinancialTransparencyLevel, FinancialTransparencyStatus,
//    onchain as financial_transparency_onchain,
// };
// Financial risk/compliance reporting now part of financial_reporting module
// Re-exports already included in financial_reporting section above
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_performance_management::{
//    FinancialPerformanceManagementMetadata, FinancialPerformanceMetric, FinancialPerformanceManagementStatus,
//    onchain as financial_performance_management_onchain,
// };
// Financial strategic/scenario planning now part of financial_planning module
// Re-exports already included in financial_planning section above
// Financial sensitivity/what_if/break_even analysis now part of financial_analysis module
// Re-exports already included in financial_analysis section above
// Financial capital structure/working_capital/cash/debt/equity/investment management now part of financial_capital module
// Re-exports already included in financial_capital section above
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_asset_liability_management::{
//    FinancialAssetLiabilityManagementMetadata, FinancialALMStrategy, FinancialALMStatus,
//    onchain as financial_asset_liability_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_hedging::{
//    FinancialHedgingMetadata, FinancialHedgingInstrument, FinancialHedgingStatus,
//    onchain as financial_hedging_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_derivatives_management::{
//    FinancialDerivativesManagementMetadata, FinancialDerivativeType, FinancialDerivativesManagementStatus,
//    onchain as financial_derivatives_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_valuation::{
//    FinancialValuationMetadata, FinancialValuationMethod, FinancialValuationStatus,
//    onchain as financial_valuation_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_due_diligence::{
//    FinancialDueDiligenceMetadata, FinancialDueDiligenceType, FinancialDueDiligenceStatus,
//    onchain as financial_due_diligence_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_merger_acquisition::{
//    FinancialMergerAcquisitionMetadata, FinancialMATransactionType, FinancialMATransactionStatus,
//    onchain as financial_merger_acquisition_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_ipo::{
//    FinancialIPOMetadata, FinancialIPOStage, FinancialIPOStatus,
//    onchain as financial_ipo_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_restructuring::{
//    FinancialRestructuringMetadata, FinancialRestructuringType, FinancialRestructuringStatus,
//    onchain as financial_restructuring_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_liquidation::{
//    FinancialLiquidationMetadata, FinancialLiquidationType, FinancialLiquidationStatus,
//    onchain as financial_liquidation_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_bankruptcy::{
//    FinancialBankruptcyMetadata, FinancialBankruptcyType, FinancialBankruptcyStatus,
//    onchain as financial_bankruptcy_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_insolvency::{
//    FinancialInsolvencyMetadata, FinancialInsolvencyType, FinancialInsolvencyStatus,
//    onchain as financial_insolvency_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_recovery::{
//    FinancialRecoveryMetadata, FinancialRecoveryStrategy, FinancialRecoveryStatus,
//    onchain as financial_recovery_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_crisis_management::{
//    FinancialCrisisManagementMetadata, FinancialCrisisType, FinancialCrisisStatus,
//    onchain as financial_crisis_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_continuity_planning::{
//    FinancialContinuityPlanningMetadata, FinancialContinuityPlanningScope, FinancialContinuityPlanningStatus,
//    onchain as financial_continuity_planning_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_disaster_recovery::{
//    FinancialDisasterRecoveryMetadata, FinancialDisasterRecoveryType, FinancialDisasterRecoveryStatus,
//    onchain as financial_disaster_recovery_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_risk_mitigation::{
//    FinancialRiskMitigationMetadata, FinancialRiskMitigationStrategy, FinancialRiskMitigationStatus,
//    onchain as financial_risk_mitigation_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_insurance::{
//    FinancialInsuranceMetadata, FinancialInsuranceType, FinancialInsuranceStatus,
//    onchain as financial_insurance_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_claims_management::{
//    FinancialClaimsManagementMetadata, FinancialClaimType, FinancialClaimStatus,
//    onchain as financial_claims_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_legal_compliance::{
//    FinancialLegalComplianceMetadata, FinancialLegalComplianceRequirement, FinancialLegalComplianceStatus,
//    onchain as financial_legal_compliance_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_regulatory_compliance::{
//    FinancialRegulatoryComplianceMetadata, FinancialRegulatoryFramework, FinancialRegulatoryComplianceStatus,
//    onchain as financial_regulatory_compliance_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_contract_management::{
//    FinancialContractManagementMetadata, FinancialContractType, FinancialContractStatus,
//    onchain as financial_contract_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_vendor_management::{
//    FinancialVendorManagementMetadata, FinancialVendorType, FinancialVendorStatus,
//    onchain as financial_vendor_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_procurement::{
//    FinancialProcurementMetadata, FinancialProcurementType, FinancialProcurementStatus,
//    onchain as financial_procurement_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_supply_chain::{
//    FinancialSupplyChainMetadata, FinancialSupplyChainStage, FinancialSupplyChainStatus,
//    onchain as financial_supply_chain_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_inventory_management::{
//    FinancialInventoryManagementMetadata, FinancialInventoryType, FinancialInventoryStatus,
//    onchain as financial_inventory_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_quality_management::{
//    FinancialQualityManagementMetadata, FinancialQualityStandard, FinancialQualityStatus,
//    onchain as financial_quality_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_project_management::{
//    FinancialProjectManagementMetadata, FinancialProjectType, FinancialProjectStatus,
//    onchain as financial_project_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_resource_management::{
//    FinancialResourceManagementMetadata, FinancialResourceType, FinancialResourceStatus,
//    onchain as financial_resource_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_capacity_planning::{
//    FinancialCapacityPlanningMetadata, FinancialCapacityType, FinancialCapacityPlanningStatus,
//    onchain as financial_capacity_planning_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_workforce_planning::{
//    FinancialWorkforcePlanningMetadata, FinancialWorkforcePlanningApproach, FinancialWorkforcePlanningStatus,
//    onchain as financial_workforce_planning_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_technology_management::{
//    FinancialTechnologyManagementMetadata, FinancialTechnologyType, FinancialTechnologyStatus,
//    onchain as financial_technology_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_innovation_management::{
//    FinancialInnovationManagementMetadata, FinancialInnovationType, FinancialInnovationStatus,
//    onchain as financial_innovation_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_research_development::{
//    FinancialResearchDevelopmentMetadata, FinancialRDType, FinancialRDStatus,
//    onchain as financial_research_development_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_intellectual_property::{
//    FinancialIntellectualPropertyMetadata, FinancialIPType, FinancialIPStatus,
//    onchain as financial_intellectual_property_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_brand_management::{
//    FinancialBrandManagementMetadata, FinancialBrandAssetType, FinancialBrandStatus,
//    onchain as financial_brand_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_marketing_management::{
//    FinancialMarketingManagementMetadata, FinancialMarketingChannel, FinancialMarketingStatus,
//    onchain as financial_marketing_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_sales_management::{
//    FinancialSalesManagementMetadata, FinancialSalesChannel, FinancialSalesStatus,
//    onchain as financial_sales_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_customer_relationship::{
//    FinancialCustomerRelationshipMetadata, FinancialCustomerRelationshipType, FinancialCustomerRelationshipStatus,
//    onchain as financial_customer_relationship_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_stakeholder_management::{
//    FinancialStakeholderManagementMetadata, FinancialStakeholderType, FinancialStakeholderStatus,
//    onchain as financial_stakeholder_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_sustainability::{
//    FinancialSustainabilityMetadata, FinancialSustainabilityDimension, FinancialSustainabilityStatus,
//    onchain as financial_sustainability_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_corporate_social_responsibility::{
//    FinancialCorporateSocialResponsibilityMetadata, FinancialCSRInitiativeType, FinancialCSRStatus,
//    onchain as financial_corporate_social_responsibility_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_environmental_management::{
//    FinancialEnvironmentalManagementMetadata, FinancialEnvironmentalAspect, FinancialEnvironmentalStatus,
//    onchain as financial_environmental_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_energy_management::{
//    FinancialEnergyManagementMetadata, FinancialEnergySource, FinancialEnergyStatus,
//    onchain as financial_energy_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_waste_management::{
//    FinancialWasteManagementMetadata, FinancialWasteType, FinancialWasteStatus,
//    onchain as financial_waste_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_carbon_management::{
//    FinancialCarbonManagementMetadata, FinancialCarbonMetric, FinancialCarbonStatus,
//    onchain as financial_carbon_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_climate_risk::{
//    FinancialClimateRiskMetadata, FinancialClimateRiskType, FinancialClimateRiskStatus,
//    onchain as financial_climate_risk_onchain,
// };
// Financial ESG reporting now part of financial_reporting module
// Re-exports already included in financial_reporting section above
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_impact_measurement::{
//    FinancialImpactMeasurementMetadata, FinancialImpactType, FinancialImpactStatus,
//    onchain as financial_impact_measurement_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_impact_investing::{
//    FinancialImpactInvestingMetadata, FinancialImpactInvestmentStrategy, FinancialImpactInvestmentStatus,
//    onchain as financial_impact_investing_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_philanthropy::{
//    FinancialPhilanthropyMetadata, FinancialPhilanthropyType, FinancialPhilanthropyStatus,
//    onchain as financial_philanthropy_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_community_investment::{
//    FinancialCommunityInvestmentMetadata, FinancialCommunityInvestmentFocus, FinancialCommunityInvestmentStatus,
//    onchain as financial_community_investment_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_social_impact::{
//    FinancialSocialImpactMetadata, FinancialSocialImpactArea, FinancialSocialImpactStatus,
//    onchain as financial_social_impact_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_diversity_inclusion::{
//    FinancialDiversityInclusionMetadata, FinancialDIDimension, FinancialDIStatus,
//    onchain as financial_diversity_inclusion_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_ethics_compliance::{
//    FinancialEthicsComplianceMetadata, FinancialEthicsFramework, FinancialEthicsStatus,
//    onchain as financial_ethics_compliance_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_governance_risk_compliance::{
//    FinancialGovernanceRiskComplianceMetadata, FinancialGRCComponent, FinancialGRCStatus,
//    onchain as financial_governance_risk_compliance_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_enterprise_risk_management::{
//    FinancialEnterpriseRiskManagementMetadata, FinancialERMRiskCategory, FinancialERMStatus,
//    onchain as financial_enterprise_risk_management_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_operational_excellence::{
//    FinancialOperationalExcellenceMetadata, FinancialOperationalExcellenceFramework, FinancialOperationalExcellenceStatus,
//    onchain as financial_operational_excellence_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_digital_transformation::{
//    FinancialDigitalTransformationMetadata, FinancialDigitalTransformationArea, FinancialDigitalTransformationStatus,
//    onchain as financial_digital_transformation_onchain,
// };
// REMOVED FOR MVP: Move to off-chain service (financial_analysis/)
// pub use financial_agility::{
//    FinancialAgilityMetadata, FinancialAgilityDimension, FinancialAgilityStatus,
//    onchain as financial_agility_onchain,
// };
// DeFi modules are in defi/ directory, not state/

// Additional state modules will be added during migration:
// - treasury
// - governance
// - and others...
