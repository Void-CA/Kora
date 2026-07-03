use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use kora_domain::finance::payroll::{PayrollEntry, Role, Worker, WorkerId};
use kora_domain::ports::payroll_entry_repository::PayrollEntryRepository;
use kora_kernel::ids::{AreaId, CycleId};
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

use crate::state::AppState;
use crate::use_cases::payroll as payroll_uc;

#[derive(Deserialize)]
pub struct RegisterWorkerDto {
    pub name: String,
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct WorkerSummary {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    pub active: bool,
}

#[derive(Deserialize)]
pub struct RecordPayrollDto {
    pub worker_id: String,
    pub amount: String,
    pub currency: String,
    pub paid_at: i64,
    pub cycle_id: Option<String>,
    pub area_id: Option<String>,
}

#[derive(Serialize)]
pub struct PayrollEntrySummary {
    pub id: String,
    pub worker_id: String,
    pub amount: String,
    pub paid_at: i64,
    pub cycle_id: Option<String>,
    pub area_id: Option<String>,
}

pub async fn register_worker(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterWorkerDto>,
) -> Result<Json<WorkerSummary>, (StatusCode, String)> {
    let role = body.role.as_deref().map(parse_role).transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let worker = payroll_uc::register_worker(
        &state,
        payroll_uc::RegisterWorkerInput { name: body.name, role },
    ).map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(worker_to_summary(&worker)))
}

pub async fn list_workers(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<WorkerSummary>> {
    let workers = state.worker_repo.lock().unwrap().all();
    Json(workers.iter().map(worker_to_summary).collect())
}

pub async fn record_payroll(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RecordPayrollDto>,
) -> Result<Json<PayrollEntrySummary>, (StatusCode, String)> {
    let amount: Decimal = body.amount.parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("invalid amount: {}", body.amount)))?;
    let currency = parse_currency(&body.currency).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let cycle_id = body.cycle_id.map(CycleId);
    let area_id = body.area_id.map(AreaId);
    let input = payroll_uc::RecordPayrollInput {
        worker_id: WorkerId(body.worker_id),
        amount: Money::new(amount, currency),
        paid_at: body.paid_at,
        cycle_id,
        area_id,
        role_at_payment: None,
    };
    let entry = payroll_uc::record_payroll(&state, input)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(payroll_to_summary(&entry)))
}

pub async fn list_for_cycle(
    State(state): State<Arc<AppState>>,
    Path(cycle_id): Path<String>,
) -> Json<Vec<PayrollEntrySummary>> {
    let entries = state.payroll_repo.lock().unwrap().for_cycle(&CycleId(cycle_id));
    Json(entries.iter().map(payroll_to_summary).collect())
}

fn worker_to_summary(w: &Worker) -> WorkerSummary {
    WorkerSummary {
        id: w.id().0.clone(),
        name: w.name().to_string(),
        role: w.role().map(|r| format!("{r:?}")),
        active: w.is_active(),
    }
}

fn payroll_to_summary(e: &PayrollEntry) -> PayrollEntrySummary {
    PayrollEntrySummary {
        id: e.id().to_string(),
        worker_id: e.worker_id().0.clone(),
        amount: format!("{} {:?}", e.amount().amount, e.amount().currency),
        paid_at: e.paid_at(),
        cycle_id: e.cycle_id().map(|c| c.0.clone()),
        area_id: e.area_id().map(|a| a.0.clone()),
    }
}

fn parse_role(s: &str) -> Result<Role, String> {
    match s {
        "Operario" => Ok(Role::Operario),
        "Supervisor" => Ok(Role::Supervisor),
        "Tractorista" => Ok(Role::Tractorista),
        "Tecnico" | "Técnico" => Ok(Role::Tecnico),
        other => Ok(Role::Otro(other.to_string())),
    }
}

fn parse_currency(s: &str) -> Result<Currency, String> {
    match s {
        "USD" | "usd" => Ok(Currency::USD),
        "NIO" | "nio" => Ok(Currency::NIO),
        _ => Err(format!("unknown currency: {s}")),
    }
}
