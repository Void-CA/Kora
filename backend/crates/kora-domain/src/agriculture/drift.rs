use std::collections::{HashMap, VecDeque};
use crate::agriculture::activity::{ActivityRecord, ActivityCategory};
use crate::agriculture::planning::{Schedule, PlannedActivity};
use crate::agriculture::ids::{PlannedActivityId, ActivityRecordId};
use crate::agriculture::cycle::CropCycle;
use kora_kernel::money::{Money, RateError};
use crate::ports::economic_data_provider::EconomicDataProvider;

// ── Scoring ──

#[derive(Debug, Clone, PartialEq)]
pub enum ConfidenceScore {
    High,
    Medium,
    Low,
}

// ── Timing variance ──

#[derive(Debug, Clone, PartialEq)]
pub enum TimingVariance {
    OnTime,
    Early(i64),
    Late(i64),
}

// ── Cost variance ──

#[derive(Debug, Clone)]
pub struct CostVariance {
    pub planned_cost: Money,
    pub actual_cost: Money,
    pub variance: Money,
}

impl CostVariance {
    pub fn new(planned_cost: Money, actual_cost: Money) -> Result<Self, RateError> {
        let variance = actual_cost.subtract(&planned_cost)?;
        Ok(Self { planned_cost, actual_cost, variance })
    }
}

// ── Matched activity (links plan to execution, with optional cost data) ──

#[derive(Debug, Clone)]
pub struct MatchedActivity {
    pub planned_id: PlannedActivityId,
    pub record: ActivityRecord,
    pub variance: TimingVariance,
    pub confidence: ConfidenceScore,
    pub cost_variance: Option<CostVariance>,
}

// ── Timing reports ──

#[derive(Debug, Clone)]
pub struct VarianceReport {
    pub matched: Vec<MatchedActivity>,
    pub unplanned: Vec<ActivityRecord>,
    pub missing: Vec<PlannedActivity>,
}

impl VarianceReport {
    pub fn new() -> Self {
        Self { matched: Vec::new(), unplanned: Vec::new(), missing: Vec::new() }
    }
}

// ── Economic report ──

#[derive(Debug, Clone)]
pub struct EconomicVarianceReport {
    pub matched: Vec<MatchedActivity>,
    pub total_planned: Option<Money>,
    pub total_actual: Option<Money>,
    pub total_variance: Option<Money>,
}

impl EconomicVarianceReport {
    pub fn new() -> Self {
        Self {
            matched: Vec::new(),
            total_planned: None,
            total_actual: None,
            total_variance: None,
        }
    }
}

// ── Config ──

#[derive(Debug, Clone)]
pub struct VarianceConfig {
    pub temporal_tolerance_days: i64,
    pub enable_confidence_scoring: bool,
}

impl Default for VarianceConfig {
    fn default() -> Self {
        Self {
            temporal_tolerance_days: 2,
            enable_confidence_scoring: true,
        }
    }
}

// ── Timing analysis service ──

pub struct VarianceService;

impl VarianceService {
    pub fn analyze_with_config(
        schedule: &Schedule,
        cycle: &CropCycle,
        config: &VarianceConfig,
    ) -> VarianceReport {
        let mut report = VarianceReport::new();
        let mut planned_by_category: HashMap<&ActivityCategory, VecDeque<&PlannedActivity>> = HashMap::new();

        for planned in schedule.activities() {
            planned_by_category
                .entry(&planned.category)
                .or_insert_with(VecDeque::new)
                .push_back(planned);
        }

        for record in cycle.executed_activities() {
            let category = &record.activity.category();

            if let Some(queue) = planned_by_category.get_mut(category) {
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
                    queue.remove(idx).unwrap();

                    let variance = if diff_days == 0 {
                        TimingVariance::OnTime
                    } else if diff_days > 0 {
                        TimingVariance::Late(diff_days)
                    } else {
                        TimingVariance::Early(diff_days.abs())
                    };

                    let confidence = if diff_days == 0 {
                        ConfidenceScore::High
                    } else if diff_days.abs() <= config.temporal_tolerance_days && config.enable_confidence_scoring {
                        ConfidenceScore::Medium
                    } else if diff_days.abs() <= config.temporal_tolerance_days {
                        ConfidenceScore::High
                    } else if config.enable_confidence_scoring {
                        ConfidenceScore::Low
                    } else {
                        ConfidenceScore::High
                    };

                    report.matched.push(MatchedActivity {
                        planned_id: planned.id.clone(),
                        record: record.clone(),
                        variance,
                        confidence,
                        cost_variance: None,
                    });
                } else {
                    report.unplanned.push(record.clone());
                }
            } else {
                report.unplanned.push(record.clone());
            }
        }

