// agriculture/domain/analysis.rs

pub enum MetricKind {
    Ph,
    Nitrogen,
    Phosphorus,
    Potassium,
    OrganicMatter,
    Conductivity,
    Custom(String), // Fallback for specialized lab tests
}

pub enum Unit {
    Percentage,
    Ppm,          // Parts per million
    MgKg,         // Milligrams per kilogram
    Millisiemens, // For conductivity
    Index,        // For pH (dimensionless)
}

pub struct AnalysisMetric {
    pub kind: MetricKind,
    pub value: f64,
    pub unit: Unit, 
}