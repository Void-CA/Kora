use axum::{http::HeaderValue, routing::{get, post}, Router};
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

    use crate::features::{
        home::handlers as home_h,
        finance::handlers as finance_h,
        soil::handlers as soil_h,
        planning::handlers as plan_h,
        payroll::handlers as payroll_h,
        incidences::handlers as inc_h,
    };

    Router::new()
        .route("/api/health", get(health::health))
        .route("/api/home", get(home_h::home))
        .route("/api/operation/today", get(operation::today))
        // Fields
        .route("/api/fields", get(fields::list))
        .route("/api/fields/:id", get(fields::get_one))
        // Cycles
        .route("/api/cycles", get(cycles::list))
        .route("/api/cycles/:id", get(cycles::get_one))
        .route("/api/cycles/:id/timeline", get(cycles::timeline))
        .route("/api/cycles/:id/variance", get(cycles::variance))
        .route("/api/cycles/:id/activities", post(cycles::register_activity))
        .route("/api/cycles/:id/profitability", get(finance_h::profitability_handler))
        // Areas
        .route("/api/areas/:id/history", get(areas::history))
        .route("/api/areas/:id/dashboard", get(areas::dashboard))
        // Soil
        .route("/api/soil/area/:area_id", get(soil_h::list_for_area))
        .route("/api/soil", post(soil_h::register))
        .route("/api/soil/link", post(soil_h::link_soil))
        // Payroll
        .route("/api/payroll/workers", get(payroll_h::list_workers).post(payroll_h::register_worker))
        .route("/api/payroll/cycle/:cycle_id", get(payroll_h::list_payroll))
        .route("/api/payroll", post(payroll_h::record_payroll))
        // Incidences
        .route("/api/incidence/cycle/:cycle_id", get(inc_h::list_for_cycle))
        .route("/api/incidence", post(inc_h::register))
        // Revenue
        .route("/api/revenue", post(finance_h::register_revenue))
        .route("/api/revenue/cycle/:cycle_id", get(finance_h::list_revenue))
        // Planning
        .route("/api/budgets", post(plan_h::create_budget))
        .route("/api/schedules/cycle/:cycle_id", get(plan_h::get_schedule))
        .route("/api/schedules/activities", post(plan_h::add_activity))
        .with_state(state)
        .layer(cors)
}
