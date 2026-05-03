// finance/application/ports/budget_repository.rs
use crate::finance::domain::budget::Budget;
use crate::shared_kernel::ids::BudgetId;

/// Port: Repository interface for Budget persistence
pub trait BudgetRepository {
    fn find_by_id(&self, id: &BudgetId) -> Option<Budget>;
    fn save(&mut self, budget: Budget);
}

// In-memory implementation for testing
pub struct InMemoryBudgetRepository {
    budgets: std::collections::HashMap<String, Budget>,
}

impl InMemoryBudgetRepository {
    pub fn new() -> Self {
        Self {
            budgets: std::collections::HashMap::new(),
        }
    }
}

impl BudgetRepository for InMemoryBudgetRepository {
    fn find_by_id(&self, id: &BudgetId) -> Option<Budget> {
        self.budgets.get(&id.0).cloned()
    }

    fn save(&mut self, budget: Budget) {
        self.budgets.insert(budget.id().0.clone(), budget);
    }
}
