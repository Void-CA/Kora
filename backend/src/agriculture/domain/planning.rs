// agriculture/domain/planning.rs
use crate::shared_kernel::ids::{ScheduleId, CycleId, PlannedActivityId};
use super::activity::ActivityCategory;

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
    pub id: PlannedActivityId, // NEW: unique ID for each planned activity (from shared_kernel)
    pub category: ActivityCategory,
    pub relative_day: i32, // Day relative to anchor (e.g., +15 = 15 days after sowing)
    pub status: ActivityStatus,
}

impl PlannedActivity {
    /// Helper to create a new PlannedActivity with a unique ID.
    pub fn new(category: ActivityCategory, relative_day: i32) -> Self {
        Self {
            id: PlannedActivityId::new(), // Uses shared_kernel implementation
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

    /// Returns the cycle ID this schedule belongs to
    pub fn cycle_id(&self) -> &CycleId {
        &self.cycle_id
    }
}