use axum::{http::HeaderValue, routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::sync::Arc;

mod operation;
mod fields;
mod health;
mod cycles;
mod areas;
mod soil;
mod payroll;
mod incidence;
mod revenue;
mod planning;

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
        .route("/api/cycles/:id/timeline", get(cycles::timeline))
        .route("/api/areas/:id/history", get(areas::history))
        .route("/api/areas/:id/dashboard", get(areas::dashboard))
        .route("/api/soil/area/:area_id", get(soil::list_for_area))
        .route("/api/soil", post(soil::register))
        .route("/api/soil/link", post(planning::link_soil))
        .route("/api/payroll/workers", get(payroll::list_workers).post(payroll::register_worker))
        .route("/api/payroll/cycle/:cycle_id", get(payroll::list_for_cycle))
        .route("/api/payroll", post(payroll::record_payroll))
        .route("/api/incidence/cycle/:cycle_id", get(incidence::list_for_cycle))
        .route("/api/incidence", post(incidence::register))
        .route("/api/revenue", post(revenue::register))
        .route("/api/revenue/cycle/:cycle_id", get(revenue::list_for_cycle))
        .route("/api/budgets", post(planning::create_budget))
        .route("/api/schedules/cycle/:cycle_id", get(planning::get_schedule))
        .route("/api/schedules/activities", post(planning::add_planned_activity))
        .with_state(state)
        .layer(cors)
}
