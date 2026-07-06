use serde::Serialize;

#[derive(Serialize)]
pub struct CycleSummary {
    pub id: String, pub crop_id: String, pub area_id: String,
    pub period_start: i64, pub period_end: i64, pub activity_count: usize,
}

#[derive(Serialize)]
pub struct CycleDetail {
    pub summary: CycleSummary,
    pub activities: Vec<ActivitySummary>,
    pub planned_activities: Vec<PlannedSummary>,
}

#[derive(Serialize)]
pub struct ActivitySummary {
    pub id: String, pub category: String, pub timestamp: i64, pub integrity: Vec<String>,
}

#[derive(Serialize)]
pub struct PlannedSummary {
    pub id: String, pub category: String, pub relative_day: i32,
}

#[derive(Serialize)]
pub struct TimelineEvent {
    pub kind: String, pub timestamp: i64, pub label: String,
    pub detail: String, pub integrity: Vec<String>,
}

#[derive(Serialize)]
pub struct CycleTimeline {
    pub cycle_id: String, pub crop_id: String, pub area_id: String,
    pub period_start: i64, pub period_end: i64,
    pub planned: Vec<TimelineEvent>, pub executed: Vec<TimelineEvent>,
    pub expenses: Vec<TimelineEvent>, pub revenues: Vec<TimelineEvent>,
    pub payroll: Vec<TimelineEvent>, pub incidences: Vec<TimelineEvent>,
}

#[derive(Serialize)]
pub struct TimingVariance {
    pub kind: String, pub days: f64,
}

#[derive(Serialize)]
pub struct MatchedActivity {
    pub planned_id: String, pub activity_id: String, pub category: String,
    pub expected_timestamp: i64, pub actual_timestamp: i64,
    pub variance: TimingVariance, pub confidence: String, pub cost: Option<CostVariance>,
}

#[derive(Serialize)]
pub struct CostVariance { pub planned: String, pub actual: String, pub variance: String }

#[derive(Serialize)]
pub struct VarianceTotals {
    pub matched_count: usize, pub unplanned_count: usize, pub missing_count: usize,
    pub total_planned: Option<String>, pub total_actual: Option<String>,
    pub total_cost_variance: Option<String>,
}

#[derive(Serialize)]
pub struct CycleVariance {
    pub cycle_id: String,
    pub matched: Vec<MatchedActivity>, pub unplanned: Vec<UnplannedActivity>,
    pub missing: Vec<MissingPlanned>, pub totals: VarianceTotals,
}

#[derive(Serialize)]
pub struct UnplannedActivity { pub activity_id: String, pub category: String, pub timestamp: i64, pub reason: String }

#[derive(Serialize)]
pub struct MissingPlanned { pub planned_id: String, pub category: String, pub relative_day: i32, pub expected_timestamp: i64 }

#[derive(Serialize)]
pub struct PlannedSuggestion {
    pub planned_id: String, pub category: String, pub relative_day: i32,
    pub expected_timestamp: i64, pub drift_days: i64,
}

#[derive(Serialize)]
pub struct RegisterActivityResponse {
    pub activity_id: String, pub category: String, pub timestamp: i64,
    pub integrity: Vec<String>, pub suggestions: Vec<PlannedSuggestion>,
}
