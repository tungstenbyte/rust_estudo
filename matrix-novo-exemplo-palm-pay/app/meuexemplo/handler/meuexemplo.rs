use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};

// Imports dos módulos (assumindo que existem)
use crate::model::{ItemsPage, Meuexemplo};
use crate::service::MeuexemploServiceIF;

// Constantes de mensagens de erro (equivalente ao app)
pub mod app {
    pub const MESSAGE: &str = "message";
    pub const MSG_OFFSET_LIMIT: &str = "Invalid offset or limit parameter";
    pub const MSG_BAD_REQUEST: &str = "Bad request";
    pub const MSG_INTERNAL_ERROR: &str = "Internal server error";
}

// Query parameters para paginação
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// Sistema de observabilidade para Handler
pub struct HandlerTracker {
    operation: String,
    started_at: Instant,
    params: std::collections::HashMap<String, String>,
    results: std::collections::HashMap<String, String>,
}

impl HandlerTracker {
    pub fn new(operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
            started_at: Instant::now(),
            params: std::collections::HashMap::new(),
            results: std::collections::HashMap::new(),
        }
    }

    pub fn add_param<T: ToString>(&mut self, key: &str, value: T) {
        self.params.insert(key.to_string(), value.to_string());
    }

    pub fn add_result<T: ToString>(&mut self, key: &str, value: T) {
        self.results.insert(key.to_string(), value.to_string());
    }

    pub fn finish(self, error: Option<&anyhow::Error>) {
        let duration = self.started_at.elapsed();
        
        if let Some(err) = error {
            error!(
                operation = %self.operation,
                duration_ms = duration.as_millis(),
                error = %err,
                params = ?self.params,
                "Handler operation failed"
            );
        } else {
            info!(
                operation = %self.operation,
                duration_ms = duration.as_millis(),
                params = ?self.params,
                results = ?self.results,
                "Handler operation completed"
            );
        }
    }
}

pub struct HandlerObservability {
    service_name: String,
}

impl HandlerObservability {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }

    pub fn track(&self, operation: &str) -> HandlerTracker {
        HandlerTracker::new(operation)
    }
}

// Logger trait para manter compatibilidade
pub trait Logger {
    fn chronometer(&self, method: &str, started_at: &Instant);
}

pub struct DefaultLogger;

impl Logger for DefaultLogger {
    fn chronometer(&self, method: &str, started_at: &Instant) {
        let duration = started_at.elapsed();
        info!(
            method = method,
            duration_ms = duration.as_millis(),
            "Handler method completed"
        );
    }
}

// Struct Handler (equivalente ao Handler do Go)
#[derive(Clone)]
pub struct Handler {
    service_if: Arc<dyn MeuexemploServiceIF + Send + Sync>,
    log: Arc<dyn Logger + Send + Sync>,
    observability: Arc<HandlerObservability>,
}

impl Handler {
    pub fn new(
        service_if: Arc<dyn MeuexemploServiceIF + Send + Sync>,
        log: Arc<dyn Logger + Send + Sync>,
    ) -> Self {
        Self {
            service_if,
            log,
            observability: Arc::new(HandlerObservability::new("meuexemplo")),
        }
    }

    // Criar router com todas as rotas
    pub fn routes(&self) -> Router {
        Router::new()
            .route("/meuexemplo", get(Self::get_meuexemplo))
            .route("/meuexemplo/:id", get(Self::get_meuexemplo_by_id))
            .route("/meuexemplo/status/:statuscode", get(Self::get_meuexemplo_by_status_code))
            .route("/meuexemplo", post(Self::insert_meuexemplo))
            .route("/meuexemplo/:id", put(Self::update_meuexemplo))
            .route("/meuexemplo/:id", delete(Self::delete_meuexemplo_by_id))
            .with_state(self.clone())
    }

