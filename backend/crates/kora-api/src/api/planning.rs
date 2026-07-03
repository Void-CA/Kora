use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use kora_domain::agriculture::activity::ActivityCategory;
use kora_domain::agriculture::soil::{LinkKind, SoilAnalysisId, SoilAnalysisLink};
use kora_domain::finance::budget::{Budget, BudgetCategory};
use kora_kernel::ids::CycleId;
use kora_kernel::money::{Currency, Money};
use kora_kernel::period::Period;
use rust_decimal::Decimal;

use crate::state::AppState;
use crate::use_cases;

#[derive(Deserialize)]
pub struct CreateBudgetDto {
    pub cycle_id: String,
    pub period_start: i64,
    pub period_end: i64,
    pub baseline_amount: String,
    pub baseline_currency: String,
    pub estimated_lines: Vec<EstimatedLineDto>,
}

#[derive(Deserialize)]
pub struct EstimatedLineDto {
    pub category: String,
    pub amount: String,
    pub currency: String,
}

#[derive(Serialize)]
pub struct BudgetSummary {
    pub id: String,
    pub cycle_id: String,
    pub baseline: String,
    pub lines: Vec<BudgetLineSummary>,
}

#[derive(Serialize)]
pub struct BudgetLineSummary {
    pub category: String,
    pub amount: String,
}

pub async fn create_budget(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateBudgetDto>,
) -> Result<Json<BudgetSummary>, (StatusCode, String)> {
    let baseline = parse_money(&body.baseline_amount, &body.baseline_currency)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let period = Period::new(body.period_start, body.period_end)
        .map_err(|e: &'static str| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let mut lines = Vec::with_capacity(body.estimated_lines.len());
    for line in &body.estimated_lines {
        let amount = parse_money(&line.amount, &line.currency)
            .map_err(|e: String| (StatusCode::BAD_REQUEST, e))?;
        let cat = parse_budget_category(&line.category)
            .map_err(|e: String| (StatusCode::BAD_REQUEST, e))?;
        lines.push((cat, amount));
    }
    let input = uses_cases_create_budget::CreateBudgetInput {
        cycle_id: CycleId(body.cycle_id),
        period,
        baseline,
        estimated_lines: lines,
    };
    let budget = uses_cases_create_budget::execute(&state, input)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(budget_to_summary(&budget)))
}

mod uses_cases_create_budget {
    pub use crate::use_cases::create_budget::*;
}

#[derive(Deserialize)]
pub struct AddPlannedActivityDto {
    pub cycle_id: String,
    pub category: String,
    pub relative_day: i32,
}

pub async fn add_planned_activity(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AddPlannedActivityDto>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let category = parse_activity_category(&body.category)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let id = uses_cases_add_planned::execute(
        &state,
        &CycleId(body.cycle_id),
        category,
        body.relative_day,
    )
    .ok_or((StatusCode::NOT_FOUND, "schedule not found".to_string()))?;
    Ok(Json(serde_json::json!({ "id": id.0 })))
}

mod uses_cases_add_planned {
    pub use crate::use_cases::add_planned_activity::*;
}

#[derive(Deserialize)]
pub struct LinkSoilDto {
    pub analysis_id: String,
    pub cycle_id: String,
    pub kind: String,
}

#[derive(Serialize)]
pub struct LinkSoilResponse {
    pub analysis_id: String,
    pub cycle_id: String,
    pub kind: String,
}

pub async fn link_soil(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LinkSoilDto>,
) -> Result<Json<LinkSoilResponse>, (StatusCode, String)> {
    let kind = parse_link_kind(&body.kind)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let link: SoilAnalysisLink = uses_cases_link_soil::execute(
        &state,
        SoilAnalysisId(body.analysis_id),
        CycleId(body.cycle_id),
        kind.clone(),
    )
    .map_err(|_| (StatusCode::NOT_FOUND, "analysis or cycle not found".to_string()))?;
    Ok(Json(LinkSoilResponse {
        analysis_id: link.analysis_id.0.clone(),
        cycle_id: link.cycle_id.0.clone(),
        kind: format!("{:?}", link.kind),
    }))
}

