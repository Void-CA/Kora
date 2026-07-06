use serde::Serialize;
#[derive(Serialize)]
pub struct IncidenceSummary {
    pub id: String, pub cycle_id: String, pub kind: String, pub severity: String,
    pub description: String, pub action_taken: String, pub detected_at: i64,
    pub resolved: bool, pub economic_impact: Option<String>,
}
