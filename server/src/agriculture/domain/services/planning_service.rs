// agriculture/domain/planning_service.rs
use super::farm::Farm;
use super::crop::Crop;
use super::cycle::CropCycle;
use super::error::AgricultureError;
use crate::shared_kernel::ids::AreaId;
use crate::shared_kernel::time::Period;

pub struct CropPlanningService;

pub struct PlanningResult {
    pub cycle: CropCycle,
    pub schedule: Schedule,
}

impl CropPlanningService {
    pub fn schedule_cycle(
        farm: &Farm,
        target_area_id: &AreaId,
        crop: &Crop,
        period: Period,
        overlapping_candidates: &[CropCycle],
    ) -> Result<PlanningResult, AgricultureError> {

        // 1. Validación de Jerarquía (Seguridad de Dominio)
        if !farm.has_area(target_area_id) {
            return Err(AgricultureError::AreaNotFound(target_area_id.clone()));
        }

        // 2. Invariante de Colisión Espacio-Temporal
        // Tip: En Rust, .iter().any(...) es muy limpio para esto
        let collision = overlapping_candidates.iter().any(|existing| {
            existing.area_id() == target_area_id && existing.period().overlaps_with(&period)
        });

        if collision {
            return Err(AgricultureError::SpaceTimeCollision {
                area_id: target_area_id.clone(),
                start: period.start(),
                end: period.end(),
            });
        }

        // 3. Creación de los Aggregates
        let cycle = CropCycle::new(
            crop.id().clone(),
            target_area_id.clone(),
            period.clone(),
        );

        // Creamos el Schedule anclado al inicio del ciclo por defecto
        let schedule = Schedule::new(
            cycle.id().clone(),
            ScheduleAnchor::CycleStart,
            period.start() // El ancla real es el inicio del periodo
        );

        Ok(PlanningResult { cycle, schedule })
    }
}