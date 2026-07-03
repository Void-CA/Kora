use rust_decimal::Decimal;
use uuid::Uuid;

use kora_kernel::ids::AreaId;
use kora_kernel::money::Money;

use super::error::SoilError;
use super::metric::{QualityLevel, SoilMetric, SoilMetricKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SoilAnalysisId(pub String);

impl SoilAnalysisId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone)]
pub struct SoilAnalysis {
    id: SoilAnalysisId,
    area_id: AreaId,
    sampled_at: i64,
    quality: QualityLevel,
    metrics: Vec<SoilMetric>,
    cost: Money,
}

impl SoilAnalysis {
    pub fn new(
        area_id: AreaId,
        sampled_at: i64,
        quality: QualityLevel,
        cost: Money,
    ) -> Result<Self, SoilError> {
        if area_id.0.is_empty() {
            return Err(SoilError::EmptyAreaId);
        }
        Ok(Self {
            id: SoilAnalysisId::new(),
            area_id,
            sampled_at,
            quality,
            metrics: Vec::new(),
            cost,
        })
    }

    pub fn add_metric(&mut self, metric: SoilMetric) -> Result<(), SoilError> {
        if self.metrics.iter().any(|m| m.kind() == metric.kind()) {
            return Err(SoilError::DuplicateMetric(metric.kind()));
        }
        self.metrics.push(metric);
        Ok(())
    }

    pub fn finalize(self) -> Result<Self, SoilError> {
        let required = self.quality.required_minimum_metrics();
        let count = self.metrics.len();
        if count == 0 {
            return Err(SoilError::NoMetrics);
        }
        if count < required {
            return Err(SoilError::QualityBelowMinimum {
                required,
                provided: count,
            });
        }
        Ok(self)
    }

    pub fn id(&self) -> &SoilAnalysisId {
        &self.id
    }

    pub fn area_id(&self) -> &AreaId {
        &self.area_id
    }

    pub fn sampled_at(&self) -> i64 {
        self.sampled_at
    }

    pub fn quality(&self) -> &QualityLevel {
        &self.quality
    }

    pub fn metrics(&self) -> &[SoilMetric] {
        &self.metrics
    }

    pub fn cost(&self) -> &Money {
        &self.cost
    }

    pub fn metric(&self, kind: SoilMetricKind) -> Option<&SoilMetric> {
        self.metrics.iter().find(|m| m.kind() == kind)
    }

    pub fn value_of(&self, kind: SoilMetricKind) -> Option<Decimal> {
        self.metric(kind).map(|m| m.value())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkKind {
    Previo,
    Seguimiento,
    Posterior,
}

#[derive(Debug, Clone)]
pub struct SoilAnalysisLink {
    pub analysis_id: SoilAnalysisId,
    pub cycle_id: kora_kernel::ids::CycleId,
    pub kind: LinkKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use kora_kernel::money::Currency;
    use std::str::FromStr;

    fn ph(value: &str) -> SoilMetric {
        SoilMetric::new(SoilMetricKind::Ph, Decimal::from_str(value).unwrap()).unwrap()
    }

    fn n(value: &str) -> SoilMetric {
        SoilMetric::new(SoilMetricKind::Nitrogen, Decimal::from_str(value).unwrap()).unwrap()
    }

    fn p(value: &str) -> SoilMetric {
        SoilMetric::new(SoilMetricKind::Phosphorus, Decimal::from_str(value).unwrap()).unwrap()
    }

    fn k(value: &str) -> SoilMetric {
        SoilMetric::new(SoilMetricKind::Potassium, Decimal::from_str(value).unwrap()).unwrap()
    }

    fn cost() -> Money {
        Money::new(Decimal::from(150), Currency::USD)
    }

    #[test]
    fn build_basic_analysis_with_single_metric() {
        let mut a = SoilAnalysis::new(AreaId::new(), 1000, QualityLevel::Basic, cost()).unwrap();
        a.add_metric(ph("6.5")).unwrap();
        let a = a.finalize().unwrap();
        assert_eq!(a.metrics().len(), 1);
    }

    #[test]
    fn complete_requires_four_metrics() {
        let mut a = SoilAnalysis::new(AreaId::new(), 1000, QualityLevel::Complete, cost()).unwrap();
        a.add_metric(ph("6.5")).unwrap();
        a.add_metric(n("2.0")).unwrap();
        a.add_metric(p("30")).unwrap();
        let result = a.finalize();
        assert!(matches!(result, Err(SoilError::QualityBelowMinimum { required: 4, .. })));
    }

    #[test]
    fn complete_with_four_metrics_succeeds() {
        let mut a = SoilAnalysis::new(AreaId::new(), 1000, QualityLevel::Complete, cost()).unwrap();
        a.add_metric(ph("6.5")).unwrap();
        a.add_metric(n("2.0")).unwrap();
        a.add_metric(p("30")).unwrap();
        a.add_metric(k("150")).unwrap();
        let a = a.finalize().unwrap();
        assert_eq!(a.metrics().len(), 4);
    }

    #[test]
    fn rejects_duplicate_metric_kind() {
        let mut a = SoilAnalysis::new(AreaId::new(), 1000, QualityLevel::Basic, cost()).unwrap();
        a.add_metric(ph("6.5")).unwrap();
        let result = a.add_metric(ph("7.0"));
        assert!(matches!(result, Err(SoilError::DuplicateMetric(SoilMetricKind::Ph))));
    }

    #[test]
    fn empty_metrics_fails_finalize() {
        let a = SoilAnalysis::new(AreaId::new(), 1000, QualityLevel::Basic, cost()).unwrap();
        assert!(matches!(a.finalize(), Err(SoilError::NoMetrics)));
    }

    #[test]
    fn empty_area_id_rejected() {
        let result = SoilAnalysis::new(AreaId(String::new()), 1000, QualityLevel::Basic, cost());
        assert!(matches!(result, Err(SoilError::EmptyAreaId)));
    }
}
