use kora_domain::finance::error::FinanceError;
use kora_domain::finance::revenue::{Revenue, RevenueSource};
use kora_domain::ports::revenue_repository::RevenueRepository;
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;

use crate::state::AppState;

pub struct RegisterRevenueInput {
    pub cycle_id: Option<CycleId>,
    pub amount: Money,
    pub received_at: i64,
    pub source: RevenueSource,
}

pub fn execute(state: &AppState, input: RegisterRevenueInput) -> Result<Revenue, FinanceError> {
    let revenue = Revenue::new(input.cycle_id, input.amount, input.received_at, input.source)?;
    let saved = revenue.clone();
    state.revenue_repo.lock().unwrap().save(revenue);
    Ok(saved)
}
