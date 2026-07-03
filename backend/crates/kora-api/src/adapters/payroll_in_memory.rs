use std::collections::HashMap;
use kora_domain::finance::payroll::PayrollEntry;
use kora_domain::ports::payroll_entry_repository::PayrollEntryRepository;
use kora_kernel::ids::CycleId;

pub struct InMemoryPayrollEntryRepository {
    entries: HashMap<String, PayrollEntry>,
}

impl InMemoryPayrollEntryRepository {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }
}

impl PayrollEntryRepository for InMemoryPayrollEntryRepository {
    fn save(&mut self, entry: PayrollEntry) {
        self.entries.insert(entry.id().to_string(), entry);
    }
    fn all(&self) -> Vec<PayrollEntry> {
        self.entries.values().cloned().collect()
    }
    fn for_cycle(&self, cycle_id: &CycleId) -> Vec<PayrollEntry> {
        self.entries
            .values()
            .filter(|e| e.cycle_id() == Some(cycle_id))
            .cloned()
            .collect()
    }
}
