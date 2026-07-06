use std::sync::Arc;
use axum::{extract::State, Json};
use crate::state::AppState;
use crate::features::home::dto::HomeResponse;
use crate::features::home::dashboard;

pub async fn home(State(state): State<Arc<AppState>>) -> Json<HomeResponse> {
    Json(dashboard::execute(&state))
}
