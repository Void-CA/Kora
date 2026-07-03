use kora_domain::finance::error::FinanceError;
use kora_domain::ports::revenue_repository::RevenueRepository;
use kora_kernel::ids::CycleId;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct Profitability {
    pub baseline: String,
    pub spent: String,
    pub revenue: String,
    pub profit: String,
    pub roi_percent: String,
    pub remaining: String,
    pub variance: String,
}

pub fn execute(
    state: &AppState,
    cycle_id: &CycleId,
) -> Result<Profitability, FinanceError> {
    let budget = {
        let repo = state.budget_repo.lock().unwrap();
        repo.all()
            .into_iter()
            .find(|b| b.cycle_id() == cycle_id)
            .ok_or(FinanceError::BudgetNotFound)?
    };

    let revenues = state.revenue_repo.lock().unwrap().for_cycle(cycle_id);
    let total_revenue = sum_revenues(&revenues);

    let spent = *budget.current_expenses();
    let baseline = *budget.baseline();

    let profit = total_revenue.subtract(&spent).unwrap_or(spent);
    let roi = compute_roi(&total_revenue, &spent);

    Ok(Profitability {
        baseline: money_str(baseline),
        spent: money_str(spent),
        revenue: money_str(total_revenue),
        profit: money_str(profit),
        roi_percent: roi.to_string(),
        remaining: budget.get_remaining().map(money_str).unwrap_or_default(),
        variance: budget.get_variance().map(money_str).unwrap_or_default(),
    })
}

fn money_str(m: kora_kernel::money::Money) -> String {
    format!("{} {:?}", m.amount, m.currency)
}

fn sum_revenues(revenues: &[kora_domain::finance::revenue::Revenue]) -> kora_kernel::money::Money {
    use kora_kernel::money::{Currency, Money};
    let mut acc: Option<Money> = None;
    for r in revenues {
        let amt = *r.amount();
        acc = match acc {
            None => Some(amt),
            Some(a) => a.add(&amt).ok().or(Some(a)),
        };
    }
    acc.unwrap_or_else(|| Money::new(Decimal::from(0), Currency::USD))
}

fn compute_roi(revenue: &kora_kernel::money::Money, spent: &kora_kernel::money::Money) -> Decimal {
    if spent.amount.is_zero() {
        return Decimal::from(0);
    }
    let profit = revenue.amount - spent.amount;
    (profit / spent.amount) * Decimal::from(100)
}
