# Observabilidade Completa com OpenTelemetry, Prometheus e Grafana

## Métricas Disponíveis

### HTTP Metrics (prefixo: usecase_)
- `http_requests_total`: Total de requisições HTTP
- `http_request_duration_seconds`: Duração das requisições HTTP  
- `http_errors_total`: Total de erros HTTP

### Database Metrics (prefixo: usecase_)
- `db_connections_active`: Conexões ativas do banco
- `db_query_duration_seconds`: Duração das queries
- `db_queries_total`: Total de queries executadas
- `db_query_errors_total`: Total de erros em queries

### Layer Metrics (NOVO!)
- `handler_duration_seconds`: Tempo na camada handler
- `service_duration_seconds`: Tempo na camada service
- `repository_duration_seconds`: Tempo na camada repository

### Timeout & Performance Metrics (NOVO!)
- `timeout_operations_total`: Operações que deram timeout
- `canceled_operations_total`: Operações canceladas
- `slow_operations_total`: Operações lentas (acima do threshold)

### System Metrics
- `memory_usage_bytes`: Uso de memória
- `goroutines_active`: Goroutines ativas
- `gc_duration_seconds`: Duração do garbage collector

### Business Metrics (prefixo: usecase_)
- `business_operations_total`: Total de operações de negócio
- `business_operation_duration_seconds`: Duração das operações de negócio

## Thresholds Configurados
- **Handler**: > 1s = operação lenta
- **Service**: > 500ms = operação lenta
- **Repository**: > 200ms = operação lenta

## Como Usar

1. Inicialize a observabilidade:
```go
observabilidade.InitObservability("service-name", "1.0.0")
```

2. Use middleware HTTP aprimorado:
```go
router.Use(observabilidade.EnhancedHTTPMetricsMiddleware("usecase-name"))
```

3. Para métricas de Service:
```go
serviceWrapper := observabilidade.NewServiceWrapper("usecase-name")
serviceWrapper.WrapOperation(ctx, "operation", func(ctx context.Context) error {
    // sua lógica aqui
})
```

4. Para métricas de Repository:
```go
dbMetrics := observabilidade.NewEnhancedDBMetricsWrapper("usecase-name")
dbMetrics.WrapQuery(ctx, "SELECT", "table", func() error {
    // sua query aqui
})
```

## Acessar Métricas

- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/admin123)
- **Métricas da App**: http://localhost:2112/metrics

## Dashboards Incluídos

- Dashboard principal com métricas HTTP, DB e sistema
- Dashboard de camadas com timing por handler/service/repository
- Métricas de timeout e operações lentas
