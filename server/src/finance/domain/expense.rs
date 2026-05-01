// finance/domain/expense.rs
use crate::shared_kernel::ids::{BudgetId, ExpenseId};
use crate::shared_kernel::money::Money;

#[derive(Debug, Clone)]
pub enum ExpenseCategory {
    Seeds,
    Fertilizers,
    Labor,
    SoilPrep,
    Other(String),
}

#[derive(Debug)]
pub struct Expense {
    id: ExpenseId,
    budget_id: BudgetId, // Necesitamos importar BudgetId
    amount: Money,
    timestamp: i64,
    category: ExpenseCategory,
}

impl Expense {
    pub fn new(
        budget_id: BudgetId,
        amount: Money,
        timestamp: i64,
        category: ExpenseCategory,
    ) -> Self {
        Self {
            id: ExpenseId(uuid::Uuid::new_v4().to_string()),
            budget_id,
            amount,
            timestamp,
            category,
        }
    }
}
