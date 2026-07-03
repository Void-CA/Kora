pub mod analysis;
pub mod error;
pub mod metric;

pub use analysis::{SoilAnalysis, SoilAnalysisId, SoilAnalysisLink, LinkKind};
pub use error::SoilError;
pub use metric::{SoilMetric, SoilMetricKind, QualityLevel};
