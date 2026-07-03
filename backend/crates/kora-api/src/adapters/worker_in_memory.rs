use std::collections::HashMap;
use kora_domain::finance::payroll::{Worker, WorkerId};
use kora_domain::ports::worker_repository::WorkerRepository;

pub struct InMemoryWorkerRepository {
    workers: HashMap<String, Worker>,
}

impl InMemoryWorkerRepository {
    pub fn new() -> Self {
        Self { workers: HashMap::new() }
    }
}

impl WorkerRepository for InMemoryWorkerRepository {
    fn find_by_id(&self, id: &WorkerId) -> Option<Worker> {
        self.workers.get(&id.0).cloned()
    }
    fn save(&mut self, worker: Worker) {
        self.workers.insert(worker.id().0.clone(), worker);
    }
    fn all(&self) -> Vec<Worker> {
        self.workers.values().cloned().collect()
    }
}
