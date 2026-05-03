// finance/domain/mod.rs
pub mod budget;
pub mod expense;
pub mod ids;

pub use budget::Budget;
pub use expense::{Expense, ExpenseCategory};
pub use ids::{BudgetId, ExpenseId};
