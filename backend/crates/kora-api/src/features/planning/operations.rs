use kora_domain::agriculture::activity::ActivityCategory;
use kora_domain::agriculture::planning::PlannedActivity;
use kora_domain::agriculture::ids::PlannedActivityId;
use kora_domain::finance::budget::{Budget, BudgetCategory};
use kora_domain::finance::error::FinanceError;
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;
use kora_kernel::period::Period;
use crate::state::AppState;
use crate::features::planning::dto::{BudgetSummary, BudgetLineSummary, ScheduleSummary, PlannedActivitySummary};

pub fn create_budget(state: &AppState, cycle_id: CycleId, period: Period, baseline: Money, lines: Vec<(BudgetCategory, Money)>) -> Result<Budget, FinanceError> {
    let mut budget = Budget::new(cycle_id, period, baseline);
    for (cat, amount) in lines { budget.estimate_category(cat, amount)?; }
    let saved = budget.clone();
    state.budget_repo.lock().unwrap().save(budget);
    Ok(saved)
}

pub fn add_planned_activity(state: &AppState, cycle_id: &CycleId, category: ActivityCategory, relative_day: i32) -> Option<PlannedActivityId> {
    let mut repo = state.schedule_repo.lock().unwrap();
    let mut schedule = repo.find_by_cycle_id(cycle_id)?;
    let planned = PlannedActivity::new(category, relative_day);
    let id = planned.id.clone();
    schedule.add_planned_activity(planned);
    repo.save(schedule);
    Some(id)
}

pub fn get_schedule(state: &AppState, cycle_id: &CycleId) -> Option<ScheduleSummary> {
    use kora_domain::ports::schedule_repository::ScheduleRepository;
    let repo = state.schedule_repo.lock().unwrap();
    let s = repo.find_by_cycle_id(cycle_id)?;
    Some(ScheduleSummary {
        id: s.id().0.clone(), cycle_id: s.cycle_id().0.clone(),
        anchor: format!("{:?}", s.anchor()), anchor_date: s.anchor_date(),
        planned: s.activities().iter().map(|a| PlannedActivitySummary { id: a.id.0.clone(), category: format!("{:?}", a.category), relative_day: a.relative_day }).collect(),
    })
}

pub fn budget_to_summary(b: &Budget) -> BudgetSummary {
    let lines: Vec<BudgetLineSummary> = b.estimated_lines().iter().map(|(cat, amt)| BudgetLineSummary { category: format!("{cat:?}"), amount: format!("{} {:?}", amt.amount, amt.currency) }).collect();
    BudgetSummary { id: b.id().0.clone(), cycle_id: b.cycle_id().0.clone(), baseline: format!("{} {:?}", b.baseline().amount, b.baseline().currency), lines }
}
