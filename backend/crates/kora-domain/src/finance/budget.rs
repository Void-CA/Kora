use crate::agriculture::ids::{PlannedActivityId, ActivityRecordId};
use crate::finance::expense::ExpenseCategory;
use kora_kernel::ids::CycleId;
use kora_kernel::period::Period;
use kora_kernel::money::{Money, ExchangeRateProvider};
use crate::finance::error::FinanceError;
use crate::finance::ids::BudgetId;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BudgetCategory {
    Seeds,
    Fertilizers,
    Labor,
    SoilPrep,
    Other(String),
}

impl From<ExpenseCategory> for BudgetCategory {
    fn from(e: ExpenseCategory) -> Self {
        match e {
            ExpenseCategory::Seeds => BudgetCategory::Seeds,
            ExpenseCategory::Fertilizers => BudgetCategory::Fertilizers,
            ExpenseCategory::Labor => BudgetCategory::Labor,
            ExpenseCategory::SoilPrep => BudgetCategory::SoilPrep,
            ExpenseCategory::Other(s) => BudgetCategory::Other(s),
        }
    }
}

#[derive(Clone)]
pub struct Budget {
    id: BudgetId,
    cycle_id: CycleId,
    period: Period,
    baseline: Money,
    current_expenses: Money,
    estimated_lines: HashMap<BudgetCategory, Money>,
    planned_costs: HashMap<String, Money>,
    actual_costs: HashMap<String, Money>,
}

impl Budget {
    pub fn new(cycle_id: CycleId, period: Period, baseline: Money) -> Self {
        Self {
            id: BudgetId(uuid::Uuid::new_v4().to_string()),
            cycle_id,
            period,
            baseline,
            current_expenses: Money::new(rust_decimal::Decimal::ZERO, baseline.currency),
            estimated_lines: HashMap::new(),
            planned_costs: HashMap::new(),
            actual_costs: HashMap::new(),
        }
    }

    pub fn register_expense(&mut self, amount: &Money, rate_provider: &dyn ExchangeRateProvider) -> Result<(), FinanceError> {
        let converted = if amount.currency != self.baseline.currency {
            amount.convert_to(self.baseline.currency, rate_provider)
                .map_err(FinanceError::RateError)?
        } else {
            amount.clone()
        };
        self.current_expenses = self.current_expenses.add(&converted)
            .map_err(FinanceError::RateError)?;
        Ok(())
    }

    pub fn plan_cost(&mut self, activity_id: PlannedActivityId, amount: Money) {
        self.planned_costs.insert(activity_id.as_str().to_string(), amount);
    }

    pub fn record_actual_cost(&mut self, record_id: ActivityRecordId, amount: &Money, rate_provider: &dyn ExchangeRateProvider) -> Result<(), FinanceError> {
        let converted = if amount.currency != self.baseline.currency {
            amount.convert_to(self.baseline.currency, rate_provider)
                .map_err(FinanceError::RateError)?
        } else {
            amount.clone()
        };
        self.actual_costs.insert(record_id.as_str().to_string(), converted.clone());
        self.current_expenses = self.current_expenses.add(&converted)
            .map_err(FinanceError::RateError)?;
        Ok(())
    }

    pub fn estimate_category(&mut self, category: BudgetCategory, amount: Money) -> Result<(), FinanceError> {
        if amount.currency != self.baseline.currency {
            return Err(FinanceError::RateError(kora_kernel::money::RateError::CurrencyMismatch(
                amount.currency, self.baseline.currency,
            )));
        }
        let entry = self.estimated_lines.entry(category).or_insert_with(|| {
            Money::new(rust_decimal::Decimal::ZERO, self.baseline.currency)
        });
        *entry = entry.add(&amount).map_err(FinanceError::RateError)?;
        Ok(())
    }

    pub fn get_remaining(&self) -> Result<Money, FinanceError> {
        self.baseline.subtract(&self.current_expenses).map_err(FinanceError::RateError)
    }

    pub fn get_variance(&self) -> Result<Money, FinanceError> {
        self.current_expenses.subtract(&self.baseline).map_err(FinanceError::RateError)
    }

    pub fn id(&self) -> &BudgetId { &self.id }
    pub fn cycle_id(&self) -> &CycleId { &self.cycle_id }
    pub fn period(&self) -> &Period { &self.period }
    pub fn baseline(&self) -> &Money { &self.baseline }
    pub fn current_expenses(&self) -> &Money { &self.current_expenses }
    pub fn estimated_lines(&self) -> &HashMap<BudgetCategory, Money> { &self.estimated_lines }

    pub fn get_planned_cost(&self, planned_id: &str) -> Option<Money> {
        self.planned_costs.get(planned_id).cloned()
    }

