//! Partnership modules for the IndrasNet DAO Partnerships program

pub mod types;
pub mod manager;
pub mod process_automation;
pub mod metrics;
pub mod revenue_sharing;
pub mod governance_participation;
pub mod analytics;
pub mod collaboration;
pub mod contracts;
pub mod events;
pub mod integrations;
pub mod notifications;
pub mod reviews;
pub mod templates;
pub mod workflows;
pub mod dashboards;
pub mod reporting;
pub mod communication;
pub mod approvals;
pub mod billing;
pub mod compliance;
pub mod documents;
pub mod financials;
pub mod milestones;
pub mod permissions;
pub mod portals;
pub mod security;
pub mod settings;
pub mod onboarding;
pub mod termination;
pub mod analytics_advanced;
pub mod partnership_lifecycle;
pub mod partnership_health;
pub mod partnership_analytics_ml;
pub mod partnership_optimization;
pub mod partnership_engagement;
pub mod partnership_analytics_predictive;
pub mod partnership_analytics_real_time;
pub mod partnership_analytics_business_intelligence;
pub mod partnership_analytics_advanced;
pub mod partnership_analytics_insights;
pub mod partnership_analytics_forecasting;
pub mod partnership_analytics_benchmarking;
pub mod partnership_analytics_visualization;
pub mod partnership_analytics_reporting;
pub mod partnership_analytics_data_mining;
pub mod partnership_analytics_statistical_analysis;
pub mod partnership_analytics_trend_analysis;
pub mod partnership_analytics_correlation_analysis;
pub mod partnership_analytics_segmentation;
pub mod partnership_analytics_cohort_analysis;
pub mod partnership_analytics_funnel_analysis;
pub mod partnership_analytics_attribution;
pub mod partnership_analytics_retention;
pub mod partnership_analytics_churn;
pub mod partnership_analytics_lifetime_value;
pub mod partnership_analytics_conversion;
pub mod partnership_analytics_engagement;
pub mod partnership_analytics_performance;
pub mod partnership_analytics_competitive_analysis;
pub mod partnership_analytics_market_analysis;
pub mod partnership_analytics_customer_analysis;
pub mod partnership_analytics_roi;
pub mod partnership_analytics_kpi;
pub mod partnership_analytics_scorecard;
pub mod partnership_analytics_dashboard;
pub mod partnership_analytics_alerts;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use types::*;
pub use manager::*;
pub use process_automation::*;
pub use metrics::*;
pub use revenue_sharing::*;
pub use governance_participation::*;
pub use analytics::{
    AnalyticsReportMetadata, AnalyticsReportType,
    onchain as analytics_onchain,
};
pub use collaboration::{
    CollaborationMetadata, CollaborationStatus,
    onchain as collaboration_onchain,
};
pub use contracts::{
    PartnershipContractMetadata, ContractStatus,
    onchain as contracts_onchain,
};
pub use events::{
    PartnershipEventMetadata, PartnershipEventType,
    onchain as events_onchain,
};
pub use integrations::{
    PartnershipIntegrationMetadata, IntegrationStatus,
    onchain as integrations_onchain,
};
pub use notifications::{
    PartnershipNotificationMetadata, NotificationStatus, NotificationType,
    onchain as notifications_onchain,
};
pub use reviews::{
    PartnershipReviewMetadata, ReviewStatus,
    onchain as reviews_onchain,
};
pub use templates::{
    PartnershipTemplateMetadata, TemplateType,
    onchain as templates_onchain,
};
pub use workflows::{
    PartnershipWorkflowMetadata, WorkflowStatus,
    onchain as workflows_onchain,
};
pub use dashboards::{
    PartnershipDashboardMetadata, DashboardType,
    onchain as dashboards_onchain,
};
pub use reporting::{
    PartnershipReportMetadata, ReportStatus,
    onchain as reporting_onchain,
};
pub use communication::{
    PartnershipCommunicationMetadata, CommunicationType, CommunicationStatus,
    onchain as communication_onchain,
};
pub use approvals::{
    PartnershipApprovalMetadata, ApprovalStatus,
    onchain as approvals_onchain,
};
pub use billing::{
    PartnershipInvoiceMetadata, InvoiceStatus,
    onchain as billing_onchain,
};
pub use compliance::{
    PartnershipComplianceMetadata, ComplianceStatus,
    onchain as compliance_onchain,
};
pub use documents::{
    PartnershipDocumentMetadata, PartnershipDocumentType, DocumentStatus,
    onchain as documents_onchain,
};
pub use financials::{
    PartnershipFinancialTransactionMetadata, FinancialTransactionType, TransactionStatus,
    onchain as financials_onchain,
};
pub use milestones::{
    PartnershipMilestoneMetadata, MilestoneStatus,
    onchain as milestones_onchain,
};
pub use permissions::{
    PartnershipPermissionMetadata, PermissionType,
    onchain as permissions_onchain,
};
pub use portals::{
    PartnershipPortalMetadata, PortalStatus,
    onchain as portals_onchain,
};
pub use security::{
    PartnershipSecurityMetadata, SecurityLevel,
    onchain as security_onchain,
};
pub use settings::{
    PartnershipSettingsMetadata,
    onchain as settings_onchain,
};
pub use onboarding::{
    PartnershipOnboardingMetadata, OnboardingStatus,
    onchain as onboarding_onchain,
};
pub use termination::{
    PartnershipTerminationMetadata, TerminationStatus,
    onchain as termination_onchain,
};
pub use analytics_advanced::{
    AdvancedAnalyticsMetadata, AnalyticsModelType,
    onchain as analytics_advanced_onchain,
};
pub use partnership_lifecycle::{
    PartnershipLifecycleMetadata, LifecycleStage,
    onchain as partnership_lifecycle_onchain,
};
pub use partnership_health::{
    PartnershipHealthMetadata, HealthStatus,
    onchain as partnership_health_onchain,
};
pub use partnership_analytics_ml::{
    PartnershipMLAnalyticsMetadata, PartnershipMLModelType, MLAnalyticsStatus,
    onchain as partnership_analytics_ml_onchain,
};
pub use partnership_optimization::{
    PartnershipOptimizationMetadata, OptimizationType, OptimizationStatus,
    onchain as partnership_optimization_onchain,
};
pub use partnership_engagement::{
    PartnershipEngagementMetadata, EngagementLevel,
    onchain as partnership_engagement_onchain,
};
pub use partnership_analytics_predictive::{
    PredictivePartnershipAnalyticsMetadata, PredictionType, PredictiveAnalyticsStatus,
    onchain as partnership_analytics_predictive_onchain,
};
pub use partnership_analytics_real_time::{
    RealTimePartnershipAnalyticsMetadata, RealTimeAnalyticsStatus,
    onchain as partnership_analytics_real_time_onchain,
};
pub use partnership_analytics_business_intelligence::{
    BIPartnershipAnalyticsMetadata, BIDashboardType, BIAnalyticsStatus,
    onchain as partnership_analytics_business_intelligence_onchain,
};
pub use partnership_analytics_advanced::{
    AdvancedPartnershipAnalyticsMetadata, AdvancedAnalyticsType, AdvancedAnalyticsStatus,
    onchain as partnership_analytics_advanced_onchain,
};
pub use partnership_analytics_insights::{
    PartnershipAnalyticsInsightMetadata, InsightType, InsightStatus,
    onchain as partnership_analytics_insights_onchain,
};
pub use partnership_analytics_forecasting::{
    PartnershipAnalyticsForecastMetadata, ForecastType, ForecastStatus,
    onchain as partnership_analytics_forecasting_onchain,
};
pub use partnership_analytics_benchmarking::{
    PartnershipAnalyticsBenchmarkingMetadata, PartnershipBenchmarkType, PartnershipBenchmarkStatus,
    onchain as partnership_analytics_benchmarking_onchain,
};
pub use partnership_analytics_visualization::{
    PartnershipAnalyticsVisualizationMetadata, PartnershipVisualizationType, PartnershipVisualizationStatus,
    onchain as partnership_analytics_visualization_onchain,
};
pub use partnership_analytics_reporting::{
    PartnershipAnalyticsReportingMetadata, PartnershipReportFormat, PartnershipReportStatus,
    onchain as partnership_analytics_reporting_onchain,
};
pub use partnership_analytics_data_mining::{
    PartnershipAnalyticsDataMiningMetadata, PartnershipMiningTechnique, PartnershipMiningStatus,
    onchain as partnership_analytics_data_mining_onchain,
};
pub use partnership_analytics_statistical_analysis::{
    PartnershipAnalyticsStatisticalAnalysisMetadata, PartnershipStatisticalMethod, PartnershipStatisticalAnalysisStatus,
    onchain as partnership_analytics_statistical_analysis_onchain,
};
pub use partnership_analytics_trend_analysis::{
    PartnershipAnalyticsTrendAnalysisMetadata, PartnershipTrendType, PartnershipTrendAnalysisStatus,
    onchain as partnership_analytics_trend_analysis_onchain,
};
pub use partnership_analytics_correlation_analysis::{
    PartnershipAnalyticsCorrelationAnalysisMetadata, PartnershipCorrelationType, PartnershipCorrelationAnalysisStatus,
    onchain as partnership_analytics_correlation_analysis_onchain,
};
pub use partnership_analytics_segmentation::{
    PartnershipAnalyticsSegmentationMetadata, PartnershipSegmentationMethod, PartnershipSegmentationStatus,
    onchain as partnership_analytics_segmentation_onchain,
};
pub use partnership_analytics_cohort_analysis::{
    PartnershipAnalyticsCohortAnalysisMetadata, PartnershipCohortType, PartnershipCohortAnalysisStatus,
    onchain as partnership_analytics_cohort_analysis_onchain,
};
pub use partnership_analytics_funnel_analysis::{
    PartnershipAnalyticsFunnelAnalysisMetadata, PartnershipFunnelStage, PartnershipFunnelAnalysisStatus,
    onchain as partnership_analytics_funnel_analysis_onchain,
};
pub use partnership_analytics_attribution::{
    PartnershipAnalyticsAttributionMetadata, PartnershipAttributionModel, PartnershipAttributionStatus,
    onchain as partnership_analytics_attribution_onchain,
};
pub use partnership_analytics_retention::{
    PartnershipAnalyticsRetentionMetadata, PartnershipRetentionMetric, PartnershipRetentionStatus,
    onchain as partnership_analytics_retention_onchain,
};
pub use partnership_analytics_churn::{
    PartnershipAnalyticsChurnMetadata, PartnershipChurnType, PartnershipChurnStatus,
    onchain as partnership_analytics_churn_onchain,
};
pub use partnership_analytics_lifetime_value::{
    PartnershipAnalyticsLifetimeValueMetadata, PartnershipLifetimeValueType, PartnershipLifetimeValueStatus,
    onchain as partnership_analytics_lifetime_value_onchain,
};
pub use partnership_analytics_conversion::{
    PartnershipAnalyticsConversionMetadata, PartnershipConversionType, PartnershipConversionStatus,
    onchain as partnership_analytics_conversion_onchain,
};
pub use partnership_analytics_engagement::{
    PartnershipAnalyticsEngagementMetadata, PartnershipEngagementMetric, PartnershipEngagementStatus,
    onchain as partnership_analytics_engagement_onchain,
};
pub use partnership_analytics_performance::{
    PartnershipAnalyticsPerformanceMetadata, PartnershipPerformanceMetric, PartnershipPerformanceStatus,
    onchain as partnership_analytics_performance_onchain,
};
pub use partnership_analytics_competitive_analysis::{
    PartnershipAnalyticsCompetitiveAnalysisMetadata, PartnershipCompetitiveAnalysisScope, PartnershipCompetitiveAnalysisStatus,
    onchain as partnership_analytics_competitive_analysis_onchain,
};
pub use partnership_analytics_market_analysis::{
    PartnershipAnalyticsMarketAnalysisMetadata, PartnershipMarketSegment, PartnershipMarketAnalysisStatus,
    onchain as partnership_analytics_market_analysis_onchain,
};
pub use partnership_analytics_customer_analysis::{
    PartnershipAnalyticsCustomerAnalysisMetadata, PartnershipCustomerSegment, PartnershipCustomerAnalysisStatus,
    onchain as partnership_analytics_customer_analysis_onchain,
};
pub use partnership_analytics_roi::{
    PartnershipAnalyticsROIMetadata, PartnershipROIType, PartnershipROIStatus,
    onchain as partnership_analytics_roi_onchain,
};
pub use partnership_analytics_kpi::{
    PartnershipAnalyticsKPIMetadata, PartnershipKPICategory, PartnershipKPIStatus,
    onchain as partnership_analytics_kpi_onchain,
};
pub use partnership_analytics_scorecard::{
    PartnershipAnalyticsScorecardMetadata, PartnershipScorecardType, PartnershipScorecardStatus,
    onchain as partnership_analytics_scorecard_onchain,
};
pub use partnership_analytics_dashboard::{
    PartnershipAnalyticsDashboardMetadata, PartnershipDashboardType, PartnershipDashboardStatus,
    onchain as partnership_analytics_dashboard_onchain,
};
pub use partnership_analytics_alerts::{
    PartnershipAnalyticsAlertsMetadata, PartnershipAlertType, PartnershipAlertStatus,
    onchain as partnership_analytics_alerts_onchain,
};
