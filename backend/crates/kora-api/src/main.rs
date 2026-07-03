#![allow(dead_code)]

mod analyze_variance;
mod api;
mod adapters;

#[tokio::main]
async fn main() {
    api::serve().await;
}
