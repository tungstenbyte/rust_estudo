mod ddl;

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::instrument;

use crate::model::{ItemsPage, Meuexemplo, MeuexemploRow};

#[async_trait]
pub trait MeuexemploRepository: Send + Sync {
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Meuexemplo>>, RepositoryError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Meuexemplo>, RepositoryError>;
    async fn get_by_status_code(&self, status_code: &str) -> Result<Option<Meuexemplo>, RepositoryError>;
    async fn insert(&self, item: &Meuexemplo) -> Result<i64, RepositoryError>;
    async fn update(&self, id: i64, item: &Meuexemplo) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i64) -> Result<bool, RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Row not found")]
    NotFound,
}

pub struct MeuexemploRepositoryImpl {
    read: Arc<PgPool>,
    write: Arc<PgPool>,
}

impl MeuexemploRepositoryImpl {
    pub fn new(read: Arc<PgPool>, write: Arc<PgPool>) -> Self {
        Self { read, write }
    }
}

#[async_trait]
impl MeuexemploRepository for MeuexemploRepositoryImpl {
    #[instrument(skip(self))]
    async fn list(&self, offset: i64, limit: i64) -> Result<ItemsPage<Vec<Meuexemplo>>, RepositoryError> {
        let rows: Vec<MeuexemploRow> = sqlx::query_as(ddl::SQL_LIST)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.read.as_ref())
            .await?;

        let total = rows.len() as i64;
        let items = rows.into_iter().map(Meuexemplo::from).collect();

        Ok(ItemsPage { offset, limit, total, items })
    }

    #[instrument(skip(self))]
    async fn get_by_id(&self, id: i64) -> Result<Option<Meuexemplo>, RepositoryError> {
        let row: Option<MeuexemploRow> = sqlx::query_as(ddl::SQL_GET_BY_ID)
            .bind(id)
            .fetch_optional(self.read.as_ref())
            .await?;

        Ok(row.map(Meuexemplo::from))
    }

    #[instrument(skip(self))]
    async fn get_by_status_code(&self, status_code: &str) -> Result<Option<Meuexemplo>, RepositoryError> {
        let row: Option<MeuexemploRow> = sqlx::query_as(ddl::SQL_GET_BY_STATUS_CODE)
            .bind(status_code)
            .fetch_optional(self.read.as_ref())
            .await?;

        Ok(row.map(Meuexemplo::from))
    }

    #[instrument(skip(self))]
    async fn insert(&self, item: &Meuexemplo) -> Result<i64, RepositoryError> {
        let row: (i64,) = sqlx::query_as(ddl::SQL_INSERT)
            .bind(&item.status_code)
            .bind(&item.name)
            .bind(&item.description)
            .bind(item.allows_transactions)
            .bind(item.max_transaction_amount)
            .bind(item.created_at)
            .bind(item.updated_at)
            .fetch_one(self.write.as_ref())
            .await?;

        Ok(row.0)
    }

    #[instrument(skip(self))]
    async fn update(&self, id: i64, item: &Meuexemplo) -> Result<(), RepositoryError> {
        let result = sqlx::query(ddl::SQL_UPDATE)
            .bind(&item.status_code)
            .bind(&item.name)
            .bind(&item.description)
            .bind(item.allows_transactions)
            .bind(item.max_transaction_amount)
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

impl From<MeuexemploRow> for Meuexemplo {
    fn from(r: MeuexemploRow) -> Self {
        Self {
            id: r.id,
            status_code: r.status_code,
            name: r.name,
            description: r.description,
            allows_transactions: r.allows_transactions,
            max_transaction_amount: r.max_transaction_amount,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
