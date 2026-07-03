use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use kora_domain::agriculture::soil::{SoilAnalysis, SoilMetricKind, QualityLevel};
use kora_kernel::ids::AreaId;
use rust_decimal::Decimal;
use kora_kernel::money::{Currency, Money};

use crate::state::AppState;
use crate::use_cases::list_soil_for_area as list_soil_uc;
use crate::use_cases::register_soil_analysis::{self as register_soil_uc, RegisterSoilAnalysisInput};

#[derive(Deserialize)]
pub struct SoilMetricDto {
    pub kind: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct RegisterSoilDto {
    pub area_id: String,
    pub sampled_at: i64,
    pub quality: String,
    pub cost_amount: String,
    pub cost_currency: String,
    pub metrics: Vec<SoilMetricDto>,
}

#[derive(Serialize)]
pub struct SoilSummary {
    pub id: String,
    pub area_id: String,
    pub sampled_at: i64,
    pub quality: String,
    pub cost: String,
    pub metric_count: usize,
}

pub async fn list_for_area(
    State(state): State<Arc<AppState>>,
    Path(area_id): Path<String>,
) -> Json<Vec<SoilSummary>> {
    let analyses = list_soil_uc::execute(&state, &AreaId(area_id));
    Json(analyses.iter().map(soil_to_summary).collect())
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterSoilDto>,
) -> Result<Json<SoilSummary>, (StatusCode, String)> {
    let quality = parse_quality(&body.quality)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let cost = parse_money(&body.cost_amount, &body.cost_currency)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let mut metrics = Vec::with_capacity(body.metrics.len());
    for m in &body.metrics {
        let kind = parse_kind(&m.kind)
            .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
        let value: Decimal = m.value.parse()
            .map_err(|_| (StatusCode::BAD_REQUEST, format!("invalid metric value: {}", m.value)))?;
        let metric = kora_domain::agriculture::soil::SoilMetric::new(kind, value)
            .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
        metrics.push(metric);
    }
    let input = RegisterSoilAnalysisInput {
        area_id: AreaId(body.area_id),
        sampled_at: body.sampled_at,
        quality,
        cost,
        metrics,
    };
    let analysis = register_soil_uc::execute(&state, input)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e:?}")))?;
    Ok(Json(soil_to_summary(&analysis)))
}

fn soil_to_summary(a: &SoilAnalysis) -> SoilSummary {
    SoilSummary {
        id: a.id().0.clone(),
        area_id: a.area_id().0.clone(),
        sampled_at: a.sampled_at(),
        quality: format!("{:?}", a.quality()),
        cost: format!("{} {:?}", a.cost().amount, a.cost().currency),
        metric_count: a.metrics().len(),
    }
}

fn parse_quality(s: &str) -> Result<QualityLevel, String> {
    match s {
        "Basic" | "basic" => Ok(QualityLevel::Basic),
        "Complete" | "complete" => Ok(QualityLevel::Complete),
        "Satellite" | "satellite" => Ok(QualityLevel::Satellite),
        _ => Err(format!("unknown quality: {s}")),
    }
}

fn parse_kind(s: &str) -> Result<SoilMetricKind, String> {
    match s {
        "Ph" | "ph" => Ok(SoilMetricKind::Ph),
        "Nitrogen" | "nitrogen" | "N" => Ok(SoilMetricKind::Nitrogen),
        "Phosphorus" | "phosphorus" | "P" => Ok(SoilMetricKind::Phosphorus),
        "Potassium" | "potassium" | "K" => Ok(SoilMetricKind::Potassium),
        "OrganicMatter" | "organic_matter" => Ok(SoilMetricKind::OrganicMatter),
        "Moisture" | "moisture" => Ok(SoilMetricKind::Moisture),
        "CationExchangeCapacity" | "cec" => Ok(SoilMetricKind::CationExchangeCapacity),
        _ => Err(format!("unknown metric kind: {s}")),
    }
}

fn parse_money(amount: &str, currency: &str) -> Result<Money, String> {
    let amount: Decimal = amount.parse().map_err(|_| format!("invalid amount: {amount}"))?;
    let currency = match currency {
        "USD" | "usd" => Currency::USD,
        "NIO" | "nio" => Currency::NIO,
        _ => return Err(format!("unknown currency: {currency}")),
    };
    Ok(Money::new(amount, currency))
}
