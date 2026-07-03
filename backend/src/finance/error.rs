use crate::shared_kernel::money::RateError;

#[derive(Debug, PartialEq)]
pub enum FinanceError {
    BudgetNotFound,
    InsufficientFunds,
    RateError(RateError),
}

impl From<RateError> for FinanceError {
    fn from(err: RateError) -> Self {
        FinanceError::RateError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared_kernel::money::RateError;
    use crate::shared_kernel::money::Currency;

    #[test]
    fn from_rate_error_converts_correctly() {
        let err = RateError::CurrencyMismatch(Currency::USD, Currency::NIO);
        let finance_err: FinanceError = err.into();
        assert!(matches!(finance_err, FinanceError::RateError(_)));
    }

    #[test]
    fn finance_error_equality() {
        assert_eq!(FinanceError::BudgetNotFound, FinanceError::BudgetNotFound);
        assert_ne!(FinanceError::BudgetNotFound, FinanceError::InsufficientFunds);
    }
}
