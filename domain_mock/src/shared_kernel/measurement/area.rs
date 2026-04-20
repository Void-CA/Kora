// shared_kernel/measurements/mod.rs
use super::error::MeasurementError;

#[derive(Debug, Clone, PartialEq)]
pub enum AreaUnit {
    Hectares,
    SquareMeters,
    Acres,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaMeasurement {
    value: f64,
    unit: AreaUnit,
}

impl AreaMeasurement {
    pub fn new(value: f64, unit: AreaUnit) -> Result<Self, MeasurementError> {
        if value <= 0.0 {
            return Err(MeasurementError::MustBePositive);
        }
        Ok(Self { value, unit })
    }

    pub fn value_in_hectares(&self) -> f64 {
        match self.unit {
            AreaUnit::Hectares => self.value,
            AreaUnit::SquareMeters => self.value / 10_000.0,
            AreaUnit::Acres => self.value * 0.404686,
        }
    }
}