use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use kora_domain::finance::revenue::{Revenue, RevenueSource};
use kora_domain::ports::revenue_repository::RevenueRepository;
use kora_kernel::ids::CycleId;
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

use crate::state::AppState;
use crate::use_cases::register_revenue::{self as register_revenue_uc, RegisterRevenueInput};

#[derive(Deserialize)]
pub struct RegisterRevenueDto {
    pub cycle_id: Option<String>,
    pub amount: String,
    pub currency: String,
    pub received_at: i64,
    pub source: String,
}

#[derive(Serialize)]
pub struct RevenueSummary {
    pub id: String,
    pub cycle_id: Option<String>,
    pub amount: String,
    pub received_at: i64,
    pub source: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRevenueDto>,
) -> Result<Json<RevenueSummary>, (StatusCode, String)> {
    let amount: Decimal = body.amount.parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("invalid amount: {}", body.amount)))?;
    let currency = parse_currency(&body.currency).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let source = parse_source(&body.source);
    let input = RegisterRevenueInput {
        cycle_id: body.cycle_id.map(CycleId),
        amount: Money::new(amount, currency),
        received_at: body.received_at,
        source,
    };
    let revenue = register_revenue_uc::execute(&state, input)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(revenue_to_summary(&revenue)))
}

pub async fn list_for_cycle(
    State(state): State<Arc<AppState>>,
    Path(cycle_id): Path<String>,
) -> Json<Vec<RevenueSummary>> {
    let revenues = state.revenue_repo.lock().unwrap().for_cycle(&CycleId(cycle_id));
    Json(revenues.iter().map(revenue_to_summary).collect())
}

fn revenue_to_summary(r: &Revenue) -> RevenueSummary {
    RevenueSummary {
        id: r.id().0.clone(),
        cycle_id: r.cycle_id().map(|c| c.0.clone()),
        amount: format!("{} {:?}", r.amount().amount, r.amount().currency),
        received_at: r.received_at(),
        source: format!("{:?}", r.source()),
    }
}

fn parse_currency(s: &str) -> Result<Currency, String> {
    match s {
        "USD" | "usd" => Ok(Currency::USD),
        "NIO" | "nio" => Ok(Currency::NIO),
        _ => Err(format!("unknown currency: {s}")),
    }
}

fn parse_source(s: &str) -> RevenueSource {
    match s {
        "Harvest" | "harvest" => RevenueSource::Harvest,
        "Sale" | "sale" => RevenueSource::Sale,
        other => RevenueSource::Other(other.to_string()),
    }
}
