use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmployeeError {
    #[error("employee not found: {0}")]
    NotFound(String),

    #[error("employee name cannot be empty")]
    EmptyName,

    #[error("hours must be greater than zero")]
    InvalidHours,

    #[error("payment period is invalid: from must be before to")]
    InvalidPaymentPeriod,
}
