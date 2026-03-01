use opentelemetry::metrics::{Counter, Histogram, UpDownCounter};
use opentelemetry::KeyValue;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time;

// HTTP Metrics
static HTTP_REQUESTS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static HTTP_REQUEST_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();
static HTTP_ERRORS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

// Database Metrics
static DB_CONNECTIONS_ACTIVE: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static DB_QUERY_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();
static DB_QUERIES_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static DB_QUERY_ERRORS: OnceLock<Counter<u64>> = OnceLock::new();

// System Metrics
static MEMORY_USAGE: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static GOROUTINES_ACTIVE: OnceLock<UpDownCounter<i64>> = OnceLock::new();
static GC_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();

// Business Metrics
static BUSINESS_OPERATIONS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static BUSINESS_OPERATION_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();

// initMetrics inicializa todas as métricas básicas
pub async fn init_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let meter = crate::observabilidade::meter();

    // HTTP Metrics
    let http_requests_total = meter
        .u64_counter("http_requests_total")
        .with_description("Total number of HTTP requests")
        .with_unit("1")
        .init();

    let http_request_duration = meter
        .f64_histogram("http_request_duration_seconds")
        .with_description("HTTP request duration in seconds")
        .with_unit("s")
        .init();

    let http_errors_total = meter
        .u64_counter("http_errors_total")
        .with_description("Total number of HTTP errors")
        .with_unit("1")
        .init();

    // Database Metrics
    let db_connections_active = meter
        .i64_up_down_counter("db_connections_active")
        .with_description("Number of active database connections")
        .with_unit("1")
        .init();

    let db_query_duration = meter
        .f64_histogram("db_query_duration_seconds")
        .with_description("Database query duration in seconds")
        .with_unit("s")
        .init();

    let db_queries_total = meter
        .u64_counter("db_queries_total")
        .with_description("Total number of database queries")
        .with_unit("1")
        .init();

    let db_query_errors = meter
        .u64_counter("db_query_errors_total")
        .with_description("Total number of database query errors")
        .with_unit("1")
        .init();

    // System Metrics
    let memory_usage = meter
        .i64_up_down_counter("memory_usage_bytes")
        .with_description("Memory usage in bytes")
        .with_unit("bytes")
        .init();

    let goroutines_active = meter
        .i64_up_down_counter("goroutines_active")
        .with_description("Number of active goroutines")
        .with_unit("1")
        .init();

    let gc_duration = meter
        .f64_histogram("gc_duration_seconds")
        .with_description("Garbage collection duration in seconds")
        .with_unit("s")
        .init();

    // Business Metrics
    let business_operations_total = meter
        .u64_counter("business_operations_total")
        .with_description("Total number of business operations")
        .with_unit("1")
        .init();

    let business_operation_duration = meter
        .f64_histogram("business_operation_duration_seconds")
        .with_description("Business operation duration in seconds")
        .with_unit("s")
        .init();

    // Set all metrics
    HTTP_REQUESTS_TOTAL.set(http_requests_total).map_err(|_| "Failed to set http_requests_total")?;
    HTTP_REQUEST_DURATION.set(http_request_duration).map_err(|_| "Failed to set http_request_duration")?;
    HTTP_ERRORS_TOTAL.set(http_errors_total).map_err(|_| "Failed to set http_errors_total")?;
    DB_CONNECTIONS_ACTIVE.set(db_connections_active).map_err(|_| "Failed to set db_connections_active")?;
    DB_QUERY_DURATION.set(db_query_duration).map_err(|_| "Failed to set db_query_duration")?;
    DB_QUERIES_TOTAL.set(db_queries_total).map_err(|_| "Failed to set db_queries_total")?;
    DB_QUERY_ERRORS.set(db_query_errors).map_err(|_| "Failed to set db_query_errors")?;
    MEMORY_USAGE.set(memory_usage).map_err(|_| "Failed to set memory_usage")?;
    GOROUTINES_ACTIVE.set(goroutines_active).map_err(|_| "Failed to set goroutines_active")?;
    GC_DURATION.set(gc_duration).map_err(|_| "Failed to set gc_duration")?;
    BUSINESS_OPERATIONS_TOTAL.set(business_operations_total).map_err(|_| "Failed to set business_operations_total")?;
    BUSINESS_OPERATION_DURATION.set(business_operation_duration).map_err(|_| "Failed to set business_operation_duration")?;

    Ok(())
}

// RecordHTTPRequest registra uma requisição HTTP
pub fn record_http_request(usecase: &str, method: &str, endpoint: &str, duration: Duration, status_code: u16) {
    let attrs = vec![
        KeyValue::new("usecase", usecase.to_string()),
        KeyValue::new("method", method.to_string()),
        KeyValue::new("endpoint", endpoint.to_string()),
        KeyValue::new("status_code", status_code as i64),
    ];

    if let Some(counter) = HTTP_REQUESTS_TOTAL.get() {
        counter.add(1, &attrs);
    }
    if let Some(histogram) = HTTP_REQUEST_DURATION.get() {
        histogram.record(duration.as_secs_f64(), &attrs);
    }

    if status_code >= 400 {
        if let Some(counter) = HTTP_ERRORS_TOTAL.get() {
            counter.add(1, &attrs);
        }
    }
}

// RecordDBQuery registra uma query de banco de dados
pub fn record_db_query(usecase: &str, operation: &str, table: &str, duration: Duration, error: Option<&anyhow::Error>) {
    let attrs = vec![
        KeyValue::new("usecase", usecase.to_string()),
        KeyValue::new("operation", operation.to_string()),
        KeyValue::new("table", table.to_string()),
    ];

    if let Some(counter) = DB_QUERIES_TOTAL.get() {
        counter.add(1, &attrs);
    }
    if let Some(histogram) = DB_QUERY_DURATION.get() {
        histogram.record(duration.as_secs_f64(), &attrs);
    }

    if error.is_some() {
        if let Some(counter) = DB_QUERY_ERRORS.get() {
            counter.add(1, &attrs);
        }
    }
}

// RecordBusinessOperation registra uma operação de negócio
pub fn record_business_operation(usecase: &str, operation: &str, duration: Duration, success: bool) {
    let attrs = vec![
        KeyValue::new("usecase", usecase.to_string()),
        KeyValue::new("operation", operation.to_string()),
        KeyValue::new("success", success),
    ];

    if let Some(counter) = BUSINESS_OPERATIONS_TOTAL.get() {
        counter.add(1, &attrs);
    }
    if let Some(histogram) = BUSINESS_OPERATION_DURATION.get() {
        histogram.record(duration.as_secs_f64(), &attrs);
    }
}

// UpdateDBConnections atualiza o número de conexões ativas do banco
pub fn update_db_connections(usecase: &str, connections: i32) {
    let attrs = vec![KeyValue::new("usecase", usecase.to_string())];
    if let Some(counter) = DB_CONNECTIONS_ACTIVE.get() {
        counter.add(connections as i64, &attrs);
    }
}

// startSystemMetrics inicia a coleta de métricas do sistema
pub async fn start_system_metrics() {
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;

        let attrs = vec![KeyValue::new("type", "heap")];

        // Note: Rust não tem equivalente direto ao runtime.ReadMemStats do Go
        // Aqui seria necessário usar crates como `sysinfo` para coletar métricas do sistema

        if let Some(counter) = MEMORY_USAGE.get() {
            // Placeholder - implementar com sysinfo
            counter.add(0, &attrs);
        }
        if let Some(counter) = GOROUTINES_ACTIVE.get() {
            // Placeholder - número de tasks async ativas
            counter.add(0, &attrs);
        }
    }
}