#![allow(dead_code)]

use kora_api::api;

#[tokio::main]
async fn main() {
    api::serve().await;
}
