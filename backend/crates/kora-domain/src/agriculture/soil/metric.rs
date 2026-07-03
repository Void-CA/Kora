use rust_decimal::Decimal;

use super::error::SoilError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SoilMetricKind {
    Ph,
    Nitrogen,
    Phosphorus,
    Potassium,
    OrganicMatter,
    Moisture,
    CationExchangeCapacity,
}

impl SoilMetricKind {
    pub fn expected_unit(self) -> &'static str {
        match self {
            SoilMetricKind::Ph => "pH",
            SoilMetricKind::Nitrogen => "%",
            SoilMetricKind::Phosphorus => "ppm",
            SoilMetricKind::Potassium => "ppm",
            SoilMetricKind::OrganicMatter => "%",
            SoilMetricKind::Moisture => "%",
            SoilMetricKind::CationExchangeCapacity => "cmol/kg",
        }
    }

    pub fn validate(self, value: Decimal) -> Result<(), SoilError> {
        let valid = match self {
            SoilMetricKind::Ph => value >= Decimal::from(0) && value <= Decimal::from(14),
            SoilMetricKind::Nitrogen => value >= Decimal::from(0) && value <= Decimal::from(100),
            SoilMetricKind::Phosphorus => value >= Decimal::from(0),
            SoilMetricKind::Potassium => value >= Decimal::from(0),
            SoilMetricKind::OrganicMatter => value >= Decimal::from(0) && value <= Decimal::from(100),
            SoilMetricKind::Moisture => value >= Decimal::from(0) && value <= Decimal::from(100),
            SoilMetricKind::CationExchangeCapacity => value >= Decimal::from(0),
        };
        if valid {
            Ok(())
        } else {
            Err(SoilError::InvalidMetricValue { kind: self, value })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QualityLevel {
    Basic,
    Complete,
    Satellite,
}

impl QualityLevel {
    pub fn required_minimum_metrics(&self) -> usize {
        match self {
            QualityLevel::Basic => 1,
            QualityLevel::Complete => 4,
            QualityLevel::Satellite => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoilMetric {
    kind: SoilMetricKind,
    value: Decimal,
    unit: String,
}

impl SoilMetric {
    pub fn new(kind: SoilMetricKind, value: Decimal) -> Result<Self, SoilError> {
        kind.validate(value)?;
        Ok(Self {
            kind,
            value,
            unit: kind.expected_unit().to_string(),
        })
    }

    pub fn kind(&self) -> SoilMetricKind {
        self.kind
    }

    pub fn value(&self) -> Decimal {
        self.value
    }

    pub fn unit(&self) -> &str {
        &self.unit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn ph_validates_range() {
        assert!(SoilMetricKind::Ph.validate(Decimal::from_str("6.5").unwrap()).is_ok());
        assert!(SoilMetricKind::Ph.validate(Decimal::from_str("15").unwrap()).is_err());
    }

    #[test]
    fn nitrogen_rejects_negative() {
        assert!(SoilMetricKind::Nitrogen.validate(Decimal::from(-1)).is_err());
    }

    #[test]
    fn metric_carries_expected_unit() {
        let m = SoilMetric::new(SoilMetricKind::Ph, Decimal::from_str("6.5").unwrap()).unwrap();
        assert_eq!(m.unit(), "pH");
    }

    #[test]
    fn quality_levels_have_minimums() {
        assert_eq!(QualityLevel::Basic.required_minimum_metrics(), 1);
        assert_eq!(QualityLevel::Complete.required_minimum_metrics(), 4);
        assert_eq!(QualityLevel::Satellite.required_minimum_metrics(), 2);
    }
}
