use crate::agriculture::activity::{Activity, ActivityRecord, IntegrityStatus};
use crate::agriculture::error::AgricultureError;
use crate::shared_kernel::period::Period;
use crate::shared_kernel::ids::{CycleId, CropId, AreaId};

#[derive(Debug, Clone)]
pub struct CropCycle {
    id: CycleId,
    crop_id: CropId,
    area_id: AreaId,
    period: Period,
    executed_activities: Vec<ActivityRecord>,
    is_closed: bool,
}

impl CropCycle {
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

    pub fn id(&self) -> &CycleId { &self.id }
    pub fn crop_id(&self) -> &CropId { &self.crop_id }
    pub fn area_id(&self) -> &AreaId { &self.area_id }
    pub fn period(&self) -> &Period { &self.period }
    pub fn executed_activities(&self) -> &Vec<ActivityRecord> { &self.executed_activities }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agriculture::activity::{Activity, ActivityCategory};

    fn create_test_cycle() -> CropCycle {
        let period = Period::new(1000, 2000).unwrap();
        CropCycle::new(CropId("crop-1".to_string()), AreaId("area-1".to_string()), period)
    }

    #[test]
    fn register_activity_within_period_returns_valid() {
        let mut cycle = create_test_cycle();
        let activity = Activity::new(1500, ActivityCategory::Sowing);
        let record = cycle.register_activity(activity).unwrap();
        assert_eq!(record.integrity[0], IntegrityStatus::Valid);
    }

    #[test]
    fn register_activity_outside_period_marks_outsider() {
        let mut cycle = create_test_cycle();
        let activity = Activity::new(2500, ActivityCategory::Sowing);
        let record = cycle.register_activity(activity).unwrap();
        assert_eq!(record.integrity[0], IntegrityStatus::OutsidePeriod);
    }

    #[test]
    fn closed_cycle_rejects_activities() {
        let mut cycle = create_test_cycle();
        cycle.close_cycle();
        let activity = Activity::new(1500, ActivityCategory::Sowing);
        assert!(cycle.register_activity(activity).is_err());
    }
}
