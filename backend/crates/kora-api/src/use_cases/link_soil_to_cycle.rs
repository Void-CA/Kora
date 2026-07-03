use kora_domain::agriculture::soil::{LinkKind, SoilAnalysisLink, SoilError};
use kora_domain::agriculture::soil::SoilAnalysisId;
use kora_domain::ports::soil_analysis_repository::SoilAnalysisRepository;
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_kernel::ids::CycleId;

use crate::state::AppState;

pub fn execute(
    state: &AppState,
    analysis_id: SoilAnalysisId,
    cycle_id: CycleId,
    kind: LinkKind,
) -> Result<SoilAnalysisLink, SoilError> {
    let mut soil_repo = state.soil_repo.lock().unwrap();
    let cycle_repo = state.cycle_repo.lock().unwrap();
    if cycle_repo.find_by_id(&cycle_id).is_none() {
        return Err(SoilError::EmptyAreaId);
    }
    drop(cycle_repo);

    let mut analysis = soil_repo
        .find_by_id(&analysis_id)
        .ok_or(SoilError::EmptyAreaId)?;
    analysis.link_to_cycle(cycle_id.clone(), kind.clone());
    let link = SoilAnalysisLink {
        analysis_id: analysis.id().clone(),
        cycle_id,
        kind,
    };
    soil_repo.save(analysis);
    Ok(link)
}
