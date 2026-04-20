// agriculture/error.rs
use crate::shared_kernel::ids::{AreaId, CycleId};

#[derive(Debug, PartialEq)]
pub enum AgricultureError {
    // Errores de Entidad
    AreaNotFound(AreaId),
    CropNotFound,
    CycleNotFound(CycleId),
    CycleAlreadyClosed,

    // Invariantes de Negocio (Reglas rotas)
    SpaceTimeCollision { area_id: AreaId, start: i64, end: i64 },
    ActivityOutsideCyclePeriod,
    AlreadyHarvested,

    // Errores de Validación (Inputs inválidos)
    InvalidMeasurement,
    InvalidPeriod,
}