// finance/domain/mod.rs
pub mod budget;
pub mod expense;

pub use budget::Budget;
pub use expense::{Expense, ExpenseCategory};
