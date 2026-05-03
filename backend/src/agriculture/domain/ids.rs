// agriculture/domain/ids.rs
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannedActivityId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivityRecordId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScheduleId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivityId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FarmId(pub String);

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

impl ScheduleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ActivityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FarmId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
