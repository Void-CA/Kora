use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use crate::state::AppState;
use crate::features::soil::{operations, dto::SoilSummary};
use kora_kernel::ids::AreaId;
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct RegisterSoilDto {
    pub area_id: String, pub sampled_at: i64, pub quality: String,
    pub cost_amount: String, pub cost_currency: String,
    pub metrics: Vec<SoilMetricDto>,
}
#[derive(Deserialize)]
pub struct SoilMetricDto { pub kind: String, pub value: String }
#[derive(Deserialize)]
pub struct LinkSoilDto { pub analysis_id: String, pub cycle_id: String, pub kind: String }

pub async fn list_for_area(State(state): State<Arc<AppState>>, Path(area_id): Path<String>) -> Json<Vec<SoilSummary>> {
    Json(operations::list_for_area(&state, &AreaId(area_id)))
}

pub async fn register(State(state): State<Arc<AppState>>, Json(body): Json<RegisterSoilDto>) -> Result<Json<SoilSummary>, (StatusCode, String)> {
    let quality = parse_quality(&body.quality)?;
    let cost = parse_money(&body.cost_amount, &body.cost_currency)?;
    let mut metrics = Vec::new();
    for m in &body.metrics {
        let kind = parse_metric_kind(&m.kind)?;
        let value: Decimal = m.value.parse().map_err(|_| (StatusCode::BAD_REQUEST, format!("bad value: {}", m.value)))?;
        metrics.push(kora_domain::agriculture::soil::SoilMetric::new(kind, value).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?);
    }
    let a = operations::register(&state, AreaId(body.area_id), body.sampled_at, quality, cost, metrics).map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:?}")))?;
    Ok(Json(SoilSummary { id: a.id().0.clone(), area_id: a.area_id().0.clone(), sampled_at: a.sampled_at(), quality: format!("{:?}", a.quality()), cost: format!("{} {:?}", a.cost().amount, a.cost().currency), metric_count: a.metrics().len() }))
}

pub async fn link_soil(State(state): State<Arc<AppState>>, Json(body): Json<LinkSoilDto>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let kind = parse_link_kind(&body.kind)?;
    operations::link_to_cycle(&state, kora_domain::agriculture::soil::SoilAnalysisId(body.analysis_id), kora_kernel::ids::CycleId(body.cycle_id), kind).map_err(|_| (StatusCode::NOT_FOUND, "not found".into()))?;
    Ok(Json(serde_json::json!({"ok": true})))
}

fn parse_quality(s: &str) -> Result<kora_domain::agriculture::soil::QualityLevel, (StatusCode, String)> {
    match s { "basic" | "Basic" => Ok(kora_domain::agriculture::soil::QualityLevel::Basic), "complete" | "Complete" => Ok(kora_domain::agriculture::soil::QualityLevel::Complete), "satellite" | "Satellite" => Ok(kora_domain::agriculture::soil::QualityLevel::Satellite), _ => Err((StatusCode::BAD_REQUEST, "bad quality".into())) }
}
fn parse_metric_kind(s: &str) -> Result<kora_domain::agriculture::soil::SoilMetricKind, (StatusCode, String)> {
    match s { "Ph" | "ph" => Ok(kora_domain::agriculture::soil::SoilMetricKind::Ph), "Nitrogen" | "nitrogen" | "N" => Ok(kora_domain::agriculture::soil::SoilMetricKind::Nitrogen), "Phosphorus" | "phosphorus" | "P" => Ok(kora_domain::agriculture::soil::SoilMetricKind::Phosphorus), "Potassium" | "potassium" | "K" => Ok(kora_domain::agriculture::soil::SoilMetricKind::Potassium), "OrganicMatter" | "organic_matter" => Ok(kora_domain::agriculture::soil::SoilMetricKind::OrganicMatter), "Moisture" | "moisture" => Ok(kora_domain::agriculture::soil::SoilMetricKind::Moisture), "CationExchangeCapacity" | "cec" => Ok(kora_domain::agriculture::soil::SoilMetricKind::CationExchangeCapacity), _ => Err((StatusCode::BAD_REQUEST, "bad metric".into())) }
}
fn parse_link_kind(s: &str) -> Result<kora_domain::agriculture::soil::LinkKind, (StatusCode, String)> {
    match s { "Previo" | "previo" | "pre" => Ok(kora_domain::agriculture::soil::LinkKind::Previo), "Seguimiento" | "seguimiento" => Ok(kora_domain::agriculture::soil::LinkKind::Seguimiento), "Posterior" | "posterior" | "post" => Ok(kora_domain::agriculture::soil::LinkKind::Posterior), _ => Err((StatusCode::BAD_REQUEST, "bad link kind".into())) }
}
fn parse_money(amount: &str, currency: &str) -> Result<Money, (StatusCode, String)> {
    let a: Decimal = amount.parse().map_err(|_| (StatusCode::BAD_REQUEST, "bad amount".into()))?;
    let c = match currency { "USD" | "usd" => Currency::USD, "NIO" | "nio" => Currency::NIO, _ => return Err((StatusCode::BAD_REQUEST, "bad currency".into())) };
    Ok(Money::new(a, c))
}
