mod config;
mod meuexemplo;
mod segundominio;
mod server;
mod shared;

use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    config::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let mut app = server::App::new();
    app.start().await;

    info!("Service started on port {}", port);
    app.run(&port).await?;
    app.stop().await;

    Ok(())
}
