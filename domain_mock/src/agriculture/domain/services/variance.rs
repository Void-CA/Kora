// agriculture/domain/services/variance.rs

pub trait VarianceAnalyzer {
    type Report;
    fn analyze(&self, schedule: &Schedule, actuals: &[Activity]) -> Self::Report;
}

pub struct CostVarianceAnalyzer;
pub struct TimelineVarianceAnalyzer;

impl VarianceAnalyzer for CostVarianceAnalyzer {
    type Report = Vec<CostDeviation>;
    fn analyze(&self, schedule: &Schedule, actuals: &[Activity]) -> Self::Report {
        // logic only for money
    }
}