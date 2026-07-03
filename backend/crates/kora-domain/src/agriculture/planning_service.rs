use crate::agriculture::farm::Farm;
use crate::agriculture::crop::Crop;
use crate::agriculture::cycle::CropCycle;
use crate::agriculture::error::AgricultureError;
use crate::agriculture::planning::{Schedule, ScheduleAnchor};
use kora_kernel::ids::AreaId;
use kora_kernel::period::Period;

pub struct CropPlanningService;

#[derive(Debug)]
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
        if !farm.has_area(target_area_id) {
            return Err(AgricultureError::AreaNotFound(target_area_id.clone()));
        }

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

        let cycle = CropCycle::new(crop.id().clone(), target_area_id.clone(), period.clone());
        let schedule = Schedule::new(cycle.id().clone(), ScheduleAnchor::CycleStart, period.start());

        Ok(PlanningResult { cycle, schedule })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agriculture::farm::Farm;
    use crate::agriculture::ids::FarmId;
    use kora_kernel::area_unit::{AreaMeasurement, AreaUnit};
    use kora_kernel::polygon::Polygon;
    use kora_kernel::ids::CropId;
    use crate::agriculture::area::Area;
    use crate::agriculture::area::AreaClassification;
    use geo_types::polygon;

    fn dummy_polygon() -> Polygon {
        Polygon::new(polygon![
            (x: -70.0, y: 12.0),
            (x: -70.001, y: 12.0),
            (x: -70.001, y: 12.001),
            (x: -70.0, y: 12.001),
            (x: -70.0, y: 12.0),
        ])
        .unwrap()
    }

    fn setup_farm_with_area(area_id: &AreaId) -> Farm {
        let mut farm = Farm::new(FarmId::new());
        let area = Area::new(
            area_id.clone(),
            "Lote A".into(),
            AreaClassification::Productive,
            AreaMeasurement::new(1.0, AreaUnit::Hectares).unwrap(),
            dummy_polygon(),
        )
        .unwrap();
        farm.add_area(area);
        farm
    }

    #[test]
    fn schedule_cycle_success() {
        let area_id = AreaId::new();
        let farm = setup_farm_with_area(&area_id);
        let crop = Crop::new(CropId::new(), "Maíz".into());
        let period = Period::new(100, 200).unwrap();

        let result = CropPlanningService::schedule_cycle(&farm, &area_id, &crop, period, &[]);
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(*plan.cycle.crop_id(), *crop.id());
        assert_eq!(*plan.cycle.area_id(), area_id);
    }

    #[test]
    fn schedule_cycle_rejects_unknown_area() {
        let farm = Farm::new(FarmId::new()); // no areas
        let crop = Crop::new(CropId::new(), "Maíz".into());
        let period = Period::new(100, 200).unwrap();

        let result = CropPlanningService::schedule_cycle(&farm, &AreaId::new(), &crop, period, &[]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AgricultureError::AreaNotFound(_)));
    }

    #[test]
    fn schedule_cycle_detects_collision() {
        let area_id = AreaId::new();
        let farm = setup_farm_with_area(&area_id);
        let crop = Crop::new(CropId::new(), "Maíz".into());
        let period = Period::new(100, 200).unwrap();

        let existing = CropCycle::new(crop.id().clone(), area_id.clone(), Period::new(150, 250).unwrap());

        let result = CropPlanningService::schedule_cycle(
            &farm, &area_id, &crop, period, &[existing],
        );
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AgricultureError::SpaceTimeCollision { .. }));
    }

    #[test]
    fn schedule_cycle_ignores_different_area() {
        let area_id = AreaId::new();
        let farm = setup_farm_with_area(&area_id);
        let crop = Crop::new(CropId::new(), "Maíz".into());
        let period = Period::new(100, 200).unwrap();

        let other_area_id = AreaId::new();
        let existing = CropCycle::new(
            CropId::new(),
            other_area_id,
            Period::new(150, 250).unwrap(),
        );

        let result = CropPlanningService::schedule_cycle(
            &farm, &area_id, &crop, period, &[existing],
        );
        assert!(result.is_ok()); // different area, no collision
    }
}
