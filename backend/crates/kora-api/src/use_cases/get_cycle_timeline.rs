use kora_domain::agriculture::activity::{ActivityRecord, IntegrityStatus};
use kora_domain::agriculture::planning::Schedule;
use kora_domain::finance::revenue::Revenue;
use kora_domain::finance::payroll::PayrollEntry;
use kora_domain::finance::budget::Budget;
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_domain::ports::schedule_repository::ScheduleRepository;
use kora_domain::ports::budget_repository::BudgetRepository;
use kora_domain::ports::revenue_repository::RevenueRepository;
use kora_domain::ports::payroll_entry_repository::PayrollEntryRepository;
use kora_domain::ports::sanitary_incidence_repository::SanitaryIncidenceRepository;
use kora_kernel::ids::CycleId;
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct TimelineEvent {
    pub kind: String,
    pub timestamp: i64,
    pub label: String,
    pub detail: String,
    pub integrity: Vec<String>,
}

#[derive(Serialize)]
pub struct CycleTimeline {
    pub cycle_id: String,
    pub crop_id: String,
    pub area_id: String,
    pub period_start: i64,
    pub period_end: i64,
    pub planned: Vec<TimelineEvent>,
    pub executed: Vec<TimelineEvent>,
    pub expenses: Vec<TimelineEvent>,
    pub revenues: Vec<TimelineEvent>,
    pub payroll: Vec<TimelineEvent>,
    pub incidences: Vec<TimelineEvent>,
}

pub fn execute(state: &AppState, cycle_id: &CycleId) -> Option<CycleTimeline> {
    let cycle = state.cycle_repo.lock().unwrap().find_by_id(cycle_id)?;
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

    let revenues: Vec<Revenue> = state.revenue_repo.lock().unwrap().for_cycle(cycle_id);
    let payroll: Vec<PayrollEntry> = state.payroll_repo.lock().unwrap().for_cycle(cycle_id);
    let incidences = state.incidence_repo.lock().unwrap().for_cycle(cycle_id);

    let mut planned = Vec::new();
    if let Some(s) = schedule {
        for p in s.activities() {
            let ts = s.anchor_date() + p.relative_day as i64;
            planned.push(event(
                "planned",
                ts,
                &format!("{:?}", p.category),
                &format!("día relativo +{}", p.relative_day),
                &[],
            ));
        }
    }
    let _ = budget;

    let executed = cycle.executed_activities().iter().map(activity_event).collect();

    let expenses: Vec<TimelineEvent> = Vec::new();

    let revenue_events: Vec<TimelineEvent> = revenues
        .iter()
        .map(|r| {
            event(
                "revenue",
                r.received_at(),
                &format!("{:?}", r.source()),
                &format!("{} {:?}", r.amount().amount, r.amount().currency),
                &[],
            )
        })
        .collect();

    let payroll_events: Vec<TimelineEvent> = payroll
        .iter()
        .map(|p| {
            event(
                "payroll",
                p.paid_at(),
                "Planilla",
                &format!("{} {:?}", p.amount().amount, p.amount().currency),
                &[],
            )
        })
        .collect();

    let incidence_events: Vec<TimelineEvent> = incidences
        .iter()
        .map(|i| {
            event(
                "incidence",
                i.detected_at(),
                &format!("{:?}", i.kind()),
                &format!("severidad {:?}", i.severity()),
                &[],
            )
        })
        .collect();

    Some(CycleTimeline {
        cycle_id: cycle.id().0.clone(),
        crop_id: cycle.crop_id().0.clone(),
        area_id: cycle.area_id().0.clone(),
        period_start: cycle.period().start(),
        period_end: cycle.period().end(),
        planned,
        executed,
        expenses,
        revenues: revenue_events,
        payroll: payroll_events,
        incidences: incidence_events,
    })
}

fn activity_event(r: &ActivityRecord) -> TimelineEvent {
    let integrity: Vec<String> = r
        .integrity
        .iter()
        .map(|i| match i {
            IntegrityStatus::Valid => "valid".to_string(),
            IntegrityStatus::OutsidePeriod => "outside_period".to_string(),
            IntegrityStatus::Unplanned => "unplanned".to_string(),
            IntegrityStatus::MatchedPlanned(pid) => format!("matched_planned:{}", pid.0),
        })
        .collect();
    event(
        "activity",
        r.activity.timestamp(),
        &format!("{:?}", r.activity.category()),
        &r.activity.notes().unwrap_or("").to_string(),
        &integrity,
    )
}

fn event(kind: &str, ts: i64, label: &str, detail: &str, integrity: &[String]) -> TimelineEvent {
    TimelineEvent {
        kind: kind.to_string(),
        timestamp: ts,
        label: label.to_string(),
        detail: detail.to_string(),
        integrity: integrity.to_vec(),
    }
}

#[allow(dead_code)]
fn _planned_marker(_: &kora_domain::agriculture::planning::PlannedActivity) {}