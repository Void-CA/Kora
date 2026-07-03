use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Ok,
    Attention,
    Critical,
    Info,
}

#[derive(Serialize)]
pub struct FieldHealth {
    pub status: HealthStatus,
    pub label: String,
    pub value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PhaseStatus {
    Done,
    Current,
    Pending,
}

#[derive(Serialize)]
pub struct CyclePhase {
    pub name: String,
    pub status: PhaseStatus,
    pub day_in_phase: Option<u32>,
    pub expected_duration_days: Option<u32>,
}

#[derive(Serialize)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub hectares: f64,
    pub crop: String,
    pub cycle_id: String,
    pub growth: String,
    pub last_activity: String,
    pub days_to_harvest: i64,
    pub health: Vec<FieldHealth>,
    pub phases: Vec<CyclePhase>,
}

pub async fn list(State(state): State<Arc<AppState>>) -> Json<Vec<Field>> {
    let fields = build_fields(&state);
    Json(fields)
}

pub async fn get_one(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Field>, StatusCode> {
    let fields = build_fields(&state);
    fields.into_iter().find(|f| f.id == id).map(Json).ok_or(StatusCode::NOT_FOUND)
}

fn build_fields(state: &AppState) -> Vec<Field> {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let budgets = state.budget_repo.lock().unwrap().all();

    state
        .farms
        .iter()
        .flat_map(|f| f.areas())
        .filter_map(|area| {
            let cycle = cycles.iter().find(|c| c.area_id() == area.id())?;
            let budget = budgets.iter().find(|b| b.cycle_id() == cycle.id());
            let over_budget = budget
                .and_then(|b| b.get_variance().ok())
                .map(|v| v.amount.is_sign_positive())
                .unwrap_or(false);

            let last_activity = cycle
                .executed_activities()
                .iter()
                .max_by_key(|r| r.activity.timestamp())
                .map(|r| format!("timestamp {}", r.activity.timestamp()))
                .unwrap_or_else(|| "sin actividad".to_string());

            let health = vec![
                FieldHealth {
                    status: if over_budget { HealthStatus::Critical } else { HealthStatus::Ok },
                    label: "Presupuesto".to_string(),
                    value: if over_budget { "Sobregirado".to_string() } else { "En rango".to_string() },
                },
                FieldHealth {
                    status: if cycle.executed_activities().is_empty() { HealthStatus::Attention } else { HealthStatus::Ok },
                    label: "Actividades".to_string(),
                    value: format!("{} registradas", cycle.executed_activities().len()),
                },
            ];

            Some(Field {
                id: area.id().0.clone(),
                name: area.name().to_string(),
                hectares: area.measurement().value_in_hectares(),
                crop: "Cultivo".to_string(),
                cycle_id: cycle.id().0.clone(),
                growth: "Crecimiento".to_string(),
                last_activity,
                days_to_harvest: 23,
                health,
                phases: vec![
                    CyclePhase { name: "Preparación".to_string(), status: PhaseStatus::Done, day_in_phase: None, expected_duration_days: Some(20) },
                    CyclePhase { name: "Siembra".to_string(), status: PhaseStatus::Done, day_in_phase: None, expected_duration_days: Some(5) },
                    CyclePhase { name: "Crecimiento".to_string(), status: PhaseStatus::Current, day_in_phase: Some(12), expected_duration_days: Some(45) },
                    CyclePhase { name: "Floración".to_string(), status: PhaseStatus::Pending, day_in_phase: None, expected_duration_days: Some(20) },
                    CyclePhase { name: "Cosecha".to_string(), status: PhaseStatus::Pending, day_in_phase: None, expected_duration_days: Some(15) },
                ],
            })
        })
        .collect()
}
