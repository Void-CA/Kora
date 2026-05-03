// finance/domain/budget.rs
use crate::agriculture::domain::ids::{PlannedActivityId, ActivityRecordId};
use crate::shared_kernel::ids::CycleId;
use crate::shared_kernel::time::Period;
use crate::shared_kernel::money::{Money, Currency, ExchangeRateProvider};
use crate::finance::application::ports::budget_repository::BudgetRepository;
use crate::finance::error::FinanceError;
use crate::finance::domain::ids::BudgetId;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Budget {
    id: BudgetId,
    cycle_id: CycleId,
    period: Period,
    baseline: Money,
    current_expenses: Money,
    // Per-activity cost tracking with proper typed IDs (from shared_kernel)
    planned_costs: HashMap<String, Money>,
    actual_costs: HashMap<String, Money>,
}

impl Budget {
    pub fn new(
        cycle_id: CycleId,
        period: Period,
        baseline: Money,
    ) -> Self {
        Self {
            id: BudgetId(uuid::Uuid::new_v4().to_string()),
            cycle_id,
            period,
            baseline,
            current_expenses: Money::new(rust_decimal::Decimal::ZERO, baseline.currency),
            planned_costs: HashMap::new(),
            actual_costs: HashMap::new(),
        }
    }

    /// Register a general expense (not linked to a specific activity).
    /// Does NOT block if it exceeds the budget (imperfection-controlled philosophy).
    pub fn register_expense(
        &mut self,
        amount: &Money,
        rate_provider: &dyn ExchangeRateProvider,
    ) -> Result<(), FinanceError> {
        let converted = if amount.currency != self.baseline.currency {
            amount.convert_to(self.baseline.currency, rate_provider)
                .map_err(|e| FinanceError::RateError(e))?
        } else {
            amount.clone()
        };
        
        self.current_expenses = self.current_expenses.add(&converted)
            .map_err(|e| FinanceError::RateError(e))?;
        Ok(())
    }

    /// Plan a cost for a specific activity (before execution).
    pub fn plan_cost(&mut self, activity_id: PlannedActivityId, amount: Money) {
        self.planned_costs.insert(activity_id.as_str().to_string(), amount);
    }

    /// Record an actual cost for a specific activity record.
    /// Also updates current_expenses (with currency conversion if needed).
    pub fn record_actual_cost(
        &mut self,
        record_id: ActivityRecordId,
        amount: &Money,
        rate_provider: &dyn ExchangeRateProvider,
    ) -> Result<(), FinanceError> {
        let converted = if amount.currency != self.baseline.currency {
            amount.convert_to(self.baseline.currency, rate_provider)
                .map_err(|e| FinanceError::RateError(e))?
        } else {
            amount.clone()
        };
        
        self.actual_costs.insert(record_id.as_str().to_string(), converted.clone());
        self.current_expenses = self.current_expenses.add(&converted)
            .map_err(|e| FinanceError::RateError(e))?;
        Ok(())
    }

    pub fn get_remaining(&self) -> Result<Money, FinanceError> {
        self.baseline.subtract(&self.current_expenses)
            .map_err(|e| FinanceError::RateError(e))
    }

    pub fn get_variance(&self) -> Result<Money, FinanceError> {
        // Variance = current - baseline (positive = over budget)
        self.current_expenses.subtract(&self.baseline)
            .map_err(|e| FinanceError::RateError(e))
    }

    pub fn id(&self) -> &BudgetId {
        &self.id
    }

    pub fn cycle_id(&self) -> &CycleId {
        &self.cycle_id
    }

    pub fn period(&self) -> &Period {
        &self.period
    }

    pub fn baseline(&self) -> &Money {
        &self.baseline
    }

    pub fn current_expenses(&self) -> &Money {
        &self.current_expenses
    }

    /// Gets the planned cost for a specific planned activity.
    pub fn get_planned_cost(&self, planned_id: &str) -> Option<Money> {
        self.planned_costs.get(planned_id).cloned()
    }

    /// Gets the actual cost for a specific activity record.
    pub fn get_actual_cost_for_activity(&self, record_id: &str) -> Option<Money> {
        self.actual_costs.get(record_id).cloned()
    }
}
