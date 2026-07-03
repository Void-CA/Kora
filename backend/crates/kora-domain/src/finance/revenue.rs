use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;

use super::error::FinanceError;
use super::ids::RevenueId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevenueSource {
    Harvest,
    Sale,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Revenue {
    id: RevenueId,
    cycle_id: Option<CycleId>,
    amount: Money,
    received_at: i64,
    source: RevenueSource,
}

impl Revenue {
    pub fn new(
        cycle_id: Option<CycleId>,
        amount: Money,
        received_at: i64,
        source: RevenueSource,
    ) -> Result<Self, FinanceError> {
        if amount.amount <= rust_decimal::Decimal::from(0) {
            return Err(FinanceError::NonPositiveRevenue);
        }
        Ok(Self {
            id: RevenueId::new(),
            cycle_id,
            amount,
            received_at,
            source,
        })
    }

    pub fn id(&self) -> &RevenueId {
        &self.id
    }

    pub fn cycle_id(&self) -> Option<&CycleId> {
        self.cycle_id.as_ref()
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn received_at(&self) -> i64 {
        self.received_at
    }

    pub fn source(&self) -> &RevenueSource {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kora_kernel::money::Currency;
    use rust_decimal::Decimal;

    #[test]
    fn revenue_with_positive_amount_succeeds() {
        let r = Revenue::new(
            Some(CycleId::new()),
            Money::new(Decimal::from(1500), Currency::USD),
            1000,
            RevenueSource::Harvest,
        ).unwrap();
        assert_eq!(r.amount().amount, Decimal::from(1500));
    }

    #[test]
    fn revenue_with_zero_amount_fails() {
        let result = Revenue::new(
            Some(CycleId::new()),
            Money::new(Decimal::from(0), Currency::USD),
            1000,
            RevenueSource::Harvest,
        );
        assert!(matches!(result, Err(FinanceError::NonPositiveRevenue)));
    }

    #[test]
    fn revenue_can_be_unattached_to_cycle() {
        let r = Revenue::new(
            None,
            Money::new(Decimal::from(100), Currency::USD),
            1000,
            RevenueSource::Sale,
        ).unwrap();
        assert!(r.cycle_id().is_none());
    }
}
