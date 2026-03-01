use std::time::Instant;

// HandlerObservability - wrapper simplificado para handlers
pub struct HandlerObservability {
    usecase: String,
}

impl HandlerObservability {
    // NewHandlerObservability cria novo observador para handler
    pub fn new(usecase: &str) -> Self {
        Self {
            usecase: usecase.to_string(),
        }
    }

    // Track inicia rastreamento de uma operação do handler
    pub fn track(&self, operation: &str) -> crate::observabilidade::ObservabilityTracker {
        crate::observabilidade::start_operation(&self.usecase, operation, "handler")
    }
}

// ServiceObservability - wrapper para services
pub struct ServiceObservability {
    usecase: String,
}

impl ServiceObservability {
    // NewServiceObservability cria novo observador para service
    pub fn new(usecase: &str) -> Self {
        Self {
            usecase: usecase.to_string(),
        }
    }

    // Track inicia rastreamento de uma operação do service
    pub fn track(&self, operation: &str) -> crate::observabilidade::ObservabilityTracker {
        crate::observabilidade::start_operation(&self.usecase, operation, "service")
    }
}

// RepositoryObservability - wrapper para repositories
pub struct RepositoryObservability {
    usecase: String,
}

impl RepositoryObservability {
    // NewRepositoryObservability cria novo observador para repository
    pub fn new(usecase: &str) -> Self {
        Self {
            usecase: usecase.to_string(),
        }
    }

    // Track inicia rastreamento de uma operação do repository
    pub fn track(&self, operation: &str) -> crate::observabilidade::ObservabilityTracker {
        crate::observabilidade::start_operation(&self.usecase, operation, "repository")
    }

    // TrackQuery rastreia especificamente queries de banco (mais específico para repositories)
    pub fn track_query(&self, operation: &str, table: &str) -> QueryTracker {
        let tracker = crate::observabilidade::start_operation(
            &self.usecase,
            &format!("{}_{}", operation, table),
            "repository"
        );
        
        QueryTracker {
            observability_tracker: tracker,
            operation: operation.to_string(),
            table: table.to_string(),
            usecase: self.usecase.clone(),
            start: Instant::now(),
        }
    }
}

// QueryTracker - especializado para queries de banco
pub struct QueryTracker {
    observability_tracker: crate::observabilidade::ObservabilityTracker,
    operation: String,
    table: String,
    usecase: String,
    start: Instant,
}

impl QueryTracker {
    pub fn add_param<T: ToString>(&mut self, key: &str, value: T) {
        self.observability_tracker.add_param(key, value);
    }

    pub fn add_result<T: ToString>(&mut self, key: &str, value: T) {
        self.observability_tracker.add_result(key, value);
    }

    // Finish finaliza com métricas específicas de DB
    pub fn finish(self, err: Option<&anyhow::Error>) {
        let duration = self.start.elapsed();

        // Registrar métricas de DB antes do finish geral
        crate::observabilidade::record_db_query(
            &self.usecase,
            &self.operation,
            &self.table,
            duration,
            err,
        );

        // Chamar finish do tracker geral
        self.observability_tracker.finish(err);
    }
}