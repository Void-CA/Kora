mod error;
mod activity;
mod cycle;
mod farm;
mod area;
mod crop;
mod planning;
mod analysis;

pub mod services;  // Make public so other domains can access traits

pub use error::AgricultureError;
pub use activity::{ActivityRecord, Activity, ActivityCategory, IntegrityStatus, Input, Outcome};
pub use cycle::CropCycle;
pub use farm::Farm;
pub use area::Area;
pub use crop::Crop;
pub use planning::{Schedule, ScheduleAnchor, PlannedActivity, PlannedActivityId, ActivityStatus};
pub use analysis::AnalysisMetric;