        for (_, queue) in planned_by_category.iter() {
            for planned in queue.iter().copied() {
                report.missing.push(planned.clone());
            }
        }

        report
    }
}

// ── Cost analysis service ──

pub struct EconomicVarianceService;

impl EconomicVarianceService {
    pub fn analyze_costs(
        matched: &[MatchedActivity],
        provider: &dyn EconomicDataProvider,
    ) -> EconomicVarianceReport {
        let mut report = EconomicVarianceReport::new();
        let mut total_planned: Option<Money> = None;
        let mut total_actual: Option<Money> = None;

        for m in matched {
            let planned_cost = provider.get_planned_cost(&m.planned_id);
            let activity_record_id = ActivityRecordId(m.record.activity.id().0.clone());
            let actual_cost = provider.get_actual_cost(&activity_record_id);

            match (planned_cost, actual_cost) {
                (Some(pc), Some(ac)) => {
                    match CostVariance::new(pc, ac) {
                        Ok(cost_var) => {
                            let mut enriched = m.clone();
                            enriched.cost_variance = Some(cost_var);
                            report.matched.push(enriched);

                            total_planned = merge_money(total_planned, pc);
                            total_actual = merge_money(total_actual, ac);
                        }
                        Err(_) => {
                            report.matched.push(m.clone());
                        }
                    }
                }
                _ => {
                    report.matched.push(m.clone());
                }
            }
        }

        report.total_planned = total_planned;
        report.total_actual = total_actual;
        report.total_variance = match (total_planned, total_actual) {
            (Some(tp), Some(ta)) => ta.subtract(&tp).ok(),
            _ => None,
        };

        report
    }
}

