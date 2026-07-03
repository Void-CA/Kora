use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::planning::Schedule;
use kora_domain::finance::budget::Budget;
use kora_kernel::ids::CycleId;
use kora_domain::finance::ids::BudgetId;
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_domain::ports::schedule_repository::ScheduleRepository;
use kora_domain::ports::budget_repository::BudgetRepository;

pub struct InMemoryCropCycleRepository {
    cycles: std::collections::HashMap<String, CropCycle>,
}

impl InMemoryCropCycleRepository {
    pub fn new() -> Self {
        Self { cycles: std::collections::HashMap::new() }
    }
}

impl CropCycleRepository for InMemoryCropCycleRepository {
    fn find_by_id(&self, id: &CycleId) -> Option<CropCycle> {
        self.cycles.get(&id.0).cloned()
    }
    fn save(&mut self, cycle: CropCycle) {
        self.cycles.insert(cycle.id().0.clone(), cycle);
    }
}

pub struct InMemoryScheduleRepository {
    schedules: std::collections::HashMap<String, Schedule>,
}

impl InMemoryScheduleRepository {
    pub fn new() -> Self {
        Self { schedules: std::collections::HashMap::new() }
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

pub struct InMemoryBudgetRepository {
    budgets: std::collections::HashMap<String, Budget>,
}

impl InMemoryBudgetRepository {
    pub fn new() -> Self {
        Self { budgets: std::collections::HashMap::new() }
    }
}

impl BudgetRepository for InMemoryBudgetRepository {
    fn find_by_id(&self, id: &BudgetId) -> Option<Budget> {
        self.budgets.get(&id.0).cloned()
    }
    fn save(&mut self, budget: Budget) {
        self.budgets.insert(budget.id().0.clone(), budget);
    }
}
