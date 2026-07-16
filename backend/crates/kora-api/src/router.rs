use axum::Router;

use crate::config::AppState;
use crate::features;

pub fn build() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/employees", features::employees::router::router())
}
