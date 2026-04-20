// labor/domain/work_record.rs
use crate::shared_kernel::ids::{WorkerId, CycleId, AreaId};
use crate::shared_kernel::time::Period;

#[derive(Debug)]
pub enum WorkContext {
    Cycle(CycleId),
    Area(AreaId),
    GeneralFarm, // Para trabajos de mantenimiento general
}

#[derive(Debug)]
pub struct WorkRecord {
    id: String,
    worker_id: WorkerId,
    context: WorkContext,
    date: i64,          // Cuándo se hizo
    cost_incurred: f64, // MVP: Se ingresa manual. A futuro: se calcula dinámicamente.
}

impl WorkRecord {
    pub fn new(worker_id: WorkerId, context: WorkContext, date: i64, cost_incurred: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            worker_id,
            context,
            date,
            cost_incurred,
        }
    }
}