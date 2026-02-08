use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{Json, Response},
    routing::get,
    Router,
};
use serde_json::{json, Value};
use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;

use crate::handler::register_meuexemplo_http_endpoints;
use crate::repository::MeuexemploRepository;
use crate::service::{MeuexemploServiceIF, Resource};

pub trait Logger: Send + Sync {
    fn info(&self, message: &str);
    fn init_logger(&self, level: &str);
}

pub struct App {
    http_server: Option<axum::serve::Serve<Router, Router>>,
    pgdb_write: Option<Arc<PgPool>>,
    pgdb_read: Option<Arc<PgPool>>,
    log: Option<Arc<dyn Logger>>,
    meuexemplo_sv: Option<Arc<dyn MeuexemploServiceIF>>,
}

pub trait ServerIF {
    async fn start(&mut self);
    async fn stop(&mut self);
    async fn run(&mut self, port: &str) -> Result<(), anyhow::Error>;
}

impl App {
    pub fn new() -> impl ServerIF {
        App {
            http_server: None,
            pgdb_write: None,
            pgdb_read: None,
            log: None,
            meuexemplo_sv: None,
        }
    }
}

impl ServerIF for App {
    async fn start(&mut self) {
        let log = create_logger();
        log.init_logger("Dpanic");
        self.log = Some(Arc::new(log));

        // Inicializar observabilidade
        if let Err(e) = init_observability("novo-exemplo-palm-pay", "1.0.0").await {
            panic!("Erro ao inicializar observabilidade: {}", e);
        }

        // Iniciar servidor de métricas
        tokio::spawn(start_metrics_server("2112"));

        self.start_pg_write().await;
        self.start_pg_read().await;

        let meuexemplo_repository = MeuexemploRepository::new(
            self.pgdb_write.as_ref().unwrap().clone(),
            self.pgdb_read.as_ref().unwrap().clone(),
            Box::new(DefaultLogger),
        );

        self.meuexemplo_sv = Some(Arc::new(Resource::new(
            Arc::new(meuexemplo_repository),
            Box::new(DefaultLogger),
        )));
    }

    async fn run(&mut self, port: &str) -> Result<(), anyhow::Error> {
        let mut router = Router::new();

        // Middlewares
        router = router
            .layer(ServiceBuilder::new()
                .layer(request_id_middleware())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(middleware::from_fn(structured_logging_middleware))
                .layer(middleware::from_fn(enhanced_http_metrics_middleware))
            );

        // Health check endpoint
        router = router.route("/health", get(health_check_handler));

        // Metrics endpoint
        router = router.route("/internal/metrics", get(metrics_handler));

        // API routes
        let api_routes = register_meuexemplo_http_endpoints(
            self.meuexemplo_sv.as_ref().unwrap().clone(),
            self.log.as_ref().unwrap().clone(),
        );

        router = router.nest("/api", api_routes);

        let addr = format!("0.0.0.0:{}", port);
        println!("≡ Microservices novo-exemplo-palm-pay Started in local Port: {} ≡", port);
        println!("📊 Métricas disponíveis em: http://localhost:2112/metrics");
        println!("🔍 Health check em: http://localhost:{}/health", port);
        println!("");

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        Ok(())
    }

    async fn stop(&mut self) {
        self.stop_pg_read();
        self.stop_pg_write();
        self.stop_http();
        if let Some(log) = &self.log {
            log.info("Finalizado com sucesso");
        }
    }
}

impl App {
    async fn start_pg_write(&mut self) {
        let url = std::env::var("POSTGRESQL_WRITE_URL").expect("POSTGRESQL_WRITE_URL not set");

        let pool = PgPool::connect(&url).await.expect("Failed to connect to PostgreSQL Write");

        // Configurar pool
        // Note: sqlx não tem configuração equivalente direta, mas pode ser configurado via URL

        if pool.acquire().await.is_err() {
            panic!("Failed to ping PostgreSQL Write");
        }

        // Registrar métricas de conexão
        update_db_connections("meuexemplo", 30).await;

        self.pgdb_write = Some(Arc::new(pool));
        if let Some(log) = &self.log {
            log.info("DB Postgresql Writer was connected...");
        }
    }

    async fn start_pg_read(&mut self) {
        let url = std::env::var("POSTGRESQL_READ_URL").expect("POSTGRESQL_READ_URL not set");

        let pool = PgPool::connect(&url).await.expect("Failed to connect to PostgreSQL Read");

        if pool.acquire().await.is_err() {
            panic!("Failed to ping PostgreSQL Read");
        }

        update_db_connections("meuexemplo", 30).await;

        self.pgdb_read = Some(Arc::new(pool));
        if let Some(log) = &self.log {
            log.info("DB Postgresql Read was connected...");
        }
    }

    fn stop_pg_write(&mut self) {
        if let Some(pool) = self.pgdb_write.take() {
            // PgPool fecha automaticamente quando sai de escopo
            if let Some(log) = &self.log {
                log.info("PG Write conexao finalizada ok");
            }
        }
    }

    fn stop_pg_read(&mut self) {
        if let Some(pool) = self.pgdb_read.take() {
            // PgPool fecha automaticamente quando sai de escopo
            if let Some(log) = &self.log {
                log.info("PG Read conexao finalizada ok");
            }
        }
    }

    fn stop_http(&mut self) {
        // Axum server para automaticamente com graceful shutdown
        if let Some(log) = &self.log {
            log.info("Http conexao finalizada ok");
        }
    }
}

// Middleware functions
fn request_id_middleware() -> axum::middleware::FromFn<impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>>> {
    middleware::from_fn(|req, next| {
        Box::pin(async move {
            // Implementar request ID middleware
            next.run(req).await
        })
    })
}

async fn structured_logging_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Implementar structured logging
    next.run(req).await
}

async fn enhanced_http_metrics_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Implementar HTTP metrics
    next.run(req).await
}

// Handlers
async fn health_check_handler() -> Result<Json<Value>, StatusCode> {
    let start = std::time::Instant::now();

    // Verificar conexões do banco seria feito aqui
    let health = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0",
        "service": "novo-exemplo-palm-pay",
        "pg_write": "ok",
        "pg_read": "ok",
        "response_time_ms": start.elapsed().as_millis()
    });

    Ok(Json(health))
}

async fn metrics_handler() -> Json<Value> {
    Json(json!({
        "metrics_endpoint": "http://localhost:2112/metrics",
        "prometheus_format": true,
        "service": "novo-exemplo-palm-pay"
    }))
}

// Utility functions
async fn init_observability(service: &str, version: &str) -> Result<(), anyhow::Error> {
    // Implementar inicialização de observabilidade
    Ok(())
}

async fn start_metrics_server(port: &str) {
    // Implementar servidor de métricas
}

async fn update_db_connections(service: &str, connections: i32) {
    // Implementar update de métricas de conexão
}

fn create_logger() -> impl Logger {
    DefaultLogger
}

struct DefaultLogger;

impl Logger for DefaultLogger {
    fn info(&self, message: &str) {
        println!("{}", message);
    }

    fn init_logger(&self, _level: &str) {
        // Implementar inicialização de logger
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}