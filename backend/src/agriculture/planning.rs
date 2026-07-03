use crate::shared_kernel::ids::CycleId;
use crate::agriculture::ids::{ScheduleId, PlannedActivityId};
use crate::agriculture::activity::ActivityCategory;

#[derive(Debug, Clone)]
pub enum ScheduleAnchor {
    CycleStart,
    SowingDate,
    HarvestStart,
}

#[derive(Debug, Clone)]
pub enum ActivityStatus {
    Planned,
    InProgress,
    Completed,
    Skipped,
}

#[derive(Debug, Clone)]
pub struct PlannedActivity {
    pub id: PlannedActivityId,
    pub category: ActivityCategory,
    pub relative_day: i32,
    pub status: ActivityStatus,
}

impl PlannedActivity {
    pub fn new(category: ActivityCategory, relative_day: i32) -> Self {
        Self {
            id: PlannedActivityId::new(),
            category,
            relative_day,
            status: ActivityStatus::Planned,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Schedule {
    id: ScheduleId,
    cycle_id: CycleId,
    anchor: ScheduleAnchor,
    anchor_date: i64,
    activities: Vec<PlannedActivity>,
    version: u32,
}

impl Schedule {
    pub fn new(cycle_id: CycleId, anchor: ScheduleAnchor, anchor_date: i64) -> Self {
        Self {
            id: ScheduleId(uuid::Uuid::new_v4().to_string()),
            cycle_id,
            anchor,
            anchor_date,
            activities: Vec::new(),
            version: 1,
        }
    }

    pub fn add_planned_activity(&mut self, activity: PlannedActivity) {
        self.activities.push(activity);
    }

    pub fn activities(&self) -> &Vec<PlannedActivity> { &self.activities }
    pub fn anchor_date(&self) -> i64 { self.anchor_date }
    pub fn cycle_id(&self) -> &CycleId { &self.cycle_id }
}
