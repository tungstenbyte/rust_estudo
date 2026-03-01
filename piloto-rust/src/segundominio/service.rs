use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use tracing::instrument;

use crate::shared::ItemsPage;
use crate::segundominio::model::Segundominio;
use crate::segundominio::repository::{RepositoryError, SegundominioRepository};

#[async_trait]
pub trait SegundominioService: Send + Sync {
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Segundominio>>, ServiceError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Segundominio>, ServiceError>;
    async fn create(&self, item: &Segundominio) -> Result<i64, ServiceError>;
    async fn update(&self, id: i64, item: &Segundominio) -> Result<(), ServiceError>;
    async fn delete(&self, id: i64) -> Result<bool, ServiceError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Not found")]
    NotFound,
}

pub struct SegundominioServiceImpl {
    repo: Arc<dyn SegundominioRepository>,
}

impl SegundominioServiceImpl {
    pub fn new(repo: Arc<dyn SegundominioRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl SegundominioService for SegundominioServiceImpl {
    #[instrument(skip(self))]
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Segundominio>>, ServiceError> {
        self.repo.list(offset, limit).await.map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn get_by_id(&self, id: i64) -> Result<Option<Segundominio>, ServiceError> {
        self.repo.get_by_id(id).await.map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn create(&self, item: &Segundominio) -> Result<i64, ServiceError> {
        let now = Utc::now();
        let item = Segundominio {
            created_at: now,
            updated_at: now,
            ..item.clone()
        };
        self.repo.insert(&item).await.map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn update(&self, id: i64, item: &Segundominio) -> Result<(), ServiceError> {
        let item = Segundominio {
            updated_at: Utc::now(),
            ..item.clone()
        };
        self.repo.update(id, &item).await.map_err(|e| match e {
            RepositoryError::NotFound => ServiceError::NotFound,
            e => ServiceError::Repository(e),
        })
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: i64) -> Result<bool, ServiceError> {
        self.repo.delete(id).await.map_err(Into::into)
    }
}
