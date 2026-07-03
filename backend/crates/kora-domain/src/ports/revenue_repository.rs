use crate::finance::revenue::Revenue;
use kora_kernel::ids::CycleId;

pub trait RevenueRepository {
    fn save(&mut self, revenue: Revenue);
    fn all(&self) -> Vec<Revenue>;
    fn for_cycle(&self, cycle_id: &CycleId) -> Vec<Revenue>;
}
