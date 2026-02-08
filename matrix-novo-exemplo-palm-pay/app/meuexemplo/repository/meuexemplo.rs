use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};
use rust_decimal::Decimal;



// Sistema de observabilidade
pub struct QueryTracker {
    operation: String,
    method: String,
    started_at: Instant,
    params: std::collections::HashMap<String, String>,
    results: std::collections::HashMap<String, String>,
}

impl QueryTracker {
    pub fn new(operation: &str, method: &str) -> Self {
        Self {
            operation: operation.to_string(),
            method: method.to_string(),
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
                method = %self.method,
                duration_ms = duration.as_millis(),
                error = %err,
                params = ?self.params,
                "Database operation failed"
            );
        } else {
            info!(
                operation = %self.operation,
                method = %self.method,
                duration_ms = duration.as_millis(),
                params = ?self.params,
                results = ?self.results,
                "Database operation completed"
            );
        }
    }
}

pub struct RepositoryObservability {
    service_name: String,
}

impl RepositoryObservability {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }

    pub fn track_query(&self, operation: &str, method: &str) -> QueryTracker {
        QueryTracker::new(operation, method)
    }
}

// Trait equivalente à interface Go
#[async_trait]
pub trait MeuexemploRepositoryIF {
    async fn get_meuexemplo(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<ItemsPage<Vec<Meuexemplo>>, anyhow::Error>;
    
    async fn get_meuexemplo_by_id(&self, id: i64) -> Result<Option<Meuexemplo>, anyhow::Error>;
    
    async fn get_meuexemplo_by_status_code(
        &self,
        status_code: &str,
    ) -> Result<Option<Meuexemplo>, anyhow::Error>;
    
    async fn insert_meuexemplo(&self, meuexemplo: &Meuexemplo) -> Result<i64, anyhow::Error>;
    
    async fn update_meuexemplo(
        &self,
        meuexemplo: &Meuexemplo,
        id: i64,
    ) -> Result<(), anyhow::Error>;
    
    async fn delete_meuexemplo_by_id(&self, id: i64) -> Result<bool, anyhow::Error>;
}

// Logger trait para manter compatibilidade
pub trait Logger {
    fn chronometer(&self, method: &str, started_at: &Instant);
    fn error(&self, message: &str, error: &str);
}

pub struct DefaultLogger;

impl Logger for DefaultLogger {
    fn chronometer(&self, method: &str, started_at: &Instant) {
        let duration = started_at.elapsed();
        info!(
            method = method,
            duration_ms = duration.as_millis(),
            "Repository method completed"
        );
    }

