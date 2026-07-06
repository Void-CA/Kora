use kora_domain::finance::error::FinanceError;
use kora_domain::finance::revenue::{Revenue, RevenueSource};
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;
use crate::state::AppState;
use crate::features::finance::dto::RevenueSummary;

pub struct RegisterRevenueInput {
    pub cycle_id: Option<CycleId>,
    pub amount: Money,
    pub received_at: i64,
    pub source: RevenueSource,
}

pub fn register(state: &AppState, input: RegisterRevenueInput) -> Result<Revenue, FinanceError> {
    let revenue = Revenue::new(input.cycle_id, input.amount, input.received_at, input.source)?;
    let saved = revenue.clone();
    state.revenue_repo.lock().unwrap().save(revenue);
    Ok(saved)
}

pub fn list_for_cycle(state: &AppState, cycle_id: &CycleId) -> Vec<RevenueSummary> {
    state.revenue_repo.lock().unwrap().for_cycle(cycle_id).iter().map(|r| RevenueSummary {
        id: r.id().0.clone(),
        cycle_id: r.cycle_id().map(|c| c.0.clone()),
        amount: format!("{} {:?}", r.amount().amount, r.amount().currency),
        received_at: r.received_at(),
        source: format!("{:?}", r.source()),
    }).collect()
}
