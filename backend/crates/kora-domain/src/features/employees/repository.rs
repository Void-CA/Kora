use async_trait::async_trait;

use super::employee::{Employee, EmployeeId};
use super::errors::EmployeeError;
use super::work_log::WorkLog;

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn insert(&self, employee: &Employee) -> Result<(), EmployeeError>;
    async fn list(&self) -> Result<Vec<Employee>, EmployeeError>;
}

#[async_trait]
pub trait WorkLogRepository: Send + Sync {
    async fn insert(&self, work_log: &WorkLog) -> Result<(), EmployeeError>;
    async fn list_by_employee(
        &self,
        employee_id: EmployeeId,
    ) -> Result<Vec<WorkLog>, EmployeeError>;
}
