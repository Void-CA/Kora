use kora_domain::finance::error::FinanceError;
use kora_domain::finance::expense::{Expense, ExpenseCategory};
use kora_domain::finance::budget::Budget;
use kora_domain::finance::ids::BudgetId;
use kora_kernel::money::{Money, ExchangeRateProvider};

pub struct RegisterExpenseInput {
    pub budget_id: BudgetId,
    pub amount: Money,
    pub timestamp: i64,
    pub category: ExpenseCategory,
    pub rate_provider: Box<dyn ExchangeRateProvider>,
}

pub struct RegisterExpenseOutput {
    pub expense: Expense,
    pub remaining: Money,
    pub variance: Money,
}

pub fn execute(
    state: &crate::state::AppState,
    input: RegisterExpenseInput,
) -> Result<RegisterExpenseOutput, FinanceError> {
    let mut repo = state.budget_repo.lock().unwrap();
    let mut budget: Budget = repo
        .find_by_id(&input.budget_id)
        .ok_or(FinanceError::BudgetNotFound)?;

    budget.register_expense(&input.amount, &*input.rate_provider)?;

    let expense = Expense::new(input.budget_id.clone(), input.amount, input.timestamp, input.category);
    let remaining = budget.get_remaining()?;
    let variance = budget.get_variance()?;

    repo.save(budget);
    Ok(RegisterExpenseOutput { expense, remaining, variance })
}
