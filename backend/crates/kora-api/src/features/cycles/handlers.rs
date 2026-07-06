use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use kora_kernel::ids::CycleId;
use crate::state::AppState;
use crate::features::cycles::{operations, dto::*};

pub async fn list(State(state): State<Arc<AppState>>) -> Json<Vec<CycleSummary>> {
    Json(operations::list_cycles(&state))
}

pub async fn get_one(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Result<Json<CycleDetail>, StatusCode> {
    operations::get_cycle(&state, &CycleId(id)).map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn timeline(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Result<Json<CycleTimeline>, StatusCode> {
    operations::get_timeline(&state, &CycleId(id)).map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn variance(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Result<Json<CycleVariance>, StatusCode> {
    operations::get_variance(&state, &CycleId(id)).map(Json).ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
pub struct RegisterActivityDto {
    pub cycle_id: String, pub timestamp: i64, pub category: String,
    pub notes: Option<String>, pub mode: String, pub match_against: Option<String>,
}

pub async fn register_activity(
    State(state): State<Arc<AppState>>, Json(body): Json<RegisterActivityDto>,
) -> Result<Json<RegisterActivityResponse>, (StatusCode, String)> {
    use kora_domain::agriculture::activity::ActivityCategory;
    let cat = match body.category.as_str() { "Sowing" | "sowing" => ActivityCategory::Sowing, "Maintenance" | "maintenance" => ActivityCategory::Maintenance, "SanitaryControl" | "sanitary_control" => ActivityCategory::SanitaryControl, "Harvest" | "harvest" => ActivityCategory::Harvest, _ => return Err((StatusCode::BAD_REQUEST, "bad category".into())) };
    let mode = match body.mode.as_str() {
        "suggested" | "Suggested" => operations::RegistrationMode::Suggested,
        "emergent" | "Emergent" => operations::RegistrationMode::Emergent,
        "confirm_match" | "ConfirmMatch" => operations::RegistrationMode::ConfirmMatch(kora_domain::agriculture::ids::PlannedActivityId(body.match_against.ok_or((StatusCode::BAD_REQUEST, "match_against required".into()))?)),
        _ => return Err((StatusCode::BAD_REQUEST, "bad mode".into())),
    };
    let reg = operations::register_activity(&state, operations::RegisterActivityInput { cycle_id: CycleId(body.cycle_id), timestamp: body.timestamp, category: cat, notes: body.notes, mode }).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    Ok(Json(RegisterActivityResponse {
        activity_id: reg.record.activity.id().0.clone(), category: format!("{:?}", reg.record.activity.category()),
        timestamp: reg.record.activity.timestamp(),
        integrity: reg.record.integrity.iter().map(|i| format!("{:?}", i)).collect(),
        suggestions: reg.suggestions,
    }))
}
