use std::collections::HashMap;
use kora_domain::finance::revenue::Revenue;
use kora_domain::ports::revenue_repository::RevenueRepository;
use kora_kernel::ids::CycleId;

pub struct InMemoryRevenueRepository {
    revenues: HashMap<String, Revenue>,
}

impl InMemoryRevenueRepository {
    pub fn new() -> Self {
        Self { revenues: HashMap::new() }
    }
}

impl RevenueRepository for InMemoryRevenueRepository {
    fn save(&mut self, revenue: Revenue) {
        self.revenues.insert(revenue.id().0.clone(), revenue);
    }
    fn all(&self) -> Vec<Revenue> {
        self.revenues.values().cloned().collect()
    }
    fn for_cycle(&self, cycle_id: &CycleId) -> Vec<Revenue> {
        self.revenues
            .values()
            .filter(|r| r.cycle_id() == Some(cycle_id))
            .cloned()
            .collect()
    }
}
