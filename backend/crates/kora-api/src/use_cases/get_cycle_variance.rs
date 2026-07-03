use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::drift::{
    EconomicVarianceReport, EconomicVarianceService, MatchedActivity, TimingVariance,
    VarianceConfig, VarianceReport, VarianceService,
};
use kora_domain::agriculture::planning::PlannedActivity;
use kora_domain::agriculture::planning::Schedule;
use kora_domain::finance::budget::Budget;
use kora_domain::agriculture::activity::ActivityRecord;
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_domain::ports::schedule_repository::ScheduleRepository;
use kora_domain::ports::budget_repository::BudgetRepository;
use kora_kernel::ids::CycleId;
use kora_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::adapters::finance_economic_provider::FinanceEconomicProvider;
use crate::state::AppState;

#[derive(Serialize)]
pub struct TimingVarianceDto {
    pub kind: String,
    pub days: f64,
}

impl TimingVarianceDto {
    pub fn from(v: &TimingVariance) -> Self {
        match v {
            TimingVariance::OnTime => Self { kind: "on_time".into(), days: 0.0 },
            TimingVariance::Early(s) => Self { kind: "early".into(), days: *s as f64 / 86400.0 },
            TimingVariance::Late(s) => Self { kind: "late".into(), days: *s as f64 / 86400.0 },
        }
    }
}

#[derive(Serialize)]
pub struct CostVarianceDto {
    pub planned: String,
    pub actual: String,
    pub variance: String,
}

#[derive(Serialize)]
pub struct MatchedActivityDto {
    pub planned_id: String,
    pub activity_id: String,
    pub category: String,
    pub expected_timestamp: i64,
    pub actual_timestamp: i64,
    pub variance: TimingVarianceDto,
    pub confidence: String,
    pub cost: Option<CostVarianceDto>,
}

#[derive(Serialize)]
pub struct UnplannedActivityDto {
    pub activity_id: String,
    pub category: String,
    pub timestamp: i64,
    pub reason: String,
}

#[derive(Serialize)]
pub struct MissingPlannedDto {
    pub planned_id: String,
    pub category: String,
    pub relative_day: i32,
    pub expected_timestamp: i64,
}

#[derive(Serialize)]
pub struct CycleVariance {
    pub cycle_id: String,
    pub matched: Vec<MatchedActivityDto>,
    pub unplanned: Vec<UnplannedActivityDto>,
    pub missing: Vec<MissingPlannedDto>,
    pub totals: VarianceTotals,
}

#[derive(Serialize)]
pub struct VarianceTotals {
    pub matched_count: usize,
    pub unplanned_count: usize,
    pub missing_count: usize,
    pub total_planned: Option<String>,
    pub total_actual: Option<String>,
    pub total_cost_variance: Option<String>,
}

