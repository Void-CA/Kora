use crate::agriculture::ids::{ActivityId, PlannedActivityId};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ActivityCategory {
    Sowing,
    Maintenance,
    SanitaryControl,
    Harvest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityStatus {
    Valid,
    OutsidePeriod,
    Unplanned,
    MatchedPlanned(PlannedActivityId),
}

#[derive(Debug, Clone)]
pub struct ActivityRecord {
    pub activity: Activity,
    pub integrity: Vec<IntegrityStatus>,
}

impl ActivityRecord {
    pub fn new(activity: Activity, integrity: Vec<IntegrityStatus>) -> Self {
        Self { activity, integrity }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub resource_name: String,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Debug, Clone)]
pub struct Outcome {
    pub result_name: String,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Debug, Clone)]
pub struct Activity {
    id: ActivityId,
    timestamp: i64,
    category: ActivityCategory,
    inputs: Vec<Input>,
    outcomes: Vec<Outcome>,
    notes: Option<String>,
}

impl Activity {
    pub fn new(timestamp: i64, category: ActivityCategory) -> Self {
        Self {
            id: ActivityId(uuid::Uuid::new_v4().to_string()),
            timestamp,
            category,
            inputs: Vec::new(),
            outcomes: Vec::new(),
            notes: None,
        }
    }

    pub fn add_input(&mut self, input: Input) {
        self.inputs.push(input);
    }

    pub fn add_outcome(&mut self, outcome: Outcome) {
        self.outcomes.push(outcome);
    }

    pub fn set_notes(&mut self, notes: String) {
        self.notes = Some(notes);
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub fn category(&self) -> &ActivityCategory {
        &self.category
    }

    pub fn id(&self) -> &ActivityId {
        &self.id
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_creates_with_unique_id() {
        let a = Activity::new(1500, ActivityCategory::Sowing);
        let b = Activity::new(1600, ActivityCategory::Harvest);
        assert_ne!(a.id().0, b.id().0);
    }

    #[test]
    fn activity_accessors() {
        let a = Activity::new(1500, ActivityCategory::Sowing);
        assert_eq!(a.timestamp(), 1500);
        assert_eq!(*a.category(), ActivityCategory::Sowing);
    }

    #[test]
    fn activity_record_holds_integrity() {
        let a = Activity::new(1500, ActivityCategory::Sowing);
        let r = ActivityRecord::new(a, vec![IntegrityStatus::Valid, IntegrityStatus::Unplanned]);
        assert_eq!(r.integrity.len(), 2);
    }
}
