use std::sync::Arc;
use axum::{extract::State, Json};
use crate::state::AppState;
use crate::use_cases::get_home_dashboard::{self, HomeResponse};

pub async fn home(State(state): State<Arc<AppState>>) -> Json<HomeResponse> {
    Json(get_home_dashboard::execute(&state))
}
