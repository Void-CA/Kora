use std::collections::HashMap;
use kora_domain::agriculture::incidence::{SanitaryIncidence, SanitaryIncidenceId};
use kora_domain::ports::sanitary_incidence_repository::SanitaryIncidenceRepository;
use kora_kernel::ids::CycleId;

pub struct InMemorySanitaryIncidenceRepository {
    incidences: HashMap<String, SanitaryIncidence>,
}

impl InMemorySanitaryIncidenceRepository {
    pub fn new() -> Self {
        Self { incidences: HashMap::new() }
    }
}

impl SanitaryIncidenceRepository for InMemorySanitaryIncidenceRepository {
    fn find_by_id(&self, id: &SanitaryIncidenceId) -> Option<SanitaryIncidence> {
        self.incidences.get(&id.0).cloned()
    }
    fn save(&mut self, incidence: SanitaryIncidence) {
        self.incidences.insert(incidence.id().0.clone(), incidence);
    }
    fn all(&self) -> Vec<SanitaryIncidence> {
        self.incidences.values().cloned().collect()
    }
    fn for_cycle(&self, cycle_id: &CycleId) -> Vec<SanitaryIncidence> {
        self.incidences
            .values()
            .filter(|i| i.cycle_id() == cycle_id)
            .cloned()
            .collect()
    }
}
