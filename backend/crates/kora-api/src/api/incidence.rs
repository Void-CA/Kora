use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use kora_domain::agriculture::incidence::{IncidenceType, SanitaryIncidence, Severity};
use kora_kernel::ids::CycleId;
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

use crate::state::AppState;
use crate::use_cases::incidence as incidence_uc;

#[derive(Deserialize)]
pub struct RegisterIncidenceDto {
    pub cycle_id: String,
    pub kind: String,
    pub severity: String,
    pub description: String,
    pub action_taken: String,
    pub detected_at: i64,
    pub economic_impact_amount: Option<String>,
    pub economic_impact_currency: Option<String>,
}

#[derive(Serialize)]
pub struct IncidenceSummary {
    pub id: String,
    pub cycle_id: String,
    pub kind: String,
    pub severity: String,
    pub description: String,
    pub action_taken: String,
    pub detected_at: i64,
    pub resolved: bool,
    pub economic_impact: Option<String>,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterIncidenceDto>,
) -> Result<Json<IncidenceSummary>, (StatusCode, String)> {
    let kind = parse_kind(&body.kind).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let severity = parse_severity(&body.severity).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let economic_impact = match (body.economic_impact_amount, body.economic_impact_currency) {
        (Some(amount), Some(currency)) => {
            let amt: Decimal = amount.parse()
                .map_err(|_| (StatusCode::BAD_REQUEST, format!("invalid impact amount: {amount}")))?;
            let cur = parse_currency(&currency).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
            Some(Money::new(amt, cur))
        }
        _ => None,
    };
    let input = incidence_uc::RegisterIncidenceInput {
        cycle_id: CycleId(body.cycle_id),
        kind,
        severity,
        description: body.description,
        action_taken: body.action_taken,
        detected_at: body.detected_at,
        economic_impact,
    };
    let incidence = incidence_uc::execute(&state, input)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(incidence_to_summary(&incidence)))
}

pub async fn list_for_cycle(
    State(state): State<Arc<AppState>>,
    Path(cycle_id): Path<String>,
) -> Json<Vec<IncidenceSummary>> {
    let incidences = incidence_uc::for_cycle(&state, &CycleId(cycle_id));
    Json(incidences.iter().map(incidence_to_summary).collect())
}

fn incidence_to_summary(i: &SanitaryIncidence) -> IncidenceSummary {
    IncidenceSummary {
        id: i.id().0.clone(),
        cycle_id: i.cycle_id().0.clone(),
        kind: format!("{:?}", i.kind()),
        severity: format!("{:?}", i.severity()),
        description: i.description().to_string(),
        action_taken: i.action_taken().to_string(),
        detected_at: i.detected_at(),
        resolved: i.is_resolved(),
        economic_impact: i.economic_impact().map(|m| format!("{} {:?}", m.amount, m.currency)),
    }
}

fn parse_kind(s: &str) -> Result<IncidenceType, String> {
    match s {
        "Pest" | "pest" => Ok(IncidenceType::Pest),
        "Disease" | "disease" => Ok(IncidenceType::Disease),
        other => Ok(IncidenceType::Otro(other.to_string())),
    }
}

fn parse_severity(s: &str) -> Result<Severity, String> {
    match s {
        "Low" | "low" => Ok(Severity::Low),
        "Medium" | "medium" => Ok(Severity::Medium),
        "High" | "high" => Ok(Severity::High),
        "Critical" | "critical" => Ok(Severity::Critical),
        _ => Err(format!("unknown severity: {s}")),
    }
}

fn parse_currency(s: &str) -> Result<Currency, String> {
    match s {
        "USD" | "usd" => Ok(Currency::USD),
        "NIO" | "nio" => Ok(Currency::NIO),
        _ => Err(format!("unknown currency: {s}")),
    }
}
