use serde::Serialize;

#[derive(Serialize)]
pub struct SoilSummary {
    pub id: String, pub area_id: String, pub sampled_at: i64,
    pub quality: String, pub cost: String, pub metric_count: usize,
}