    pub fn get_actual_cost_for_activity(&self, record_id: &str) -> Option<Money> {
        self.actual_costs.get(record_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    use kora_kernel::money::Currency;

    struct MockRate;
    impl ExchangeRateProvider for MockRate {
        fn get_rate(&self, from: Currency, to: Currency) -> Result<rust_decimal::Decimal, kora_kernel::money::RateError> {
            match (from, to) {
                (Currency::USD, Currency::NIO) => Ok(rust_decimal::Decimal::from(36)),
                (Currency::NIO, Currency::USD) => Ok(rust_decimal::Decimal::from_str("0.0277").unwrap()),
                _ => Err(kora_kernel::money::RateError::RateNotAvailable(from, to)),
            }
        }
    }

    #[test]
    fn budget_tracks_expenses() {
        let period = Period::new(1000, 2000).unwrap();
        let mut b = Budget::new(CycleId::new(), period, Money::new(rust_decimal::Decimal::from(1000), Currency::USD));
        b.register_expense(&Money::new(rust_decimal::Decimal::from(200), Currency::USD), &MockRate).unwrap();
        assert_eq!(b.current_expenses().amount, rust_decimal::Decimal::from(200));
    }

    #[test]
    fn budget_no_block_on_overage() {
        let period = Period::new(1000, 2000).unwrap();
        let mut b = Budget::new(CycleId::new(), period, Money::new(rust_decimal::Decimal::from(1000), Currency::USD));
        assert!(b.register_expense(&Money::new(rust_decimal::Decimal::from(1500), Currency::USD), &MockRate).is_ok());
    }

    #[test]
    fn budget_variance_positive_is_overage() {
        let period = Period::new(1000, 2000).unwrap();
        let mut b = Budget::new(CycleId::new(), period, Money::new(rust_decimal::Decimal::from(1000), Currency::USD));
        b.register_expense(&Money::new(rust_decimal::Decimal::from(1200), Currency::USD), &MockRate).unwrap();
        assert_eq!(b.get_variance().unwrap().amount, rust_decimal::Decimal::from(200));
    }

    #[test]
    fn budget_plan_and_actual_costs() {
        let period = Period::new(1000, 2000).unwrap();
        let mut b = Budget::new(CycleId::new(), period, Money::new(rust_decimal::Decimal::from(1000), Currency::USD));
        let pid = PlannedActivityId::new();
        b.plan_cost(pid.clone(), Money::new(rust_decimal::Decimal::from(100), Currency::USD));
        assert_eq!(b.get_planned_cost(pid.as_str()).unwrap().amount, rust_decimal::Decimal::from(100));

        let rid = ActivityRecordId::new();
        b.record_actual_cost(rid.clone(), &Money::new(rust_decimal::Decimal::from(120), Currency::USD), &MockRate).unwrap();
        assert_eq!(b.get_actual_cost_for_activity(rid.as_str()).unwrap().amount, rust_decimal::Decimal::from(120));
    }

    #[test]
    fn budget_estimate_category_accumulates() {
        let period = Period::new(1000, 2000).unwrap();
        let mut b = Budget::new(CycleId::new(), period, Money::new(rust_decimal::Decimal::from(5000), Currency::USD));
        b.estimate_category(BudgetCategory::Seeds, Money::new(rust_decimal::Decimal::from(500), Currency::USD)).unwrap();
        b.estimate_category(BudgetCategory::Seeds, Money::new(rust_decimal::Decimal::from(300), Currency::USD)).unwrap();
        b.estimate_category(BudgetCategory::Fertilizers, Money::new(rust_decimal::Decimal::from(800), Currency::USD)).unwrap();
        let lines = b.estimated_lines();
        assert_eq!(lines.get(&BudgetCategory::Seeds).unwrap().amount, rust_decimal::Decimal::from(800));
        assert_eq!(lines.get(&BudgetCategory::Fertilizers).unwrap().amount, rust_decimal::Decimal::from(800));
    }

    #[test]
    fn budget_estimate_rejects_currency_mismatch() {
        let period = Period::new(1000, 2000).unwrap();
        let mut b = Budget::new(CycleId::new(), period, Money::new(rust_decimal::Decimal::from(5000), Currency::USD));
        let result = b.estimate_category(BudgetCategory::Seeds, Money::new(rust_decimal::Decimal::from(100), Currency::NIO));
        assert!(result.is_err());
    }

    #[test]
    fn budget_category_from_expense_category() {
        let from_seeds: BudgetCategory = ExpenseCategory::Seeds.into();
        assert_eq!(from_seeds, BudgetCategory::Seeds);
        let from_other: BudgetCategory = ExpenseCategory::Other("Pesticidas".into()).into();
        assert_eq!(from_other, BudgetCategory::Other("Pesticidas".into()));
    }
}