    // GET /meuexemplo?limit=10&offset=0
    pub async fn get_meuexemplo(
        State(handler): State<Handler>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        let started_at = Instant::now();
        let mut tracker = handler.observability.track("get_list");
        let mut error_result: Option<anyhow::Error> = None;

        let result = async {
            // Validar parâmetros
            let limit = pagination.limit.unwrap_or(10);
            let offset = pagination.offset.unwrap_or(0);

            tracker.add_param("limit", limit);
            tracker.add_param("offset", offset);

            if limit <= 0 || offset < 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ app::MESSAGE: app::MSG_OFFSET_LIMIT }))
                ));
            }

            // Chamar service
            let result = handler.service_if.get_meuexemplo(offset, limit).await;
            
            match result {
                Ok(items_page) => {
                    tracker.add_result("count", items_page.items.len());
                    tracker.add_result("total", items_page.total);
                    
                    Ok(Json(serde_json::to_value(&items_page).unwrap()))
                }
                Err(e) => {
                    error_result = Some(e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ app::MESSAGE: app::MSG_BAD_REQUEST }))
                    ))
                }
            }
        }.await;

        // Logging e tracking
        tracker.finish(error_result.as_ref());
        handler.log.chronometer("MeuexemploHandle -> GetMeuexemplo", &started_at);
        
        result
    }

    // GET /meuexemplo/:id
    pub async fn get_meuexemplo_by_id(
        State(handler): State<Handler>,
        Path(id): Path<String>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        let started_at = Instant::now();
        let mut tracker = handler.observability.track("get_by_id");
        let mut error_result: Option<anyhow::Error> = None;

        let result = async {
            tracker.add_param("id", &id);

            // Parse ID
            let id = match id.parse::<i64>() {
                Ok(id) => id,
                Err(_) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({ app::MESSAGE: app::MSG_BAD_REQUEST }))
                    ));
                }
            };

            // Chamar service
            let result = handler.service_if.get_meuexemplo_by_id(id).await;
            
            match result {
                Ok(meuexemplo) => {
                    tracker.add_result("found", meuexemplo.is_some());
                    if let Some(ref item) = meuexemplo {
                        tracker.add_result("id", item.id);
                    }
                    
                    Ok(Json(serde_json::to_value(&meuexemplo).unwrap()))
                }
                Err(e) => {
                    error_result = Some(e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ app::MESSAGE: app::MSG_BAD_REQUEST }))
                    ))
                }
            }
        }.await;

        // Logging e tracking
        tracker.finish(error_result.as_ref());
        handler.log.chronometer("MeuexemploHandle -> GetMeuexemploById", &started_at);
        
        result
    }

    // GET /meuexemplo/status/:statuscode
    pub async fn get_meuexemplo_by_status_code(
        State(handler): State<Handler>,
        Path(statuscode): Path<String>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        let started_at = Instant::now();
        let mut tracker = handler.observability.track("handler.GetMeuexemploByStatusCode");
        let mut error_result: Option<anyhow::Error> = None;

        let result = async {
            tracker.add_param("handler.GetMeuexemploByStatusCode.status_code", &statuscode);

            if statuscode.is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ app::MESSAGE: app::MSG_BAD_REQUEST }))
                ));
            }

            // Chamar service
            let result = handler.service_if.get_meuexemplo_by_status_code(&statuscode).await;
            
            match result {
                Ok(meuexemplo) => {
                    tracker.add_result("handler.GetMeuexemploByStatusCode.found", meuexemplo.is_some());
                    Ok(Json(serde_json::to_value(&meuexemplo).unwrap()))
                }
                Err(e) => {
                    error_result = Some(e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ app::MESSAGE: app::MSG_BAD_REQUEST }))
                    ))
                }
            }
        }.await;

        // Logging e tracking
        tracker.finish(error_result.as_ref());
        handler.log.chronometer("MeuexemploHandle -> GetMeuexemploByStatusCode", &started_at);
        
        result
    }

    // DELETE /meuexemplo/:id
    pub async fn delete_meuexemplo_by_id(
        State(handler): State<Handler>,
        Path(id): Path<String>,
    ) -> Result<StatusCode, (StatusCode, Json<Value>)> {
        let started_at = Instant::now();
        let mut tracker = handler.observability.track("handler.DeleteMeuexemploById");
        let mut error_result: Option<anyhow::Error> = None;

        let result = async {
            tracker.add_param("handler.DeleteMeuexemploById.id", &id);

            // Parse ID
            let id = match id.parse::<i64>() {
                Ok(id) => id,
                Err(_) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({ app::MESSAGE: app::MSG_BAD_REQUEST }))
                    ));
                }
            };

            // Chamar service
            let result = handler.service_if.delete_meuexemplo_by_id(id).await;
            
            match result {
                Ok(deleted) => {
                    if !deleted {
                        tracker.add_result("handler.DeleteMeuexemploById.not_found", true);
                        return Err((
                            StatusCode::NOT_FOUND,
                            Json(json!({ app::MESSAGE: "Item not found" }))
                        ));
                    }

                    tracker.add_result("handler.DeleteMeuexemploById.deleted", true);
                    Ok(StatusCode::NO_CONTENT)
                }
                Err(e) => {
                    error_result = Some(e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ app::MESSAGE: app::MSG_INTERNAL_ERROR }))
                    ))
                }
            }
        }.await;

        // Logging e tracking
        tracker.finish(error_result.as_ref());
        handler.log.chronometer("MeuexemploHandle -> DeleteMeuexemploById", &started_at);
        
        result
    }

    // POST /meuexemplo
    pub async fn insert_meuexemplo(
        State(handler): State<Handler>,
        Json(meuexemplo): Json<Meuexemplo>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        let started_at = Instant::now();
        let mut tracker = handler.observability.track("handler.InsertMeuexemplo");
        let mut error_result: Option<anyhow::Error> = None;

        let result = async {
            tracker.add_param("handler.InsertMeuexemplo.name", &meuexemplo.name);
            tracker.add_param("handler.InsertMeuexemplo.status_code", &meuexemplo.status_code);

            // Validação
            if meuexemplo.name.is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ app::MESSAGE: "Name is required" }))
                ));
            }

            // Chamar service
            let result = handler.service_if.insert_meuexemplo(&meuexemplo).await;
            
            match result {
                Ok(inserted_id) => {
                    tracker.add_result("handler.InsertMeuexemplo.inserted_id", inserted_id);
                    Ok(Json(json!({ "id": inserted_id })))
                }
                Err(e) => {
                    error_result = Some(e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ app::MESSAGE: app::MSG_INTERNAL_ERROR }))
                    ))
                }
            }
        }.await;

        // Logging e tracking
        tracker.finish(error_result.as_ref());
        handler.log.chronometer("MeuexemploHandle -> InsertMeuexemplo", &started_at);
        
        result
    }

    // PUT /meuexemplo/:id
    pub async fn update_meuexemplo(
        State(handler): State<Handler>,
        Path(id): Path<String>,
        Json(meuexemplo): Json<Meuexemplo>,
    ) -> Result<StatusCode, (StatusCode, Json<Value>)> {
        let started_at = Instant::now();
        let mut tracker = handler.observability.track("handler.UpdateMeuexemplo");
        let mut error_result: Option<anyhow::Error> = None;

        let result = async {
            tracker.add_param("handler.UpdateMeuexemplo.id", &id);

            // Parse ID
            let id = match id.parse::<i64>() {
                Ok(id) => id,
                Err(_) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({ app::MESSAGE: app::MSG_BAD_REQUEST }))
                    ));
                }
            };

            tracker.add_param("name", &meuexemplo.name);
            tracker.add_param("handler.UpdateMeuexemplo.status_code", &meuexemplo.status_code);

            // Chamar service
            let result = handler.service_if.update_meuexemplo(&meuexemplo, id).await;
            
            match result {
                Ok(()) => {
                    tracker.add_result("handler.UpdateMeuexemplo.updated", true);
                    Ok(StatusCode::NO_CONTENT)
                }
                Err(e) => {
                    error_result = Some(e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ app::MESSAGE: app::MSG_INTERNAL_ERROR }))
                    ))
                }
            }
        }.await;

        // Logging e tracking
        tracker.finish(error_result.as_ref());
        handler.log.chronometer("MeuexemploHandle -> UpdateMeuexemplo", &started_at);
        
        result
    }
}

