use kora_domain::finance::payroll::{PayrollEntry, PayrollError, Role};
use kora_domain::finance::payroll::Worker;
use kora_domain::ports::worker_repository::WorkerRepository;
use kora_domain::ports::payroll_entry_repository::PayrollEntryRepository;
use kora_domain::finance::payroll::WorkerId;
use kora_kernel::ids::{AreaId, CycleId};
use kora_kernel::money::Money;

pub struct RegisterWorkerInput {
    pub name: String,
    pub role: Option<Role>,
}

pub fn register_worker(
    state: &crate::state::AppState,
    input: RegisterWorkerInput,
) -> Result<Worker, PayrollError> {
    let mut worker = Worker::new(input.name)?;
    if let Some(r) = input.role {
        worker.set_role(r);
    }
    let id = worker.id().clone();
    state.worker_repo.lock().unwrap().save(worker);
    let saved = state.worker_repo.lock().unwrap().find_by_id(&id).ok_or(PayrollError::EmptyWorkerId)?;
    Ok(saved)
}

pub struct RecordPayrollInput {
    pub worker_id: WorkerId,
    pub amount: Money,
    pub paid_at: i64,
    pub cycle_id: Option<CycleId>,
    pub area_id: Option<AreaId>,
    pub role_at_payment: Option<Role>,
}

pub fn record_payroll(
    state: &crate::state::AppState,
    input: RecordPayrollInput,
) -> Result<PayrollEntry, PayrollError> {
    let mut entry = PayrollEntry::new(
        input.worker_id,
        input.amount,
        input.paid_at,
        input.cycle_id,
        input.area_id,
    )?;
    if let Some(r) = input.role_at_payment {
        entry = entry.with_role_at_payment(r);
    }
    let saved = entry.clone();
    state.payroll_repo.lock().unwrap().save(entry);
    Ok(saved)
}
