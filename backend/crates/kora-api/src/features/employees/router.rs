use axum::routing::{get, post};
use axum::Router;

use crate::config::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_employees))
        .route("/", post(handlers::create_employee))
}
