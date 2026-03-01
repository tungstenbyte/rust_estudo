use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::instrument;

use crate::shared::ItemsPage;
use crate::segundominio::ddl;
use crate::segundominio::model::{Segundominio, SegundominioRow};

#[async_trait]
pub trait SegundominioRepository: Send + Sync {
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Segundominio>>, RepositoryError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Segundominio>, RepositoryError>;
    async fn insert(&self, item: &Segundominio) -> Result<i64, RepositoryError>;
    async fn update(&self, id: i64, item: &Segundominio) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i64) -> Result<bool, RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Row not found")]
    NotFound,
}

pub struct SegundominioRepositoryImpl {
    read: Arc<PgPool>,
    write: Arc<PgPool>,
}

impl SegundominioRepositoryImpl {
    pub fn new(read: Arc<PgPool>, write: Arc<PgPool>) -> Self {
        Self { read, write }
    }
}

#[async_trait]
impl SegundominioRepository for SegundominioRepositoryImpl {
    #[instrument(skip(self))]
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Segundominio>>, RepositoryError> {
        let rows: Vec<SegundominioRow> = sqlx::query_as(ddl::SQL_LIST)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.read.as_ref())
            .await?;

        let total = rows.len() as i64;
        let items = rows.into_iter().map(Segundominio::from).collect();

        Ok(ItemsPage { offset, limit, total, items })
    }

    #[instrument(skip(self))]
    async fn get_by_id(&self, id: i64) -> Result<Option<Segundominio>, RepositoryError> {
        let row: Option<SegundominioRow> = sqlx::query_as(ddl::SQL_GET_BY_ID)
            .bind(id)
            .fetch_optional(self.read.as_ref())
            .await?;

        Ok(row.map(Segundominio::from))
    }

    #[instrument(skip(self))]
    async fn insert(&self, item: &Segundominio) -> Result<i64, RepositoryError> {
        let row: (i64,) = sqlx::query_as(ddl::SQL_INSERT)
            .bind(&item.name)
            .bind(item.created_at)
            .bind(item.updated_at)
            .fetch_one(self.write.as_ref())
            .await?;

        Ok(row.0)
    }

    #[instrument(skip(self))]
    async fn update(&self, id: i64, item: &Segundominio) -> Result<(), RepositoryError> {
        let result = sqlx::query(ddl::SQL_UPDATE)
            .bind(&item.name)
            .bind(item.updated_at)
            .bind(id)
            .execute(self.write.as_ref())
            .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: i64) -> Result<bool, RepositoryError> {
        let result = sqlx::query(ddl::SQL_DELETE)
            .bind(id)
            .execute(self.write.as_ref())
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

impl From<SegundominioRow> for Segundominio {
    fn from(r: SegundominioRow) -> Self {
        Self {
            id: r.id,
            name: r.name,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
