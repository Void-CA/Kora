// finance/domain/budget.rs
use crate::shared_kernel::ids::{BudgetId, CycleId};
use crate::shared_kernel::time::Period;
use crate::shared_kernel::money::{Money, ExchangeRateProvider};
use crate::finance::error::FinanceError;
use std::collections::HashMap;

pub struct Budget {
    id: BudgetId,
    cycle_id: CycleId,
    period: Period,
    baseline: Money,
    current_expenses: Money,
    rate_provider: Box<dyn ExchangeRateProvider>,
    // NEW: Per-activity cost tracking (using String keys for cross-domain compatibility)
    planned_costs: HashMap<String, Money>,      // Key: PlannedActivityId.0 (String)
    actual_costs: HashMap<String, Money>,       // Key: activity record ID (String)
}

impl Budget {
    pub fn new(
        cycle_id: CycleId,
        period: Period,
        baseline: Money,
        rate_provider: Box<dyn ExchangeRateProvider>,
    ) -> Self {
        Self {
            id: BudgetId(uuid::Uuid::new_v4().to_string()),
            cycle_id,
            period,
            baseline,
            current_expenses: Money::new(rust_decimal::Decimal::ZERO, baseline.currency),
            rate_provider,
            planned_costs: HashMap::new(),
            actual_costs: HashMap::new(),
        }
    }

    pub fn register_expense(&mut self, amount: &Money) -> Result<(), FinanceError> {
        // NO bloqueamos si supera el presupuesto (filosofía imperfección controlada)
        self.current_expenses = self.current_expenses.add(amount)
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

    // --- NEW: Per-activity cost tracking ---
    
    /// Sets the planned cost for a specific planned activity (using String key for cross-domain compatibility).
    pub fn set_planned_cost(&mut self, planned_id: String, cost: Money) {
        self.planned_costs.insert(planned_id, cost);
    }

    /// Registers an actual expense and associates it with an activity record ID.
    /// Also updates current_expenses (for backward compatibility).
    pub fn register_expense_for_activity(&mut self, amount: &Money, activity_record_id: String) -> Result<(), FinanceError> {
        self.actual_costs.insert(activity_record_id, *amount);
        self.current_expenses = self.current_expenses.add(amount)
            .map_err(|e| FinanceError::RateError(e))?;
        Ok(())
    }

    /// Gets the planned cost for a specific planned activity.
    pub fn get_planned_cost(&self, planned_id: &str) -> Option<Money> {
        self.planned_costs.get(planned_id).copied()
    }

    /// Gets the actual cost for a specific activity record.
    pub fn get_actual_cost_for_activity(&self, activity_record_id: &str) -> Option<Money> {
        self.actual_costs.get(activity_record_id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared_kernel::ids::CycleId;
    use crate::shared_kernel::time::Period;
    use crate::shared_kernel::money::{Money, Currency, ExchangeRateProvider};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    struct MockRateProvider;
    impl ExchangeRateProvider for MockRateProvider {
        fn get_rate(&self, from: Currency, to: Currency) -> Result<Decimal, crate::shared_kernel::money::RateError> {
            match (from, to) {
                (Currency::USD, Currency::NIO) => Ok(Decimal::from(36)),
                (Currency::NIO, Currency::USD) => Ok(Decimal::from_str("0.0277").unwrap()),
                _ => Err(crate::shared_kernel::money::RateError::RateNotAvailable(from, to)),
            }
        }
    }

    fn create_test_budget() -> Budget {
        let period = Period::new(1000, 2000).unwrap();
        let baseline = Money::new(Decimal::from(1000), Currency::USD);
        let provider = Box::new(MockRateProvider);
        Budget::new(CycleId("cycle-1".to_string()), period, baseline, provider)
    }

    #[test]
    fn budget_new_initializes_correctly() {
        let budget = create_test_budget();
        assert_eq!(budget.current_expenses().amount, Decimal::ZERO);
        assert_eq!(budget.baseline().amount, Decimal::from(1000));
    }

    #[test]
    fn register_expense_updates_current() {
        let mut budget = create_test_budget();
        let expense = Money::new(Decimal::from(200), Currency::USD);
        let result = budget.register_expense(&expense);
        assert!(result.is_ok());
        assert_eq!(budget.current_expenses().amount, Decimal::from(200));
    }

    #[test]
    fn register_expense_exceeding_budget_no_block() {
        let mut budget = create_test_budget(); // baseline = 1000 USD
        let big_expense = Money::new(Decimal::from(1500), Currency::USD); // exceeds baseline
        let result = budget.register_expense(&big_expense);
        assert!(result.is_ok()); // NO error! (imperfection-controlled)
        assert_eq!(budget.current_expenses().amount, Decimal::from(1500));
    }

    #[test]
    fn get_remaining_under_budget() {
        let mut budget = create_test_budget(); // baseline = 1000
        let expense = Money::new(Decimal::from(300), Currency::USD);
        budget.register_expense(&expense).unwrap();
        let remaining = budget.get_remaining().unwrap();
        assert_eq!(remaining.amount, Decimal::from(700)); // 1000 - 300
    }

    #[test]
    fn get_variance_over_budget() {
        let mut budget = create_test_budget(); // baseline = 1000
        let expense = Money::new(Decimal::from(1200), Currency::USD);
        budget.register_expense(&expense).unwrap();
        let variance = budget.get_variance().unwrap();
        assert_eq!(variance.amount, Decimal::from(200)); // 1200 - 1000 = 200 over
    }

    // Test 4.7: FinanceEconomicProvider integrates with Budget
    #[test]
    fn finance_economic_provider_integrates_with_budget() {
        use crate::agriculture::domain::services::economic_variance::EconomicDataProvider;
        use crate::finance::domain::adapters::FinanceEconomicProvider;
        use crate::agriculture::domain::PlannedActivityId;

        let period = Period::new(1000, 2000).unwrap();
        let baseline = Money::new(Decimal::from(1000), Currency::USD);
        let provider = Box::new(MockRateProvider);
        let mut budget = Budget::new(CycleId("cycle-1".to_string()), period, baseline, provider);

        // Set planned cost for an activity (using String key)
        let planned_id = PlannedActivityId("p1".to_string());
        let planned_cost = Money::new(Decimal::from(100), Currency::USD);
        budget.set_planned_cost(planned_id.0.clone(), planned_cost);

        // Register actual expense for an activity record
        let activity_record_id = "rec-1".to_string();
        let actual_cost = Money::new(Decimal::from(120), Currency::USD);
        budget.register_expense_for_activity(&actual_cost, activity_record_id.clone()).unwrap();

        // Create FinanceEconomicProvider with this budget
        let finance_provider = FinanceEconomicProvider::new(budget);

        // Test: get_planned_cost
        let result_planned = finance_provider.get_planned_cost(&planned_id);
        assert!(result_planned.is_some());
        assert_eq!(result_planned.unwrap().amount, Decimal::from(100));

        // Test: get_actual_cost
        let result_actual = finance_provider.get_actual_cost(&activity_record_id);
        assert!(result_actual.is_some());
        assert_eq!(result_actual.unwrap().amount, Decimal::from(120));
    }
}
