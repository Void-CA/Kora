// agriculture/domain/planning.rs
use crate::shared_kernel::ids::{ScheduleId, CycleId};
use super::activity::ActivityCategory;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedActivityId(pub String);

impl PlannedActivityId {
    /// Returns the inner string for cross-domain compatibility (e.g., Budget keys).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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
    pub id: PlannedActivityId,  // NEW: unique ID for each planned activity
    pub category: ActivityCategory,
    pub relative_day: i32,
    pub status: ActivityStatus,
}

#[derive(Debug)]
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

    pub fn add_planned_activity(&mut self, mut activity: PlannedActivity) {
        // Auto-generate ID if empty (shouldn't happen with new struct, but safety check)
        if activity.id.0.is_empty() {
            activity.id = PlannedActivityId(uuid::Uuid::new_v4().to_string());
        }
        self.activities.push(activity);
    }

    pub fn activities(&self) -> &Vec<PlannedActivity> {
        &self.activities
    }

    pub fn anchor_date(&self) -> i64 {
        self.anchor_date
    }
}