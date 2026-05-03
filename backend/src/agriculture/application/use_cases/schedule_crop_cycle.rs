// agriculture/application/use_cases/schedule_crop_cycle.rs

use crate::agriculture::domain::planning_service::CropPlanningService;
// ... imports de puertos/repositorios ...

pub struct ScheduleCropCycleUseCase<F, C, CY> 
where
    F: FarmRepository,
    C: CropRepository,
    CY: CropCycleRepository,
{
    farm_repo: F,
    crop_repo: C,
    cycle_repo: CY,
}

impl<F, C, CY> ScheduleCropCycleUseCase<F, C, CY> 
where
    // ... trait bounds ...
{
    pub fn execute(&self, command: ScheduleCycleCommand) -> Result<(), AppError> {
        // 1. Cargar solo los Aggregates necesarios
        let farm = self.farm_repo.get_by_id(&command.farm_id)?;
        let crop = self.crop_repo.get_by_id(&command.crop_id)?;
        
        // 2. Consulta inteligente a la BD: 
        // "Dame SOLO los ciclos de esta área que toquen estas fechas"
        let overlapping_candidates = self.cycle_repo.find_overlapping(
            &command.area_id, 
            &command.period
        )?;

        // 3. Orquestar el Dominio Puro
        let new_cycle = CropPlanningService::schedule_cycle(
            &farm,
            &command.area_id,
            &crop,
            &command.period,
            &overlapping_candidates
        )?;

        // 4. Persistir el nuevo estado
        self.cycle_repo.save(new_cycle)?;

        Ok(())
    }
}