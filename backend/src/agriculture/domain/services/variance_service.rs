// agriculture/domain/services/variance_service.rs
use std::collections::{HashMap, VecDeque};
use super::super::activity::{ActivityRecord, Activity, ActivityCategory};
use super::super::planning::{Schedule, PlannedActivity};
use crate::shared_kernel::ids::PlannedActivityId;
use super::super::cycle::CropCycle;
use super::economic_variance::{CostVariance, EconomicDataProvider};

// --- NEW: ConfidenceScore ---
#[derive(Debug, Clone, PartialEq)]
pub enum ConfidenceScore {
    High,    // Exact match: same category + OnTime
    Medium,  // Within tolerance but not OnTime
    Low,     // Outside tolerance
}

// --- NEW: MatchedActivity (Explicit matching) ---
#[derive(Debug, Clone)]
pub struct MatchedActivity {
    pub planned_id: PlannedActivityId,
    pub record: ActivityRecord,  // Contains Activity with ActivityId inside
    pub variance: TimingVariance,
    pub confidence: ConfidenceScore,
    pub cost_variance: Option<super::economic_variance::CostVariance>,  // NEW: Economic data (populated later)
}

// --- NEW: TimingVariance ---
#[derive(Debug, Clone, PartialEq)]
pub enum TimingVariance {
    OnTime,
    Early(i64),  // days early (positive number)
    Late(i64),    // days late (positive number)
}

// --- NEW: VarianceConfig (Externalized tolerance) ---
#[derive(Debug, Clone)]
pub struct VarianceConfig {
    pub temporal_tolerance_days: i64,
    pub enable_early_detection: bool,
    pub enable_confidence_scoring: bool,
}

// --- NEW: VarianceReport (Explicit matching) ---
#[derive(Debug, Clone)]
pub struct VarianceReport {
    pub matched: Vec<MatchedActivity>,  // EXPLICIT: planned_id + record + variance + confidence
    pub unplanned: Vec<ActivityRecord>,
    pub missing: Vec<PlannedActivity>,
}

impl VarianceReport {
    pub fn new() -> Self {
        Self {
            matched: Vec::new(),
            unplanned: Vec::new(),
            missing: Vec::new(),
        }
    }
}

// --- Service ---
pub struct VarianceService;

