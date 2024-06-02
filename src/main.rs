#![feature(fn_traits, unboxed_closures)]

use TalkHub::config;
use TalkHub::start_server;

#[tokio::main]
pub async fn main() {
    let config = config::config().await;
    start_server(config).await.expect("Failed to start server");
}
