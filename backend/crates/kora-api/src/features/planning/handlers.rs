use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use crate::state::AppState;
use crate::features::planning::{operations, dto::{BudgetSummary, ScheduleSummary}};
use kora_kernel::ids::CycleId;
use kora_kernel::money::{Currency, Money};
use kora_kernel::period::Period;
use kora_domain::finance::budget::BudgetCategory;
use kora_domain::agriculture::activity::ActivityCategory;
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct CreateBudgetDto {
    pub cycle_id: String, pub period_start: i64, pub period_end: i64,
    pub baseline_amount: String, pub baseline_currency: String,
    pub estimated_lines: Vec<EstimatedLineDto>,
}
#[derive(Deserialize)]
pub struct EstimatedLineDto { pub category: String, pub amount: String, pub currency: String }
#[derive(Deserialize)]
pub struct AddActivityDto { pub cycle_id: String, pub category: String, pub relative_day: i32 }

pub async fn create_budget(State(state): State<Arc<AppState>>, Json(body): Json<CreateBudgetDto>) -> Result<Json<BudgetSummary>, (StatusCode, String)> {
    let baseline = parse_money(&body.baseline_amount, &body.baseline_currency)?;
    let period = Period::new(body.period_start, body.period_end).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let mut lines = Vec::new();
    for l in &body.estimated_lines {
        let amt = parse_money(&l.amount, &l.currency)?;
        let cat = match l.category.as_str() { "Seeds" | "seeds" => BudgetCategory::Seeds, "Fertilizers" | "fertilizers" => BudgetCategory::Fertilizers, "Labor" | "labor" => BudgetCategory::Labor, "SoilPrep" | "soil_prep" => BudgetCategory::SoilPrep, other => BudgetCategory::Other(other.into()) };
        lines.push((cat, amt));
    }
    operations::create_budget(&state, CycleId(body.cycle_id), period, baseline, lines).map(|b| Json(operations::budget_to_summary(&b))).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))
}

pub async fn add_activity(State(state): State<Arc<AppState>>, Json(body): Json<AddActivityDto>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let cat = match body.category.as_str() { "Sowing" | "sowing" => ActivityCategory::Sowing, "Maintenance" | "maintenance" => ActivityCategory::Maintenance, "SanitaryControl" | "sanitary_control" => ActivityCategory::SanitaryControl, "Harvest" | "harvest" => ActivityCategory::Harvest, _ => return Err((StatusCode::BAD_REQUEST, "bad category".into())) };
    operations::add_planned_activity(&state, &CycleId(body.cycle_id), cat, body.relative_day).ok_or((StatusCode::NOT_FOUND, "cycle not found".into())).map(|id| Json(serde_json::json!({"id": id.0})))
}

pub async fn get_schedule(State(state): State<Arc<AppState>>, Path(cycle_id): Path<String>) -> Result<Json<ScheduleSummary>, StatusCode> {
    operations::get_schedule(&state, &CycleId(cycle_id)).map(Json).ok_or(StatusCode::NOT_FOUND)
}

fn parse_money(amount: &str, currency: &str) -> Result<Money, (StatusCode, String)> {
    let a: Decimal = amount.parse().map_err(|_| (StatusCode::BAD_REQUEST, "bad amount".into()))?;
    let c = match currency { "USD" | "usd" => Currency::USD, "NIO" | "nio" => Currency::NIO, _ => return Err((StatusCode::BAD_REQUEST, "bad currency".into())) };
    Ok(Money::new(a, c))
}
