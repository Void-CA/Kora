use serde::Serialize;

#[derive(Serialize)]
pub struct Profitability {
    pub baseline: String,
    pub spent: String,
    pub revenue: String,
    pub profit: String,
    pub roi_percent: String,
    pub remaining: String,
    pub variance: String,
}

#[derive(Serialize)]
pub struct RevenueSummary {
    pub id: String,
    pub cycle_id: Option<String>,
    pub amount: String,
    pub received_at: i64,
    pub source: String,
}
