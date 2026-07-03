use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;

pub async fn serve() {
    let app = router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Kora · API listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/api/operation/today", get(operation_today))
        .route("/api/fields", get(list_fields))
        .route("/api/fields/:id", get(get_field))
}

#[derive(Serialize)]
pub struct StatusCounts {
    pub ok: u32,
    pub attention: u32,
    pub critical: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize)]
pub struct NextAction {
    pub title: String,
    pub field: String,
    pub lot: String,
    pub crop: String,
    pub when: String,
    pub priority: Priority,
    pub reason: String,
    pub consequence: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AttentionKind {
    Delay,
    Budget,
    Weather,
    Info,
}

#[derive(Serialize)]
pub struct AttentionItem {
    pub kind: AttentionKind,
    pub text: String,
    pub metric: String,
}

#[derive(Serialize)]
pub struct OperationToday {
    pub context_note: String,
    pub status: StatusCounts,
    pub next_action: NextAction,
    pub attention: Vec<AttentionItem>,
}

async fn operation_today() -> Json<OperationToday> {
    Json(OperationToday {
        context_note: "Operación · hoy".to_string(),
        status: StatusCounts { ok: 18, attention: 4, critical: 1 },
        next_action: NextAction {
            title: "Fertilización nitrogenada".to_string(),
            field: "Campo Norte".to_string(),
            lot: "Lote A".to_string(),
            crop: "Maíz".to_string(),
            when: "Hoy 09:00 · ventana 24h".to_string(),
            priority: Priority::High,
            reason: "NDVI 0.62 en Lote A, 18% por debajo del óptimo para la fase".to_string(),
            consequence: "Si no se aplica antes de la lluvia del viernes, se pierde la ventana de asimilación".to_string(),
        },
        attention: vec![
            AttentionItem {
                kind: AttentionKind::Budget,
                text: "Presupuesto Campo Norte sobre 100%".to_string(),
                metric: "112% usado · $120 sobre".to_string(),
            },
            AttentionItem {
                kind: AttentionKind::Delay,
                text: "2 lotes atrasados en cronograma".to_string(),
                metric: "Lote B −3 días · Lote C −1 día".to_string(),
            },
            AttentionItem {
                kind: AttentionKind::Weather,
                text: "Lluvia prevista en 8h".to_string(),
                metric: "32mm acumulados · revisar fumigación".to_string(),
            },
        ],
    })
}

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

fn field_data(id: &str) -> Option<Field> {
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
                FieldHealth { status: HealthStatus::Attention, label: "NDVI".to_string(), value: "0.62 · −18% óptimo".to_string() },
                FieldHealth { status: HealthStatus::Ok, label: "Humedad suelo".to_string(), value: "32% · en rango".to_string() },
                FieldHealth { status: HealthStatus::Critical, label: "Presupuesto".to_string(), value: "112% usado".to_string() },
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

async fn get_field(Path(id): Path<String>) -> Result<Json<Field>, axum::http::StatusCode> {
    field_data(&id).map(Json).ok_or(axum::http::StatusCode::NOT_FOUND)
}

async fn list_fields() -> Json<Vec<Field>> {
    Json(vec![
        field_data("campo-norte").unwrap(),
        field_data("campo-sur").unwrap(),
    ])
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/operation/today", get(operation_today))
        .route("/api/fields", get(list_fields))
        .route("/api/fields/:id", get(get_field));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Kora · API listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
