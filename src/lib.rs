use std::net::SocketAddr;

use actix_web::dev::ServerHandle;
use actix_web::web::{to, Data};
use actix_web::{App, HttpServer};
use tokio::signal::unix::SignalKind;

use talk_hub_api::api_route_http;
use talk_hub_domain::result::TalkHubResult;

use crate::config::Config;
use talk_hub_api::context::TalkHubContext;
use talk_hub_database::utils::build_db_pool;

pub mod config;

pub async fn start_server(config: &Config) -> TalkHubResult<()> {
    let server = create_server(config).await?;

    let mut interrupt = tokio::signal::unix::signal(SignalKind::interrupt())?;
    let mut terminate = tokio::signal::unix::signal(SignalKind::terminate())?;

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
        tracing::warn!("Received ctrl-c, shutting down gracefully...");
      }
      _ = interrupt.recv() => {
        tracing::warn!("Received interrupt, shutting down gracefully...");
      }
      _ = terminate.recv() => {
        tracing::warn!("Received terminate, shutting down gracefully...");
      }
    }
    server.stop(true).await;
    Ok(())
}

async fn create_server(config: &Config) -> TalkHubResult<ServerHandle> {
    let db_pool = build_db_pool(
        &config.database_url().to_string(),
        config.database_pool_size(),
    )
    .await?;
    let context = TalkHubContext::create(db_pool);

    let bind = SocketAddr::from((config.server_host().clone(), config.server_port()));

    let server = HttpServer::new(move || {
        let app = App::new().app_data(Data::new(context.clone()));
        app.configure(|cfg| {
            api_route_http::config(cfg);
        })
    })
    .disable_signals()
    .bind(bind)?
    .run();

    let handle = server.handle();
    tokio::task::spawn(server);
    Ok(handle)
}
