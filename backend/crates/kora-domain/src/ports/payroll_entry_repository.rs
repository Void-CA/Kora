use crate::finance::payroll::PayrollEntry;
use kora_kernel::ids::CycleId;

pub trait PayrollEntryRepository {
    fn save(&mut self, entry: PayrollEntry);
    fn all(&self) -> Vec<PayrollEntry>;
    fn for_cycle(&self, cycle_id: &CycleId) -> Vec<PayrollEntry>;
}
