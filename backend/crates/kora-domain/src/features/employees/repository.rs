use async_trait::async_trait;

use super::employee::{Employee, EmployeeId};
use super::errors::EmployeeError;
use super::payment::{Payment, PaymentId};
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
    async fn list_unpaid_by_employee(
        &self,
        employee_id: EmployeeId,
    ) -> Result<Vec<WorkLog>, EmployeeError>;
    async fn mark_paid(
        &self,
        work_log_ids: &[WorkLogId],
        payment_id: PaymentId,
    ) -> Result<(), EmployeeError>;
}

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn insert(&self, payment: &Payment) -> Result<(), EmployeeError>;
    async fn list_by_employee(
        &self,
        employee_id: EmployeeId,
    ) -> Result<Vec<Payment>, EmployeeError>;
    async fn find_with_work_logs(
        &self,
        payment_id: PaymentId,
    ) -> Result<(Payment, Vec<WorkLog>), EmployeeError>;
}

// Re-export WorkLogId for use in PaymentRepository without circular dependency
pub use super::work_log::WorkLogId;
