use std::sync::Arc;
use axum::{extract::State, Json};
use crate::state::AppState;
use crate::features;

pub async fn fields_overview(State(state): State<Arc<AppState>>) -> Json<features::fields::overview::FieldsOverview> {
    Json(features::fields::overview::execute(&state))
}
pub async fn operations_today(State(state): State<Arc<AppState>>) -> Json<features::operations::overview::OperationsToday> {
    Json(features::operations::overview::execute(&state))
}
pub async fn team_overview(State(state): State<Arc<AppState>>) -> Json<features::payroll::overview::TeamOverview> {
    Json(features::payroll::overview::execute(&state))
}
pub async fn finances_overview(State(state): State<Arc<AppState>>) -> Json<features::finance::overview::FinancesOverview> {
    Json(features::finance::overview::execute(&state))
}
pub async fn history_overview(State(state): State<Arc<AppState>>) -> Json<features::history::overview::HistoryOverview> {
    Json(features::history::overview::execute(&state))
}
