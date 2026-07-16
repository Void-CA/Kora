use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use kora_domain::features::employees::employee::Employee;

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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateWorkLogRequest {
    pub employee_id: Uuid,
    pub worked_on: NaiveDate,
    pub hours: f64,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct WorkLogResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub worked_on: NaiveDate,
    pub hours: f64,
}
