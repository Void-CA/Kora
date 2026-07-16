use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::employee::EmployeeId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkLog {
    pub id: WorkLogId,
    pub employee_id: EmployeeId,
    pub worked_on: NaiveDate,
    pub hours: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct WorkLogId(pub Uuid);

impl WorkLogId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for WorkLogId {
    fn default() -> Self {
        Self::new()
    }
}
