use crate::shared_kernel::money::Money;
use std::sync::Arc;
use crate::agriculture::ids::{PlannedActivityId, ActivityRecordId};
use crate::ports::economic_data_provider::EconomicDataProvider;
use crate::ports::budget_repository::BudgetRepository;
use crate::finance::ids::BudgetId;

/// Finance-side adapter that implements agriculture's EconomicDataProvider trait.
/// Bridges bounded contexts without coupling them.
pub struct FinanceEconomicProvider {
    budget_id: BudgetId,
    budget_repo: Arc<dyn BudgetRepository>,
}

impl FinanceEconomicProvider {
    pub fn new(budget_id: BudgetId, budget_repo: Arc<dyn BudgetRepository>) -> Self {
        Self { budget_id, budget_repo }
    }
}

impl EconomicDataProvider for FinanceEconomicProvider {
    fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money> {
        let budget = self.budget_repo.find_by_id(&self.budget_id)?;
        budget.get_planned_cost(planned_id.as_str())
    }

    fn get_actual_cost(&self, record_id: &ActivityRecordId) -> Option<Money> {
        let budget = self.budget_repo.find_by_id(&self.budget_id)?;
        budget.get_actual_cost_for_activity(record_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::in_memory_repositories::InMemoryBudgetRepository;
    use crate::finance::budget::Budget;
    use crate::shared_kernel::ids::CycleId;
    use crate::shared_kernel::period::Period;
    use crate::shared_kernel::money::{Money, Currency, ExchangeRateProvider, RateError};
    use rust_decimal::Decimal;
    use std::sync::Arc;

    struct NoOpProvider;
    impl ExchangeRateProvider for NoOpProvider {
        fn get_rate(&self, _: Currency, _: Currency) -> Result<Decimal, RateError> {
            unreachable!("same-currency test should not call provider")
        }
    }

    #[test]
    fn provider_returns_planned_cost_from_budget() {
        let mut repo = InMemoryBudgetRepository::new();
        let mut budget = Budget::new(
            CycleId::new(),
            Period::new(100, 200).unwrap(),
            Money::new(Decimal::from(1000), Currency::USD),
        );
        let budget_id = budget.id().clone();
        let planned_id = PlannedActivityId::new();
        budget.plan_cost(planned_id.clone(), Money::new(Decimal::from(150), Currency::USD));
        repo.save(budget);

        let provider = FinanceEconomicProvider::new(budget_id, Arc::new(repo));
        let cost = provider.get_planned_cost(&planned_id);

        assert_eq!(cost.unwrap().amount, Decimal::from(150));
    }

    #[test]
    fn provider_returns_actual_cost_from_budget() {
        let mut repo = InMemoryBudgetRepository::new();
        let mut budget = Budget::new(
            CycleId::new(),
            Period::new(100, 200).unwrap(),
            Money::new(Decimal::from(1000), Currency::USD),
        );
        let budget_id = budget.id().clone();
        let record_id = ActivityRecordId::new();
        budget.record_actual_cost(
            record_id.clone(),
            &Money::new(Decimal::from(200), Currency::USD),
            &NoOpProvider,
        ).unwrap();
        repo.save(budget);

        let provider = FinanceEconomicProvider::new(budget_id, Arc::new(repo));
        let cost = provider.get_actual_cost(&record_id);

        assert_eq!(cost.unwrap().amount, Decimal::from(200));
    }

    #[test]
    fn provider_returns_none_on_missing_budget() {
        let repo = InMemoryBudgetRepository::new();
        let provider = FinanceEconomicProvider::new(BudgetId::new(), Arc::new(repo));
        assert!(provider.get_planned_cost(&PlannedActivityId::new()).is_none());
        assert!(provider.get_actual_cost(&ActivityRecordId::new()).is_none());
    }
}
