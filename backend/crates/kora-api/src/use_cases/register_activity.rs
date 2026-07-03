use kora_domain::agriculture::activity::{Activity, ActivityCategory, ActivityRecord, IntegrityStatus};
use kora_domain::agriculture::error::AgricultureError;
use kora_domain::agriculture::ids::PlannedActivityId;
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_domain::ports::schedule_repository::ScheduleRepository;
use kora_kernel::ids::CycleId;
use serde::Serialize;

const MATCH_WINDOW_DAYS: i64 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistrationMode {
    Suggested,
    ConfirmMatch(PlannedActivityId),
    Emergent,
}

pub struct RegisterActivityInput {
    pub cycle_id: CycleId,
    pub timestamp: i64,
    pub category: ActivityCategory,
    pub notes: Option<String>,
    pub mode: RegistrationMode,
}

#[derive(Serialize)]
pub struct PlannedSuggestion {
    pub planned_id: String,
    pub category: String,
    pub relative_day: i32,
    pub expected_timestamp: i64,
    pub drift_days: i64,
}

pub struct ActivityRegistration {
    pub record: ActivityRecord,
    pub suggestions: Vec<PlannedSuggestion>,
}

pub fn execute(
    state: &crate::state::AppState,
    input: RegisterActivityInput,
) -> Result<ActivityRegistration, AgricultureError> {
    let mut repo = state.cycle_repo.lock().unwrap();
    let mut cycle = repo
        .find_by_id(&input.cycle_id)
        .ok_or(AgricultureError::CycleNotFound(input.cycle_id.clone()))?;

    let category = input.category.clone();
    let mut activity = Activity::new(input.timestamp, category.clone());
    if let Some(notes) = input.notes {
        activity.set_notes(notes);
    }

    let record = cycle.register_activity(activity)?;
    let planned_id = match &input.mode {
        RegistrationMode::ConfirmMatch(pid) => Some(pid.clone()),
        _ => None,
    };

    let suggestions = collect_suggestions(state, &input.cycle_id, &category, input.timestamp);

    let final_record = annotate_match(record, planned_id, &category, &suggestions);
    repo.save(cycle);
    Ok(ActivityRegistration {
        record: final_record,
        suggestions,
    })
}

fn annotate_match(
    mut record: ActivityRecord,
    planned_id: Option<PlannedActivityId>,
    category: &ActivityCategory,
    suggestions: &[PlannedSuggestion],
) -> ActivityRecord {
    if let Some(pid) = planned_id {
        let matches = suggestions
            .iter()
            .any(|s| s.planned_id == pid.0 && suggestion_category_matches(&s.category, category));
        if matches {
            record.integrity.push(IntegrityStatus::MatchedPlanned(pid));
            return record;
        }
    }

    let best = suggestions
        .iter()
        .find(|s| suggestion_category_matches(&s.category, category));
    if best.is_none() {
        record.integrity.push(IntegrityStatus::Unplanned);
    }
    record
}

pub fn suggestion_category_matches(s: &str, expected: &ActivityCategory) -> bool {
    let s_norm = s.to_lowercase();
    match expected {
        ActivityCategory::Sowing => s_norm == "sowing",
        ActivityCategory::Maintenance => s_norm == "maintenance",
        ActivityCategory::SanitaryControl => s_norm == "sanitarycontrol",
        ActivityCategory::Harvest => s_norm == "harvest",
    }
}

fn collect_suggestions(
    state: &crate::state::AppState,
    cycle_id: &CycleId,
    category: &ActivityCategory,
    timestamp: i64,
) -> Vec<PlannedSuggestion> {
    let schedule = {
        let repo = state.schedule_repo.lock().unwrap();
        repo.find_by_cycle_id(cycle_id)
    };
    let Some(schedule) = schedule else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for planned in schedule.activities() {
        let expected_ts = schedule.anchor_date() + planned.relative_day as i64;
        let drift_days = timestamp - expected_ts;
        if drift_days.abs() > MATCH_WINDOW_DAYS {
            continue;
        }
        if !suggestion_category_matches(&format!("{:?}", planned.category), category) {
            continue;
        }
        out.push(PlannedSuggestion {
            planned_id: planned.id.0.clone(),
            category: format!("{:?}", planned.category),
            relative_day: planned.relative_day,
            expected_timestamp: expected_ts,
            drift_days,
        });
    }
    out.sort_by_key(|s| s.drift_days.abs());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_match_is_case_insensitive() {
        assert!(suggestion_category_matches("Sowing", &ActivityCategory::Sowing));
        assert!(suggestion_category_matches("sowing", &ActivityCategory::Sowing));
        assert!(suggestion_category_matches("Harvest", &ActivityCategory::Harvest));
        assert!(!suggestion_category_matches("Sowing", &ActivityCategory::Maintenance));
    }

    #[test]
    fn annotate_match_pushes_matched_planned_when_pid_matches() {
        let record = ActivityRecord::new(Activity::new(1000, ActivityCategory::Sowing), vec![IntegrityStatus::Valid]);
        let pid = PlannedActivityId::new();
        let suggestions = vec![PlannedSuggestion {
            planned_id: pid.0.clone(),
            category: "Sowing".to_string(),
            relative_day: 0,
            expected_timestamp: 1000,
            drift_days: 0,
        }];
        let annotated = annotate_match(record, Some(pid.clone()), &ActivityCategory::Sowing, &suggestions);
        assert!(annotated.integrity.iter().any(|i| matches!(i, IntegrityStatus::MatchedPlanned(_))));
    }

    #[test]
    fn annotate_match_pushes_unplanned_when_no_suggestions() {
        let record = ActivityRecord::new(Activity::new(1000, ActivityCategory::Sowing), vec![IntegrityStatus::Valid]);
        let annotated = annotate_match(record, None, &ActivityCategory::Sowing, &[]);
        assert!(annotated.integrity.iter().any(|i| matches!(i, IntegrityStatus::Unplanned)));
    }

    #[test]
    fn annotate_match_pushes_unplanned_when_suggestions_are_other_category() {
        let record = ActivityRecord::new(Activity::new(1000, ActivityCategory::Maintenance), vec![IntegrityStatus::Valid]);
        let suggestions = vec![PlannedSuggestion {
            planned_id: "x".to_string(),
            category: "Sowing".to_string(),
            relative_day: 0,
            expected_timestamp: 1000,
            drift_days: 0,
        }];
        let annotated = annotate_match(record, None, &ActivityCategory::Maintenance, &suggestions);
        assert!(annotated.integrity.iter().any(|i| matches!(i, IntegrityStatus::Unplanned)));
    }
}
