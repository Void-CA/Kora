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

        // 2. Lógica de solapamiento extraída de la Finca
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

// agriculture/planning_service.rs (al final del archivo)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agriculture::domain::farm::Farm;
    use crate::agriculture::domain::crop::Crop;
    use crate::agriculture::domain::area::{Area, AreaClassification};
    use crate::shared_kernel::ids::{FarmId, AreaId, CropId};
    use crate::shared_kernel::time::Period;
    use crate::shared_kernel::space::{Measurement, Polygon};

    #[test]
    fn schedule_cycle_prevents_space_time_collisions() {
        // 1. Arrange: Preparar el estado en memoria
        let mut farm = Farm::new(FarmId("farm_01".into()));
        let area_id = AreaId("plot_A".into());
        let polygon = Polygon::new(vec![(1.0, 1.0), (2.0, 2.0), (1.0, 5.0)]);
        let area = Area::new(
            area_id.clone(),
            "Plot A".into(),
            AreaClassification::Productive,
            Measurement::new(5.0).unwrap(),
            polygon.unwrap()
        );
        farm.add_area(area);

        let crop = Crop::new(CropId("crop_corn".into()), "Corn".into());

        // Dos periodos que se solapan (Ej: Enero-Marzo y Febrero-Abril)
        let period_1 = Period::new(1000, 3000).unwrap();
        let period_2 = Period::new(2000, 4000).unwrap();

        // 2. Act: Ejecutar el servicio
        let first_schedule = CropPlanningService::schedule_cycle(
            &mut farm, &crop, area_id.clone(), period_1
        );
        let overlapping_schedule = CropPlanningService::schedule_cycle(
            &mut farm, &crop, area_id.clone(), period_2
        );

        // 3. Assert: Verificar que el dominio se defiende
        assert!(first_schedule.is_ok());

        match overlapping_schedule {
            Err(crate::agriculture::domain::error::AgricultureError::SpaceTimeCollision { area_id: failed_area, .. }) => {
                assert_eq!(failed_area, area_id);
            },
            _ => panic!("Expected SpaceTimeCollision error!"),
        }
    }
}