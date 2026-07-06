use kora_domain::finance::payroll::{PayrollEntry, PayrollError, Role, Worker, WorkerId};
use kora_kernel::ids::{AreaId, CycleId};
use kora_kernel::money::Money;
use crate::state::AppState;
use crate::features::payroll::dto::{WorkerSummary, PayrollEntrySummary};

pub fn register_worker(state: &AppState, name: String, role: Option<Role>) -> Result<Worker, PayrollError> {
    let mut w = Worker::new(name)?;
    if let Some(r) = role { w.set_role(r); }
    let id = w.id().clone();
    state.worker_repo.lock().unwrap().save(w);
    state.worker_repo.lock().unwrap().find_by_id(&id).ok_or(PayrollError::EmptyName)
}

pub fn list_workers(state: &AppState) -> Vec<WorkerSummary> {
    state.worker_repo.lock().unwrap().all().iter().map(|w| WorkerSummary { id: w.id().0.clone(), name: w.name().to_string(), role: w.role().map(|r| format!("{:?}", r)), active: w.is_active() }).collect()
}

pub fn record_payroll(state: &AppState, worker_id: WorkerId, amount: Money, paid_at: i64, cycle_id: Option<CycleId>, area_id: Option<AreaId>) -> Result<PayrollEntry, PayrollError> {
    let entry = PayrollEntry::new(worker_id, amount, paid_at, cycle_id, area_id)?;
    let saved = entry.clone();
    state.payroll_repo.lock().unwrap().save(entry);
    Ok(saved)
}

pub fn list_payroll(state: &AppState, cycle_id: &CycleId) -> Vec<PayrollEntrySummary> {
    state.payroll_repo.lock().unwrap().for_cycle(cycle_id).iter().map(|e| PayrollEntrySummary { id: e.id().to_string(), worker_id: e.worker_id().0.clone(), amount: format!("{} {:?}", e.amount().amount, e.amount().currency), paid_at: e.paid_at(), cycle_id: e.cycle_id().map(|c| c.0.clone()), area_id: e.area_id().map(|a| a.0.clone()) }).collect()
}
