// shared_kernel/ids.rs
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CycleId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CropId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AreaId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BudgetId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpenseId(pub String);

// --- IDs for other domains ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FarmId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScheduleId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkerId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivityId(pub String);

// --- Cross-domain IDs for cost tracking ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedActivityId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivityRecordId(pub String);

// Implement new() for all ID types
impl CycleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl CropId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl AreaId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl BudgetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl ExpenseId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl FarmId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl ScheduleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl WorkerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl ActivityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl PlannedActivityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ActivityRecordId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}