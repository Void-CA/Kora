// agriculture/domain/services/variance_service.rs
use super::super::activity::{ActivityRecord, IntegrityStatus};
use super::super::planning::{Schedule, PlannedActivity};
use super::super::cycle::CropCycle;

pub struct VarianceReport {
    pub on_time: Vec<ActivityRecord>,
    pub delayed: Vec<(PlannedActivity, ActivityRecord)>,
    pub unplanned: Vec<ActivityRecord>,
}

impl VarianceReport {
    pub fn new() -> Self {
        Self {
            on_time: Vec::new(),
            delayed: Vec::new(),
            unplanned: Vec::new(),
        }
    }
}

pub struct VarianceService;

impl VarianceService {
    pub fn analyze(schedule: &Schedule, cycle: &mut CropCycle) -> VarianceReport {
        let mut report = VarianceReport::new();

        // Get mutable access to executed_activities
        let executed_activities = cycle.executed_activities_mut();

        for record in executed_activities.iter_mut() {
            let mut matched_planned: Option<&PlannedActivity> = None;

            // Find first planned activity with matching category
            for planned in schedule.activities() {
                if planned.category == *record.activity.category() {
                    matched_planned = Some(planned);
                    break;
                }
            }

            if let Some(planned) = matched_planned {
                let expected_ts = schedule.anchor_date() + planned.relative_day as i64;
                if record.activity.timestamp() == expected_ts {
                    // On time: category and timestamp match
                    report.on_time.push(record.clone());
                } else {
                    // Delayed: category match but timestamp mismatch
                    report.delayed.push((planned.clone(), record.clone()));
                }
            } else {
                // Unplanned: no matching PlannedActivity
                record.integrity.push(IntegrityStatus::Unplanned);
                report.unplanned.push(record.clone());
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::activity::{Activity, ActivityCategory, ActivityRecord, IntegrityStatus};
    use super::super::super::planning::{Schedule, PlannedActivity, ActivityStatus};
    use crate::shared_kernel::ids::{ScheduleId, CycleId, CropId, AreaId};
    use crate::shared_kernel::time::Period;

    fn create_test_schedule() -> Schedule {
        let mut schedule = Schedule::new(
            CycleId("cycle-1".to_string()),
            super::super::super::planning::ScheduleAnchor::CycleStart,
            1500,
        );
        schedule.add_planned_activity(PlannedActivity {
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: ActivityStatus::Planned,
        });
        schedule.add_planned_activity(PlannedActivity {
            category: ActivityCategory::Maintenance,
            relative_day: 15,
            status: ActivityStatus::Planned,
        });
        schedule
    }

    fn create_test_cycle() -> CropCycle {
        use super::super::super::cycle::CropCycle;
        use crate::shared_kernel::time::Period;
        use crate::shared_kernel::ids::AreaId;

        let period = Period::new(1000, 2000).unwrap();
        CropCycle::new(
            CropId("crop-1".to_string()),
            AreaId("area-1".to_string()),
            period,
        )
    }

    #[test]
    fn variance_report_new_returns_empty_buckets() {
        let report = VarianceReport::new();
        assert_eq!(report.on_time.len(), 0);
        assert_eq!(report.delayed.len(), 0);
        assert_eq!(report.unplanned.len(), 0);
    }

    #[test]
    fn analyze_with_perfect_match_returns_on_time() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Activity matches Sowing at relative_day 0: expected_ts = 1500 + 0 = 1500
        let activity = Activity::new(1500, ActivityCategory::Sowing);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let report = VarianceService::analyze(&schedule, &mut cycle);

        assert_eq!(report.on_time.len(), 1);
        assert_eq!(report.delayed.len(), 0);
        assert_eq!(report.unplanned.len(), 0);
    }

    #[test]
    fn analyze_with_delayed_activity_returns_delayed() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Maintenance planned at relative_day 15: expected_ts = 1500 + 15 = 1515
        // But activity is at timestamp 1520 (5 days late)
        let activity = Activity::new(1520, ActivityCategory::Maintenance);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let report = VarianceService::analyze(&schedule, &mut cycle);

        assert_eq!(report.on_time.len(), 0);
        assert_eq!(report.delayed.len(), 1);
        assert_eq!(report.unplanned.len(), 0);
    }

    #[test]
    fn analyze_with_unplanned_activity_returns_unplanned() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Harvest is not in the schedule
        let activity = Activity::new(2000, ActivityCategory::Harvest);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let report = VarianceService::analyze(&schedule, &mut cycle);

        assert_eq!(report.on_time.len(), 0);
        assert_eq!(report.delayed.len(), 0);
        assert_eq!(report.unplanned.len(), 1);
        assert_eq!(report.unplanned[0].integrity.contains(&IntegrityStatus::Unplanned), true);
    }

    #[test]
    fn analyze_adds_unplanned_integrity_to_cycle_activities() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Unplanned activity
        let activity = Activity::new(2000, ActivityCategory::Harvest);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let _report = VarianceService::analyze(&schedule, &mut cycle);

        // Check that the original cycle activity has IntegrityStatus::Unplanned
        assert_eq!(cycle.executed_activities()[0].integrity.contains(&IntegrityStatus::Unplanned), true);
    }
}
