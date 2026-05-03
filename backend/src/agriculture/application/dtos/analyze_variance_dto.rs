// agriculture/application/dtos/analyze_variance_dto.rs
use crate::agriculture::domain::services::variance_service::VarianceConfig;
use crate::shared_kernel::ids::{CycleId, CropId, AreaId};
use crate::agriculture::domain::ids::ScheduleId;

/// Input DTO for analyze_variance use case
/// Contains ONLY data, NO dependencies
#[derive(Debug, Clone)]
pub struct AnalyzeVarianceInputDTO {
    pub cycle_id: CycleId,
    pub schedule_id: ScheduleId,
    pub config: VarianceConfig,
}

/// Output DTO for analyze_variance use case
#[derive(Debug, Clone)]
pub struct AnalyzeVarianceOutputDTO {
    pub timing_report: VarianceReportDTO,
    pub economic_report: Option<EconomicVarianceReportDTO>,
}

/// DTO for VarianceReport
#[derive(Debug, Clone)]
pub struct VarianceReportDTO {
    pub matched: Vec<MatchedActivityDTO>,
    pub unplanned: Vec<ActivityRecordDTO>,
    pub missing: Vec<PlannedActivityDTO>,
}

#[derive(Debug, Clone)]
pub struct MatchedActivityDTO {
    pub planned_id: String,
    pub activity_id: String,
    pub variance: TimingVarianceDTO,
    pub confidence: ConfidenceScoreDTO,
    pub cost_variance: Option<CostVarianceDTO>,
}

#[derive(Debug, Clone)]
pub enum TimingVarianceDTO {
    OnTime,
    Early(i64),
    Late(i64),
}

#[derive(Debug, Clone)]
pub enum ConfidenceScoreDTO {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct CostVarianceDTO {
    pub planned_cost: MoneyDTO,
    pub actual_cost: MoneyDTO,
    pub variance: MoneyDTO,
}

#[derive(Debug, Clone)]
pub struct MoneyDTO {
    pub amount: String,  // Using String for easy serialization
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct ActivityRecordDTO {
    pub activity_id: String,
    pub category: String,
    pub timestamp: i64,
    pub integrity: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PlannedActivityDTO {
    pub id: String,
    pub category: String,
    pub relative_day: i32,
    pub status: String,
}

/// Economic variance report DTO
#[derive(Debug, Clone)]
pub struct EconomicVarianceReportDTO {
    pub matched: Vec<MatchedActivityDTO>,
    pub total_planned: Option<MoneyDTO>,
    pub total_actual: Option<MoneyDTO>,
    pub total_variance: Option<MoneyDTO>,
}