    fn error(&self, message: &str, error: &str) {
        error!(message = message, error = error);
    }
}

// Implementação do repositório
pub struct MeuexemploRepository {
    pg_read: Arc<PgPool>,
    pg_write: Arc<PgPool>,
    log: Box<dyn Logger + Send + Sync>,
    observability: RepositoryObservability,
}

impl MeuexemploRepository {
    pub fn new(
        pg_write: Arc<PgPool>,
        pg_read: Arc<PgPool>,
        log: Box<dyn Logger + Send + Sync>,
    ) -> Self {
        Self {
            pg_read,
            pg_write,
            log,
            observability: RepositoryObservability::new("meuexemplo"),
        }
    }
}

#[async_trait]
impl MeuexemploRepositoryIF for MeuexemploRepository {
    async fn get_meuexemplo(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<ItemsPage<Vec<Meuexemplo>>, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track_query("SELECT", "repository.GetMeuexemplo");
        
        tracker.add_param("offset", offset);
        tracker.add_param("limit", limit);

        let result = async {
            let mut meuexemplos: Vec<Meuexemplo> = Vec::new();

            let rows = sqlx::query(SQL_MEUEXEMPLO_LIST)
                .bind(limit)
                .bind(offset)
                .fetch_all(&*self.pg_read)
                .await;

            let rows = match rows {
                Ok(rows) => rows,
                Err(e) => {
                    self.log.error("MeuexemploRepository.repository.GetMeuexemplos.PG: ", &e.to_string());
                    return Err(anyhow::anyhow!(e));
                }
            };

            for row in rows {
                let meuexemplo = Meuexemplo {
                    id: row.get("id"),
                    status_code: row.get("status_code"),
                    name: row.get("name"),
                    description: row.get("description"),
                    allows_transactions: row.get("allows_transactions"),
                    max_transaction_amount: row.get("max_transaction_amount"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    full_count: 0, // Will be set below if needed
                };
                meuexemplos.push(meuexemplo);
            }

            let qty_records = if !meuexemplos.is_empty() {
                meuexemplos[0].full_count
            } else {
                0
            };

            tracker.add_result("rows_returned", meuexemplos.len());
            tracker.add_result("total_count", qty_records);

            Ok::<ItemsPage<Vec<Meuexemplo>>, anyhow::Error>(ItemsPage {
                offset,
                limit,
                total: qty_records,
                items: meuexemplos,
            })
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploRepository -> GetMeuexemplo", &started_at);
        result
    }

    async fn get_meuexemplo_by_id(&self, id: i64) -> Result<Option<Meuexemplo>, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track_query("SELECT", "repository.GetMeuexemploById");
        
        tracker.add_param("repository.GetMeuexemploById.id", id);

        let result = async {
            let row = sqlx::query(SQL_GET_MEUEXEMPLO_BY_ID)
                .bind(id)
                .fetch_optional(&*self.pg_read)
                .await;

            let row = match row {
                Ok(row) => row,
                Err(e) => {
                    self.log.error("MeuexemploRepository.repository.GetMeuexemploById: ", &e.to_string());
                    return Err(anyhow::anyhow!(e));
                }
            };

            if let Some(row) = row {
                let meuexemplo = Meuexemplo {
                    id: row.get("id"),
                    status_code: row.get("status_code"),
                    name: row.get("name"),
                    description: row.get("description"),
                    allows_transactions: row.get("allows_transactions"),
                    max_transaction_amount: row.get("max_transaction_amount"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    full_count: 0,
                };

                tracker.add_result("repository.GetMeuexemploById.found", true);
                Ok(Some(meuexemplo))
            } else {
                Ok(None)
            }
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploRepository -> GetMeuexemploById", &started_at);
        result
    }

    async fn get_meuexemplo_by_status_code(
        &self,
        status_code: &str,
    ) -> Result<Option<Meuexemplo>, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track_query("SELECT", "repository.GetMeuexemploByStatusCode");
        
        tracker.add_param("repository.GetMeuexemploByStatusCode.status_code", status_code);

        let result = async {
            let row = sqlx::query(SQL_GET_MEUEXEMPLO_BY_STATUS_CODE)
                .bind(status_code)
                .fetch_optional(&*self.pg_read)
                .await;

            let row = match row {
                Ok(row) => row,
                Err(e) => {
                    self.log.error("MeuexemploRepository.repository.GetMeuexemploBystatuscode: ", &e.to_string());
                    return Err(anyhow::anyhow!(e));
                }
            };

            if let Some(row) = row {
                let meuexemplo = Meuexemplo {
                    id: row.get("id"),
                    status_code: row.get("status_code"),
                    name: row.get("name"),
                    description: row.get("description"),
                    allows_transactions: row.get("allows_transactions"),
                    max_transaction_amount: row.get("max_transaction_amount"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    full_count: 0,
                };

                tracker.add_result("found", true);
                Ok(Some(meuexemplo))
            } else {
                Ok(None)
            }
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploRepository -> GetMeuexemploByStatusCode", &started_at);
        result
    }

    async fn delete_meuexemplo_by_id(&self, id: i64) -> Result<bool, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track_query("DELETE", "repository.DeleteMeuexemploById");
        
        tracker.add_param("id", id);

        let result = async {
            let command_tag = sqlx::query(SQL_MEUEXEMPLO_DELETE_BY_ID)
                .bind(id)
                .execute(&*self.pg_write)
                .await;

            let command_tag = match command_tag {
                Ok(tag) => tag,
                Err(e) => {
                    self.log.error("MeuexemploRepository.repository.DeleteMeuexemploById: ", &e.to_string());
                    return Err(anyhow::anyhow!(e));
                }
            };

            let rows_affected = command_tag.rows_affected();
            let result = rows_affected > 0;

            tracker.add_result("repository.DeleteMeuexemploById.rows_affected", rows_affected);
            tracker.add_result("repository.DeleteMeuexemploById.deleted", result);

            Ok::<bool, anyhow::Error>(result)
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploRepository -> DeleteMeuexemploById", &started_at);
        result
    }

    async fn insert_meuexemplo(&self, meuexemplo: &Meuexemplo) -> Result<i64, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track_query("INSERT", "repository.InsertMeuexemplo");
        
        tracker.add_param("repository.InsertMeuexemplo.name", &meuexemplo.name);
        tracker.add_param("repository.InsertMeuexemplo.status_code", &meuexemplo.status_code);

        let result = async {
            let row = sqlx::query(SQL_MEUEXEMPLO_INSERT)
                .bind(&meuexemplo.status_code)
                .bind(&meuexemplo.name)
                .bind(&meuexemplo.description)
                .bind(meuexemplo.allows_transactions)
                .bind(meuexemplo.max_transaction_amount)
                .bind(meuexemplo.created_at)
                .bind(meuexemplo.updated_at)
                .fetch_one(&*self.pg_write)
                .await;

            let row = match row {
                Ok(row) => row,
                Err(e) => {
                    self.log.error("MeuexemploRepository.repository.InsertMeuexemplo.PG: ", &e.to_string());
                    return Err(anyhow::anyhow!(e));
                }
            };

            let inserted_id: i64 = row.get("id");
            tracker.add_result("inserted_id", inserted_id);

            Ok::<i64, anyhow::Error>(inserted_id)
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploRepository -> InsertMeuexemplo", &started_at);
        result
    }

    async fn update_meuexemplo(
        &self,
        meuexemplo: &Meuexemplo,
        id: i64,
    ) -> Result<(), anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track_query("UPDATE", "repository.UpdateMeuexemplo");
        
        tracker.add_param("repository.UpdateMeuexemplo.id", id);
        tracker.add_param("repository.UpdateMeuexemplo.name", &meuexemplo.name);
        tracker.add_param("repository.UpdateMeuexemplo.status_code", &meuexemplo.status_code);

        let result = async {
            let command_tag = sqlx::query(SQL_MEUEXEMPLO_UPDATE)
                .bind(&meuexemplo.status_code)
                .bind(&meuexemplo.name)
                .bind(&meuexemplo.description)
                .bind(meuexemplo.allows_transactions)
                .bind(meuexemplo.max_transaction_amount)
                .bind(meuexemplo.created_at)
                .bind(meuexemplo.updated_at)
                .bind(id)
                .execute(&*self.pg_write)
                .await;

            let command_tag = match command_tag {
                Ok(tag) => tag,
                Err(e) => {
                    self.log.error("MeuexemploRepository.repository.UpdateMeuexemplo.PG: ", &e.to_string());
                    return Err(anyhow::anyhow!(e));
                }
            };

            let rows_affected = command_tag.rows_affected();
            if rows_affected == 0 {
                let err = anyhow::anyhow!("no rows affected");
                self.log.error("CustomerRepository.repository.UpdateMeuexemplo.PG: ", &err.to_string());
                return Err(err);
            }

            tracker.add_result("repository.UpdateMeuexemplo.rows_affected", rows_affected);
            Ok::<(), anyhow::Error>(())
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploRepository -> UpdateMeuexemplo", &started_at);
        result
    }
}