// shared_kernel/ids.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FarmId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AreaId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CropId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CycleId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivityId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkerId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScheduleId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BudgetId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpenseId(pub String);