use kora_domain::agriculture::incidence::{SanitaryIncidence, IncidenceError, IncidenceType, Severity};
use kora_domain::ports::sanitary_incidence_repository::SanitaryIncidenceRepository;
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;

pub struct RegisterIncidenceInput {
    pub cycle_id: CycleId,
    pub kind: IncidenceType,
    pub severity: Severity,
    pub description: String,
    pub action_taken: String,
    pub detected_at: i64,
    pub economic_impact: Option<Money>,
}

pub fn execute(
    state: &crate::state::AppState,
    input: RegisterIncidenceInput,
) -> Result<SanitaryIncidence, IncidenceError> {
    let mut incidence = SanitaryIncidence::new(
        input.cycle_id,
        input.kind,
        input.severity,
        input.description,
        input.action_taken,
        input.detected_at,
    )?;
    if let Some(impact) = input.economic_impact {
        incidence.set_economic_impact(impact);
    }
    let saved = incidence.clone();
    state.incidence_repo.lock().unwrap().save(incidence);
    Ok(saved)
}

pub fn for_cycle(state: &crate::state::AppState, cycle_id: &CycleId) -> Vec<SanitaryIncidence> {
    state.incidence_repo.lock().unwrap().for_cycle(cycle_id)
}
