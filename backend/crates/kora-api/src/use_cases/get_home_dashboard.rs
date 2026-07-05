use serde::Serialize;
use kora_kernel::ids::AreaId;

use crate::state::AppState;

// ── DTOs that match the frontend HomeView ──

#[derive(Serialize)]
pub struct HomeResponse {
    pub greeting: String,
    pub date: String,
    pub today: TodaySection,
    pub fields: Vec<HomeFieldPreview>,
    pub team: TeamPreview,
    pub finances: FinancePreview,
    pub alerts: Vec<AlertItem>,
    pub weather: Option<WeatherInfo>,
}

#[derive(Serialize)]
pub struct TodaySection {
    pub pending: u32,
    pub critical: u32,
    pub completed: u32,
    pub next_action: Option<NextActionData>,
}

#[derive(Serialize)]
pub struct NextActionData {
    pub title: String,
    pub field: String,
    pub crop: String,
    pub when: String,
    pub priority: String,
}

#[derive(Serialize)]
pub struct HomeFieldPreview {
    pub id: String,
    pub name: String,
    pub crop: String,
    pub hectares: f64,
    pub progress_percent: u32,
    pub health: String,
    pub days_to_harvest: i64,
    pub last_activity: String,
    pub next_activity: String,
}

#[derive(Serialize)]
pub struct TeamPreview {
    pub total: usize,
    pub working_today: usize,
    pub missing_yesterday: usize,
    pub entries: Vec<TeamEntry>,
}

#[derive(Serialize)]
pub struct TeamEntry {
    pub name: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct FinancePreview {
    pub total_budget: String,
    pub budget_used_percent: u32,
    pub total_spent: String,
    pub alerts: Vec<FinanceAlert>,
}

#[derive(Serialize)]
pub struct FinanceAlert {
    pub cycle: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct AlertItem {
    pub kind: String,
    pub text: String,
    pub field: Option<String>,
    pub severity: String,
}

#[derive(Serialize)]
pub struct WeatherInfo {
    pub forecast: String,
    pub rain_expected: bool,
    pub temp: String,
    pub humidity: String,
}

pub fn execute(state: &AppState) -> HomeResponse {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let budgets = state.budget_repo.lock().unwrap().all();
    let workers = state.worker_repo.lock().unwrap().all();
    let schedules = state.schedule_repo.lock().unwrap().all();

    // Today stats
    let total_activities: usize = cycles.iter().map(|c| c.executed_activities().len()).sum();
    let (pending, critical, completed) = categorize_activities(&cycles, &budgets);

    // Next action — first planned activity across all schedules
    let next_action = find_next_action(&schedules, &cycles, state);

    // Fields preview
    let field_previews: Vec<HomeFieldPreview> = state
        .farms
        .iter()
        .flat_map(|f| f.areas())
        .filter_map(|area| build_field_preview(area, &cycles, &budgets))
        .collect();

    // Team
    let team_total = workers.len();
    let team_entries: Vec<TeamEntry> = workers
        .iter()
        .map(|w| TeamEntry {
            name: w.name().to_string(),
            status: if w.is_active() { "working".into() } else { "missing".into() },
        })
        .collect();
    let working_today = team_entries.iter().filter(|e| e.status == "working").count();
    let missing_yesterday = team_entries.iter().filter(|e| e.status == "missing").count();

    // Finances
    let total_budget: rust_decimal::Decimal = budgets.iter().map(|b| b.baseline().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
    let total_spent: rust_decimal::Decimal = budgets.iter().map(|b| b.current_expenses().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
    let pct = if total_budget.is_zero() { 0 } else { ((total_spent / total_budget) * rust_decimal::Decimal::from(100)).try_into().unwrap_or(0u32) };
    let finance_alerts: Vec<FinanceAlert> = budgets
        .iter()
        .filter_map(|b| {
            let v = b.get_variance().ok()?;
            if v.amount.is_sign_positive() {
                Some(FinanceAlert {
                    cycle: b.cycle_id().0.clone(),
                    text: format!("Presupuesto excedido por {} {:?}", v.amount, v.currency),
                })
            } else {
                let used_pct = if b.baseline().amount.is_zero() { 0 } else {
                    ((b.current_expenses().amount / b.baseline().amount) * rust_decimal::Decimal::from(100)).try_into().unwrap_or(0u32)
                };
                if used_pct > 80 {
                    Some(FinanceAlert {
                        cycle: b.cycle_id().0.clone(),
                        text: format!("Presupuesto al {}%", used_pct),
                    })
                } else { None }
            }
        })
        .collect();

    // Alerts
    let mut alerts: Vec<AlertItem> = Vec::new();
    for cycle in &cycles {
        if cycle.executed_activities().is_empty() {
            alerts.push(AlertItem {
                kind: "delay".into(),
                text: format!("Ciclo sin actividades registradas"),
                field: Some(cycle.area_id().0.clone()),
                severity: "medium".into(),
            });
        }
    }
    alerts.extend(finance_alerts.iter().map(|fa| AlertItem {
        kind: "budget".into(),
        text: fa.text.clone(),
        field: None,
        severity: "high".into(),
    }));
    alerts.truncate(5);

    HomeResponse {
        greeting: "Buenos días".into(),
        date: chrono_now(),
        today: TodaySection {
            pending: pending as u32,
            critical: critical as u32,
            completed: completed as u32,
            next_action,
        },
        fields: field_previews,
        team: TeamPreview {
            total: team_total,
            working_today,
            missing_yesterday,
            entries: team_entries,
        },
        finances: FinancePreview {
            total_budget: format!("${}", total_budget),
            budget_used_percent: pct,
            total_spent: format!("${}", total_spent),
            alerts: finance_alerts,
        },
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

fn build_field_preview(
    area: &kora_domain::agriculture::area::Area,
    cycles: &[kora_domain::agriculture::cycle::CropCycle],
    budgets: &[kora_domain::finance::budget::Budget],
) -> Option<HomeFieldPreview> {
    let cycle = cycles.iter().find(|c| c.area_id() == area.id())?;
    let budget = budgets.iter().find(|b| b.cycle_id() == cycle.id());
    let over_budget = budget
        .and_then(|b| b.get_variance().ok())
        .map(|v| v.amount.is_sign_positive())
        .unwrap_or(false);
    let period_days = cycle.period().end().max(cycle.period().start()) - cycle.period().start().min(cycle.period().end());
    let elapsed = if period_days > 0 {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        let passed = now - cycle.period().start();
        ((passed as f64 / period_days as f64) * 100.0) as u32
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
}

fn chrono_now() -> String {
    "hoy".into()
}
