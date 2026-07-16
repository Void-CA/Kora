use super::employee::{Employee, EmployeeId};
use super::errors::EmployeeError;
use super::work_log::WorkLog;

pub trait EmployeeRepository: Send + Sync {
    fn insert(&self, employee: &Employee) -> Result<(), EmployeeError>;
    fn list(&self) -> Result<Vec<Employee>, EmployeeError>;
}

pub trait WorkLogRepository: Send + Sync {
    fn insert(&self, work_log: &WorkLog) -> Result<(), EmployeeError>;
    fn list_by_employee(&self, employee_id: EmployeeId) -> Result<Vec<WorkLog>, EmployeeError>;
}
