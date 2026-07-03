use kora_domain::agriculture::activity::ActivityCategory;
use kora_domain::agriculture::planning::PlannedActivity;
use kora_domain::ports::schedule_repository::ScheduleRepository;
use kora_kernel::ids::CycleId;

use crate::state::AppState;

pub fn execute(
    state: &AppState,
    cycle_id: &CycleId,
    category: ActivityCategory,
    relative_day: i32,
) -> Option<PlannedActivityId> {
    let mut repo = state.schedule_repo.lock().unwrap();
    let mut schedule = repo.find_by_cycle_id(cycle_id)?;
    let planned = PlannedActivity::new(category, relative_day);
    let id = planned.id.clone();
    schedule.add_planned_activity(planned);
    repo.save(schedule);
    Some(id)
}

pub use kora_domain::agriculture::ids::PlannedActivityId;
