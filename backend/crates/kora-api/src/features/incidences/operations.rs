use kora_domain::agriculture::incidence::{SanitaryIncidence, IncidenceError, IncidenceType, Severity};
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;
use crate::state::AppState;
use crate::features::incidences::dto::IncidenceSummary;

pub fn register(state: &AppState, cycle_id: CycleId, kind: IncidenceType, severity: Severity, description: String, action_taken: String, detected_at: i64, economic_impact: Option<Money>) -> Result<SanitaryIncidence, IncidenceError> {
    let mut inc = SanitaryIncidence::new(cycle_id, kind, severity, description, action_taken, detected_at)?;
    if let Some(impact) = economic_impact { inc.set_economic_impact(impact); }
    let saved = inc.clone();
    state.incidence_repo.lock().unwrap().save(inc);
    Ok(saved)
}

pub fn list_for_cycle(state: &AppState, cycle_id: &CycleId) -> Vec<IncidenceSummary> {
    state.incidence_repo.lock().unwrap().for_cycle(cycle_id).iter().map(|i| IncidenceSummary {
        id: i.id().0.clone(), cycle_id: i.cycle_id().0.clone(), kind: format!("{:?}", i.kind()),
        severity: format!("{:?}", i.severity()), description: i.description().to_string(),
        action_taken: i.action_taken().to_string(), detected_at: i.detected_at(), resolved: i.is_resolved(),
        economic_impact: i.economic_impact().map(|m| format!("{} {:?}", m.amount, m.currency)),
    }).collect()
}
