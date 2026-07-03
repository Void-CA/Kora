use crate::error::MeasurementError;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_zero() {
        assert!(AreaMeasurement::new(0.0, AreaUnit::Hectares).is_err());
    }

    #[test]
    fn new_rejects_negative() {
        assert!(AreaMeasurement::new(-1.0, AreaUnit::Hectares).is_err());
    }

    #[test]
    fn hectares_stays_unchanged() {
        let m = AreaMeasurement::new(5.0, AreaUnit::Hectares).unwrap();
        assert!((m.value_in_hectares() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn square_meters_to_hectares() {
        let m = AreaMeasurement::new(25_000.0, AreaUnit::SquareMeters).unwrap();
        assert!((m.value_in_hectares() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn acres_to_hectares() {
        let m = AreaMeasurement::new(10.0, AreaUnit::Acres).unwrap();
        assert!((m.value_in_hectares() - 4.04686).abs() < 1e-5);
    }
}
