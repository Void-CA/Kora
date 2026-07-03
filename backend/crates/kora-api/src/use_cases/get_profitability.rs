use kora_domain::finance::error::FinanceError;
use kora_domain::finance::budget::Budget;
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;
use serde::Serialize;

#[derive(Serialize)]
pub struct Profitability {
    pub baseline: String,
    pub spent: String,
    pub remaining: String,
    pub variance: String,
}

pub fn execute(
    state: &crate::state::AppState,
    cycle_id: &CycleId,
) -> Result<Profitability, FinanceError> {
    let repo = state.budget_repo.lock().unwrap();
    let budget: Budget = repo
        .all()
        .into_iter()
        .find(|b| b.cycle_id() == cycle_id)
        .ok_or(FinanceError::BudgetNotFound)?;

    Ok(Profitability {
        baseline: money_str(*budget.baseline()),
        spent: money_str(*budget.current_expenses()),
        remaining: budget.get_remaining().map(money_str).unwrap_or_default(),
        variance: budget.get_variance().map(money_str).unwrap_or_default(),
    })
}

fn money_str(m: Money) -> String {
    format!("{} {:?}", m.amount, m.currency)
}
