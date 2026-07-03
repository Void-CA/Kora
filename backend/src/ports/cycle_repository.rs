use crate::agriculture::cycle::CropCycle;
use crate::shared_kernel::ids::CycleId;

pub trait CropCycleRepository {
    fn find_by_id(&self, id: &CycleId) -> Option<CropCycle>;
    fn save(&mut self, cycle: CropCycle);
}
