// finance/domain/budget.rs
use crate::shared_kernel::ids::{BudgetId, CycleId};
use crate::shared_kernel::time::Period;
use crate::shared_kernel::money::{Money, ExchangeRateProvider};
use crate::finance::error::FinanceError;

pub struct Budget {
    id: BudgetId,
    cycle_id: CycleId,
    period: Period,
    baseline: Money,
    current_expenses: Money,
    rate_provider: Box<dyn ExchangeRateProvider>,
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
}
