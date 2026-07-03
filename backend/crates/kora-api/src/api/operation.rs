use std::sync::Arc;
use axum::{extract::State, Json};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct StatusCounts {
    pub ok: u32,
    pub attention: u32,
    pub critical: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize)]
pub struct NextAction {
    pub title: String,
    pub field: String,
    pub crop: String,
    pub when: String,
    pub priority: Priority,
    pub reason: String,
    pub consequence: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AttentionKind {
    Delay,
    Budget,
    Info,
}

#[derive(Serialize)]
pub struct AttentionItem {
    pub kind: AttentionKind,
    pub text: String,
    pub metric: String,
}

#[derive(Serialize)]
pub struct OperationToday {
    pub context_note: String,
    pub status: StatusCounts,
    pub next_action: NextAction,
    pub attention: Vec<AttentionItem>,
}

pub async fn today(State(state): State<Arc<AppState>>) -> Json<OperationToday> {
    let cycles = state.list_cycles();
    let total = cycles.len() as u32;

    let next_action = build_next_action(&state);
    let (status, attention) = build_status_and_attention(&state);

    Json(OperationToday {
        context_note: format!("Operación · {total} ciclo(s) activos"),
        status,
        next_action,
        attention,
    })
}

fn build_next_action(state: &AppState) -> NextAction {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let schedules = state.schedule_repo.lock().unwrap().all();

    if let Some(cycle) = cycles.first() {
        if let Some(schedule) = schedules.iter().find(|s| s.cycle_id() == cycle.id()) {
            if let Some(planned) = schedule.activities().first() {
                return NextAction {
                    title: format!("{:?}", planned.category),
                    field: cycle.area_id().0.clone(),
                    crop: "Cultivo".to_string(),
                    when: format!("Día +{}", planned.relative_day),
                    priority: Priority::High,
                    reason: "Actividad planificada en el cronograma del ciclo".to_string(),
                    consequence: "Registrarla mantiene la trazabilidad de varianza".to_string(),
                };
            }
        }
    }

    NextAction {
        title: "Sin acciones pendientes".to_string(),
        field: "—".to_string(),
        crop: "—".to_string(),
        when: "—".to_string(),
        priority: Priority::Low,
        reason: "No hay cronogramas activos con actividades".to_string(),
        consequence: "Crea un ciclo para empezar a planificar".to_string(),
    }
}

fn build_status_and_attention(state: &AppState) -> (StatusCounts, Vec<AttentionItem>) {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let budgets = state.budget_repo.lock().unwrap().all();

    let mut ok = 0u32;
    let mut attention = 0u32;
    let mut critical = 0u32;
    let mut items = Vec::new();

    for cycle in &cycles {
        let has_activities = !cycle.executed_activities().is_empty();
        let budget = budgets.iter().find(|b| b.cycle_id() == cycle.id());
        let over_budget = budget
            .and_then(|b| b.get_variance().ok())
            .map(|v| v.amount.is_sign_positive())
            .unwrap_or(false);

        match (has_activities, over_budget) {
            (_, true) => critical += 1,
            (true, false) => ok += 1,
            (false, false) => attention += 1,
        }

        if over_budget {
            if let Some(b) = budget {
                if let Ok(v) = b.get_variance() {
                    items.push(AttentionItem {
                        kind: AttentionKind::Budget,
                        text: format!("Sobregiro en ciclo {}", cycle.id().0),
                        metric: format!("+{} USD", v.amount),
                    });
                }
            }
        }
        if !has_activities {
            items.push(AttentionItem {
                kind: AttentionKind::Delay,
                text: format!("Ciclo {} sin actividades", cycle.id().0),
                metric: "0 actividades".to_string(),
            });
        }
    }

    (StatusCounts { ok, attention, critical }, items)
}
