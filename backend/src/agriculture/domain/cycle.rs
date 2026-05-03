// agriculture/domain/cycle.rs
use super::activity::{Activity, ActivityCategory, ActivityRecord, IntegrityStatus};
use super::error::AgricultureError;
use super::crop::Crop;
use crate::shared_kernel::time::Period;
use crate::shared_kernel::ids::{CycleId, CropId, AreaId};

#[derive(Debug, Clone)]
pub struct CropCycle {
    id: CycleId,
    crop_id: CropId,
    area_id: AreaId,
    period: Period,
    executed_activities: Vec<ActivityRecord>,
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

    pub fn register_activity(&mut self, activity: Activity) -> Result<ActivityRecord, AgricultureError> {
        if self.is_closed {
            return Err(AgricultureError::CycleAlreadyClosed);
        }

        let mut integrity = vec![];
        if !self.period.contains(activity.timestamp()) {
            integrity.push(IntegrityStatus::OutsidePeriod);
        } else {
            integrity.push(IntegrityStatus::Valid);
        }

        let record = ActivityRecord::new(activity, integrity);
        self.executed_activities.push(record.clone());
        Ok(record)
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

    pub fn executed_activities_mut(&mut self) -> &mut Vec<ActivityRecord> {
        &mut self.executed_activities
    }

    pub fn executed_activities(&self) -> &Vec<ActivityRecord> {
        &self.executed_activities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::activity::{Activity, ActivityCategory, ActivityRecord, IntegrityStatus};
    use crate::shared_kernel::time::Period;
    use crate::shared_kernel::ids::{CycleId, CropId, AreaId};

    fn create_test_cycle() -> CropCycle {
        let period = Period::new(1000, 2000).unwrap();
        CropCycle::new(
            CropId("crop-1".to_string()),
            AreaId("area-1".to_string()),
            period,
        )
    }

    fn create_test_activity(timestamp: i64) -> Activity {
        Activity::new(timestamp, ActivityCategory::Sowing)
    }

    #[test]
    fn register_activity_within_period_returns_valid_integrity() {
        let mut cycle = create_test_cycle();
        let activity = create_test_activity(1500); // dentro de [1000, 2000]

        let result = cycle.register_activity(activity);
        assert!(result.is_ok());
        let record = result.unwrap();
        assert_eq!(record.integrity.len(), 1);
        assert_eq!(record.integrity[0], IntegrityStatus::Valid);
    }

    #[test]
    fn register_activity_outside_period_returns_outside_period_no_error() {
        let mut cycle = create_test_cycle();
        let activity = create_test_activity(2500); // fuera de [1000, 2000]

        let result = cycle.register_activity(activity);
        assert!(result.is_ok()); // NO error, acepta la actividad
        let record = result.unwrap();
        assert_eq!(record.integrity.len(), 1);
        assert_eq!(record.integrity[0], IntegrityStatus::OutsidePeriod);
    }

    #[test]
    fn register_activity_on_closed_cycle_returns_error() {
        let mut cycle = create_test_cycle();
        cycle.close_cycle();
        let activity = create_test_activity(1500);

        let result = cycle.register_activity(activity);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AgricultureError::CycleAlreadyClosed);
    }

    #[test]
    fn executed_activities_contains_activity_record() {
        let mut cycle = create_test_cycle();
        let activity = create_test_activity(1500);

        let result = cycle.register_activity(activity);
        assert!(result.is_ok());
        
        // Verificamos que executed_activities contiene el ActivityRecord
        assert_eq!(cycle.executed_activities.len(), 1);
        assert_eq!(cycle.executed_activities[0].integrity[0], IntegrityStatus::Valid);
    }
}