use crate::features::home::dto::{TeamPreview, TeamEntry};
use kora_domain::finance::payroll::Worker;

pub fn build(workers: &[Worker]) -> TeamPreview {
    let entries: Vec<TeamEntry> = workers.iter()
        .map(|w| TeamEntry {
            name: w.name().to_string(),
            status: if w.is_active() { "working".into() } else { "missing".into() },
        })
        .collect();
    let working_today = entries.iter().filter(|e| e.status == "working").count();
    let missing_yesterday = entries.iter().filter(|e| e.status == "missing").count();
    TeamPreview {
        total: workers.len(),
        working_today,
        missing_yesterday,
        entries,
    }
}
