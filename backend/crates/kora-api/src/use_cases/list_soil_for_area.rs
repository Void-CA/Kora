use kora_domain::agriculture::soil::SoilAnalysis;
use kora_domain::ports::soil_analysis_repository::SoilAnalysisRepository;
use kora_kernel::ids::AreaId;

pub fn execute(state: &crate::state::AppState, area_id: &AreaId) -> Vec<SoilAnalysis> {
    state.soil_repo.lock().unwrap().for_area(area_id)
}
