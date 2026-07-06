use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use crate::state::AppState;
use crate::features::incidences::{operations, dto::IncidenceSummary};
use kora_domain::agriculture::incidence::{IncidenceType, Severity};
use kora_kernel::ids::CycleId;
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct RegisterIncidenceDto {
    pub cycle_id: String, pub kind: String, pub severity: String,
    pub description: String, pub action_taken: String, pub detected_at: i64,
    pub economic_impact_amount: Option<String>, pub economic_impact_currency: Option<String>,
}

pub async fn list_for_cycle(State(state): State<Arc<AppState>>, Path(cycle_id): Path<String>) -> Json<Vec<IncidenceSummary>> {
    Json(operations::list_for_cycle(&state, &CycleId(cycle_id)))
}

pub async fn register(State(state): State<Arc<AppState>>, Json(body): Json<RegisterIncidenceDto>) -> Result<Json<IncidenceSummary>, (StatusCode, String)> {
    let kind = match body.kind.as_str() { "Pest" | "pest" => IncidenceType::Pest, "Disease" | "disease" => IncidenceType::Disease, other => IncidenceType::Otro(other.into()) };
    let severity = match body.severity.as_str() { "Low" | "low" => Severity::Low, "Medium" | "medium" => Severity::Medium, "High" | "high" => Severity::High, "Critical" | "critical" => Severity::Critical, _ => return Err((StatusCode::BAD_REQUEST, "bad severity".into())) };
    let impact = match (body.economic_impact_amount, body.economic_impact_currency) {
        (Some(amt), Some(cur)) => { let a: Decimal = amt.parse().map_err(|_| (StatusCode::BAD_REQUEST, "bad impact amount".into()))?; let c = match cur.as_str() { "USD" | "usd" => Currency::USD, "NIO" | "nio" => Currency::NIO, _ => return Err((StatusCode::BAD_REQUEST, "bad currency".into())) }; Some(Money::new(a, c)) }
        _ => None,
    };
    let inc = operations::register(&state, CycleId(body.cycle_id), kind, severity, body.description, body.action_taken, body.detected_at, impact).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    Ok(Json(IncidenceSummary { id: inc.id().0.clone(), cycle_id: inc.cycle_id().0.clone(), kind: format!("{:?}", inc.kind()), severity: format!("{:?}", inc.severity()), description: inc.description().to_string(), action_taken: inc.action_taken().to_string(), detected_at: inc.detected_at(), resolved: inc.is_resolved(), economic_impact: inc.economic_impact().map(|m| format!("{} {:?}", m.amount, m.currency)) }))
}
