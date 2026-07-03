use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Ok,
    Attention,
    Critical,
    Info,
}

#[derive(Serialize)]
pub struct FieldHealth {
    pub status: HealthStatus,
    pub label: String,
    pub value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PhaseStatus {
    Done,
    Current,
    Pending,
}

#[derive(Serialize)]
pub struct CyclePhase {
    pub name: String,
    pub status: PhaseStatus,
    pub day_in_phase: Option<u32>,
    pub expected_duration_days: Option<u32>,
}

#[derive(Serialize)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub hectares: f64,
    pub lots: u32,
    pub crop: String,
    pub growth: String,
    pub last_activity: String,
    pub days_to_harvest: u32,
    pub estimated_yield_t_per_ha: f64,
    pub historical_yield_t_per_ha: f64,
    pub health: Vec<FieldHealth>,
    pub phases: Vec<CyclePhase>,
}

fn data(id: &str) -> Option<Field> {
    match id {
        "campo-norte" => Some(Field {
            id: "campo-norte".to_string(),
            name: "Campo Norte".to_string(),
            hectares: 12.0,
            lots: 3,
            crop: "Maíz".to_string(),
            growth: "Crecimiento".to_string(),
            last_activity: "hace 3 días".to_string(),
            days_to_harvest: 23,
            estimated_yield_t_per_ha: 8.5,
            historical_yield_t_per_ha: 8.2,
            health: vec![
                FieldHealth {
                    status: HealthStatus::Attention,
                    label: "NDVI".to_string(),
                    value: "0.62 · −18% óptimo".to_string(),
                },
                FieldHealth {
                    status: HealthStatus::Ok,
                    label: "Humedad suelo".to_string(),
                    value: "32% · en rango".to_string(),
                },
                FieldHealth {
                    status: HealthStatus::Critical,
                    label: "Presupuesto".to_string(),
                    value: "112% usado".to_string(),
                },
            ],
            phases: vec![
                CyclePhase { name: "Preparación".to_string(), status: PhaseStatus::Done, day_in_phase: None, expected_duration_days: Some(20) },
                CyclePhase { name: "Siembra".to_string(), status: PhaseStatus::Done, day_in_phase: None, expected_duration_days: Some(5) },
                CyclePhase { name: "Crecimiento".to_string(), status: PhaseStatus::Current, day_in_phase: Some(12), expected_duration_days: Some(45) },
                CyclePhase { name: "Floración".to_string(), status: PhaseStatus::Pending, day_in_phase: None, expected_duration_days: Some(20) },
                CyclePhase { name: "Cosecha".to_string(), status: PhaseStatus::Pending, day_in_phase: None, expected_duration_days: Some(15) },
            ],
        }),
        "campo-sur" => Some(Field {
            id: "campo-sur".to_string(),
            name: "Campo Sur".to_string(),
            hectares: 8.0,
            lots: 2,
            crop: "Frijol".to_string(),
            growth: "Siembra".to_string(),
            last_activity: "hoy".to_string(),
            days_to_harvest: 78,
            estimated_yield_t_per_ha: 2.3,
            historical_yield_t_per_ha: 2.1,
            health: vec![
                FieldHealth { status: HealthStatus::Ok, label: "NDVI".to_string(), value: "0.71 · en rango".to_string() },
                FieldHealth { status: HealthStatus::Ok, label: "Humedad suelo".to_string(), value: "28% · en rango".to_string() },
                FieldHealth { status: HealthStatus::Ok, label: "Presupuesto".to_string(), value: "20% usado".to_string() },
            ],
            phases: vec![
                CyclePhase { name: "Preparación".to_string(), status: PhaseStatus::Done, day_in_phase: None, expected_duration_days: Some(15) },
                CyclePhase { name: "Siembra".to_string(), status: PhaseStatus::Current, day_in_phase: Some(2), expected_duration_days: Some(10) },
                CyclePhase { name: "Crecimiento".to_string(), status: PhaseStatus::Pending, day_in_phase: None, expected_duration_days: Some(40) },
                CyclePhase { name: "Floración".to_string(), status: PhaseStatus::Pending, day_in_phase: None, expected_duration_days: Some(20) },
                CyclePhase { name: "Cosecha".to_string(), status: PhaseStatus::Pending, day_in_phase: None, expected_duration_days: Some(15) },
            ],
        }),
        _ => None,
    }
}

pub async fn list() -> Json<Vec<Field>> {
    Json(vec![
        data("campo-norte").unwrap(),
        data("campo-sur").unwrap(),
    ])
}

pub async fn get_one(Path(id): Path<String>) -> Result<Json<Field>, StatusCode> {
    data(&id).map(Json).ok_or(StatusCode::NOT_FOUND)
}
