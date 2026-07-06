use serde::Serialize;
#[derive(Serialize)]
pub struct WorkerSummary { pub id: String, pub name: String, pub role: Option<String>, pub active: bool }
#[derive(Serialize)]
pub struct PayrollEntrySummary { pub id: String, pub worker_id: String, pub amount: String, pub paid_at: i64, pub cycle_id: Option<String>, pub area_id: Option<String> }
