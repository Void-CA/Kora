use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use kora_kernel::ids::AreaId;
use crate::state::AppState;
use crate::features::fields::{operations, dto::*};

pub async fn history(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Json<HistoryResponse> {
    Json(operations::get_history(&state, &AreaId(id)))
}

pub async fn dashboard(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Json<AreaDashboard> {
    Json(operations::get_dashboard(&state, &AreaId(id)))
}
