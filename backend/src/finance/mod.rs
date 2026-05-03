// finance/mod.rs
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod error;

pub use domain::budget::Budget;
pub use domain::expense::{Expense, ExpenseCategory};
pub use error::FinanceError;
