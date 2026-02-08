use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};

// Imports dos módulos (assumindo que existem)
use crate::model::{ItemsPage, Meuexemplo};
use crate::repository::MeuexemploRepositoryIF;

// Constantes de mensagens de erro (equivalente ao app.MsgRepositoryError)
pub mod app {
    pub const MSG_REPOSITORY_ERROR: &str = "Repository error occurred";
}

// Sistema de observabilidade para Service
pub struct ServiceTracker {
    operation: String,
    started_at: Instant,
    params: std::collections::HashMap<String, String>,
    results: std::collections::HashMap<String, String>,
}

impl ServiceTracker {
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
                "Service operation failed"
            );
        } else {
            info!(
                operation = %self.operation,
                duration_ms = duration.as_millis(),
                params = ?self.params,
                results = ?self.results,
                "Service operation completed"
            );
        }
    }
}

pub struct ServiceObservability {
    service_name: String,
}

impl ServiceObservability {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }

    pub fn track(&self, operation: &str) -> ServiceTracker {
        ServiceTracker::new(operation)
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
            "Service method completed"
        );
    }
}

// Trait equivalente à interface Go
#[async_trait]
pub trait MeuexemploServiceIF {
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

// Struct Resource (equivalente ao Resource do Go)
pub struct Resource {
    meuexemplo_repo: Arc<dyn MeuexemploRepositoryIF + Send + Sync>,
    log: Box<dyn Logger + Send + Sync>,
    observability: ServiceObservability,
}

impl Resource {
    pub fn new(
        meuexemplo_repo: Arc<dyn MeuexemploRepositoryIF + Send + Sync>,
        log: Box<dyn Logger + Send + Sync>,
    ) -> Self {
        Self {
            meuexemplo_repo,
            log,
            observability: ServiceObservability::new("service.meuexemplo"),
        }
    }
}

#[async_trait]
impl MeuexemploServiceIF for Resource {
    async fn get_meuexemplo(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<ItemsPage<Vec<Meuexemplo>>, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track("service.GetMeuexemplo.get_list");
        
        tracker.add_param("service.GetMeuexemplo.offset", offset);
        tracker.add_param("service.GetMeuexemplo.limit", limit);

        let result = async {
            let items_page = self.meuexemplo_repo.get_meuexemplo(offset, limit).await;
            
            let items_page = match items_page {
                Ok(items_page) => items_page,
                Err(_) => {
                    return Err(anyhow::anyhow!(app::MSG_REPOSITORY_ERROR));
                }
            };

            // Adicionar resultado
            tracker.add_result("service.GetMeuexemplo.count", items_page.items.len());
            tracker.add_result("service.GetMeuexemplo.total", items_page.total);

            Ok::<ItemsPage<Vec<Meuexemplo>>, anyhow::Error>(items_page)
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploService -> GetMeuexemplo", &started_at);
        result
    }

    async fn get_meuexemplo_by_id(&self, id: i64) -> Result<Option<Meuexemplo>, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track("service.GetMeuexemploById");
        
        tracker.add_param("service.GetMeuexemploById.id", id);

        let result = async {
            let meuexemplo = self.meuexemplo_repo.get_meuexemplo_by_id(id).await;
            
            let meuexemplo = match meuexemplo {
                Ok(meuexemplo) => meuexemplo,
                Err(_) => {
                    return Err(anyhow::anyhow!(app::MSG_REPOSITORY_ERROR));
                }
            };

            tracker.add_result("service.GetMeuexemploById.found", meuexemplo.is_some());
            Ok::<Option<Meuexemplo>, anyhow::Error>(meuexemplo)
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploService -> GetMeuexemploById", &started_at);
        result
    }

    async fn get_meuexemplo_by_status_code(
        &self,
        status_code: &str,
    ) -> Result<Option<Meuexemplo>, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track("service.GetMeuexemploByStatusCode");
        
        tracker.add_param("service.GetMeuexemploByStatusCode.status_code", status_code);

        let result = async {
            let meuexemplo = self.meuexemplo_repo.get_meuexemplo_by_status_code(status_code).await;
            
            let meuexemplo = match meuexemplo {
                Ok(meuexemplo) => meuexemplo,
                Err(_) => {
                    return Err(anyhow::anyhow!(app::MSG_REPOSITORY_ERROR));
                }
            };

            tracker.add_result("service.GetMeuexemploByStatusCode.found", meuexemplo.is_some());
            Ok::<Option<Meuexemplo>, anyhow::Error>(meuexemplo)
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploService -> GetMeuexemploByStatusCode", &started_at);
        result
    }

    async fn delete_meuexemplo_by_id(&self, id: i64) -> Result<bool, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track("service.DeleteMeuexemploById.delete");
        
        tracker.add_param("service.DeleteMeuexemploById.id", id);

        let result = async {
            let result = self.meuexemplo_repo.delete_meuexemplo_by_id(id).await;
            
            let result = match result {
                Ok(result) => result,
                Err(_) => {
                    return Err(anyhow::anyhow!(app::MSG_REPOSITORY_ERROR));
                }
            };

            tracker.add_result("service.DeleteMeuexemploById.deleted", result);
            Ok::<bool, anyhow::Error>(result)
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploService -> DeleteMeuexemploById", &started_at);
        result
    }

    async fn insert_meuexemplo(&self, meuexemplo: &Meuexemplo) -> Result<i64, anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track("service.InsertMeuexemplo");
        
        tracker.add_param("service.InsertMeuexemplo.name", &meuexemplo.name);
        tracker.add_param("service.InsertMeuexemplo.status_code", &meuexemplo.status_code);

        let result = async {
            let inserted_id = self.meuexemplo_repo.insert_meuexemplo(meuexemplo).await;
            
            let inserted_id = match inserted_id {
                Ok(inserted_id) => inserted_id,
                Err(_) => {
                    return Err(anyhow::anyhow!(app::MSG_REPOSITORY_ERROR));
                }
            };

            tracker.add_result("service.InsertMeuexemplo.inserted_id", inserted_id);
            Ok::<i64, anyhow::Error>(inserted_id)
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploService -> InsertMeuexemplo", &started_at);
        result
    }

    async fn update_meuexemplo(
        &self,
        meuexemplo: &Meuexemplo,
        id: i64,
    ) -> Result<(), anyhow::Error> {
        let started_at = Instant::now();
        let mut tracker = self.observability.track("service.UpdateMeuexemplo");
        
        tracker.add_param("service.UpdateMeuexemplo.id", id);
        tracker.add_param("service.UpdateMeuexemplo.name", &meuexemplo.name);
        tracker.add_param("service.UpdateMeuexemplo.status_code", &meuexemplo.status_code);

        let result = async {
            let result = self.meuexemplo_repo.update_meuexemplo(meuexemplo, id).await;
            
            match result {
                Ok(()) => {
                    tracker.add_result("service.UpdateMeuexemplo.updated", true);
                    Ok::<(), anyhow::Error>(())
                }
                Err(_) => {
                    Err(anyhow::anyhow!(app::MSG_REPOSITORY_ERROR))
                }
            }
        }.await;

        match &result {
            Ok(_) => tracker.finish(None),
            Err(e) => tracker.finish(Some(e)),
        }

        self.log.chronometer("MeuexemploService -> UpdateMeuexemplo", &started_at);
        result
    }
}

// Exemplo de uso e testes
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    // Mock do Repository para testes
    struct MockRepository;

