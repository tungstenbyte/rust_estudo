use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue},
    middleware::Next,
    response::Response,
};
use opentelemetry::metrics::{Counter, Meter};
use opentelemetry::KeyValue;
use rand::Rng;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

static REQUESTS_WITHOUT_ID: OnceLock<Counter<u64>> = OnceLock::new();
static REQUEST_ID_GENERATED: OnceLock<Counter<u64>> = OnceLock::new();

// initContextMetrics inicializa métricas de contexto
pub async fn init_context_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let meter = crate::observabilidade::meter();

    let requests_without_id = meter
        .u64_counter("requests_without_id_total")
        .with_description("Total number of requests without request ID")
        .with_unit("1")
        .init();

    let request_id_generated = meter
        .u64_counter("request_id_generated_total")
        .with_description("Total number of generated request IDs")
        .with_unit("1")
        .init();

    REQUESTS_WITHOUT_ID.set(requests_without_id).map_err(|_| "Failed to set requests_without_id")?;
    REQUEST_ID_GENERATED.set(request_id_generated).map_err(|_| "Failed to set request_id_generated")?;

    Ok(())
}

// generateRequestID gera um ID único para a requisição
fn generate_request_id() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 8] = rng.gen();
    hex::encode(bytes)
}

// RequestIDMiddleware adiciona Request ID a todas as requisições
pub async fn request_id_middleware(mut request: Request, next: Next) -> Result<Response, axum::http::StatusCode> {
    // Verificar se já existe um Request ID no header
    let mut request_id = request
        .headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let generated = if request_id.is_empty() {
        // Gerar novo Request ID
        request_id = generate_request_id();
        true
    } else {
        false
    };

    // Registrar métrica
    if let Some(counter) = REQUEST_ID_GENERATED.get() {
        counter.add(1, &[KeyValue::new("generated", generated.to_string())]);
    }

    // Adicionar Request ID ao contexto
    request.extensions_mut().insert(RequestId(request_id.clone()));

    // Processar requisição
    let mut response = next.run(request).await;

    // Adicionar ao response header
    response.headers_mut().insert(
        "X-Request-ID",
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("unknown"))
    );

    Ok(response)
}

// StructuredLoggingMiddleware adiciona logging estruturado
pub fn structured_logging_middleware(usecase: String) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, axum::http::StatusCode>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let usecase = usecase.clone();
        Box::pin(async move {
            let start = Instant::now();

            // Capturar informações da requisição
            let method = request.method().to_string();
            let uri = request.uri().to_string();
            let user_agent = request
                .headers()
                .get("user-agent")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown")
                .to_string();

            let request_id = request
                .extensions()
                .get::<RequestId>()
                .map(|r| r.0.clone())
                .unwrap_or_else(|| "unknown".to_string());

            // Processar requisição
            let response = next.run(request).await;

            // Calcular duração
            let duration = start.elapsed();

            // Log de saída da requisição
            let log_attrs = vec![
                KeyValue::new("usecase", usecase),
                KeyValue::new("request_id", request_id),
                KeyValue::new("method", method),
                KeyValue::new("path", uri),
                KeyValue::new("user_agent", user_agent),
                KeyValue::new("status_code", response.status().as_u16() as i64),
                KeyValue::new("duration_seconds", duration.as_secs_f64()),
            ];

            // Registrar métrica de log estruturado
            record_structured_log(log_attrs);

            Ok(response)
        })
    }
}

// TimeoutMiddleware adiciona timeout padrão se não existir
pub async fn timeout_middleware(timeout: Duration) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, axum::http::StatusCode>> + Send>> {
    move |request: Request, next: Next| {
        Box::pin(async move {
            tokio::time::timeout(timeout, next.run(request))
                .await
                .unwrap_or_else(|_| Err(axum::http::StatusCode::REQUEST_TIMEOUT))
        })
    }
}

// RecordStructuredLog registra log estruturado como métrica
fn record_structured_log(attrs: Vec<KeyValue>) {
    // Encontrar status code para determinar se foi erro
    let mut status_code = 0;
    for attr in &attrs {
        if attr.key.as_str() == "status_code" {
            if let opentelemetry::Value::I64(code) = &attr.value {
                status_code = *code as u16;
                break;
            }
        }
    }

    // Criar métrica baseada no status
    if status_code >= 400 {
        if let Some(counter) = REQUESTS_WITHOUT_ID.get() {
            counter.add(1, &attrs);
        }
    }
}

// GetRequestIDFromContext obtém Request ID do contexto
pub fn get_request_id_from_context(extensions: &axum::http::Extensions) -> String {
    extensions
        .get::<RequestId>()
        .map(|r| r.0.clone())
        .unwrap_or_else(|| "unknown".to_string())
}

// Estrutura para armazenar Request ID nas extensões
#[derive(Clone)]
pub struct RequestId(pub String);