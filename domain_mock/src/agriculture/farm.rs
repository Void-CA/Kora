// agriculture/farm.rs
use super::cycle::CropCycle;
use super::area::Area;
use super::activity::Activity;
use super::error::AgricultureError;
use crate::shared_kernel::ids::{FarmId, AreaId, CycleId};
use crate::shared_kernel::time::Period;

pub struct Farm {
    id: FarmId,
    areas: Vec<Area>,
    cycles: Vec<CropCycle>,
}

impl Farm {
    pub fn new(id: FarmId) -> Self {
        Self {
            id,
            areas: Vec::new(),
            cycles: Vec::new(),
        }
    }

    pub fn has_area(&self, area_id: &AreaId) -> bool {
        self.areas.iter().any(|a| a.id() == area_id)
    }

    pub fn is_area_occupied_in_period(&self, area_id: &AreaId, period: &Period) -> bool {
        self.cycles.iter()
            .filter(|c| c.area_id() == area_id)
            .any(|c| c.period().overlaps_with(period))
    }

    /// Nota: Es `pub(crate)`, NO `pub`.
    /// Esto es vital: prohíbe que la capa de aplicación inyecte un ciclo
    /// saltándose las validaciones complejas. Solo el `CropPlanningService`
    /// (que vive en el mismo módulo) puede llamar a este método.
    pub(crate) fn register_cycle(&mut self, cycle: CropCycle) {
        self.cycles.push(cycle);
    }

    /// La ejecución de actividades sigue siendo responsabilidad directa
    /// de la Farm, ya que no requiere orquestar agregados externos (como Crop).
    pub fn execute_activity(
        &mut self,
        cycle_id: &CycleId,
        activity: Activity,
    ) -> Result<(), AgricultureError> {

        let cycle = self.cycles.iter_mut()
            .find(|c| c.id() == cycle_id)
            .ok_or_else(|| AgricultureError::CycleNotFound(cycle_id.clone()))?;

        cycle.register_activity(activity)?;

        Ok(())
    }

    pub fn add_area(&mut self, area: Area) {
        self.areas.push(area);
    }
}