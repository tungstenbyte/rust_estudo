use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

// Imports dos módulos necessários
use crate::handler::Handler;
use crate::service::MeuexemploServiceIF;

// Logger trait
pub trait Logger: Send + Sync {}

// Middleware de métricas (equivalente ao observabilidade.EnhancedHTTPMetricsMiddleware)
pub fn enhanced_http_metrics_middleware(service_name: &str) -> axum::middleware::FromFn<impl Fn(axum::extract::Request, axum::middleware::Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<axum::response::Response, axum::http::StatusCode>> + Send>>> {
    let service_name = service_name.to_string();
    axum::middleware::from_fn(move |req, next| {
        let service_name = service_name.clone();
        Box::pin(async move {
            // Aqui viria a lógica de métricas
            next.run(req).await
        })
    })
}

// Função equivalente ao RegisterMeuexemploHTTPEndpoints
pub fn register_meuexemplo_http_endpoints(
    uc: Arc<dyn MeuexemploServiceIF + Send + Sync>,
    log: Arc<dyn Logger>,
) -> Router {
    let h = Handler::new(uc, log);

    let meuexemplo_group = Router::new()
        .route("", get(Handler::get_meuexemplo))
        .route("/:id", get(Handler::get_meuexemplo_by_id))
        .route("/statuscode/:statuscode", get(Handler::get_meuexemplo_by_status_code))
        .route("", post(Handler::insert_meuexemplo))
        .route("/:id", put(Handler::update_meuexemplo))
        .route("/:id", delete(Handler::delete_meuexemplo_by_id))
        .with_state(h)
        .layer(enhanced_http_metrics_middleware("meuexemplo"));

    Router::new().nest("/meuexemplo", meuexemplo_group)
}