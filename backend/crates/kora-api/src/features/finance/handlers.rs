use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use crate::state::AppState;
use crate::features::finance::{profitability, revenue};
use crate::features::finance::dto::{Profitability, RevenueSummary};
use kora_kernel::ids::CycleId;
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

pub async fn profitability_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Profitability>, StatusCode> {
    profitability::execute(&state, &CycleId(id)).map(Json).map_err(|_| StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
pub struct RegisterRevenueDto {
    pub cycle_id: Option<String>,
    pub amount: String,
    pub currency: String,
    pub received_at: i64,
    pub source: String,
}

pub async fn register_revenue(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRevenueDto>,
) -> Result<Json<RevenueSummary>, (StatusCode, String)> {
    let amount: Decimal = body.amount.parse().map_err(|_| (StatusCode::BAD_REQUEST, "bad amount".into()))?;
    let currency = match body.currency.as_str() { "USD" | "usd" => Currency::USD, "NIO" | "nio" => Currency::NIO, _ => return Err((StatusCode::BAD_REQUEST, "bad currency".into())) };
    let source = match body.source.as_str() { "Harvest" | "harvest" => kora_domain::finance::revenue::RevenueSource::Harvest, "Sale" | "sale" => kora_domain::finance::revenue::RevenueSource::Sale, other => kora_domain::finance::revenue::RevenueSource::Other(other.into()) };
    let input = revenue::RegisterRevenueInput { cycle_id: body.cycle_id.map(CycleId), amount: Money::new(amount, currency), received_at: body.received_at, source };
    let r = revenue::register(&state, input).map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(RevenueSummary {
        id: r.id().0.clone(),
        cycle_id: r.cycle_id().map(|c| c.0.clone()),
        amount: format!("{} {:?}", r.amount().amount, r.amount().currency),
        received_at: r.received_at(),
        source: format!("{:?}", r.source()),
    }))
}

pub async fn list_revenue(
    State(state): State<Arc<AppState>>,
    Path(cycle_id): Path<String>,
) -> Json<Vec<RevenueSummary>> {
    Json(revenue::list_for_cycle(&state, &CycleId(cycle_id)))
}
