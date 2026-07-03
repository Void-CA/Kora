use kora_domain::agriculture::soil::{SoilAnalysis, SoilError, QualityLevel, SoilMetric};
use kora_domain::ports::soil_analysis_repository::SoilAnalysisRepository;
use kora_kernel::ids::AreaId;
use kora_kernel::money::Money;

pub struct RegisterSoilAnalysisInput {
    pub area_id: AreaId,
    pub sampled_at: i64,
    pub quality: QualityLevel,
    pub cost: Money,
    pub metrics: Vec<SoilMetric>,
}

pub fn execute(
    state: &crate::state::AppState,
    input: RegisterSoilAnalysisInput,
) -> Result<SoilAnalysis, SoilError> {
    let mut analysis = SoilAnalysis::new(
        input.area_id,
        input.sampled_at,
        input.quality,
        input.cost,
    )?;
    for metric in input.metrics {
        analysis.add_metric(metric)?;
    }
    let analysis = analysis.finalize()?;
    state.soil_repo.lock().unwrap().save(analysis.clone());
    Ok(analysis)
}
