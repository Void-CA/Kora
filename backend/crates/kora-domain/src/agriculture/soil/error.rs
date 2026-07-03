use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub enum SoilError {
    DuplicateMetric(super::SoilMetricKind),
    NoMetrics,
    InvalidMetricValue { kind: super::SoilMetricKind, value: Decimal },
    QualityBelowMinimum { required: usize, provided: usize },
    EmptyAreaId,
}
