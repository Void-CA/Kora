use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::planning::Schedule;
use kora_domain::finance::budget::Budget;
use kora_kernel::ids::{AreaId, CycleId};

pub struct FieldHistory {
    pub area_id: AreaId,
    pub cycles: Vec<CropCycle>,
    pub schedules: Vec<Schedule>,
    pub budgets: Vec<Budget>,
}

pub fn execute(state: &crate::state::AppState, area_id: &AreaId) -> FieldHistory {
    let cycles: Vec<CropCycle> = state
        .cycle_repo
        .lock()
        .unwrap()
        .all()
        .into_iter()
        .filter(|c| c.area_id() == area_id)
        .collect();

    let cycle_ids: std::collections::HashSet<CycleId> = cycles.iter().map(|c| c.id().clone()).collect();

    let schedules: Vec<Schedule> = state
        .schedule_repo
        .lock()
        .unwrap()
        .all()
        .into_iter()
        .filter(|s| cycle_ids.contains(s.cycle_id()))
        .collect();

    let budgets: Vec<Budget> = state
        .budget_repo
        .lock()
        .unwrap()
        .all()
        .into_iter()
        .filter(|b| cycle_ids.contains(b.cycle_id()))
        .collect();

    FieldHistory {
        area_id: area_id.clone(),
        cycles,
        schedules,
        budgets,
    }
}
