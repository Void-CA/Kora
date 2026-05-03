// agriculture/application/ports/cycle_repository.rs
use crate::agriculture::domain::cycle::CropCycle;
use crate::shared_kernel::ids::CycleId;

/// Port: Repository interface for CropCycle persistence
pub trait CropCycleRepository {
    fn find_by_id(&self, id: &CycleId) -> Option<CropCycle>;
    fn save(&mut self, cycle: CropCycle);
}

// In-memory implementation for testing
pub struct InMemoryCropCycleRepository {
    cycles: std::collections::HashMap<String, CropCycle>,
}

impl InMemoryCropCycleRepository {
    pub fn new() -> Self {
        Self {
            cycles: std::collections::HashMap::new(),
        }
    }
}

impl CropCycleRepository for InMemoryCropCycleRepository {
    fn find_by_id(&self, id: &CycleId) -> Option<CropCycle> {
        self.cycles.get(&id.0).cloned()
    }

    fn save(&mut self, cycle: CropCycle) {
        self.cycles.insert(cycle.id().0.clone(), cycle);
    }
}
