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
use kora_domain::agriculture::ids::PlannedActivityId;

use crate::state::AppState;
use crate::features::finance::dto::Profitability;
use crate::features::finance::profitability;
use crate::use_cases::register_activity::RegistrationMode;

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
    profitability::execute(&state, &cycle_id)
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn timeline(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::use_cases::get_cycle_timeline::CycleTimeline>, StatusCode> {
    let cycle_id = CycleId(id);
    crate::use_cases::get_cycle_timeline::execute(&state, &cycle_id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn variance(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::use_cases::get_cycle_variance::CycleVariance>, StatusCode> {
    let cycle_id = CycleId(id);
    crate::use_cases::get_cycle_variance::execute(&state, &cycle_id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
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

#[derive(serde::Deserialize)]
pub struct RegisterActivityDto {
    pub cycle_id: String,
    pub timestamp: i64,
    pub category: String,
    pub notes: Option<String>,
    pub mode: String,
    pub match_against: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RegisterActivityResponse {
    pub activity_id: String,
    pub category: String,
    pub timestamp: i64,
    pub integrity: Vec<String>,
    pub suggestions: Vec<crate::use_cases::register_activity::PlannedSuggestion>,
}

pub async fn register_activity(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterActivityDto>,
) -> Result<Json<RegisterActivityResponse>, (StatusCode, String)> {
    use crate::use_cases::register_activity::{self as register_activity_uc, RegisterActivityInput};
    let category = parse_category(&body.category).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let mode = parse_mode(&body.mode, body.match_against.as_deref())
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let input = RegisterActivityInput {
        cycle_id: CycleId(body.cycle_id),
        timestamp: body.timestamp,
        category,
        notes: body.notes,
        mode,
    };
    let reg = register_activity_uc::execute(&state, input)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(RegisterActivityResponse {
        activity_id: reg.record.activity.id().0.clone(),
        category: format!("{:?}", reg.record.activity.category()),
        timestamp: reg.record.activity.timestamp(),
        integrity: reg
            .record
            .integrity
            .iter()
            .map(|i| match i {
                kora_domain::agriculture::activity::IntegrityStatus::Valid => "valid".to_string(),
                kora_domain::agriculture::activity::IntegrityStatus::OutsidePeriod => "outside_period".to_string(),
                kora_domain::agriculture::activity::IntegrityStatus::Unplanned => "unplanned".to_string(),
                kora_domain::agriculture::activity::IntegrityStatus::MatchedPlanned(pid) => {
                    format!("matched_planned:{}", pid.0)
                }
            })
            .collect(),
        suggestions: reg.suggestions,
    }))
}

fn parse_category(s: &str) -> Result<kora_domain::agriculture::activity::ActivityCategory, String> {
    use kora_domain::agriculture::activity::ActivityCategory;
    match s {
        "Sowing" | "sowing" => Ok(ActivityCategory::Sowing),
        "Maintenance" | "maintenance" => Ok(ActivityCategory::Maintenance),
        "SanitaryControl" | "sanitary_control" => Ok(ActivityCategory::SanitaryControl),
        "Harvest" | "harvest" => Ok(ActivityCategory::Harvest),
        _ => Err(format!("unknown category: {s}")),
    }
}

fn parse_mode(
    s: &str,
    match_against: Option<&str>,
) -> Result<RegistrationMode, String> {
    match s {
        "suggested" | "Suggested" => Ok(RegistrationMode::Suggested),
        "emergent" | "Emergent" => Ok(RegistrationMode::Emergent),
        "confirm_match" | "ConfirmMatch" => {
            let pid = match_against.ok_or("match_against required for confirm_match")?;
            Ok(RegistrationMode::ConfirmMatch(PlannedActivityId(pid.to_string())))
        }
        _ => Err(format!("unknown mode: {s}")),
    }
}