impl VarianceService {
    // PURE function with config injection
    pub fn analyze_with_config(
        schedule: &Schedule,
        cycle: &CropCycle,
        config: &VarianceConfig,
    ) -> VarianceReport {
        let mut report = VarianceReport::new();

        // Build VecDeque per category (to enable pop_front for no-reuse)
        let mut planned_by_category: HashMap<ActivityCategory, VecDeque<&PlannedActivity>> = HashMap::new();
        for planned in schedule.activities() {
            planned_by_category
                .entry(planned.category.clone())
                .or_insert_with(VecDeque::new)
                .push_back(planned);
        }

        // Process each executed activity
        for record in cycle.executed_activities() {
            let category = record.activity.category();

            if let Some(queue) = planned_by_category.get_mut(category) {
                // Find best match by temporal proximity
                let mut best_match: Option<(usize, &PlannedActivity, i64)> = None;
                let mut idx = 0;

                for planned in queue.iter().copied() {
                    let expected_ts = schedule.anchor_date() + planned.relative_day as i64;
                    let diff_days = record.activity.timestamp() - expected_ts;

                    if best_match.is_none() || diff_days.abs() < best_match.unwrap().2.abs() {
                        best_match = Some((idx, planned, diff_days));
                    }
                    idx += 1;
                }

                if let Some((idx, planned, diff_days)) = best_match {
                    // NO REUSE: Remove from queue
                    queue.remove(idx).unwrap();

                    // Calculate variance
                    let variance = if diff_days == 0 {
                        TimingVariance::OnTime
                    } else if diff_days > 0 {
                        TimingVariance::Late(diff_days)
                    } else {
                        TimingVariance::Early(diff_days.abs())
                    };

                    // Apply tolerance: if within tolerance, keep variance; else set to Early/Late with full diff
                    let (variance, confidence) = if diff_days == 0 {
                        (variance, ConfidenceScore::High)
                    } else if diff_days.abs() <= config.temporal_tolerance_days {
                        // Within tolerance
                        let confidence = if config.enable_confidence_scoring {
                            ConfidenceScore::Medium
                        } else {
                            ConfidenceScore::High  // Default if scoring disabled
                        };
                        (variance, confidence)
                    } else {
                        // Outside tolerance
                        let variance = if diff_days > 0 {
                            TimingVariance::Late(diff_days)
                        } else {
                            TimingVariance::Early(diff_days.abs())
                        };
                        let confidence = if config.enable_confidence_scoring {
                            ConfidenceScore::Low
                        } else {
                            ConfidenceScore::High  // Default if scoring disabled
                        };
                        (variance, confidence)
                    };

                    // Create explicit MatchedActivity
                    report.matched.push(MatchedActivity {
                        planned_id: planned.id.clone(),
                        record: record.clone(),
                        variance,
                        confidence,
                        cost_variance: None,  // Economic data not filled yet
                    });
                } else {
                    // No good match found in queue (shouldn't happen if we got here)
                    report.unplanned.push(record.clone());
                }
            } else {
                // No matching category: unplanned
                report.unplanned.push(record.clone());
            }
        }

        // Remaining in queues = missing
        for (_, queue) in planned_by_category.iter() {
            for planned in queue.iter().copied() {
                report.missing.push(planned.clone());
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
        use crate::shared_kernel::ids::{CycleId, CropId, AreaId, ScheduleId, PlannedActivityId};
        use crate::shared_kernel::time::Period;

    fn create_test_schedule() -> Schedule {
        let mut schedule = Schedule::new(
            CycleId::new(),
            super::super::super::planning::ScheduleAnchor::CycleStart,
            1500,
        );
        schedule.add_planned_activity(PlannedActivity {
            id: PlannedActivityId::new(),
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: ActivityStatus::Planned,
        });
        schedule.add_planned_activity(PlannedActivity {
            id: PlannedActivityId::new(),
            category: ActivityCategory::Maintenance,
            relative_day: 15,
            status: ActivityStatus::Planned,
        });
        schedule
    }

    fn create_test_cycle() -> CropCycle {
        let period = Period::new(1000, 2000).unwrap();
        CropCycle::new(
            CropId::new(),
            AreaId::new(),
            period,
        )
    }

    fn default_config() -> VarianceConfig {
        VarianceConfig {
            temporal_tolerance_days: 2,
            enable_early_detection: true,
            enable_confidence_scoring: true,
        }
    }

    #[test]
    fn matched_activity_explicit_linking() {
        let mut schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Add activity matching Sowing (relative_day 0, expected 1500)
        let activity = Activity::new(1500, ActivityCategory::Sowing);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let config = default_config();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &config);

        assert_eq!(report.matched.len(), 1);
        assert_eq!(report.matched[0].planned_id, schedule.activities()[0].id);
        assert!(matches!(report.matched[0].variance, TimingVariance::OnTime));
        assert!(matches!(report.matched[0].confidence, ConfidenceScore::High));
    }

    #[test]
    fn no_reuse_of_planned_activities() {
        let mut schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Only 1 planned Sowing, but 2 executed
        let activity1 = Activity::new(1500, ActivityCategory::Sowing);
        let result1 = cycle.register_activity(activity1);
        assert!(result1.is_ok());

        let activity2 = Activity::new(1501, ActivityCategory::Sowing);  // Second Sowing
        let result2 = cycle.register_activity(activity2);
        assert!(result2.is_ok());

        let config = default_config();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &config);

        // Only 1 matched (the first), second goes to unplanned
        assert_eq!(report.matched.len(), 1);
        assert_eq!(report.unplanned.len(), 1);
    }

    #[test]
    fn variance_config_injection() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Maintenance planned at relative_day 15: expected_ts = 1500 + 15 = 1515
        // Activity at 1520 (5 days late)
        let activity = Activity::new(1520, ActivityCategory::Maintenance);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        // Config with tolerance = 5
        let config = VarianceConfig {
            temporal_tolerance_days: 5,
            enable_early_detection: true,
            enable_confidence_scoring: true,
        };

        let report = VarianceService::analyze_with_config(&schedule, &cycle, &config);

        // Within new tolerance → Late(5)
        assert_eq!(report.matched.len(), 1);
        assert!(matches!(report.matched[0].variance, TimingVariance::Late(5)));
    }

    #[test]
    fn confidence_score_high() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Exact match: Sowing at day 0 (timestamp 1500)
        let activity = Activity::new(1500, ActivityCategory::Sowing);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let config = default_config();  // tolerance = 2
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &config);

        assert_eq!(report.matched.len(), 1);
        assert!(matches!(report.matched[0].confidence, ConfidenceScore::High));
    }

    #[test]
    fn confidence_score_medium() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Maintenance planned at day 15 (1515), activity at 1517 (2 days late, within tolerance)
        let activity = Activity::new(1517, ActivityCategory::Maintenance);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let config = default_config();  // tolerance = 2
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &config);

        // Within tolerance but not OnTime → Medium
        assert_eq!(report.matched.len(), 1);
        assert!(matches!(report.matched[0].confidence, ConfidenceScore::Medium));
    }

    #[test]
    fn missing_bucket_with_deque() {
        let schedule = create_test_schedule();
        let cycle = create_test_cycle();  // No activities registered

        let config = default_config();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &config);

        // 2 planned activities, 0 executed → both in missing
        assert_eq!(report.missing.len(), 2);
        assert_eq!(report.matched.len(), 0);
        assert_eq!(report.unplanned.len(), 0);
    }

    #[test]
    fn unplanned_activity() {
        let schedule = create_test_schedule();
        let mut cycle = create_test_cycle();

        // Harvest is not in the schedule
        let activity = Activity::new(2000, ActivityCategory::Harvest);
        let result = cycle.register_activity(activity);
        assert!(result.is_ok());

        let config = default_config();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &config);

        assert_eq!(report.unplanned.len(), 1);
        assert_eq!(report.missing.len(), 2);  // Sowing and Maintenance not executed
    }
}
