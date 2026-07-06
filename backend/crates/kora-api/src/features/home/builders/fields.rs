use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::area::Area;
use kora_domain::finance::budget::Budget;
use crate::features::home::dto::HomeFieldPreview;

pub fn build(
    areas: &[&Area],
    cycles: &[CropCycle],
    budgets: &[Budget],
) -> Vec<HomeFieldPreview> {
    areas.iter().filter_map(|area| {
        let cycle = cycles.iter().find(|c| c.area_id() == area.id())?;
        let budget = budgets.iter().find(|b| b.cycle_id() == cycle.id());
        let over_budget = budget
            .and_then(|b| b.get_variance().ok())
            .map(|v| v.amount.is_sign_positive())
            .unwrap_or(false);
        let period = cycle.period().end().max(cycle.period().start()) - cycle.period().start().min(cycle.period().end());
        let elapsed = if period > 0 {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
            ((now - cycle.period().start()) as f64 / period as f64 * 100.0) as u32
        } else { 50 };
        let health = if over_budget { "critical" } else if cycle.executed_activities().is_empty() { "attention" } else { "ok" };
        Some(HomeFieldPreview {
            id: area.id().0.clone(),
            name: area.name().to_string(),
            crop: "—".into(),
            hectares: area.measurement().value_in_hectares(),
            progress_percent: elapsed.min(100),
            health: health.into(),
            days_to_harvest: 23,
            last_activity: if cycle.executed_activities().is_empty() { "sin actividad".into() } else { "reciente".into() },
            next_activity: "—".into(),
        })
    }).collect()
}
