#[allow(unused_imports)]
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

use crate::model::{ItemsPage, Meuexemplo};
use crate::service::{MeuexemploService, ServiceError};

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<dyn MeuexemploService>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn routes(service: Arc<dyn MeuexemploService>) -> Router {
    let state = AppState { service };
    Router::new()
        .route("/meuexemplo", get(list).post(create))
        .route(
            "/meuexemplo/:id",
            get(get_by_id).put(update).delete(delete_by_id),
        )
        .route("/meuexemplo/status/:status_code", get(get_by_status_code))
        .with_state(state)
}

#[instrument(skip(state))]
async fn list(
    State(state): State<AppState>,
    Query(p): Query<PaginationQuery>,
) -> Result<Json<ItemsPage<Vec<Meuexemplo>>>, AppError> {
    let limit = p.limit.unwrap_or(10).clamp(1, 100);
    let offset = p.offset.unwrap_or(0).max(0);

    let page = state.service.list(offset, limit).await?;
    Ok(Json(page))
}

#[instrument(skip(state))]
async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Option<Meuexemplo>>, AppError> {
    let id: i64 = id.parse().map_err(|_| AppError::BadRequest("Invalid id"))?;
    let item = state.service.get_by_id(id).await?;
    Ok(Json(item))
}

#[instrument(skip(state))]
async fn get_by_status_code(
    State(state): State<AppState>,
    Path(status_code): Path<String>,
) -> Result<Json<Option<Meuexemplo>>, AppError> {
    let item = state.service.get_by_status_code(&status_code).await?;
    Ok(Json(item))
}

#[instrument(skip(state))]
async fn create(
    State(state): State<AppState>,
    Json(mut item): Json<Meuexemplo>,
) -> Result<Json<serde_json::Value>, AppError> {
    if item.name.is_empty() {
        return Err(AppError::BadRequest("Name is required"));
    }
    item.id = 0;
    let id = state.service.create(&item).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

#[instrument(skip(state))]
async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(item): Json<Meuexemplo>,
) -> Result<StatusCode, AppError> {
    let id: i64 = id.parse().map_err(|_| AppError::BadRequest("Invalid id"))?;
    state.service.update(id, &item).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state))]
async fn delete_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let id: i64 = id.parse().map_err(|_| AppError::BadRequest("Invalid id"))?;
    let deleted = state.service.delete(id).await?;
    if !deleted {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug)]
pub enum AppError {
    Service(ServiceError),
    BadRequest(&'static str),
    NotFound,
}

impl From<ServiceError> for AppError {
    fn from(e: ServiceError) -> Self {
        AppError::Service(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AppError::Service(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, (*msg).to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        };
        (status, Json(serde_json::json!({ "message": message }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    struct MockService;

    #[async_trait::async_trait]
    impl MeuexemploService for MockService {
        async fn list(&self, _: i64, _: i64) -> Result<ItemsPage<Vec<Meuexemplo>>, ServiceError> {
            Ok(ItemsPage { offset: 0, limit: 10, total: 0, items: vec![] })
        }
        async fn get_by_id(&self, _: i64) -> Result<Option<Meuexemplo>, ServiceError> {
            Ok(None)
        }
        async fn get_by_status_code(&self, _: &str) -> Result<Option<Meuexemplo>, ServiceError> {
            Ok(None)
        }
        async fn create(&self, _: &Meuexemplo) -> Result<i64, ServiceError> {
            Ok(1)
        }
        async fn update(&self, _: i64, _: &Meuexemplo) -> Result<(), ServiceError> {
            Ok(())
        }
        async fn delete(&self, _: i64) -> Result<bool, ServiceError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn list_returns_200() {
        let app = routes(Arc::new(MockService));
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/meuexemplo?limit=10&offset=0")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
    }
}
