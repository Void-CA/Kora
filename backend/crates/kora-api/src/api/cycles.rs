use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;

use kora_kernel::ids::CycleId;
use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::activity::ActivityRecord;
use kora_domain::agriculture::planning::PlannedActivity;

use crate::state::AppState;
use crate::use_cases::get_profitability::{Profitability, execute as run_profitability};

#[derive(Serialize)]
pub struct CycleSummary {
    pub id: String,
    pub crop_id: String,
    pub area_id: String,
    pub period_start: i64,
    pub period_end: i64,
    pub activity_count: usize,
}

#[derive(Serialize)]
pub struct CycleDetail {
    pub summary: CycleSummary,
    pub activities: Vec<ActivitySummary>,
    pub planned_activities: Vec<PlannedSummary>,
}

#[derive(Serialize)]
pub struct ActivitySummary {
    pub id: String,
    pub category: String,
    pub timestamp: i64,
    pub integrity: Vec<String>,
}

#[derive(Serialize)]
pub struct PlannedSummary {
    pub id: String,
    pub category: String,
    pub relative_day: i32,
}

pub async fn list(State(state): State<Arc<AppState>>) -> Json<Vec<CycleSummary>> {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let summaries: Vec<CycleSummary> = cycles.iter().map(cycle_to_summary).collect();
    Json(summaries)
}

pub async fn get_one(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<CycleDetail>, StatusCode> {
    let cycle_id = CycleId(id);
    let cycles = state.cycle_repo.lock().unwrap().all();
    let cycle = cycles.iter().find(|c| c.id() == &cycle_id).ok_or(StatusCode::NOT_FOUND)?;
    let schedules = state.schedule_repo.lock().unwrap().all();
    let schedule = schedules.iter().find(|s| s.cycle_id() == &cycle_id);
    let activities: Vec<ActivitySummary> = cycle.executed_activities().iter().map(activity_to_summary).collect();
    let planned_activities: Vec<PlannedSummary> = schedule
        .map(|s| s.activities().iter().map(planned_to_summary).collect())
        .unwrap_or_default();
    Ok(Json(CycleDetail {
        summary: cycle_to_summary(cycle),
        activities,
        planned_activities,
    }))
}

pub async fn profitability(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Profitability>, StatusCode> {
    let cycle_id = CycleId(id);
    run_profitability(&state, &cycle_id)
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

fn cycle_to_summary(c: &CropCycle) -> CycleSummary {
    CycleSummary {
        id: c.id().0.clone(),
        crop_id: c.crop_id().0.clone(),
        area_id: c.area_id().0.clone(),
        period_start: c.period().start(),
        period_end: c.period().end(),
        activity_count: c.executed_activities().len(),
    }
}

fn activity_to_summary(r: &ActivityRecord) -> ActivitySummary {
    ActivitySummary {
        id: r.activity.id().0.clone(),
        category: format!("{:?}", r.activity.category()),
        timestamp: r.activity.timestamp(),
        integrity: r.integrity.iter().map(|i| format!("{:?}", i)).collect(),
    }
}

fn planned_to_summary(p: &PlannedActivity) -> PlannedSummary {
    PlannedSummary {
        id: p.id.0.clone(),
        category: format!("{:?}", p.category),
        relative_day: p.relative_day,
    }
}
