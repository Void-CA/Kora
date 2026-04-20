// agriculture/cycle.rs
use super::activity::{Activity, ActivityCategory};
use super::error::AgricultureError;
use crate::shared_kernel::time::Period;
use crate::shared_kernel::ids::{CycleId, CropId, AreaId};

#[derive(Debug)]
pub struct CropCycle {
    id: CycleId,
    crop_id: CropId,
    area_id: AreaId,
    period: Period,
    executed_activities: Vec<Activity>,
}

impl CropCycle {
    pub(crate) fn start(crop_id: CropId, area_id: AreaId, period: Period) -> Self {
        Self {
            id: CycleId(uuid::Uuid::new_v4().to_string()),
            crop_id,
            area_id,
            period,
            executed_activities: Vec::new(),
        }
    }

    pub(crate) fn register_activity(&mut self, activity: Activity) -> Result<(), AgricultureError> {
        // 1. Invariante temporal
        if !self.period.contains(activity.timestamp()) {
            return Err(AgricultureError::ActivityOutsideCyclePeriod);
        }

        // 2. Invariante de negocio: Unicidad de cosecha
        if activity.category() == &ActivityCategory::Harvest {
            if self.has_been_harvested() {
                return Err(AgricultureError::AlreadyHarvested);
            }
        }

        self.executed_activities.push(activity);
        Ok(())
    }

    fn has_been_harvested(&self) -> bool {
        self.executed_activities.iter()
            .any(|a| a.category() == &ActivityCategory::Harvest)
    }
    

    pub fn id(&self) -> &CycleId {
        &self.id
    }

    pub fn area_id(&self) -> &AreaId {
        &self.area_id
    }

    pub fn period(&self) -> &Period {
        &self.period
    }
}