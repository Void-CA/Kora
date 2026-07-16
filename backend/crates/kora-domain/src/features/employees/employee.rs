use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    pub id: EmployeeId,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct EmployeeId(pub Uuid);

impl EmployeeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EmployeeId {
    fn default() -> Self {
        Self::new()
    }
}
