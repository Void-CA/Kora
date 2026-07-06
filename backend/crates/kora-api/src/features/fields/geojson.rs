use serde_json::{json, Value};
use crate::state::AppState;

pub fn execute(state: &AppState) -> Value {
    let cycles = state.cycle_repo.lock().unwrap().all();
    let budgets = state.budget_repo.lock().unwrap().all();

    let features: Vec<Value> = state
        .farms
        .iter()
        .flat_map(|f| f.areas())
        .filter_map(|area| {
            let cycle = cycles.iter().find(|c| c.area_id() == area.id());
            let budget = cycle.and_then(|c| budgets.iter().find(|b| b.cycle_id() == c.id()));
            let over_budget = budget.and_then(|b| b.get_variance().ok()).map(|v| v.amount.is_sign_positive()).unwrap_or(false);
            let has_activities = cycle.map(|c| !c.executed_activities().is_empty()).unwrap_or(false);
            let health = if over_budget { "critical" } else if !has_activities { "attention" } else { "ok" };

            // Build GeoJSON coordinates from polygon
            use geo_types::Point;
            let raw = area.geometry().inner();
            let coords: Vec<Vec<f64>> = raw
                .exterior()
                .points()
                .map(|p: Point<f64>| vec![p.x(), p.y()])
                .collect();

            Some(json!({
                "type": "Feature",
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [coords],
                },
                "properties": {
                    "id": area.id().0,
                    "name": area.name(),
                    "hectares": area.measurement().value_in_hectares(),
                    "health": health,
                },
            }))
        })
        .collect();

    json!({
        "type": "FeatureCollection",
        "features": features,
    })
}
