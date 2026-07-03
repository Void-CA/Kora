use crate::agriculture::planning::Schedule;
use crate::shared_kernel::ids::CycleId;

pub trait ScheduleRepository {
    fn find_by_cycle_id(&self, cycle_id: &CycleId) -> Option<Schedule>;
    fn save(&mut self, schedule: Schedule);
}
