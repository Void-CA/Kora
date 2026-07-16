use axum::routing::{get, post};
use axum::Router;

use crate::config::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_employees))
        .route("/", post(handlers::create_employee))
        .route("/{id}/work-logs", get(handlers::list_work_logs))
}

pub fn work_logs_router() -> Router<AppState> {
    Router::new().route("/", post(handlers::create_work_log))
}