fn merge_money(acc: Option<Money>, val: Money) -> Option<Money> {
    match acc {
        None => Some(val),
        Some(m) => m.add(&val).ok().or(Some(m)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agriculture::activity::{Activity, ActivityCategory, ActivityRecord, IntegrityStatus};
    use crate::agriculture::planning::{ScheduleAnchor, ActivityStatus};
    use kora_kernel::ids::{CycleId, CropId, AreaId};
    use kora_kernel::period::Period;
    use kora_kernel::money::{Currency, Money};

    // ── Helpers ──

    fn test_schedule() -> Schedule {
        let mut s = Schedule::new(CycleId::new(), ScheduleAnchor::CycleStart, 1500);
        s.add_planned_activity(PlannedActivity {
            id: PlannedActivityId::new(),
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: ActivityStatus::Planned,
        });
        s.add_planned_activity(PlannedActivity {
            id: PlannedActivityId::new(),
            category: ActivityCategory::Maintenance,
            relative_day: 15,
            status: ActivityStatus::Planned,
        });
        s
    }

    fn test_cycle() -> CropCycle {
        CropCycle::new(CropId::new(), AreaId::new(), Period::new(1000, 2000).unwrap())
    }

    fn default_config() -> VarianceConfig {
        VarianceConfig { temporal_tolerance_days: 2, enable_confidence_scoring: true }
    }

    // ── Timing tests ──

    #[test]
    fn timing_on_time_high_confidence() {
        let schedule = test_schedule();
        let mut cycle = test_cycle();
        let activity = Activity::new(1500, ActivityCategory::Sowing);
        cycle.register_activity(activity).unwrap();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &default_config());
        assert_eq!(report.matched.len(), 1);
        assert_eq!(report.matched[0].variance, TimingVariance::OnTime);
        assert_eq!(report.matched[0].confidence, ConfidenceScore::High);
    }

    #[test]
    fn no_reuse_of_planned_activities() {
        let schedule = test_schedule();
        let mut cycle = test_cycle();
        cycle.register_activity(Activity::new(1500, ActivityCategory::Sowing)).unwrap();
        cycle.register_activity(Activity::new(1501, ActivityCategory::Sowing)).unwrap();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &default_config());
        assert_eq!(report.matched.len(), 1);
        assert_eq!(report.unplanned.len(), 1);
    }

    #[test]
    fn missing_when_no_activity() {
        let schedule = test_schedule();
        let cycle = test_cycle();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &default_config());
        assert_eq!(report.missing.len(), 2);
        assert_eq!(report.matched.len(), 0);
    }

    #[test]
    fn unplanned_activity() {
        let schedule = test_schedule();
        let mut cycle = test_cycle();
        cycle.register_activity(Activity::new(2000, ActivityCategory::Harvest)).unwrap();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &default_config());
        assert_eq!(report.unplanned.len(), 1);
        assert_eq!(report.missing.len(), 2);
    }

    #[test]
    fn within_tolerance_medium_confidence() {
        let schedule = test_schedule();
        let mut cycle = test_cycle();
        // Maintenance planned at relative_day 15 → expected 1515
        cycle.register_activity(Activity::new(1517, ActivityCategory::Maintenance)).unwrap();
        let report = VarianceService::analyze_with_config(&schedule, &cycle, &default_config());
        assert_eq!(report.matched.len(), 1);
        assert_eq!(report.matched[0].confidence, ConfidenceScore::Medium);
    }

    // ── Economic tests ──

    struct MockEconomicProvider {
        planned: std::collections::HashMap<String, Money>,
        actual: std::collections::HashMap<String, Money>,
    }

    impl EconomicDataProvider for MockEconomicProvider {
        fn get_planned_cost(&self, id: &PlannedActivityId) -> Option<Money> {
            self.planned.get(&id.0).copied()
        }
        fn get_actual_cost(&self, id: &ActivityRecordId) -> Option<Money> {
            self.actual.get(&id.0).copied()
        }
    }

    #[test]
    fn cost_variance_creation() {
        let cv = CostVariance::new(
            Money::new(rust_decimal::Decimal::from(100), Currency::USD),
            Money::new(rust_decimal::Decimal::from(120), Currency::USD),
        ).unwrap();
        assert_eq!(cv.variance.amount, rust_decimal::Decimal::from(20));
    }

    #[test]
    fn analyze_costs_with_mocked_provider() {
        let planned_id = PlannedActivityId::new();
        let mut provider = MockEconomicProvider {
            planned: std::collections::HashMap::new(),
            actual: std::collections::HashMap::new(),
        };
        provider.planned.insert(planned_id.0.clone(), Money::new(rust_decimal::Decimal::from(100), Currency::USD));
        let activity = Activity::new(1500, ActivityCategory::Sowing);
        let record = ActivityRecord::new(activity, vec![IntegrityStatus::Valid]);
        let record_id_str = record.activity.id().0.clone();
        provider.actual.insert(record_id_str, Money::new(rust_decimal::Decimal::from(120), Currency::USD));

        let matched = MatchedActivity {
            planned_id: planned_id.clone(),
            record,
            variance: TimingVariance::OnTime,
            confidence: ConfidenceScore::High,
            cost_variance: None,
        };

        let report = EconomicVarianceService::analyze_costs(&[matched], &provider);
        assert_eq!(report.matched.len(), 1);
        assert!(report.matched[0].cost_variance.is_some());
    }

    #[test]
    fn graceful_degradation_no_economic_data() {
        let provider = MockEconomicProvider {
            planned: std::collections::HashMap::new(),
            actual: std::collections::HashMap::new(),
        };
        let matched = MatchedActivity {
            planned_id: PlannedActivityId::new(),
            record: ActivityRecord::new(Activity::new(1500, ActivityCategory::Sowing), vec![IntegrityStatus::Valid]),
            variance: TimingVariance::OnTime,
            confidence: ConfidenceScore::High,
            cost_variance: None,
        };
        let report = EconomicVarianceService::analyze_costs(&[matched], &provider);
        assert_eq!(report.matched.len(), 1);
        assert!(report.matched[0].cost_variance.is_none());
    }

    // ── Orchestrated flow (timing + economic chained) ──

    fn run_orchestrated(economic_provider: Option<&dyn EconomicDataProvider>) -> (VarianceReport, Option<EconomicVarianceReport>) {
        let mut schedule = Schedule::new(CycleId::new(), ScheduleAnchor::SowingDate, 1500);
        schedule.add_planned_activity(PlannedActivity {
            id: PlannedActivityId::new(),
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: crate::agriculture::planning::ActivityStatus::Planned,
        });

        let period = Period::new(1000, 2000).unwrap();
        let mut cycle = CropCycle::new(CropId::new(), AreaId::new(), period);
        cycle.register_activity(Activity::new(1500, ActivityCategory::Sowing)).unwrap();

        let config = VarianceConfig { temporal_tolerance_days: 5, enable_confidence_scoring: true };
        let timing_report = VarianceService::analyze_with_config(&schedule, &cycle, &config);
        let economic_report = economic_provider.map(|p| EconomicVarianceService::analyze_costs(&timing_report.matched, p));
        (timing_report, economic_report)
    }

    #[test]
    fn orchestrated_timing_only() {
        let (timing, economic) = run_orchestrated(None);
        assert_eq!(timing.matched.len(), 1);
        assert_eq!(timing.matched[0].variance, TimingVariance::OnTime);
        assert!(economic.is_none());
    }

    #[test]
    fn orchestrated_timing_and_economic() {
        let provider = MockEconomicProvider {
            planned: std::collections::HashMap::new(),
            actual: std::collections::HashMap::new(),
        };
        let (timing, economic) = run_orchestrated(Some(&provider));
        assert_eq!(timing.matched.len(), 1);
        assert!(economic.is_some());
    }
}
