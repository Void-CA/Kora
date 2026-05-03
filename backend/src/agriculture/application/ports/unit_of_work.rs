// agriculture/application/ports/unit_of_work.rs
// Simplified for Phase 2 - will add sqlx later in infrastructure

/// UnitOfWork trait for transactional consistency
/// Lives in application ports, NOT domain (it's a technical detail)
pub trait UnitOfWork {
    /// Execute a block of work
    /// F: closure that returns a Result<T>
    fn execute<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>;
}

// Mock UnitOfWork for testing
pub struct MockUnitOfWork;

impl UnitOfWork for MockUnitOfWork {
    fn execute<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>
    {
        // Just execute the closure (no real transaction in mock)
        f()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn unit_of_work_executes_closure() {
        let uow = MockUnitOfWork;
        let result: Result<i32> = uow.execute(|| {
            Ok(42)
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }
    
    #[test]
    fn unit_of_work_propagates_error() {
        let uow = MockUnitOfWork;
        let result: Result<i32> = uow.execute(|| {
            Err("test error".to_string())
        });
        assert!(result.is_err());
    }
}
