// agriculture/planning_service.rs
use super::farm::Farm;
use super::crop::Crop;
use super::cycle::CropCycle;
use super::error::AgricultureError;
use crate::shared_kernel::ids::AreaId;
use crate::shared_kernel::time::Period;

/// Un Domain Service no tiene estado, solo orquesta reglas de negocio
/// que involucran a múltiples agregados/entidades.
pub struct CropPlanningService;

impl CropPlanningService {
    pub fn schedule_cycle(
        farm: &mut Farm,
        crop: &Crop, // Garantizamos que el Crop realmente existe en el modelo
        area_id: AreaId,
        period: Period,
    ) -> Result<(), AgricultureError> {

        // 1. Validar que el área le pertenece a la finca
        if !farm.has_area(&area_id) {
            return Err(AgricultureError::AreaNotFound(area_id));
        }

        // 2. Lógica compleja de solapamiento extraída de la Finca
        if farm.is_area_occupied_in_period(&area_id, &period) {
            return Err(AgricultureError::SpaceTimeCollision {
                area_id,
                start: period.start(),
                end: period.end()
            });
        }

        // 3. (Futuro) Evaluar reglas agronómicas del Crop vs Area
        // Ej: ¿Es el suelo compatible? ¿Respeta la rotación de cultivos?

        // 4. Instanciar y registrar
        let new_cycle = CropCycle::start(crop.id().clone(), area_id, period);
        farm.register_cycle(new_cycle);

        Ok(())
    }
}