    #[async_trait]
    impl MeuexemploRepositoryIF for MockRepository {
        async fn get_meuexemplo(
            &self,
            _offset: i64,
            _limit: i64,
        ) -> Result<ItemsPage<Vec<Meuexemplo>>, anyhow::Error> {
            Ok(ItemsPage {
                offset: 0,
                limit: 10,
                total: 1,
                items: vec![Meuexemplo {
                    id: 1,
                    status_code: "ACTIVE".to_string(),
                    name: "Test".to_string(),
                    description: "Test description".to_string(),
                    allows_transactions: true,
                    max_transaction_amount: Decimal::from_str("1000.0").unwrap(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    full_count: 1,
                }],
            })
        }

        async fn get_meuexemplo_by_id(&self, _id: i64) -> Result<Option<Meuexemplo>, anyhow::Error> {
            Ok(Some(Meuexemplo {
                id: 1,
                status_code: "ACTIVE".to_string(),
                name: "Test".to_string(),
                description: "Test description".to_string(),
                allows_transactions: true,
                max_transaction_amount: Decimal::from_str("1000.0").unwrap(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                full_count: 1,
            }))
        }

        async fn get_meuexemplo_by_status_code(
            &self,
            _status_code: &str,
        ) -> Result<Option<Meuexemplo>, anyhow::Error> {
            Ok(Some(Meuexemplo {
                id: 1,
                status_code: "ACTIVE".to_string(),
                name: "Test".to_string(),
                description: "Test description".to_string(),
                allows_transactions: true,
                max_transaction_amount: Decimal::from_str("1000.0").unwrap(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                full_count: 1,
            }))
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
    async fn test_service_operations() {
        let repo = Arc::new(MockRepository);
        let logger = Box::new(DefaultLogger);
        let service = Resource::new(repo, logger);

        // Test get list
        let result = service.get_meuexemplo(0, 10).await;
        assert!(result.is_ok());

        // Test get by id
        let result = service.get_meuexemplo_by_id(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Test get by status code
        let result = service.get_meuexemplo_by_status_code("ACTIVE").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Test insert
        let meuexemplo = Meuexemplo {
            id: 0,
            status_code: "ACTIVE".to_string(),
            name: "Test".to_string(),
            description: "Test description".to_string(),
            allows_transactions: true,
            max_transaction_amount: Decimal::from_str("1000.0").unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            full_count: 0,
        };

        let result = service.insert_meuexemplo(&meuexemplo).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Test update
        let result = service.update_meuexemplo(&meuexemplo, 1).await;
        assert!(result.is_ok());

        // Test delete
        let result = service.delete_meuexemplo_by_id(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}