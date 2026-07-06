use kora_domain::agriculture::activity::{Activity, ActivityCategory, ActivityRecord, IntegrityStatus};
use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::drift::{
    EconomicVarianceReport, EconomicVarianceService, VarianceConfig, VarianceReport, VarianceService,
};
use kora_domain::agriculture::error::AgricultureError;
use kora_domain::agriculture::ids::PlannedActivityId;
use kora_domain::agriculture::planning::{PlannedActivity, Schedule};
use kora_domain::agriculture::planning_service::CropPlanningService;
use kora_domain::agriculture::crop::Crop;
use kora_domain::finance::budget::Budget;
use kora_domain::finance::revenue::Revenue;
use kora_domain::finance::payroll::PayrollEntry;
use kora_kernel::ids::{AreaId, CropId, CycleId};
use kora_kernel::money::Money;
use kora_kernel::period::Period;
use crate::adapters::finance_economic_provider::FinanceEconomicProvider;
use crate::state::AppState;
use crate::features::cycles::dto::*;

const MATCH_WINDOW_DAYS: i64 = 2;

// ── Register cycle ──

pub fn register_cycle(state: &AppState, input: RegisterCycleInput) -> Result<CropCycle, AgricultureError> {
    let farm = state.farm_for_area(&input.area_id).ok_or_else(|| AgricultureError::AreaNotFound(input.area_id.clone()))?;
    let crop = Crop::new(input.crop_id, "Cultivo".into());
    let existing: Vec<CropCycle> = state.cycle_repo.lock().unwrap().all();
    let planning = CropPlanningService::schedule_cycle(farm, &input.area_id, &crop, input.period, &existing)?;
    let mut schedule = planning.schedule;
    for (cat, day) in input.planned_activities {
        schedule.add_planned_activity(PlannedActivity::new(cat, day));
    }
    state.schedule_repo.lock().unwrap().save(schedule);
    state.cycle_repo.lock().unwrap().save(planning.cycle.clone());
    Ok(planning.cycle)
}

pub struct RegisterCycleInput {
    pub crop_id: CropId, pub area_id: AreaId, pub period: Period,
    pub planned_activities: Vec<(ActivityCategory, i32)>,
}

// ── Register activity ──

pub enum RegistrationMode {
    Suggested, ConfirmMatch(PlannedActivityId), Emergent,
}

pub struct RegisterActivityInput {
    pub cycle_id: CycleId, pub timestamp: i64, pub category: ActivityCategory,
    pub notes: Option<String>, pub mode: RegistrationMode,
}

pub struct ActivityRegistration {
    pub record: ActivityRecord, pub suggestions: Vec<PlannedSuggestion>,
}

pub fn register_activity(state: &AppState, input: RegisterActivityInput) -> Result<ActivityRegistration, AgricultureError> {
    let mut repo = state.cycle_repo.lock().unwrap();
    let mut cycle = repo.find_by_id(&input.cycle_id).ok_or(AgricultureError::CycleNotFound(input.cycle_id.clone()))?;
    let category = input.category.clone();
    let mut activity = Activity::new(input.timestamp, category.clone());
    if let Some(notes) = input.notes { activity.set_notes(notes); }
    let record = cycle.register_activity(activity)?;
    let planned_id = match &input.mode { RegistrationMode::ConfirmMatch(pid) => Some(pid.clone()), _ => None };
    let suggestions = collect_suggestions(state, &input.cycle_id, &category, input.timestamp);
    let final_record = annotate_match(record, planned_id, &category, &suggestions);
    repo.save(cycle);
    Ok(ActivityRegistration { record: final_record, suggestions })
}

fn annotate_match(mut record: ActivityRecord, planned_id: Option<PlannedActivityId>, category: &ActivityCategory, suggestions: &[PlannedSuggestion]) -> ActivityRecord {
    if let Some(pid) = planned_id {
        if suggestions.iter().any(|s| s.planned_id == pid.0 && s.category.to_lowercase() == format!("{:?}", category).to_lowercase()) {
            record.integrity.push(IntegrityStatus::MatchedPlanned(pid)); return record;
        }
    }
    if !suggestions.iter().any(|s| s.category.to_lowercase() == format!("{:?}", category).to_lowercase()) {
        record.integrity.push(IntegrityStatus::Unplanned);
    }
    record
}

