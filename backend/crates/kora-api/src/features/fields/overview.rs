use serde::Serialize;
use crate::state::AppState;

#[derive(Serialize)]
pub struct FieldsOverview {
    pub title: String,
    pub fields: Vec<FieldCardData>,
}

#[derive(Serialize)]
pub struct FieldCardData {
    pub id: String, pub name: String, pub crop: String, pub hectares: f64,
    pub progress_percent: u32, pub days_to_harvest: i64,
    pub days_since_last_activity: i64, pub last_activity_name: String,
    pub responsible: String, pub cost_accumulated: String,
    pub health: String,
    pub phases: Vec<PhaseData>,
}

#[derive(Serialize)]
pub struct PhaseData {
    pub name: String, pub status: String, pub day: u32, pub total: u32,
}

pub fn execute(state: &AppState) -> FieldsOverview {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let budgets = state.budget_repo.lock().unwrap().all();
    let areas: Vec<_> = state.farms.iter().flat_map(|f| f.areas()).collect();

    let fields = areas.iter().map(|area| {
        let cycle = cycles.iter().find(|c| c.area_id() == area.id());
        let budget = cycle.and_then(|c| budgets.iter().find(|b| b.cycle_id() == c.id()));
        let has_activities = cycle.map(|c| !c.executed_activities().is_empty()).unwrap_or(false);
        let last_act = cycle.and_then(|c| c.executed_activities().iter().max_by_key(|r| r.activity.timestamp()));
        let over_budget = budget.and_then(|b| b.get_variance().ok()).map(|v| v.amount.is_sign_positive()).unwrap_or(false);
        let health = if over_budget { "critical".into() } else if !has_activities { "attention".into() } else { "ok".into() };

        FieldCardData {
            id: area.id().0.clone(),
            name: area.name().to_string(),
            crop: cycle.map(|c| "—".into()).unwrap_or_else(|| "—".into()),
            hectares: area.measurement().value_in_hectares(),
            progress_percent: cycle.map(|c| { let d = c.period().end() - c.period().start(); if d > 0 { let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64; ((now - c.period().start()).max(0) as f64 / d as f64 * 100.0) as u32 } else { 50 } }).unwrap_or(0).min(100),
            days_to_harvest: 23,
            days_since_last_activity: last_act.map(|a| (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64 - a.activity.timestamp()) / 86400).unwrap_or(0),
            last_activity_name: last_act.map(|a| format!("{:?}", a.activity.category())).unwrap_or_else(|| "—".into()),
            responsible: "—".into(),
            cost_accumulated: budget.map(|b| format!("${}", b.current_expenses().amount)).unwrap_or_else(|| "$0".into()),
            health,
            phases: vec![
                PhaseData { name: "Preparación".into(), status: "done".into(), day: 20, total: 20 },
                PhaseData { name: "Siembra".into(), status: "done".into(), day: 5, total: 5 },
                PhaseData { name: "Crecimiento".into(), status: "current".into(), day: 12, total: 45 },
                PhaseData { name: "Floración".into(), status: "pending".into(), day: 0, total: 20 },
                PhaseData { name: "Cosecha".into(), status: "pending".into(), day: 0, total: 15 },
            ],
        }
    }).collect();

    FieldsOverview { title: "Campos".into(), fields }
}
