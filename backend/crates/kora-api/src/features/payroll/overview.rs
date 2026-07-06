use serde::Serialize;
use crate::state::AppState;

#[derive(Serialize)]
pub struct TeamOverview {
    pub title: String,
    pub today: TeamTodayStatus,
    pub workers: Vec<TeamWorker>,
    pub recent_payments: Vec<PaymentEntry>,
}

#[derive(Serialize)]
pub struct TeamTodayStatus { pub working: usize, pub absent: usize, pub total: usize }

#[derive(Serialize)]
pub struct TeamWorker {
    pub id: String, pub name: String, pub role: String,
    pub status: String, pub today_activity: Option<String>, pub last_payment: Option<String>,
}

#[derive(Serialize)]
pub struct PaymentEntry {
    pub worker: String, pub amount: String, pub date: String, pub cycle: String,
}

pub fn execute(state: &AppState) -> TeamOverview {
    let workers = state.worker_repo.lock().unwrap().all();
    let active_count = workers.iter().filter(|w| w.is_active()).count();
    let inactive_count = workers.len() - active_count;

    TeamOverview {
        title: "Equipo".into(),
        today: TeamTodayStatus { working: active_count, absent: inactive_count, total: workers.len() },
        workers: workers.iter().map(|w| TeamWorker {
            id: w.id().0.clone(), name: w.name().to_string(),
            role: w.role().map(|r| format!("{:?}", r)).unwrap_or_default(),
            status: if w.is_active() { "working".into() } else { "absent".into() },
            today_activity: None, last_payment: None,
        }).collect(),
        recent_payments: vec![],
    }
}
