// finance/domain/ids.rs
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BudgetId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpenseId(pub String);

impl BudgetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ExpenseId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
