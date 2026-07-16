use axum::Router;

use crate::config::AppState;
use crate::features::employees;

pub fn build() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/employees", employees::router::router())
        .nest("/api/v1/work-logs", employees::router::work_logs_router())
}
