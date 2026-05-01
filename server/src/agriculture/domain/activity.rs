// agriculture/activity.rs
use crate::shared_kernel::ids::ActivityId;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ActivityCategory {
    Sowing,
    Maintenance,
    SanitaryControl, // Fumigación, poda, etc.
    Harvest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntegrityStatus {
    Valid,
    OutsidePeriod,
    Unplanned,
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
    pub resource_name: String, // Podría ser un ResourceId en el futuro
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
    inputs: Vec<Input>,      // Ej: Fertilizante usado
    outcomes: Vec<Outcome>,  // Ej: Kilos cosechados
    notes: Option<String>,
}

impl ActivityId {
    /// Returns the inner string for cross-domain compatibility.
    pub fn as_str(&self) -> &str {
        &self.0
    }
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