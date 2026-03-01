use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::KeyValue;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

// Métricas por Camada
static HANDLER_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();
static SERVICE_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();
static REPOSITORY_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();

// Métricas de Timeout
static TIMEOUT_OPERATIONS: OnceLock<Counter<u64>> = OnceLock::new();
static CANCELED_OPERATIONS: OnceLock<Counter<u64>> = OnceLock::new();

// Métricas de Performance
static SLOW_OPERATIONS: OnceLock<Counter<u64>> = OnceLock::new();

// Métricas específicas do handler
static HANDLER_VALIDATION_ERRORS: OnceLock<Counter<u64>> = OnceLock::new();
static HANDLER_SERVICE_ERRORS: OnceLock<Counter<u64>> = OnceLock::new();
static HANDLER_OPERATIONS: OnceLock<Counter<u64>> = OnceLock::new();
static HANDLER_CRITICAL_OPS: OnceLock<Counter<u64>> = OnceLock::new();
static HANDLER_NOT_FOUND: OnceLock<Counter<u64>> = OnceLock::new();

// initLayerMetrics inicializa métricas específicas por camada
pub async fn init_layer_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let meter = crate::observabilidade::meter();

    // Handler Layer Metrics
    let handler_duration = meter
        .f64_histogram("handler_duration_seconds")
        .with_description("Time spent in handler layer")
        .with_unit("s")
        .init();

    // Service Layer Metrics
    let service_duration = meter
        .f64_histogram("service_duration_seconds")
        .with_description("Time spent in service layer")
        .with_unit("s")
        .init();

    // Repository Layer Metrics
    let repository_duration = meter
        .f64_histogram("repository_duration_seconds")
        .with_description("Time spent in repository layer")
        .with_unit("s")
        .init();

    // Timeout Metrics
    let timeout_operations = meter
        .u64_counter("timeout_operations_total")
        .with_description("Total number of operations that timed out")
        .with_unit("1")
        .init();

    let canceled_operations = meter
        .u64_counter("canceled_operations_total")
        .with_description("Total number of canceled operations")
        .with_unit("1")
        .init();

    // Performance Metrics
    let slow_operations = meter
        .u64_counter("slow_operations_total")
        .with_description("Total number of slow operations (>threshold)")
        .with_unit("1")
        .init();

    // Handler specific metrics
    let handler_validation_errors = meter
        .u64_counter("handler_validation_errors_total")
        .with_description("Total number of validation errors in handler")
        .with_unit("1")
        .init();

    let handler_service_errors = meter
        .u64_counter("handler_service_errors_total")
        .with_description("Total number of service errors in handler")
        .with_unit("1")
        .init();

    let handler_operations = meter
        .u64_counter("handler_operations_total")
        .with_description("Total number of handler operations")
        .with_unit("1")
        .init();

    let handler_critical_ops = meter
        .u64_counter("handler_critical_operations_total")
        .with_description("Total number of critical operations (create/update/delete)")
        .with_unit("1")
        .init();

    let handler_not_found = meter
        .u64_counter("handler_not_found_total")
        .with_description("Total number of not found cases")
        .with_unit("1")
        .init();

    // Set all metrics
    HANDLER_DURATION.set(handler_duration).map_err(|_| "Failed to set handler_duration")?;
    SERVICE_DURATION.set(service_duration).map_err(|_| "Failed to set service_duration")?;
    REPOSITORY_DURATION.set(repository_duration).map_err(|_| "Failed to set repository_duration")?;
    TIMEOUT_OPERATIONS.set(timeout_operations).map_err(|_| "Failed to set timeout_operations")?;
    CANCELED_OPERATIONS.set(canceled_operations).map_err(|_| "Failed to set canceled_operations")?;
    SLOW_OPERATIONS.set(slow_operations).map_err(|_| "Failed to set slow_operations")?;
    HANDLER_VALIDATION_ERRORS.set(handler_validation_errors).map_err(|_| "Failed to set handler_validation_errors")?;
    HANDLER_SERVICE_ERRORS.set(handler_service_errors).map_err(|_| "Failed to set handler_service_errors")?;
    HANDLER_OPERATIONS.set(handler_operations).map_err(|_| "Failed to set handler_operations")?;
    HANDLER_CRITICAL_OPS.set(handler_critical_ops).map_err(|_| "Failed to set handler_critical_ops")?;
    HANDLER_NOT_FOUND.set(handler_not_found).map_err(|_| "Failed to set handler_not_found")?;

    Ok(())
}

// LayerTiming estrutura para medir tempo por camada
pub struct LayerTiming {
    usecase: String,
    operation: String,
    layer: String,
    start: Instant,
}

impl LayerTiming {
    // NewLayerTiming cria um novo timer para uma camada
    pub fn new(usecase: &str, operation: &str, layer: &str) -> Self {
        Self {
            usecase: usecase.to_string(),
            operation: operation.to_string(),
            layer: layer.to_string(),
            start: Instant::now(),
        }
    }

    // Finish finaliza o timing e registra a métrica
    pub fn finish(self, error: Option<&anyhow::Error>) {
        let duration = self.start.elapsed();

        let attrs = vec![
            KeyValue::new("usecase", self.usecase.clone()),
            KeyValue::new("operation", self.operation.clone()),
            KeyValue::new("layer", self.layer.clone()),
            KeyValue::new("success", error.is_none()),
        ];

        // Registrar duração baseada na camada
        match self.layer.as_str() {
            "handler" => {
                if let Some(histogram) = HANDLER_DURATION.get() {
                    histogram.record(duration.as_secs_f64(), &attrs);
                }
            }
            "service" => {
                if let Some(histogram) = SERVICE_DURATION.get() {
                    histogram.record(duration.as_secs_f64(), &attrs);
                }
            }
            "repository" => {
                if let Some(histogram) = REPOSITORY_DURATION.get() {
                    histogram.record(duration.as_secs_f64(), &attrs);
                }
            }
            _ => {}
        }

        // Detectar operações lentas (>1s para handler, >500ms para service, >200ms para repository)
        let threshold = match self.layer.as_str() {
            "handler" => Duration::from_secs(1),
            "service" => Duration::from_millis(500),
            "repository" => Duration::from_millis(200),
            _ => Duration::from_secs(1),
        };

        if duration > threshold {
            if let Some(counter) = SLOW_OPERATIONS.get() {
                counter.add(1, &attrs);
            }
        }
    }
}

// TimeoutDetector estrutura para detectar timeouts
pub struct TimeoutDetector {
    usecase: String,
    operation: String,
    start: Instant,
}

impl TimeoutDetector {
    // NewTimeoutDetector cria um novo detector de timeout
    pub fn new(usecase: &str, operation: &str) -> Self {
        Self {
            usecase: usecase.to_string(),
            operation: operation.to_string(),
            start: Instant::now(),
        }
    }

    // CheckTimeout verifica se houve timeout
    pub fn check_timeout(&self, is_timeout: bool, is_canceled: bool) {
        let attrs = vec![
            KeyValue::new("usecase", self.usecase.clone()),
            KeyValue::new("operation", self.operation.clone()),
        ];

        if is_timeout {
            if let Some(counter) = TIMEOUT_OPERATIONS.get() {
                counter.add(1, &attrs);
            }
        } else if is_canceled {
            if let Some(counter) = CANCELED_OPERATIONS.get() {
                counter.add(1, &attrs);
            }
        }
    }
}