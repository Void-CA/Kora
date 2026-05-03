// finance/infrastructure/adapters/agriculture_economic_provider.rs
use std::sync::Arc;
use crate::shared_kernel::money::Money;
use crate::shared_kernel::ids::{PlannedActivityId, ActivityRecordId};
use crate::agriculture::domain::services::economic_variance::EconomicDataProvider;
use crate::finance::application::ports::budget_repository::BudgetRepository;
use crate::shared_kernel::ids::BudgetId;

/// Finance-side adapter that implements agriculture's EconomicDataProvider trait.
/// This allows agriculture's EconomicVarianceService to pull cost data from finance
/// without agriculture depending on finance's internals.
///
/// IMPORTANT: This does NOT hold Budget directly. It uses BudgetId + BudgetRepository
/// to resolve data, following the rule: "Adapter must NOT expose provider's internal entities"
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
        // Resolve Budget by ID using repository (no direct exposure of Budget entity)
        let budget = self.budget_repo.find_by_id(&self.budget_id)?;
        budget.get_planned_cost(planned_id)
    }

    fn get_actual_cost(&self, record_id: &ActivityRecordId) -> Option<Money> {
        let budget = self.budget_repo.find_by_id(&self.budget_id)?;
        budget.get_actual_cost_for_activity(record_id)
    }
}
