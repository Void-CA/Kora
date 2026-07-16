use kora_domain::features::employees::employee::{Employee, EmployeeId};
use kora_domain::features::employees::errors::EmployeeError;
use kora_domain::features::employees::repository::EmployeeRepository;
use kora_domain::features::employees::repository::WorkLogRepository;
use kora_domain::features::employees::work_log::WorkLog;

// Placeholder repository — replaced with SQLx implementation in PR 2.
pub struct PgEmployeeRepository;

impl EmployeeRepository for PgEmployeeRepository {
    fn insert(&self, _employee: &Employee) -> Result<(), EmployeeError> {
        Err(EmployeeError::NotFound("not implemented".into()))
    }

    fn list(&self) -> Result<Vec<Employee>, EmployeeError> {
        Ok(vec![])
    }
}

impl WorkLogRepository for PgEmployeeRepository {
    fn insert(&self, _work_log: &WorkLog) -> Result<(), EmployeeError> {
        Err(EmployeeError::NotFound("not implemented".into()))
    }

    fn list_by_employee(&self, _employee_id: EmployeeId) -> Result<Vec<WorkLog>, EmployeeError> {
        Ok(vec![])
    }
}