pub fn execute(state: &AppState, cycle_id: &CycleId) -> Option<CycleVariance> {
    let cycle: CropCycle = state.cycle_repo.lock().unwrap().find_by_id(cycle_id)?;
    let schedule: Option<Schedule> = state
        .schedule_repo
        .lock()
        .unwrap()
        .find_by_cycle_id(cycle_id);
    let budget: Option<Budget> = state
        .budget_repo
        .lock()
        .unwrap()
        .all()
        .into_iter()
        .find(|b| b.cycle_id() == cycle_id);

    let schedule = schedule?;
    let config = VarianceConfig::default();
    let timing_report: VarianceReport =
        VarianceService::analyze_with_config(&schedule, &cycle, &config);

    let economic_report: Option<EconomicVarianceReport> = budget.as_ref().and_then(|b| {
        let provider = FinanceEconomicProvider::new(b.id().clone(), state.budget_repo.clone());
        Some(EconomicVarianceService::analyze_costs(&timing_report.matched, &provider))
    });

    let expected_timestamps = build_expected_map(&schedule);
    let mut planned_by_id = std::collections::HashMap::new();
    for p in schedule.activities() {
        planned_by_id.insert(p.id.clone(), p);
    }

    let mut matched = Vec::with_capacity(timing_report.matched.len());
    for m in &timing_report.matched {
        let expected = expected_timestamps
            .get(&m.planned_id.0)
            .copied()
            .unwrap_or(0);
        let cost = economic_report
            .as_ref()
            .and_then(|r| {
                r.matched
                    .iter()
                    .find(|em| em.planned_id == m.planned_id)
                    .and_then(|em| em.cost_variance.as_ref())
            })
            .map(|c| CostVarianceDto {
                planned: format!("{} {:?}", c.planned_cost.amount, c.planned_cost.currency),
                actual: format!("{} {:?}", c.actual_cost.amount, c.actual_cost.currency),
                variance: format!("{} {:?}", c.variance.amount, c.variance.currency),
            });
        matched.push(MatchedActivityDto {
            planned_id: m.planned_id.0.clone(),
            activity_id: m.record.activity.id().0.clone(),
            category: format!("{:?}", m.record.activity.category()),
            expected_timestamp: expected,
            actual_timestamp: m.record.activity.timestamp(),
            variance: TimingVarianceDto::from(&m.variance),
            confidence: format!("{:?}", m.confidence),
            cost,
        });
    }

    let unplanned: Vec<UnplannedActivityDto> = timing_report
        .unplanned
        .iter()
        .map(|r: &ActivityRecord| {
            let reason = if has_planned_matched(r) {
                "no_planned_match_in_category".to_string()
            } else {
                "outside_schedule_scope".to_string()
            };
            UnplannedActivityDto {
                activity_id: r.activity.id().0.clone(),
                category: format!("{:?}", r.activity.category()),
                timestamp: r.activity.timestamp(),
                reason,
            }
        })
        .collect();

    let missing: Vec<MissingPlannedDto> = timing_report
        .missing
        .iter()
        .map(|p: &PlannedActivity| {
            let expected = expected_timestamps
                .get(&p.id.0)
                .copied()
                .unwrap_or(0);
            MissingPlannedDto {
                planned_id: p.id.0.clone(),
                category: format!("{:?}", p.category),
                relative_day: p.relative_day,
                expected_timestamp: expected,
            }
        })
        .collect();

    let totals = VarianceTotals {
        matched_count: matched.len(),
        unplanned_count: unplanned.len(),
        missing_count: missing.len(),
        total_planned: economic_report
            .as_ref()
            .and_then(|r| r.total_planned)
            .map(money_str),
        total_actual: economic_report
            .as_ref()
            .and_then(|r| r.total_actual)
            .map(money_str),
        total_cost_variance: economic_report
            .as_ref()
            .and_then(|r| r.total_variance)
            .map(money_str),
    };

    Some(CycleVariance {
        cycle_id: cycle.id().0.clone(),
        matched,
        unplanned,
        missing,
        totals,
    })
}

fn money_str(m: Money) -> String {
    format!("{} {:?}", m.amount, m.currency)
}

fn has_planned_matched(r: &ActivityRecord) -> bool {
    r.integrity.iter().any(|i| {
        matches!(i, kora_domain::agriculture::activity::IntegrityStatus::MatchedPlanned(_))
    })
}

fn build_expected_map(schedule: &Schedule) -> std::collections::HashMap<String, i64> {
    let mut out = std::collections::HashMap::new();
    for p in schedule.activities() {
        let ts = schedule.anchor_date() + p.relative_day as i64;
        out.insert(p.id.0.clone(), ts);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use kora_kernel::ids::{CropId, AreaId};
    use kora_kernel::period::Period;
    use kora_kernel::money::Currency;

    #[test]
    fn timing_variance_dto_maps_ontime() {
        let dto = TimingVarianceDto::from(&TimingVariance::OnTime);
        assert_eq!(dto.kind, "on_time");
        assert_eq!(dto.days, 0.0);
    }

    #[test]
    fn timing_variance_dto_maps_late() {
        let dto = TimingVarianceDto::from(&TimingVariance::Late(86400));
        assert_eq!(dto.kind, "late");
        assert_eq!(dto.days, 1.0);
    }

    #[test]
    fn timing_variance_dto_maps_early() {
        let dto = TimingVarianceDto::from(&TimingVariance::Early(2 * 86400));
        assert_eq!(dto.kind, "early");
        assert_eq!(dto.days, 2.0);
    }

    #[test]
    fn money_str_preserves_decimal() {
        let m = Money::new(Decimal::from(150), Currency::USD);
        let s = money_str(m);
        assert_eq!(s, "150 USD");
    }

    #[allow(dead_code)]
    fn _compile_check_uses(_: &MatchedActivity, _: &CropCycle, _: &CropId, _: &AreaId, _: &Period) {}
}
