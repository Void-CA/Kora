use kora_domain::agriculture::cycle::CropCycle;
use crate::features::home::dto::{AlertItem, FinanceAlert};

pub fn build(cycles: &[CropCycle], finance_alerts: &[FinanceAlert]) -> Vec<AlertItem> {
    let mut alerts: Vec<AlertItem> = cycles.iter().filter(|c| c.executed_activities().is_empty()).map(|cycle| AlertItem {
        kind: "delay".into(),
        text: "Ciclo sin actividades registradas".into(),
        field: Some(cycle.area_id().0.clone()),
        severity: "medium".into(),
    }).collect();

    alerts.extend(finance_alerts.iter().map(|fa| AlertItem {
        kind: "budget".into(),
        text: fa.text.clone(),
        field: None,
        severity: "high".into(),
    }));
    alerts.truncate(5);
    alerts
}
