use kora_kernel::ids::AreaId;
use crate::state::AppState;
use crate::features::fields::dto::*;

pub fn get_history(state: &AppState, area_id: &AreaId) -> HistoryResponse {
    let cycles = state.cycle_repo.lock().unwrap().all().into_iter().filter(|c| c.area_id() == area_id).collect::<Vec<_>>();
    let cycle_ids: std::collections::HashSet<_> = cycles.iter().map(|c| c.id().clone()).collect();
    let schedules = state.schedule_repo.lock().unwrap().all().into_iter().filter(|s| cycle_ids.contains(s.cycle_id())).collect::<Vec<_>>();
    let budgets = state.budget_repo.lock().unwrap().all().into_iter().filter(|b| cycle_ids.contains(b.cycle_id())).collect::<Vec<_>>();
    let area_name = state.farms.iter().flat_map(|f| f.areas()).find(|a| a.id() == area_id).map(|a| a.name().to_string()).unwrap_or_default();
    HistoryResponse {
        area_id: area_id.0.clone(), area_name,
        cycles: cycles.iter().map(|c| c.id().0.clone()).collect(),
        schedules: schedules.iter().map(|s| s.cycle_id().0.clone()).collect(),
        budgets: budgets.iter().map(|b| BudgetSummary {
            id: b.id().0.clone(), cycle_id: b.cycle_id().0.clone(),
            baseline: b.baseline().amount.to_string(), spent: b.current_expenses().amount.to_string(),
            remaining: b.get_remaining().map(|m| m.amount.to_string()).unwrap_or_default(),
            variance: b.get_variance().map(|m| m.amount.to_string()).unwrap_or_default(),
        }).collect(),
    }
}

pub fn get_dashboard(state: &AppState, area_id: &AreaId) -> AreaDashboard {
    let cycles: Vec<_> = state.cycle_repo.lock().unwrap().all().into_iter().filter(|c| c.area_id() == area_id).collect();
    let cycle_ids: std::collections::HashSet<_> = cycles.iter().map(|c| c.id().clone()).collect();
    let budgets: Vec<_> = state.budget_repo.lock().unwrap().all().into_iter().filter(|b| cycle_ids.contains(b.cycle_id())).collect();
    let soil = state.soil_repo.lock().unwrap().for_area(area_id);
    let incidences: Vec<_> = cycle_ids.iter().flat_map(|cid| state.incidence_repo.lock().unwrap().for_cycle(cid)).collect();
    let area_name = state.farms.iter().flat_map(|f| f.areas()).find(|a| a.id() == area_id).map(|a| a.name().to_string()).unwrap_or_default();

    let cycles_s: Vec<DashboardCycleSummary> = cycles.iter().map(|c| {
        let b = budgets.iter().find(|b| b.cycle_id() == c.id());
        DashboardCycleSummary {
            id: c.id().0.clone(), crop_id: c.crop_id().0.clone(), period_start: c.period().start(), period_end: c.period().end(),
            activity_count: c.executed_activities().len(),
            budget_baseline: b.map(|b| format!("{} {:?}", b.baseline().amount, b.baseline().currency)).unwrap_or_default(),
            budget_spent: b.map(|b| format!("{} {:?}", b.current_expenses().amount, b.current_expenses().currency)).unwrap_or_default(),
            budget_variance: b.and_then(|b| b.get_variance().ok()).map(|v| format!("{} {:?}", v.amount, v.currency)).unwrap_or_default(),
        }
    }).collect();

    AreaDashboard {
        area_id: area_id.0.clone(), area_name, cycles: cycles_s,
        soil_analyses: soil.iter().map(|s| DashboardSoilSummary { id: s.id().0.clone(), sampled_at: s.sampled_at(), quality: format!("{:?}", s.quality()), metric_count: s.metrics().len(), cost: format!("{} {:?}", s.cost().amount, s.cost().currency) }).collect(),
        incidences: incidences.iter().map(|i| DashboardIncidenceSummary { id: i.id().0.clone(), cycle_id: i.cycle_id().0.clone(), kind: format!("{:?}", i.kind()), severity: format!("{:?}", i.severity()), description: i.description().to_string(), detected_at: i.detected_at(), economic_impact: i.economic_impact().map(|m| format!("{} {:?}", m.amount, m.currency)) }).collect(),
        totals: AreaTotals {
            total_baseline: budgets.iter().map(|b| b.baseline().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b).to_string(),
            total_spent: budgets.iter().map(|b| b.current_expenses().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b).to_string(),
            total_variance: (budgets.iter().map(|b| b.current_expenses().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b) - budgets.iter().map(|b| b.baseline().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b)).to_string(),
            cycle_count: cycles.len(), incidence_count: incidences.len(),
        },
    }
}
