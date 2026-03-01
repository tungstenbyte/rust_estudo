use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::{Duration, Instant};

// HTTPMetricsMiddleware middleware básico para coletar métricas HTTP
pub fn http_metrics_middleware(usecase: String) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, axum::http::StatusCode>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let usecase = usecase.clone();
        Box::pin(async move {
            let start = Instant::now();
            let method = request.method().to_string();
            let path = request.uri().path().to_string();

            // Processar requisição
            let response = next.run(request).await;

            // Registrar métricas
            let duration = start.elapsed();
            let status_code = response.status().as_u16();

            crate::observabilidade::record_http_request(
                &usecase,
                &method,
                &path,
                duration,
                status_code,
            );

            Ok(response)
        })
    }
}

// EnhancedHTTPMetricsMiddleware middleware avançado para métricas HTTP
pub fn enhanced_http_metrics_middleware(usecase: String) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, axum::http::StatusCode>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let usecase = usecase.clone();
        Box::pin(async move {
            let method = request.method().to_string();
            let path = request.uri().path().to_string();

            // Timing da camada handler
            let handler_timing = crate::observabilidade::LayerTiming::new(&usecase, &path, "handler");

            // Detector de timeout
            let timeout_detector = crate::observabilidade::TimeoutDetector::new(&usecase, &path);

            let start = Instant::now();

            // Processar requisição com timeout de 30 segundos
            let result = tokio::time::timeout(Duration::from_secs(30), next.run(request)).await;

            let (response, error, is_timeout) = match result {
                Ok(response) => (response, None, false),
                Err(_) => {
                    // Timeout occurred
                    return Err(axum::http::StatusCode::REQUEST_TIMEOUT);
                }
            };

            // Verificar timeout
            timeout_detector.check_timeout(is_timeout, false);

            // Finalizar timing do handler
            handler_timing.finish(error.as_ref());

            // Registrar métricas HTTP originais
            let duration = start.elapsed();
            let status_code = response.status().as_u16();

            crate::observabilidade::record_http_request(
                &usecase,
                &method,
                &path,
                duration,
                status_code,
            );

            Ok(response)
        })
    }
}