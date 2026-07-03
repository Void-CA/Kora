use axum::{http::HeaderValue, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::sync::Arc;

mod operation;
mod fields;
mod health;
mod cycles;
mod areas;

use crate::state::AppState;

const DEV_ORIGIN: &str = "http://localhost:4200";

pub async fn serve() {
    let state = Arc::new(AppState::new());
    let app = router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Kora · API listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router(state: Arc<AppState>) -> Router {
    let origin: HeaderValue = DEV_ORIGIN.parse().expect("valid dev origin");
    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health::health))
        .route("/api/operation/today", get(operation::today))
        .route("/api/fields", get(fields::list))
        .route("/api/fields/:id", get(fields::get_one))
        .route("/api/cycles", get(cycles::list))
        .route("/api/cycles/:id", get(cycles::get_one))
        .route("/api/cycles/:id/profitability", get(cycles::profitability))
        .route("/api/areas/:id/history", get(areas::history))
        .with_state(state)
        .layer(cors)
}
