#![allow(dead_code)]

pub mod ports;
pub mod adapters;
pub mod shared_kernel;
pub mod agriculture;
pub mod finance;

mod analyze_variance;
mod api;

#[tokio::main]
async fn main() {
    api::serve().await;
}
