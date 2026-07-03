use rust_decimal::Decimal;
use kora_kernel::ids::{AreaId, CycleId};
use kora_kernel::money::Money;

use super::error::PayrollError;
use super::worker::{WorkerId, Role};

#[derive(Debug, Clone)]
pub struct PayrollEntry {
    id: String,
    worker_id: WorkerId,
    amount: Money,
    paid_at: i64,
    cycle_id: Option<CycleId>,
    area_id: Option<AreaId>,
    role_at_payment: Option<Role>,
}

impl PayrollEntry {
    pub fn new(
        worker_id: WorkerId,
        amount: Money,
        paid_at: i64,
        cycle_id: Option<CycleId>,
        area_id: Option<AreaId>,
    ) -> Result<Self, PayrollError> {
        if amount.amount <= Decimal::from(0) {
            return Err(PayrollError::NonPositiveAmount);
        }
        if worker_id.0.is_empty() {
            return Err(PayrollError::EmptyWorkerId);
        }
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            worker_id,
            amount,
            paid_at,
            cycle_id,
            area_id,
            role_at_payment: None,
        })
    }

    pub fn with_role_at_payment(mut self, role: Role) -> Self {
        self.role_at_payment = Some(role);
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn worker_id(&self) -> &WorkerId {
        &self.worker_id
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn paid_at(&self) -> i64 {
        self.paid_at
    }

    pub fn cycle_id(&self) -> Option<&CycleId> {
        self.cycle_id.as_ref()
    }

    pub fn area_id(&self) -> Option<&AreaId> {
        self.area_id.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kora_kernel::money::Currency;

    #[test]
    fn entry_creation_with_cycle() {
        let e = PayrollEntry::new(
            WorkerId::new(),
            Money::new(Decimal::from(500), Currency::USD),
            1000,
            Some(CycleId::new()),
            None,
        ).unwrap();
        assert!(e.cycle_id().is_some());
        assert_eq!(e.amount().amount, Decimal::from(500));
    }

    #[test]
    fn entry_with_area_only() {
        let e = PayrollEntry::new(
            WorkerId::new(),
            Money::new(Decimal::from(300), Currency::USD),
            1000,
            None,
            Some(AreaId::new()),
        ).unwrap();
        assert!(e.area_id().is_some());
        assert!(e.cycle_id().is_none());
    }

    #[test]
    fn rejects_zero_amount() {
        let result = PayrollEntry::new(
            WorkerId::new(),
            Money::new(Decimal::from(0), Currency::USD),
            1000,
            None,
            None,
        );
        assert!(matches!(result, Err(PayrollError::NonPositiveAmount)));
    }

    #[test]
    fn rejects_empty_worker_id() {
        let result = PayrollEntry::new(
            WorkerId(String::new()),
            Money::new(Decimal::from(100), Currency::USD),
            1000,
            None,
            None,
        );
        assert!(matches!(result, Err(PayrollError::EmptyWorkerId)));
    }
}
