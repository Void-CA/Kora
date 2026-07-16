use chrono::NaiveDate;

use super::employee::{Employee, EmployeeId};
use super::errors::EmployeeError;
use super::repository::{EmployeeRepository, WorkLogRepository};
use super::work_log::{WorkLog, WorkLogId};

pub async fn create_employee(
    repo: &dyn EmployeeRepository,
    name: String,
) -> Result<Employee, EmployeeError> {
    if name.trim().is_empty() {
        return Err(EmployeeError::EmptyName);
    }

    let employee = Employee {
        id: EmployeeId::new(),
        name: name.trim().to_string(),
        active: true,
    };

    repo.insert(&employee).await?;
    Ok(employee)
}

pub async fn list_employees(
    repo: &dyn EmployeeRepository,
) -> Result<Vec<Employee>, EmployeeError> {
    repo.list().await
}

pub async fn register_work_log(
    repo: &dyn WorkLogRepository,
    employee_id: EmployeeId,
    hours: f64,
    worked_on: NaiveDate,
) -> Result<WorkLog, EmployeeError> {
    if hours <= 0.0 {
        return Err(EmployeeError::InvalidHours);
    }

    let work_log = WorkLog {
        id: WorkLogId::new(),
        employee_id,
        worked_on,
        hours,
    };

    repo.insert(&work_log).await?;
    Ok(work_log)
}

pub async fn list_work_logs(
    repo: &dyn WorkLogRepository,
    employee_id: EmployeeId,
) -> Result<Vec<WorkLog>, EmployeeError> {
    repo.list_by_employee(employee_id).await
}
