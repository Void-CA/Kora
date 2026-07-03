use crate::shared_kernel::ids::{AreaId, CycleId};

#[derive(Debug, PartialEq)]
pub enum AgricultureError {
    AreaNotFound(AreaId),
    CropNotFound,
    CycleNotFound(CycleId),
    CycleAlreadyClosed,
    SpaceTimeCollision { area_id: AreaId, start: i64, end: i64 },
    AlreadyHarvested,
    ProductiveAreaExceedsBounds { declared_ha: f64, max_theoretical_ha: f64 },
    InvalidMeasurement,
    InvalidPeriod,
    InvalidStateTransition,
    EmptyPlan,
}
