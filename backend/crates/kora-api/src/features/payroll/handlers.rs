use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use crate::state::AppState;
use crate::features::payroll::{operations, dto::{WorkerSummary, PayrollEntrySummary}};
use kora_domain::finance::payroll::{Role, WorkerId};
use kora_kernel::ids::{AreaId, CycleId};
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct RegisterWorkerDto { pub name: String, pub role: Option<String> }
#[derive(Deserialize)]
pub struct RecordPayrollDto { pub worker_id: String, pub amount: String, pub currency: String, pub paid_at: i64, pub cycle_id: Option<String>, pub area_id: Option<String> }

pub async fn list_workers(State(state): State<Arc<AppState>>) -> Json<Vec<WorkerSummary>> { Json(operations::list_workers(&state)) }

pub async fn register_worker(State(state): State<Arc<AppState>>, Json(body): Json<RegisterWorkerDto>) -> Result<Json<WorkerSummary>, (StatusCode, String)> {
    let role = body.role.as_deref().map(|s| match s { "Operario" => Role::Operario, "Supervisor" => Role::Supervisor, "Tractorista" => Role::Tractorista, "Tecnico" | "Técnico" => Role::Tecnico, other => Role::Otro(other.into()) });
    let w = operations::register_worker(&state, body.name, role).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    Ok(Json(WorkerSummary { id: w.id().0.clone(), name: w.name().to_string(), role: w.role().map(|r| format!("{:?}", r)), active: w.is_active() }))
}

pub async fn record_payroll(State(state): State<Arc<AppState>>, Json(body): Json<RecordPayrollDto>) -> Result<Json<PayrollEntrySummary>, (StatusCode, String)> {
    let amount: Decimal = body.amount.parse().map_err(|_| (StatusCode::BAD_REQUEST, "bad amount".into()))?;
    let currency = match body.currency.as_str() { "USD" | "usd" => Currency::USD, "NIO" | "nio" => Currency::NIO, _ => return Err((StatusCode::BAD_REQUEST, "bad currency".into())) };
    let entry = operations::record_payroll(&state, WorkerId(body.worker_id), Money::new(amount, currency), body.paid_at, body.cycle_id.map(CycleId), body.area_id.map(AreaId)).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    Ok(Json(PayrollEntrySummary { id: entry.id().to_string(), worker_id: entry.worker_id().0.clone(), amount: format!("{} {:?}", entry.amount().amount, entry.amount().currency), paid_at: entry.paid_at(), cycle_id: entry.cycle_id().map(|c| c.0.clone()), area_id: entry.area_id().map(|a| a.0.clone()) }))
}

pub async fn list_payroll(State(state): State<Arc<AppState>>, Path(cycle_id): Path<String>) -> Json<Vec<PayrollEntrySummary>> {
    Json(operations::list_payroll(&state, &CycleId(cycle_id)))
}
