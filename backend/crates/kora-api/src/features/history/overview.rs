use serde::Serialize;
use crate::state::AppState;

#[derive(Serialize)]
pub struct HistoryOverview {
    pub campaigns: Vec<CampaignCard>,
}

#[derive(Serialize)]
pub struct CampaignCard {
    pub id: String, pub crop: String, pub field: String,
    pub started: String, pub ended: Option<String>, pub status: String,
    pub progress_percent: u32, pub total_activities: usize, pub completed_activities: usize,
    pub budget: String, pub spent: String, pub revenue: String,
    pub profitability: String, pub health: String,
}

pub fn execute(state: &AppState) -> HistoryOverview {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let budgets = state.budget_repo.lock().unwrap().all();
    let revenues_list = state.revenue_repo.lock().unwrap().all();

    let campaigns: Vec<CampaignCard> = cycles.iter().map(|c| {
        let area_name = state.farms.iter().flat_map(|f| f.areas()).find(|a| a.id() == c.area_id()).map(|a| a.name()).unwrap_or("—");
        let b = budgets.iter().find(|b| b.cycle_id() == c.id());
        let rev = revenues_list.iter().filter(|r| r.cycle_id() == Some(c.id())).map(|r| r.amount().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
        let spent = b.map(|b| b.current_expenses().amount).unwrap_or(rust_decimal::Decimal::from(0));
        let profit = &rev - &spent;
        let period = c.period().end() - c.period().start();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        let elapsed = if period > 0 { ((now - c.period().start()).max(0) as f64 / period as f64 * 100.0) as u32 } else { 50 };
        let has_activities = !c.executed_activities().is_empty();
        let over_budget = b.and_then(|b| b.get_variance().ok()).map(|v| v.amount.is_sign_positive()).unwrap_or(false);
        let health = if over_budget { "attention".into() } else { "ok".into() };

        CampaignCard {
            id: c.id().0.clone(),
            crop: format!("{:?}", c.crop_id()),
            field: area_name.to_string(),
            started: c.period().start().to_string(),
            ended: None,
            status: if now > c.period().end() { "completed".into() } else { "active".into() },
            progress_percent: elapsed.min(100),
            total_activities: c.executed_activities().len(),
            completed_activities: c.executed_activities().len(),
            budget: b.map(|b| format!("${}", b.baseline().amount)).unwrap_or_default(),
            spent: format!("${}", spent),
            revenue: format!("${}", rev),
            profitability: format!("${}", profit),
            health,
        }
    }).collect();

    HistoryOverview { campaigns }
}
