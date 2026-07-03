use crate::finance::payroll::{Worker, WorkerId};

pub trait WorkerRepository {
    fn find_by_id(&self, id: &WorkerId) -> Option<Worker>;
    fn save(&mut self, worker: Worker);
    fn all(&self) -> Vec<Worker>;
}
