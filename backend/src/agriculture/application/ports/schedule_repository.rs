// agriculture/application/ports/schedule_repository.rs
use crate::agriculture::domain::planning::Schedule;
use crate::shared_kernel::ids::CycleId;

/// Port: Repository interface for Schedule persistence
pub trait ScheduleRepository {
    fn find_by_cycle_id(&self, cycle_id: &CycleId) -> Option<Schedule>;
    fn save(&mut self, schedule: Schedule);
}

// In-memory implementation for testing
pub struct InMemoryScheduleRepository {
    schedules: std::collections::HashMap<String, Schedule>,
}

impl InMemoryScheduleRepository {
    pub fn new() -> Self {
        Self {
            schedules: std::collections::HashMap::new(),
        }
    }
}

impl ScheduleRepository for InMemoryScheduleRepository {
    fn find_by_cycle_id(&self, cycle_id: &CycleId) -> Option<Schedule> {
        self.schedules.get(&cycle_id.0).cloned()
    }

    fn save(&mut self, schedule: Schedule) {
        self.schedules.insert(schedule.cycle_id().0.clone(), schedule);
    }
}
