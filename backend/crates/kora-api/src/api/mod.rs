use axum::{http::HeaderValue, routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::sync::Arc;

mod operation;
mod fields;
mod health;
mod overview;

use crate::state::AppState;
use crate::features::{
    home::handlers as home_h,
    finance::handlers as finance_h,
    soil::handlers as soil_h,
    planning::handlers as plan_h,
    payroll::handlers as payroll_h,
    incidences::handlers as inc_h,
    cycles::handlers as cycles_h,
    fields::handlers as fields_h,
};

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
        .route("/api/home", get(home_h::home))
        .route("/api/fields/overview", get(overview::fields_overview))
        .route("/api/fields/geojson", get(overview::fields_geojson))
        .route("/api/operations/today", get(overview::operations_today))
        .route("/api/team/overview", get(overview::team_overview))
        .route("/api/finances/overview", get(overview::finances_overview))
        .route("/api/history/overview", get(overview::history_overview))
        .route("/api/operation/today", get(operation::today))
        .route("/api/fields", get(fields::list))
        .route("/api/fields/:id", get(fields::get_one))
        .route("/api/cycles", get(cycles_h::list))
        .route("/api/cycles/:id", get(cycles_h::get_one))
        .route("/api/cycles/:id/timeline", get(cycles_h::timeline))
        .route("/api/cycles/:id/variance", get(cycles_h::variance))
        .route("/api/cycles/:id/profitability", get(finance_h::profitability_handler))
        .route("/api/cycles/:id/activities", post(cycles_h::register_activity))
        .route("/api/areas/:id/history", get(fields_h::history))
        .route("/api/areas/:id/dashboard", get(fields_h::dashboard))
        .route("/api/soil/area/:area_id", get(soil_h::list_for_area))
        .route("/api/soil", post(soil_h::register))
        .route("/api/soil/link", post(soil_h::link_soil))
        .route("/api/payroll/workers", get(payroll_h::list_workers).post(payroll_h::register_worker))
        .route("/api/payroll/cycle/:cycle_id", get(payroll_h::list_payroll))
        .route("/api/payroll", post(payroll_h::record_payroll))
        .route("/api/incidence/cycle/:cycle_id", get(inc_h::list_for_cycle))
        .route("/api/incidence", post(inc_h::register))
        .route("/api/revenue", post(finance_h::register_revenue))
        .route("/api/revenue/cycle/:cycle_id", get(finance_h::list_revenue))
        .route("/api/budgets", post(plan_h::create_budget))
        .route("/api/schedules/cycle/:cycle_id", get(plan_h::get_schedule))
        .route("/api/schedules/activities", post(plan_h::add_activity))
        .with_state(state)
        .layer(cors)
}
