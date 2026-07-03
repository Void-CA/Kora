use crate::agriculture::soil::{SoilAnalysis, SoilAnalysisId};
use kora_kernel::ids::AreaId;

pub trait SoilAnalysisRepository {
    fn find_by_id(&self, id: &SoilAnalysisId) -> Option<SoilAnalysis>;
    fn save(&mut self, analysis: SoilAnalysis);
    fn all(&self) -> Vec<SoilAnalysis>;
    fn for_area(&self, area_id: &AreaId) -> Vec<SoilAnalysis>;
}
