// finance/domain/adapters/agriculture_economic_provider.rs
use crate::shared_kernel::money::Money;
use crate::agriculture::domain::PlannedActivityId;
use crate::agriculture::domain::services::economic_variance::EconomicDataProvider;
use crate::finance::domain::budget::Budget;

/// Finance-side adapter that implements agriculture's EconomicDataProvider trait.
/// This allows agriculture's EconomicVarianceService to pull cost data from finance
/// without agriculture depending on finance's internals.
///
/// This implementation uses Budget's per-activity cost tracking API.
pub struct FinanceEconomicProvider {
    budget: Budget,
}

impl FinanceEconomicProvider {
    pub fn new(budget: Budget) -> Self {
        Self { budget }
    }
}

impl EconomicDataProvider for FinanceEconomicProvider {
    fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money> {
        // Delegate to Budget's per-activity cost tracking
        self.budget.get_planned_cost(&planned_id.0)
    }

    fn get_actual_cost(&self, record_id: &str) -> Option<Money> {
        // Delegate to Budget's per-activity cost tracking
        self.budget.get_actual_cost_for_activity(record_id)
    }
}
