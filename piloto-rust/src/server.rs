use axum::{routing::get, Json, Router};
use prometheus::{Encoder, TextEncoder};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tracing::info;

use crate::meuexemplo::{self, MeuexemploRepositoryImpl, MeuexemploServiceImpl};
use crate::segundominio::{self, SegundominioRepositoryImpl, SegundominioServiceImpl};

pub struct App {
    read_pool: Option<Arc<PgPool>>,
    write_pool: Option<Arc<PgPool>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            read_pool: None,
            write_pool: None,
        }
    }

    pub async fn start(&mut self) {
        let write_url = std::env::var("POSTGRESQL_WRITE_URL")
            .expect("POSTGRESQL_WRITE_URL must be set");
        let read_url = std::env::var("POSTGRESQL_READ_URL")
            .expect("POSTGRESQL_READ_URL must be set");

        let write_pool = PgPool::connect(&write_url).await.expect("Failed to connect to PostgreSQL (write)");
        let read_pool = PgPool::connect(&read_url).await.expect("Failed to connect to PostgreSQL (read)");

        write_pool.acquire().await.expect("Failed to ping PostgreSQL (write)");
        read_pool.acquire().await.expect("Failed to ping PostgreSQL (read)");

        let write_pool = Arc::new(write_pool);
        let read_pool = Arc::new(read_pool);

        self.write_pool = Some(write_pool.clone());
        self.read_pool = Some(read_pool.clone());

        let metrics_port = std::env::var("METRICS_PORT").unwrap_or_else(|_| "2112".to_string());
        tokio::spawn(metrics_server(metrics_port));

        info!("Database connected, metrics server started");
    }

    pub async fn run(&mut self, port: &str) -> anyhow::Result<()> {
        let read_pool = self.read_pool.clone().expect("Call start() first");
        let write_pool = self.write_pool.clone().expect("Call start() first");

        // Registro de domínios: cada um com repo + service + routes
        let meuexemplo_repo: Arc<dyn meuexemplo::MeuexemploRepository> =
            Arc::new(MeuexemploRepositoryImpl::new(read_pool.clone(), write_pool.clone()));
        let meuexemplo_svc: Arc<dyn meuexemplo::MeuexemploService> =
            Arc::new(MeuexemploServiceImpl::new(meuexemplo_repo));

        let segundominio_repo: Arc<dyn segundominio::SegundominioRepository> =
            Arc::new(SegundominioRepositoryImpl::new(read_pool.clone(), write_pool.clone()));
        let segundominio_svc: Arc<dyn segundominio::SegundominioService> =
            Arc::new(SegundominioServiceImpl::new(segundominio_repo));

        let api = Router::new()
            .nest("/meuexemplo", meuexemplo::routes(meuexemplo_svc))
            .nest("/segundominio", segundominio::routes(segundominio_svc));

        let app = Router::new()
            .route("/health", get(health))
            .route("/internal/metrics", get(internal_metrics))
            .nest("/api", api)
            .layer(ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(30))));

        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        info!("Listening on {}", addr);
        info!("Health: http://localhost:{}/health", port);
        info!("API: http://localhost:{}/api/meuexemplo", port);
        info!("API: http://localhost:{}/api/segundominio", port);

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        Ok(())
    }

    pub async fn stop(&mut self) {
        self.write_pool = None;
        self.read_pool = None;
        info!("Server stopped");
    }
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "piloto-rust",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn internal_metrics() -> (axum::http::StatusCode, String) {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    if encoder.encode(&metric_families, &mut buffer).is_err() {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, String::new());
    }
    let output = String::from_utf8(buffer).unwrap_or_default();
    (axum::http::StatusCode::OK, output)
}

async fn metrics_server(port: String) {
    let app = Router::new().route("/metrics", get(metrics_handler));
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Metrics available at http://localhost:{}/metrics", port);
    axum::serve(listener, app).await.unwrap();
}

async fn metrics_handler() -> (axum::http::StatusCode, String) {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    if encoder.encode(&metric_families, &mut buffer).is_err() {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, String::new());
    }
    (
        axum::http::StatusCode::OK,
        String::from_utf8(buffer).unwrap_or_default(),
    )
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
