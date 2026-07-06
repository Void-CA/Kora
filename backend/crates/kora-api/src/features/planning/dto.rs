use serde::Serialize;

#[derive(Serialize)]
pub struct BudgetSummary {
    pub id: String, pub cycle_id: String, pub baseline: String,
    pub lines: Vec<BudgetLineSummary>,
}
#[derive(Serialize)]
pub struct BudgetLineSummary { pub category: String, pub amount: String }
#[derive(Serialize)]
pub struct ScheduleSummary {
    pub id: String, pub cycle_id: String, pub anchor: String,
    pub anchor_date: i64, pub planned: Vec<PlannedActivitySummary>,
}
#[derive(Serialize)]
pub struct PlannedActivitySummary { pub id: String, pub category: String, pub relative_day: i32 }
