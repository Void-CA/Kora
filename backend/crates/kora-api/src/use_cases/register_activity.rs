use kora_domain::agriculture::activity::{Activity, ActivityCategory, ActivityRecord};
use kora_domain::agriculture::error::AgricultureError;
use kora_kernel::ids::CycleId;

pub struct RegisterActivityInput {
    pub cycle_id: CycleId,
    pub timestamp: i64,
    pub category: ActivityCategory,
    pub notes: Option<String>,
}

pub fn execute(
    state: &crate::state::AppState,
    input: RegisterActivityInput,
) -> Result<ActivityRecord, AgricultureError> {
    let mut repo = state.cycle_repo.lock().unwrap();
    let mut cycle = repo
        .find_by_id(&input.cycle_id)
        .ok_or(AgricultureError::CycleNotFound(input.cycle_id.clone()))?;

    let mut activity = Activity::new(input.timestamp, input.category);
    if let Some(notes) = input.notes {
        activity.set_notes(notes);
    }

    let record = cycle.register_activity(activity)?;
    repo.save(cycle);
    Ok(record)
}
