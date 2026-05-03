// finance/error.rs
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
