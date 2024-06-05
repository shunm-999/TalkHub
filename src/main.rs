#![feature(fn_traits, unboxed_closures)]

use talk_hub::config;
use talk_hub::start_server;

#[tokio::main]
pub async fn main() {
    let config = config::config().await;
    start_server(config).await.expect("Failed to start server");
}
