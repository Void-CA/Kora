use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::planning::Schedule;
use kora_domain::agriculture::planning_service::CropPlanningService;
use kora_domain::agriculture::planning::PlannedActivity;
use kora_domain::agriculture::error::AgricultureError;
use kora_domain::agriculture::crop::Crop;
use kora_domain::agriculture::activity::ActivityCategory;
use kora_kernel::ids::{AreaId, CropId};
use kora_kernel::period::Period;

pub struct RegisterCycleInput {
    pub crop_id: CropId,
    pub area_id: AreaId,
    pub period: Period,
    pub planned_activities: Vec<(ActivityCategory, i32)>,
}

pub struct RegisterCycleOutput {
    pub cycle: CropCycle,
    pub schedule: Schedule,
}

pub fn execute(
    state: &crate::state::AppState,
    input: RegisterCycleInput,
) -> Result<RegisterCycleOutput, AgricultureError> {
    let farm = state
        .farm_for_area(&input.area_id)
        .ok_or_else(|| AgricultureError::AreaNotFound(input.area_id.clone()))?;

    let crop = Crop::new(input.crop_id, "Cultivo".into());

    let existing_cycles: Vec<CropCycle> = state.cycle_repo.lock().unwrap().all();

    let planning = CropPlanningService::schedule_cycle(
        farm,
        &input.area_id,
        &crop,
        input.period,
        &existing_cycles,
    )?;

    let mut schedule = planning.schedule;
    for (category, day) in input.planned_activities {
        schedule.add_planned_activity(PlannedActivity::new(category, day));
    }

    state.schedule_repo.lock().unwrap().save(schedule.clone());
    state.cycle_repo.lock().unwrap().save(planning.cycle.clone());

    Ok(RegisterCycleOutput {
        cycle: planning.cycle,
        schedule,
    })
}
