// shared_kernel/ids.rs
use uuid::Uuid;

// --- Genuinely shared IDs (used across multiple Bounded Contexts) ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CycleId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CropId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AreaId(pub String);

// Implement new() for shared IDs (UUID v4 internal representation)
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