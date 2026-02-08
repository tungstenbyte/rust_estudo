use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics::MeterProvider;
use opentelemetry_sdk::Resource;
use prometheus::{Encoder, TextEncoder};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time;

static METER: OnceLock<opentelemetry::metrics::Meter> = OnceLock::new();

pub fn meter() -> &'static opentelemetry::metrics::Meter {
    METER.get().expect("Meter not initialized")
}

// InitObservability inicializa OpenTelemetry com Prometheus
pub async fn init_observability(service_name: &str, service_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Criar resource
    let resource = Resource::new(vec![
        KeyValue::new("service.name", service_name.to_string()),
        KeyValue::new("service.version", service_version.to_string()),
    ]);

    // Configurar Prometheus exporter
    let exporter = opentelemetry_prometheus::exporter()
        .with_resource(resource)
        .build()?;

    // Criar metric provider
    let provider = MeterProvider::builder()
        .with_reader(exporter)
        .build();

    // Definir como global
    global::set_meter_provider(provider.clone());

    // Criar meter global
    let meter = provider.meter(service_name);
    METER.set(meter).map_err(|_| "Failed to set global meter")?;

    // Inicializar métricas básicas
    init_metrics().await?;

    // Inicializar métricas por camada
    init_layer_metrics().await?;

    // Inicializar métricas de contexto
    init_context_metrics().await?;

    // Inicializar métricas de sistema
    tokio::spawn(start_system_metrics());

    println!("✅ Observabilidade completa inicializada com sucesso");
    Ok(())
}

// StartMetricsServer inicia o servidor de métricas
pub async fn start_metrics_server(port: &str) {
    use axum::{routing::get, Router};
    use axum::response::Response;
    use axum::http::header;

    async fn metrics_handler() -> Response<String> {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        
        Response::builder()
            .header(header::CONTENT_TYPE, encoder.format_type())
            .body(String::from_utf8(buffer).unwrap())
            .unwrap()
    }

    let app = Router::new().route("/metrics", get(metrics_handler));

    let addr = format!("0.0.0.0:{}", port);
    println!("🔍 Servidor de métricas iniciado na porta {}", port);
    println!("📊 Métricas disponíveis em: http://localhost:{}/metrics", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn init_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Implementar inicialização de métricas básicas
    Ok(())
}

async fn init_layer_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Implementar inicialização de métricas por camada
    Ok(())
}

async fn init_context_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Implementar inicialização de métricas de contexto
    Ok(())
}

async fn start_system_metrics() {
    // Implementar métricas de sistema
}