fn collect_suggestions(state: &AppState, cycle_id: &CycleId, category: &ActivityCategory, timestamp: i64) -> Vec<PlannedSuggestion> {
    let schedule = { let repo = state.schedule_repo.lock().unwrap(); repo.find_by_cycle_id(cycle_id) };
    let Some(schedule) = schedule else { return Vec::new(); };
    let mut out: Vec<PlannedSuggestion> = schedule.activities().iter().filter(|p| {
        let expected = schedule.anchor_date() + p.relative_day as i64;
        (timestamp - expected).abs() <= MATCH_WINDOW_DAYS
            && format!("{:?}", p.category).to_lowercase() == format!("{:?}", category).to_lowercase()
    }).map(|p| {
        let expected = schedule.anchor_date() + p.relative_day as i64;
        PlannedSuggestion {
            planned_id: p.id.0.clone(), category: format!("{:?}", p.category),
            relative_day: p.relative_day, expected_timestamp: expected,
            drift_days: timestamp - expected,
        }
    }).collect();
    out.sort_by_key(|s| s.drift_days.abs());
    out
}

// ── Timeline ──

pub fn get_timeline(state: &AppState, cycle_id: &CycleId) -> Option<CycleTimeline> {
    let cycle = state.cycle_repo.lock().unwrap().find_by_id(cycle_id)?;
    let schedule: Option<Schedule> = state.schedule_repo.lock().unwrap().find_by_cycle_id(cycle_id);
    let revenues: Vec<Revenue> = state.revenue_repo.lock().unwrap().for_cycle(cycle_id);
    let payroll: Vec<PayrollEntry> = state.payroll_repo.lock().unwrap().for_cycle(cycle_id);
    let incidences = state.incidence_repo.lock().unwrap().for_cycle(cycle_id);

    let mut planned = Vec::new();
    if let Some(s) = &schedule {
        for p in s.activities() {
            planned.push(TimelineEvent {
                kind: "planned".into(), timestamp: s.anchor_date() + p.relative_day as i64,
                label: format!("{:?}", p.category), detail: format!("día +{}", p.relative_day), integrity: vec![],
            });
        }
    }
    let executed: Vec<TimelineEvent> = cycle.executed_activities().iter().map(|r| TimelineEvent {
        kind: "activity".into(), timestamp: r.activity.timestamp(), label: format!("{:?}", r.activity.category()),
        detail: r.activity.notes().unwrap_or("").to_string(),
        integrity: r.integrity.iter().map(|i| format!("{:?}", i)).collect(),
    }).collect();

    Some(CycleTimeline {
        cycle_id: cycle.id().0.clone(), crop_id: cycle.crop_id().0.clone(), area_id: cycle.area_id().0.clone(),
        period_start: cycle.period().start(), period_end: cycle.period().end(),
        planned, executed, expenses: vec![],
        revenues: revenues.iter().map(|r| TimelineEvent { kind: "revenue".into(), timestamp: r.received_at(), label: format!("{:?}", r.source()), detail: format!("{} {:?}", r.amount().amount, r.amount().currency), integrity: vec![] }).collect(),
        payroll: payroll.iter().map(|p| TimelineEvent { kind: "payroll".into(), timestamp: p.paid_at(), label: "Planilla".into(), detail: format!("{} {:?}", p.amount().amount, p.amount().currency), integrity: vec![] }).collect(),
        incidences: incidences.iter().map(|i| TimelineEvent { kind: "incidence".into(), timestamp: i.detected_at(), label: format!("{:?}", i.kind()), detail: format!("severidad {:?}", i.severity()), integrity: vec![] }).collect(),
    })
}

// ── Variance ──

