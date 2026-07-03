use std::sync::Arc;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use kora_kernel::ids::AreaId;

use crate::state::AppState;
use crate::use_cases::get_field_history::{self, FieldHistory};

#[derive(Serialize)]
pub struct HistoryResponse {
    pub area_id: String,
    pub area_name: String,
    pub cycles: Vec<String>,
    pub schedules: Vec<String>,
    pub budgets: Vec<BudgetSummary>,
}

#[derive(Serialize)]
pub struct BudgetSummary {
    pub id: String,
    pub cycle_id: String,
    pub baseline: String,
    pub spent: String,
    pub remaining: String,
    pub variance: String,
}

pub async fn history(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<HistoryResponse> {
    let area_id = AreaId(id.clone());
    let result: FieldHistory = get_field_history::execute(&state, &area_id);
    let area_name = state
        .farms
        .iter()
        .flat_map(|f| f.areas())
        .find(|a| a.id() == &area_id)
        .map(|a| a.name().to_string())
        .unwrap_or_else(|| id.clone());

    let budgets: Vec<BudgetSummary> = result
        .budgets
        .iter()
        .map(|b| BudgetSummary {
            id: b.id().0.clone(),
            cycle_id: b.cycle_id().0.clone(),
            baseline: b.baseline().amount.to_string(),
            spent: b.current_expenses().amount.to_string(),
            remaining: b.get_remaining().map(|m| m.amount.to_string()).unwrap_or_default(),
            variance: b.get_variance().map(|m| m.amount.to_string()).unwrap_or_default(),
        })
        .collect();

    Json(HistoryResponse {
        area_id: id,
        area_name,
        cycles: result.cycles.iter().map(|c| c.id().0.clone()).collect(),
        schedules: result.schedules.iter().map(|s| s.cycle_id().0.clone()).collect(),
        budgets,
    })
}
