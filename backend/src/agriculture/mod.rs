// agriculture/mod.rs
pub mod domain;
pub mod application;
pub mod infrastructure;

pub use crate::agriculture::domain::farm::Farm;
pub use crate::agriculture::domain::area::Area;
pub use crate::agriculture::domain::crop::Crop;
pub use crate::agriculture::domain::services::variance_service::VarianceService;
pub use crate::agriculture::domain::services::economic_variance::EconomicVarianceService;
