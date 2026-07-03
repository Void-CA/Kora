use axum::{http::HeaderValue, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

mod operation;
mod fields;
mod health;

const DEV_ORIGIN: &str = "http://localhost:4200";

pub async fn serve() {
    let app = router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Kora · API listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    let origin: HeaderValue = DEV_ORIGIN.parse().expect("valid dev origin");
    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health::health))
        .route("/api/operation/today", get(operation::today))
        .route("/api/fields", get(fields::list))
        .route("/api/fields/:id", get(fields::get_one))
        .layer(cors)
}
