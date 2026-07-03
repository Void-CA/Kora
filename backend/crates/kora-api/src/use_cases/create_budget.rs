use kora_domain::finance::budget::{Budget, BudgetCategory};
use kora_domain::finance::error::FinanceError;
use kora_domain::ports::budget_repository::BudgetRepository;
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;
use kora_kernel::period::Period;

use crate::state::AppState;

pub struct CreateBudgetInput {
    pub cycle_id: CycleId,
    pub period: Period,
    pub baseline: Money,
    pub estimated_lines: Vec<(BudgetCategory, Money)>,
}

pub fn execute(state: &AppState, input: CreateBudgetInput) -> Result<Budget, FinanceError> {
    let mut budget = Budget::new(input.cycle_id, input.period, input.baseline);
    for (cat, amount) in input.estimated_lines {
        budget.estimate_category(cat, amount)?;
    }
    let saved = budget.clone();
    state.budget_repo.lock().unwrap().save(budget);
    Ok(saved)
}
