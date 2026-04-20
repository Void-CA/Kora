// agriculture/domain/planning_service.rs
use super::farm::Farm;
use super::crop::Crop;
use super::cycle::CropCycle;
use super::error::AgricultureError;
use crate::shared_kernel::ids::AreaId;
use crate::shared_kernel::time::Period;

pub struct CropPlanningService;

impl CropPlanningService {
    pub fn schedule_cycle(
        farm: &Farm,
        target_area_id: &AreaId,
        crop: &Crop,
        period: &Period,
        overlapping_candidates: &[CropCycle], // Inyectados por el Use Case
    ) -> Result<CropCycle, AgricultureError> {

        // 1. Validar jerarquía: El área debe pertenecer a esta finca
        if !farm.has_area(target_area_id) {
            return Err(AgricultureError::AreaNotFound(target_area_id.clone()));
        }

        // 2. Validar invariante de Espacio-Tiempo
        for existing_cycle in overlapping_candidates {
            if existing_cycle.period().overlaps_with(period) {
                // NOTA: En el futuro aquí usaremos intersección de polígonos
                if existing_cycle.area_id() == target_area_id {
                    return Err(AgricultureError::SpaceTimeCollision {
                        area_id: target_area_id.clone(),
                        start: period.start(),
                        end: period.end()
                    });
                }
            }
        }

        // 3. Crear el nuevo Aggregate
        let new_cycle = CropCycle::new(
            crop.id().clone(), 
            target_area_id.clone(), 
            period.clone()
        );

        Ok(new_cycle)
    }
}

