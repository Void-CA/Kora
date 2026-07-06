use serde::Serialize;
use crate::state::AppState;

#[derive(Serialize)]
pub struct OperationsToday {
    pub date: String,
    pub pending: Vec<ActivityCard>,
    pub in_progress: Vec<ActivityCard>,
    pub completed: Vec<ActivityCard>,
}

#[derive(Serialize)]
pub struct ActivityCard {
    pub id: String, pub title: String, pub field: String, pub crop: String,
    pub scheduled_time: String, pub status: String,
    pub responsible: Option<String>, pub notes: String,
}

pub fn execute(state: &AppState) -> OperationsToday {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let schedules = state.schedule_repo.lock().unwrap().all();

    let mut pending: Vec<ActivityCard> = Vec::new();
    let mut in_progress: Vec<ActivityCard> = Vec::new();
    let mut completed: Vec<ActivityCard> = Vec::new();

    for cycle in &cycles {
        let area_name = state.farms.iter().flat_map(|f| f.areas()).find(|a| a.id() == cycle.area_id()).map(|a| a.name()).unwrap_or("—");
        let schedule = schedules.iter().find(|s| s.cycle_id() == cycle.id());

        // Planned activities become "pending"
        if let Some(s) = schedule {
            for p in s.activities() {
                pending.push(ActivityCard {
                    id: p.id.0.clone(),
                    title: format!("{:?}", p.category),
                    field: area_name.to_string(),
                    crop: "—".into(),
                    scheduled_time: format!("Día +{}", p.relative_day),
                    status: "pending".into(),
                    responsible: None,
                    notes: String::new(),
                });
            }
        }

        // Executed activities become "completed"
        for a in cycle.executed_activities() {
            completed.push(ActivityCard {
                id: a.activity.id().0.clone(),
                title: format!("{:?}", a.activity.category()),
                field: area_name.to_string(),
                crop: "—".into(),
                scheduled_time: format!("{:?}", a.activity.timestamp()),
                status: "completed".into(),
                responsible: None,
                notes: a.activity.notes().unwrap_or("").to_string(),
            });
        }
    }

    pending.truncate(5);
    completed.truncate(5);

    OperationsToday { date: "hoy".into(), pending, in_progress, completed }
}
