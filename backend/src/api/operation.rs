use axum::Json;
use serde::Serialize;

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

pub async fn today() -> Json<OperationToday> {
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
