use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::soil::SoilAnalysis;
use kora_domain::agriculture::incidence::SanitaryIncidence;
use kora_domain::finance::budget::Budget;
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_domain::ports::soil_analysis_repository::SoilAnalysisRepository;
use kora_domain::ports::sanitary_incidence_repository::SanitaryIncidenceRepository;
use kora_kernel::ids::AreaId;
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct CycleSummary {
    pub id: String,
    pub crop_id: String,
    pub period_start: i64,
    pub period_end: i64,
    pub activity_count: usize,
    pub budget_baseline: String,
    pub budget_spent: String,
    pub budget_variance: String,
}

#[derive(Serialize)]
pub struct IncidenceSummary {
    pub id: String,
    pub cycle_id: String,
    pub kind: String,
    pub severity: String,
    pub description: String,
    pub detected_at: i64,
    pub economic_impact: Option<String>,
}

#[derive(Serialize)]
pub struct SoilSummary {
    pub id: String,
    pub sampled_at: i64,
    pub quality: String,
    pub metric_count: usize,
    pub cost: String,
}

#[derive(Serialize)]
pub struct AreaDashboard {
    pub area_id: String,
    pub area_name: String,
    pub cycles: Vec<CycleSummary>,
    pub soil_analyses: Vec<SoilSummary>,
    pub incidences: Vec<IncidenceSummary>,
    pub totals: AreaTotals,
}

#[derive(Serialize)]
pub struct AreaTotals {
    pub total_baseline: String,
    pub total_spent: String,
    pub total_variance: String,
    pub cycle_count: usize,
    pub incidence_count: usize,
}

pub fn execute(state: &AppState, area_id: &AreaId) -> AreaDashboard {
    let cycles: Vec<CropCycle> = state
        .cycle_repo
        .lock()
        .unwrap()
        .all()
        .into_iter()
        .filter(|c| c.area_id() == area_id)
        .collect();

    let cycle_ids: std::collections::HashSet<_> =
        cycles.iter().map(|c| c.id().clone()).collect();

    let budgets: Vec<Budget> = state
        .budget_repo
        .lock()
        .unwrap()
        .all()
        .into_iter()
        .filter(|b| cycle_ids.contains(b.cycle_id()))
        .collect();

    let soil_analyses: Vec<SoilAnalysis> =
        state.soil_repo.lock().unwrap().for_area(area_id);

    let mut incidences: Vec<SanitaryIncidence> = Vec::new();
    for cid in &cycle_ids {
        incidences.extend(state.incidence_repo.lock().unwrap().for_cycle(cid));
    }

    let area_name = state
        .farms
        .iter()
        .flat_map(|f| f.areas())
        .find(|a| a.id() == area_id)
        .map(|a| a.name().to_string())
        .unwrap_or_else(|| area_id.0.clone());

    let cycle_summaries: Vec<CycleSummary> = cycles
        .iter()
        .map(|c| {
            let budget = budgets.iter().find(|b| b.cycle_id() == c.id());
            CycleSummary {
                id: c.id().0.clone(),
                crop_id: c.crop_id().0.clone(),
                period_start: c.period().start(),
                period_end: c.period().end(),
                activity_count: c.executed_activities().len(),
                budget_baseline: budget
                    .map(|b| format!("{} {:?}", b.baseline().amount, b.baseline().currency))
                    .unwrap_or_default(),
                budget_spent: budget
                    .map(|b| format!("{} {:?}", b.current_expenses().amount, b.current_expenses().currency))
                    .unwrap_or_default(),
                budget_variance: budget
                    .and_then(|b| b.get_variance().ok())
                    .map(|v| format!("{} {:?}", v.amount, v.currency))
                    .unwrap_or_default(),
            }
        })
        .collect();

    let total_baseline: rust_decimal::Decimal = budgets
        .iter()
        .map(|b| b.baseline().amount)
        .fold(rust_decimal::Decimal::from(0), |a, b| a + b);

    let total_spent: rust_decimal::Decimal = budgets
        .iter()
        .map(|b| b.current_expenses().amount)
        .fold(rust_decimal::Decimal::from(0), |a, b| a + b);

    let total_variance = total_spent - total_baseline;

    let totals = AreaTotals {
        total_baseline: format!("{total_baseline}"),
        total_spent: format!("{total_spent}"),
        total_variance: format!("{total_variance}"),
        cycle_count: cycles.len(),
        incidence_count: incidences.len(),
    };

    AreaDashboard {
        area_id: area_id.0.clone(),
        area_name,
        cycles: cycle_summaries,
        soil_analyses: soil_analyses.iter().map(soil_to_summary).collect(),
        incidences: incidences.iter().map(incidence_to_summary).collect(),
        totals,
    }
}

fn soil_to_summary(a: &SoilAnalysis) -> SoilSummary {
    SoilSummary {
        id: a.id().0.clone(),
        sampled_at: a.sampled_at(),
        quality: format!("{:?}", a.quality()),
        metric_count: a.metrics().len(),
        cost: format!("{} {:?}", a.cost().amount, a.cost().currency),
    }
}

fn incidence_to_summary(i: &SanitaryIncidence) -> IncidenceSummary {
    IncidenceSummary {
        id: i.id().0.clone(),
        cycle_id: i.cycle_id().0.clone(),
        kind: format!("{:?}", i.kind()),
        severity: format!("{:?}", i.severity()),
        description: i.description().to_string(),
        detected_at: i.detected_at(),
        economic_impact: i
            .economic_impact()
            .map(|m| format!("{} {:?}", m.amount, m.currency)),
    }
}
