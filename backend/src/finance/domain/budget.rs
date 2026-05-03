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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use crate::shared_kernel::ids::CycleId;
    use crate::shared_kernel::time::Period;
    use crate::shared_kernel::money::{Money, Currency, ExchangeRateProvider};
    use crate::agriculture::domain::ids::{PlannedActivityId, ActivityRecordId};

    struct MockRateProvider;
    impl ExchangeRateProvider for MockRateProvider {
        fn get_rate(&self, from: Currency, to: Currency) -> Result<rust_decimal::Decimal, crate::shared_kernel::money::RateError> {
            match (from, to) {
                (Currency::USD, Currency::NIO) => Ok(rust_decimal::Decimal::from(36)),
                (Currency::NIO, Currency::USD) => Ok(rust_decimal::Decimal::from_str("0.0277").unwrap()),
                _ => Err(crate::shared_kernel::money::RateError::RateNotAvailable(from, to)),
            }
        }
    }

    fn create_test_budget() -> Budget {
        let period = Period::new(1000, 2000).unwrap();
        let baseline = Money::new(rust_decimal::Decimal::from(1000), Currency::USD);
        Budget::new(CycleId::new(), period, baseline)
    }

    #[test]
    fn budget_new_initializes_correctly() {
        let budget = create_test_budget();
        assert_eq!(budget.current_expenses().amount, rust_decimal::Decimal::ZERO);
        assert_eq!(budget.baseline().amount, rust_decimal::Decimal::from(1000));
    }

    #[test]
    fn register_expense_updates_current() {
        let mut budget = create_test_budget();
        let expense = Money::new(rust_decimal::Decimal::from(200), Currency::USD);
        let result = budget.register_expense(&expense, &MockRateProvider);
        assert!(result.is_ok());
        assert_eq!(budget.current_expenses().amount, rust_decimal::Decimal::from(200));
    }

    #[test]
    fn register_expense_exceeding_budget_no_block() {
        let mut budget = create_test_budget();
        let big_expense = Money::new(rust_decimal::Decimal::from(1500), Currency::USD);
        let result = budget.register_expense(&big_expense, &MockRateProvider);
        assert!(result.is_ok());
        assert_eq!(budget.current_expenses().amount, rust_decimal::Decimal::from(1500));
    }

    #[test]
    fn get_remaining_under_budget() {
        let mut budget = create_test_budget();
        let expense = Money::new(rust_decimal::Decimal::from(300), Currency::USD);
        budget.register_expense(&expense, &MockRateProvider).unwrap();
        let remaining = budget.get_remaining().unwrap();
        assert_eq!(remaining.amount, rust_decimal::Decimal::from(700));
    }

    #[test]
    fn get_variance_over_budget() {
        let mut budget = create_test_budget();
        let expense = Money::new(rust_decimal::Decimal::from(1200), Currency::USD);
        budget.register_expense(&expense, &MockRateProvider).unwrap();
        let variance = budget.get_variance().unwrap();
        assert_eq!(variance.amount, rust_decimal::Decimal::from(200));
    }

    #[test]
    fn plan_cost_stores_planned_activity() {
        let mut budget = create_test_budget();
        let planned_id = PlannedActivityId::new();
        let cost = Money::new(rust_decimal::Decimal::from(100), Currency::USD);
        budget.plan_cost(planned_id.clone(), cost);
        
        let retrieved = budget.get_planned_cost(planned_id.as_str());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().amount, rust_decimal::Decimal::from(100));
    }

    #[test]
    fn record_actual_cost_stores_activity_record() {
        let mut budget = create_test_budget();
        let record_id = ActivityRecordId::new();
        let cost = Money::new(rust_decimal::Decimal::from(120), Currency::USD);
        budget.record_actual_cost(record_id.clone(), &cost, &MockRateProvider).unwrap();
        
        let retrieved = budget.get_actual_cost_for_activity(record_id.as_str());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().amount, rust_decimal::Decimal::from(120));
    }
}
