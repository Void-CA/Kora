// labor/domain/worker.rs
use crate::labor::domain::ids::WorkerId;

#[derive(Debug)]
pub struct Worker {
    id: WorkerId,
    name: String,
    is_active: bool,
    // En el futuro aquí irán: base_wage, skills, certifications, etc.
}

impl Worker {
    pub fn new(id: WorkerId, name: String) -> Self {
        Self {
            id,
            name,
            is_active: true,
        }
    }

    pub fn id(&self) -> &WorkerId {
        &self.id
    }
}