mod uses_cases_link_soil {
    pub use crate::use_cases::link_soil_to_cycle::*;
}

pub async fn get_schedule(
    State(state): State<Arc<AppState>>,
    Path(cycle_id): Path<String>,
) -> Result<Json<ScheduleSummary>, StatusCode> {
    use kora_domain::ports::schedule_repository::ScheduleRepository;
    let repo = state.schedule_repo.lock().unwrap();
    let schedule = repo.find_by_cycle_id(&CycleId(cycle_id.clone())).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(ScheduleSummary {
        id: schedule.id().0.clone(),
        cycle_id: schedule.cycle_id().0.clone(),
        anchor: format!("{:?}", schedule.anchor()),
        anchor_date: schedule.anchor_date(),
        planned: schedule.activities().iter().map(|a| PlannedActivitySummary {
            id: a.id.0.clone(),
            category: format!("{:?}", a.category),
            relative_day: a.relative_day,
        }).collect(),
    }))
}

#[derive(Serialize)]
pub struct ScheduleSummary {
    pub id: String,
    pub cycle_id: String,
    pub anchor: String,
    pub anchor_date: i64,
    pub planned: Vec<PlannedActivitySummary>,
}

#[derive(Serialize)]
pub struct PlannedActivitySummary {
    pub id: String,
    pub category: String,
    pub relative_day: i32,
}

fn budget_to_summary(b: &Budget) -> BudgetSummary {
    let lines: Vec<BudgetLineSummary> = b
        .estimated_lines()
        .iter()
        .map(|(cat, amt)| BudgetLineSummary {
            category: format!("{cat:?}"),
            amount: format!("{} {:?}", amt.amount, amt.currency),
        })
        .collect();
    BudgetSummary {
        id: b.id().0.clone(),
        cycle_id: b.cycle_id().0.clone(),
        baseline: format!("{} {:?}", b.baseline().amount, b.baseline().currency),
        lines,
    }
}

fn parse_money(amount: &str, currency: &str) -> Result<Money, String> {
    let amount: Decimal = amount.parse().map_err(|_| format!("invalid amount: {amount}"))?;
    let currency = match currency {
        "USD" | "usd" => Currency::USD,
        "NIO" | "nio" => Currency::NIO,
        _ => return Err(format!("unknown currency: {currency}")),
    };
    Ok(Money::new(amount, currency))
}

fn parse_budget_category(s: &str) -> Result<BudgetCategory, String> {
    match s {
        "Seeds" | "seeds" => Ok(BudgetCategory::Seeds),
        "Fertilizers" | "fertilizers" => Ok(BudgetCategory::Fertilizers),
        "Labor" | "labor" => Ok(BudgetCategory::Labor),
        "SoilPrep" | "soil_prep" => Ok(BudgetCategory::SoilPrep),
        other => Ok(BudgetCategory::Other(other.to_string())),
    }
}

fn parse_activity_category(s: &str) -> Result<ActivityCategory, String> {
    match s {
        "Sowing" | "sowing" => Ok(ActivityCategory::Sowing),
        "Maintenance" | "maintenance" => Ok(ActivityCategory::Maintenance),
        "SanitaryControl" | "sanitary_control" => Ok(ActivityCategory::SanitaryControl),
        "Harvest" | "harvest" => Ok(ActivityCategory::Harvest),
        _ => Err(format!("unknown activity category: {s}")),
    }
}

fn parse_link_kind(s: &str) -> Result<LinkKind, String> {
    match s {
        "Previo" | "previo" | "pre" => Ok(LinkKind::Previo),
        "Seguimiento" | "seguimiento" => Ok(LinkKind::Seguimiento),
        "Posterior" | "posterior" | "post" => Ok(LinkKind::Posterior),
        _ => Err(format!("unknown link kind: {s}")),
    }
}
