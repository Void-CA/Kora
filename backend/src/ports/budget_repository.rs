use crate::finance::budget::Budget;
use crate::finance::ids::BudgetId;

pub trait BudgetRepository {
    fn find_by_id(&self, id: &BudgetId) -> Option<Budget>;
    fn save(&mut self, budget: Budget);
}
