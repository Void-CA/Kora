use serde::Serialize;
use crate::state::AppState;

#[derive(Serialize)]
pub struct FinancesOverview {
    pub total_budget: String, pub total_spent: String, pub total_revenue: String,
    pub profit: String, pub roi: String,
    pub cycles: Vec<FinanceCycle>,
}

#[derive(Serialize)]
pub struct FinanceCycle {
    pub cycle_name: String, pub field: String, pub budget: String,
    pub spent: String, pub revenue: String, pub profit: String, pub status: String,
}

pub fn execute(state: &AppState) -> FinancesOverview {
    let budgets = state.budget_repo.lock().unwrap().all();
    let revenues_list = state.revenue_repo.lock().unwrap().all();
    let cycles = state.cycle_repo.lock().unwrap().all();

    let total_budget: rust_decimal::Decimal = budgets.iter().map(|b| b.baseline().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
    let total_spent: rust_decimal::Decimal = budgets.iter().map(|b| b.current_expenses().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
    let total_revenue: rust_decimal::Decimal = revenues_list.iter().map(|r| r.amount().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
    let profit = &total_revenue - &total_spent;
    let roi = if total_spent.is_zero() { rust_decimal::Decimal::from(0) } else { (profit / &total_spent) * rust_decimal::Decimal::from(100) };

    let cycle_rows: Vec<FinanceCycle> = cycles.iter().map(|c| {
        let area_name = state.farms.iter().flat_map(|f| f.areas()).find(|a| a.id() == c.area_id()).map(|a| a.name()).unwrap_or("—");
        let b = budgets.iter().find(|b| b.cycle_id() == c.id());
        let rev = revenues_list.iter().filter(|r| r.cycle_id() == Some(c.id())).map(|r| r.amount().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
        let spent = b.map(|b| b.current_expenses().amount).unwrap_or(rust_decimal::Decimal::from(0));
        let p = &rev - &spent;
        let over_budget = b.and_then(|b| b.get_variance().ok()).map(|v| v.amount.is_sign_positive()).unwrap_or(false);
        FinanceCycle {
            cycle_name: format!("Ciclo {}", &c.id().0[..8]),
            field: area_name.to_string(), budget: b.map(|b| format!("${}", b.baseline().amount)).unwrap_or_default(),
            spent: format!("${}", spent), revenue: format!("${}", rev), profit: format!("${}", p),
            status: if over_budget { "critical".into() } else { "ok".into() },
        }
    }).collect();

    FinancesOverview {
        total_budget: format!("${}", total_budget), total_spent: format!("${}", total_spent),
        total_revenue: format!("${}", total_revenue), profit: format!("${}", profit), roi: format!("{}%", roi),
        cycles: cycle_rows,
    }
}
