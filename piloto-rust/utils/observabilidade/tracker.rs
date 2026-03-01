use opentelemetry::KeyValue;
use std::collections::HashMap;
use std::time::Instant;

// ObservabilityTracker rastreia uma operação completa com defer
pub struct ObservabilityTracker {
    usecase: String,
    operation: String,
    layer: String,
    start: Instant,
    params: HashMap<String, String>,
    result_data: HashMap<String, String>,
    timeout_detector: crate::observabilidade::TimeoutDetector,
    layer_timing: crate::observabilidade::LayerTiming,
}

impl ObservabilityTracker {
    // AddParam adiciona parâmetros para logging (opcional)
    pub fn add_param<T: ToString>(&mut self, key: &str, value: T) {
        self.params.insert(key.to_string(), value.to_string());
    }

    // AddResult adiciona dados do resultado (opcional)
    pub fn add_result<T: ToString>(&mut self, key: &str, value: T) {
        self.result_data.insert(key.to_string(), value.to_string());
    }

    // Finish finaliza o rastreamento - DEVE ser chamado com defer
    pub fn finish(self, err: Option<&anyhow::Error>) {
        let duration = self.start.elapsed();

        // 1. Verificar timeout
        self.timeout_detector.check_timeout(false, false); // Simplificado

        // 2. Finalizar timing da camada
        self.layer_timing.finish(err);

        // 3. Registrar métricas específicas por camada
        self.record_layer_metrics(duration, err);

        // 4. Registrar métricas de negócio se for service layer
        if self.layer == "service" {
            crate::observabilidade::record_business_operation(
                &self.usecase,
                &self.operation,
                duration,
                err.is_none(),
            );
        }

        // 5. Log estruturado automático
        self.record_structured_log(duration, err);
    }

    // recordLayerMetrics registra métricas específicas da camada
    fn record_layer_metrics(&self, duration: std::time::Duration, err: Option<&anyhow::Error>) {
        let mut attrs = vec![
            KeyValue::new("usecase", self.usecase.clone()),
            KeyValue::new("operation", self.operation.clone()),
            KeyValue::new("layer", self.layer.clone()),
            KeyValue::new("success", err.is_none()),
        ];

        // Adicionar parâmetros como atributos (limitado)
        for (key, value) in &self.params {
            if attrs.len() < 10 {
                attrs.push(KeyValue::new(format!("param_{}", key), value.clone()));
            }
        }

        // Registrar métricas baseadas na camada seria feito aqui usando as funções já definidas
        // Por exemplo: record_handler_duration, etc.

        // Registrar operação geral
        let status = if err.is_some() { "error" } else { "success" };
        attrs.push(KeyValue::new("status", status));
    }

    // recordStructuredLog cria log estruturado automático
    fn record_structured_log(&self, duration: std::time::Duration, err: Option<&anyhow::Error>) {
        let mut log_data = HashMap::new();
        log_data.insert("usecase".to_string(), self.usecase.clone());
        log_data.insert("operation".to_string(), self.operation.clone());
        log_data.insert("layer".to_string(), self.layer.clone());
        log_data.insert("duration_ms".to_string(), duration.as_millis().to_string());
        log_data.insert("success".to_string(), err.is_none().to_string());
        log_data.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());

        // Adicionar parâmetros
        for (key, value) in &self.params {
            log_data.insert(format!("param_{}", key), value.clone());
        }

        // Adicionar resultado
        for (key, value) in &self.result_data {
            log_data.insert(format!("result_{}", key), value.clone());
        }

        // Adicionar erro se existir
        if let Some(error) = err {
            log_data.insert("error".to_string(), error.to_string());
            log_data.insert("error_type".to_string(), get_error_type(error));
        }

        // Registrar como métrica de log estruturado
        record_structured_log_metric(log_data);
    }
}

// StartOperation inicia o rastreamento de uma operação
pub fn start_operation(usecase: &str, operation: &str, layer: &str) -> ObservabilityTracker {
    ObservabilityTracker {
        usecase: usecase.to_string(),
        operation: operation.to_string(),
        layer: layer.to_string(),
        start: Instant::now(),
        params: HashMap::new(),
        result_data: HashMap::new(),
        timeout_detector: crate::observabilidade::TimeoutDetector::new(usecase, operation),
        layer_timing: crate::observabilidade::LayerTiming::new(usecase, operation, layer),
    }
}

// Helpers
fn is_validation_error(err: &anyhow::Error) -> bool {
    let err_str = err.to_string();
    err_str.contains("validation") ||
        err_str.contains("invalid") ||
        err_str.contains("required") ||
        err_str.contains("Bad Request")
}

fn get_error_type(err: &anyhow::Error) -> String {
    let err_str = err.to_string();

    if err_str.contains("timeout") || err_str.contains("deadline") {
        "timeout".to_string()
    } else if err_str.contains("connection") || err_str.contains("network") {
        "network".to_string()
    } else if is_validation_error(err) {
        "validation".to_string()
    } else if err_str.contains("not found") {
        "not_found".to_string()
    } else {
        "internal".to_string()
    }
}

fn record_structured_log_metric(log_data: HashMap<String, String>) {
    let mut attrs = Vec::new();

    for (key, value) in log_data {
        if attrs.len() < 15 {
            attrs.push(KeyValue::new(key, value));
        }
    }

    // Registrar como métrica seria feito aqui
}