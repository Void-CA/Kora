use serde::Serialize;
use kora_kernel::ids::AreaId;

pub type CycleSummary = crate::features::cycles::dto::CycleSummary;

#[derive(Serialize)]
pub struct HistoryResponse {
    pub area_id: String, pub area_name: String,
    pub cycles: Vec<String>, pub schedules: Vec<String>,
    pub budgets: Vec<BudgetSummary>,
}
#[derive(Serialize)]
pub struct BudgetSummary {
    pub id: String, pub cycle_id: String,
    pub baseline: String, pub spent: String,
    pub remaining: String, pub variance: String,
}

#[derive(Serialize)]
pub struct AreaDashboard {
    pub area_id: String, pub area_name: String,
    pub cycles: Vec<DashboardCycleSummary>,
    pub soil_analyses: Vec<DashboardSoilSummary>,
    pub incidences: Vec<DashboardIncidenceSummary>,
    pub totals: AreaTotals,
}
#[derive(Serialize)]
pub struct DashboardCycleSummary {
    pub id: String, pub crop_id: String, pub period_start: i64, pub period_end: i64,
    pub activity_count: usize, pub budget_baseline: String, pub budget_spent: String, pub budget_variance: String,
}
#[derive(Serialize)]
pub struct DashboardSoilSummary {
    pub id: String, pub sampled_at: i64, pub quality: String, pub metric_count: usize, pub cost: String,
}
#[derive(Serialize)]
pub struct DashboardIncidenceSummary {
    pub id: String, pub cycle_id: String, pub kind: String, pub severity: String,
    pub description: String, pub detected_at: i64, pub economic_impact: Option<String>,
}
#[derive(Serialize)]
pub struct AreaTotals {
    pub total_baseline: String, pub total_spent: String, pub total_variance: String,
    pub cycle_count: usize, pub incidence_count: usize,
}
