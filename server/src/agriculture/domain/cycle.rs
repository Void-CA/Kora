// agriculture/domain/cycle.rs
use super::activity::{Activity, ActivityCategory};
use super::error::AgricultureError;
use super::crop::Crop;
use crate::shared_kernel::time::Period;
use crate::shared_kernel::ids::{CycleId, CropId, AreaId};

#[derive(Debug)]
pub struct CropCycle {
    id: CycleId,
    crop_id: CropId,
    area_id: AreaId,
    period: Period,
    executed_activities: Vec<Activity>,
    is_closed: bool, // Nueva bandera de estado
}

impl CropCycle {
    /// Solo el CropPlanningService debería invocar este método.
    pub(crate) fn new(crop_id: CropId, area_id: AreaId, period: Period) -> Self {
        Self {
            id: CycleId(uuid::Uuid::new_v4().to_string()),
            crop_id,
            area_id,
            period,
            executed_activities: Vec::new(),
            is_closed: false,
        }
    }

    pub fn register_activity(&mut self, activity: Activity) -> Result<(), AgricultureError> {
        if self.is_closed {
            return Err(AgricultureError::CycleAlreadyClosed);
        }

        if !self.period.contains(activity.timestamp()) {
            return Err(AgricultureError::ActivityOutsideCyclePeriod);
        }

        self.executed_activities.push(activity);
        Ok(())
    }

    pub fn close_cycle(&mut self) {
        self.is_closed = true;
    }

    pub fn id(&self) -> &CycleId {
        &self.id
    }

    pub fn crop_id(&self) -> &CropId {
        &self.crop_id
    }

    pub fn area_id(&self) -> &AreaId {
        &self.area_id
    }

    pub fn period(&self) -> &Period {
        &self.period
    }
}