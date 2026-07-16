use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use kora_domain::features::employees::employee::Employee;
use kora_domain::features::employees::work_log::WorkLog;

// --- Employee ---

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct EmployeeResponse {
    pub id: Uuid,
    pub name: String,
    pub active: bool,
}

impl From<Employee> for EmployeeResponse {
    fn from(e: Employee) -> Self {
        Self {
            id: e.id.0,
            name: e.name,
            active: e.active,
        }
    }
}

// --- WorkLog ---

#[derive(Debug, Deserialize)]
pub struct CreateWorkLogRequest {
    pub employee_id: Uuid,
    pub worked_on: NaiveDate,
    pub hours: f64,
}

#[derive(Debug, Serialize)]
pub struct WorkLogResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub worked_on: NaiveDate,
    pub hours: f64,
}

impl From<WorkLog> for WorkLogResponse {
    fn from(w: WorkLog) -> Self {
        Self {
            id: w.id.0,
            employee_id: w.employee_id.0,
            worked_on: w.worked_on,
            hours: w.hours,
        }
    }
}
