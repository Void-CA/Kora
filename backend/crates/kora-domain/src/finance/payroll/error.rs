#[derive(Debug, PartialEq)]
pub enum PayrollError {
    EmptyName,
    EmptyWorkerId,
    NonPositiveAmount,
    FuturePayrollDate { now: i64, payroll: i64 },
}