// Exemplo de configuração do servidor
pub async fn create_app() -> Router {
    // Setup do service (isso viria de dependency injection)
    let service = create_mock_service(); // Implementar conforme necessário
    let logger = Arc::new(DefaultLogger);
    
    // Criar handler
    let handler = Handler::new(service, logger);
    
    // Retornar router com todas as rotas
    handler.routes()
}

// Mock service para exemplo (remover em produção)
fn create_mock_service() -> Arc<dyn MeuexemploServiceIF + Send + Sync> {
    // Implementar com o service real
    unimplemented!("Implement with real service")
}

// Exemplo de main function
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Configurar tracing/logging
    tracing_subscriber::fmt::init();

    // Criar app
    let app = create_app().await;

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Method, Request};
    use axum::body::Body;
    use tower::ServiceExt;

    // Mock service para testes
    struct MockService;

    #[async_trait::async_trait]
    impl MeuexemploServiceIF for MockService {
        async fn get_meuexemplo(
            &self,
            _offset: i64,
            _limit: i64,
        ) -> Result<ItemsPage<Vec<Meuexemplo>>, anyhow::Error> {
            Ok(ItemsPage {
                offset: 0,
                limit: 10,
                total: 1,
                items: vec![],
            })
        }

        async fn get_meuexemplo_by_id(&self, _id: i64) -> Result<Option<Meuexemplo>, anyhow::Error> {
            Ok(None)
        }

        async fn get_meuexemplo_by_status_code(
            &self,
            _status_code: &str,
        ) -> Result<Option<Meuexemplo>, anyhow::Error> {
            Ok(None)
        }

        async fn insert_meuexemplo(&self, _meuexemplo: &Meuexemplo) -> Result<i64, anyhow::Error> {
            Ok(1)
        }

        async fn update_meuexemplo(
            &self,
            _meuexemplo: &Meuexemplo,
            _id: i64,
        ) -> Result<(), anyhow::Error> {
            Ok(())
        }

        async fn delete_meuexemplo_by_id(&self, _id: i64) -> Result<bool, anyhow::Error> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_get_meuexemplo() {
        let service = Arc::new(MockService);
        let logger = Arc::new(DefaultLogger);
        let handler = Handler::new(service, logger);
        let app = handler.routes();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/meuexemplo?limit=10&offset=0")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_meuexemplo_by_id() {
        let service = Arc::new(MockService);
        let logger = Arc::new(DefaultLogger);
        let handler = Handler::new(service, logger);
        let app = handler.routes();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/meuexemplo/1")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}