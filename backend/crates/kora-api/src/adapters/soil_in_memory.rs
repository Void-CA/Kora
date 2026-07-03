use std::collections::HashMap;
use kora_domain::agriculture::soil::{SoilAnalysis, SoilAnalysisId};
use kora_domain::ports::soil_analysis_repository::SoilAnalysisRepository;
use kora_kernel::ids::AreaId;

pub struct InMemorySoilAnalysisRepository {
    analyses: HashMap<String, SoilAnalysis>,
}

impl InMemorySoilAnalysisRepository {
    pub fn new() -> Self {
        Self { analyses: HashMap::new() }
    }
}

impl SoilAnalysisRepository for InMemorySoilAnalysisRepository {
    fn find_by_id(&self, id: &SoilAnalysisId) -> Option<SoilAnalysis> {
        self.analyses.get(&id.0).cloned()
    }
    fn save(&mut self, analysis: SoilAnalysis) {
        self.analyses.insert(analysis.id().0.clone(), analysis);
    }
    fn all(&self) -> Vec<SoilAnalysis> {
        self.analyses.values().cloned().collect()
    }
    fn for_area(&self, area_id: &AreaId) -> Vec<SoilAnalysis> {
        self.analyses
            .values()
            .filter(|a| a.area_id() == area_id)
            .cloned()
            .collect()
    }
}
