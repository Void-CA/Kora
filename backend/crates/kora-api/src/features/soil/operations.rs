use kora_domain::agriculture::soil::{SoilAnalysis, SoilError, QualityLevel, SoilMetric, SoilMetricKind};
use kora_kernel::ids::AreaId;
use kora_kernel::money::Money;
use crate::state::AppState;
use crate::features::soil::dto::SoilSummary;

pub fn register(state: &AppState, area_id: AreaId, sampled_at: i64, quality: QualityLevel, cost: Money, metrics: Vec<SoilMetric>) -> Result<SoilAnalysis, SoilError> {
    let mut analysis = SoilAnalysis::new(area_id, sampled_at, quality, cost)?;
    for m in metrics { analysis.add_metric(m)?; }
    let analysis = analysis.finalize()?;
    state.soil_repo.lock().unwrap().save(analysis.clone());
    Ok(analysis)
}

pub fn list_for_area(state: &AppState, area_id: &AreaId) -> Vec<SoilSummary> {
    state.soil_repo.lock().unwrap().for_area(area_id).iter().map(|a| SoilSummary {
        id: a.id().0.clone(), area_id: a.area_id().0.clone(), sampled_at: a.sampled_at(),
        quality: format!("{:?}", a.quality()), cost: format!("{} {:?}", a.cost().amount, a.cost().currency),
        metric_count: a.metrics().len(),
    }).collect()
}

pub fn link_to_cycle(state: &AppState, analysis_id: kora_domain::agriculture::soil::SoilAnalysisId, cycle_id: kora_kernel::ids::CycleId, kind: kora_domain::agriculture::soil::LinkKind) -> Result<(), SoilError> {
    let mut soil = state.soil_repo.lock().unwrap().find_by_id(&analysis_id).ok_or(SoilError::EmptyAreaId)?;
    soil.link_to_cycle(cycle_id, kind);
    state.soil_repo.lock().unwrap().save(soil);
    Ok(())
}