pub fn get_variance(state: &AppState, cycle_id: &CycleId) -> Option<CycleVariance> {
    let cycle = state.cycle_repo.lock().unwrap().find_by_id(cycle_id)?;
    let schedule = state.schedule_repo.lock().unwrap().find_by_cycle_id(cycle_id)?;
    let budget: Option<Budget> = state.budget_repo.lock().unwrap().all().into_iter().find(|b| b.cycle_id() == cycle_id);
    let config = VarianceConfig::default();
    let timing: VarianceReport = VarianceService::analyze_with_config(&schedule, &cycle, &config);
    let economic: Option<EconomicVarianceReport> = budget.as_ref().map(|b| {
        let provider = FinanceEconomicProvider::new(b.id().clone(), state.budget_repo.clone());
        EconomicVarianceService::analyze_costs(&timing.matched, &provider)
    });
    let expected = |pid: &str| schedule.activities().iter().find(|p| p.id.0 == pid).map(|p| schedule.anchor_date() + p.relative_day as i64);

    let matched: Vec<MatchedActivity> = timing.matched.iter().map(|m| {
        let cost = economic.as_ref().and_then(|r| r.matched.iter().find(|em| em.planned_id == m.planned_id).and_then(|em| em.cost_variance.as_ref()))
            .map(|c| CostVariance { planned: format!("{} {:?}", c.planned_cost.amount, c.planned_cost.currency), actual: format!("{} {:?}", c.actual_cost.amount, c.actual_cost.currency), variance: format!("{} {:?}", c.variance.amount, c.variance.currency) });
        MatchedActivity {
            planned_id: m.planned_id.0.clone(), activity_id: m.record.activity.id().0.clone(), category: format!("{:?}", m.record.activity.category()),
            expected_timestamp: expected(&m.planned_id.0).unwrap_or(0), actual_timestamp: m.record.activity.timestamp(),
            variance: match &m.variance { kora_domain::agriculture::drift::TimingVariance::OnTime => TimingVariance { kind: "on_time".into(), days: 0.0 }, kora_domain::agriculture::drift::TimingVariance::Early(s) => TimingVariance { kind: "early".into(), days: *s as f64 / 86400.0 }, kora_domain::agriculture::drift::TimingVariance::Late(s) => TimingVariance { kind: "late".into(), days: *s as f64 / 86400.0 } },
            confidence: format!("{:?}", m.confidence), cost,
        }
    }).collect();

    let matched_count = matched.len();
    Some(CycleVariance {
        cycle_id: cycle.id().0.clone(), matched,
        unplanned: timing.unplanned.iter().map(|r| UnplannedActivity { activity_id: r.activity.id().0.clone(), category: format!("{:?}", r.activity.category()), timestamp: r.activity.timestamp(), reason: "unplanned".into() }).collect(),
        missing: timing.missing.iter().map(|p| MissingPlanned { planned_id: p.id.0.clone(), category: format!("{:?}", p.category), relative_day: p.relative_day, expected_timestamp: expected(&p.id.0).unwrap_or(0) }).collect(),
        totals: VarianceTotals {
            matched_count, unplanned_count: timing.unplanned.len(), missing_count: timing.missing.len(),
            total_planned: economic.as_ref().and_then(|r| r.total_planned).map(|m| format!("{} {:?}", m.amount, m.currency)),
            total_actual: economic.as_ref().and_then(|r| r.total_actual).map(|m| format!("{} {:?}", m.amount, m.currency)),
            total_cost_variance: economic.as_ref().and_then(|r| r.total_variance).map(|m| format!("{} {:?}", m.amount, m.currency)),
        },
    })
}

// ── List ──

pub fn list_cycles(state: &AppState) -> Vec<CycleSummary> {
    state.cycle_repo.lock().unwrap().all().iter().map(|c| CycleSummary { id: c.id().0.clone(), crop_id: c.crop_id().0.clone(), area_id: c.area_id().0.clone(), period_start: c.period().start(), period_end: c.period().end(), activity_count: c.executed_activities().len() }).collect()
}

pub fn get_cycle(state: &AppState, cycle_id: &CycleId) -> Option<CycleDetail> {
    let cycle = state.cycle_repo.lock().unwrap().find_by_id(cycle_id)?;
    let schedule = state.schedule_repo.lock().unwrap().find_by_cycle_id(cycle_id);
    Some(CycleDetail {
        summary: CycleSummary { id: cycle.id().0.clone(), crop_id: cycle.crop_id().0.clone(), area_id: cycle.area_id().0.clone(), period_start: cycle.period().start(), period_end: cycle.period().end(), activity_count: cycle.executed_activities().len() },
        activities: cycle.executed_activities().iter().map(|r| ActivitySummary { id: r.activity.id().0.clone(), category: format!("{:?}", r.activity.category()), timestamp: r.activity.timestamp(), integrity: r.integrity.iter().map(|i| format!("{:?}", i)).collect() }).collect(),
        planned_activities: schedule.map(|s| s.activities().iter().map(|p| PlannedSummary { id: p.id.0.clone(), category: format!("{:?}", p.category), relative_day: p.relative_day }).collect()).unwrap_or_default(),
    })
}
