use crate::state::AppState;
use crate::features::home::dto::*;
use crate::features::home::builders;

pub fn execute(state: &AppState) -> HomeResponse {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let budgets = state.budget_repo.lock().unwrap().all();
    let workers = state.worker_repo.lock().unwrap().all();
    let schedules = state.schedule_repo.lock().unwrap().all();

    let areas: Vec<_> = state.farms.iter().flat_map(|f| f.areas()).collect();

    let (pending, critical, completed) = categorize_activities(&cycles, &budgets);
    let next_action = find_next_action(&schedules, &cycles, state);
    let field_previews = builders::fields::build(&areas, &cycles, &budgets);
    let team = builders::team::build(&workers);
    let finances = builders::finance::build(&budgets);
    let alerts = builders::alerts::build(&cycles, &finances.alerts);

    HomeResponse {
        greeting: "Buenos días".into(),
        date: "hoy".into(),
        today: TodaySection { pending: pending as u32, critical: critical as u32, completed: completed as u32, next_action },
        fields: field_previews,
        team,
        finances,
        alerts,
        weather: None,
    }
}

fn categorize_activities(cycles: &[kora_domain::agriculture::cycle::CropCycle], budgets: &[kora_domain::finance::budget::Budget]) -> (usize, usize, usize) {
    let mut ok = 0usize;
    let mut attention = 0usize;
    let mut critical = 0usize;
    for cycle in cycles {
        let has_activities = !cycle.executed_activities().is_empty();
        let over_budget = budgets.iter()
            .find(|b| b.cycle_id() == cycle.id())
            .and_then(|b| b.get_variance().ok())
            .map(|v| v.amount.is_sign_positive())
            .unwrap_or(false);
        match (has_activities, over_budget) {
            (_, true) => critical += 1,
            (true, false) => ok += 1,
            (false, false) => attention += 1,
        }
    }
    (ok, attention, critical)
}

fn find_next_action(
    schedules: &[kora_domain::agriculture::planning::Schedule],
    cycles: &[kora_domain::agriculture::cycle::CropCycle],
    state: &AppState,
) -> Option<NextActionData> {
    let schedule = schedules.first()?;
    let planned = schedule.activities().first()?;
    let cycle = cycles.iter().find(|c| c.id() == schedule.cycle_id())?;
    let area_name = state.farms.iter()
        .flat_map(|f| f.areas())
        .find(|a| a.id() == cycle.area_id())
        .map(|a| a.name().to_string())
        .unwrap_or_else(|| cycle.area_id().0.clone());
    Some(NextActionData {
        title: format!("{:?}", planned.category),
        field: area_name,
        crop: "—".into(),
        when: format!("Día +{}", planned.relative_day),
        priority: "high".into(),
    })
}
