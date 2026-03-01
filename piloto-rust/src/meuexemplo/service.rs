use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use tracing::instrument;

use crate::shared::ItemsPage;
use crate::meuexemplo::model::Meuexemplo;
use crate::meuexemplo::repository::{MeuexemploRepository, RepositoryError};

#[async_trait]
pub trait MeuexemploService: Send + Sync {
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Meuexemplo>>, ServiceError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Meuexemplo>, ServiceError>;
    async fn get_by_status_code(&self, status_code: &str) -> Result<Option<Meuexemplo>, ServiceError>;
    async fn create(&self, item: &Meuexemplo) -> Result<i64, ServiceError>;
    async fn update(&self, id: i64, item: &Meuexemplo) -> Result<(), ServiceError>;
    async fn delete(&self, id: i64) -> Result<bool, ServiceError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Not found")]
    NotFound,
}

pub struct MeuexemploServiceImpl {
    repo: Arc<dyn MeuexemploRepository>,
}

impl MeuexemploServiceImpl {
    pub fn new(repo: Arc<dyn MeuexemploRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl MeuexemploService for MeuexemploServiceImpl {
    #[instrument(skip(self))]
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Meuexemplo>>, ServiceError> {
        self.repo.list(offset, limit).await.map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn get_by_id(&self, id: i64) -> Result<Option<Meuexemplo>, ServiceError> {
        self.repo.get_by_id(id).await.map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn get_by_status_code(&self, status_code: &str) -> Result<Option<Meuexemplo>, ServiceError> {
        self.repo.get_by_status_code(status_code).await.map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn create(&self, item: &Meuexemplo) -> Result<i64, ServiceError> {
        let now = Utc::now();
        let item = Meuexemplo {
            created_at: now,
            updated_at: now,
            ..item.clone()
        };
        self.repo.insert(&item).await.map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn update(&self, id: i64, item: &Meuexemplo) -> Result<(), ServiceError> {
        let item = Meuexemplo {
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
