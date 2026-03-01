#!/bin/bash

# <---- adicionado aqui: Script para testar toda a observabilidade

set -e

BASE_URL="http://localhost:8080"
METRICS_URL="http://localhost:2112"

echo "🔍 Testando Sistema de Observabilidade..."
echo "======================================="

# Função para fazer requisições e mostrar métricas
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local description=$4
    
    echo ""
    echo "📊 Testando: $description"
    echo "Endpoint: $method $endpoint"
    
    if [ "$method" = "POST" ] || [ "$method" = "PUT" ]; then
        response=$(curl -s -w "\nSTATUS:%{http_code}\nTIME:%{time_total}" \
                       -H "Content-Type: application/json" \
                       -H "X-Request-ID: test-$(date +%s)" \
                       -X $method \
                       -d "$data" \
                       "$BASE_URL$endpoint")
    else
        response=$(curl -s -w "\nSTATUS:%{http_code}\nTIME:%{time_total}" \
                       -H "X-Request-ID: test-$(date +%s)" \
                       -X $method \
                       "$BASE_URL$endpoint")
    fi
    
    echo "Resposta: $response"
    echo "✅ Requisição concluída"
}

# 1. Testar Health Check
echo "1️⃣ Testando Health Check..."
curl -s "$BASE_URL/health" | jq '.' || echo "Health check OK (sem jq)"

# 2. Testar endpoints básicos
test_endpoint "GET" "/api/meuexemplo?limit=5&offset=0" "" "Listar com paginação"

# 3. Testar validação (erro)
test_endpoint "GET" "/api/meuexemplo?limit=abc&offset=0" "" "Erro de validação (limit inválido)"

# 4. Testar GET por ID
test_endpoint "GET" "/api/meuexemplo/1" "" "Buscar por ID"

# 5. Testar GET por Status Code
test_endpoint "GET" "/api/meuexemplo/statuscode/active" "" "Buscar por status code"

# 6. Testar INSERT
insert_data='{
    "status_code": "active",
    "name": "Teste Observabilidade",
    "description": "Item criado para testar observabilidade",
    "allows_transactions": true,
    "max_transaction_amount": 1000.50,
    "created_at": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
    "updated_at": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}'

test_endpoint "POST" "/api/meuexemplo" "$insert_data" "Criar novo item"

# 7. Testar UPDATE
update_data='{
    "status_code": "updated",
    "name": "Teste Observabilidade Updated",
    "description": "Item atualizado para testar observabilidade",
    "allows_transactions": false,
    "max_transaction_amount": 2000.75,
    "created_at": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
    "updated_at": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}'

test_endpoint "PUT" "/api/meuexemplo/1" "$update_data" "Atualizar item"

# 8. Testar DELETE
test_endpoint "DELETE" "/api/meuexemplo/999" "" "Deletar item inexistente (404)"

# 9. Gerar carga para métricas
echo ""
echo "🚀 Gerando carga para métricas..."
for i in {1..10}; do
    curl -s -H "X-Request-ID: load-test-$i" "$BASE_URL/api/meuexemplo?limit=10&offset=0" > /dev/null &
done
wait

echo ""
echo "📈 Aguardando métricas serem coletadas..."
sleep 3

# 10. Verificar métricas disponíveis
echo ""
echo "📊 Verificando métricas principais..."

check_metric() {
    local metric_name=$1
    local description=$2
    
    count=$(curl -s "$METRICS_URL/metrics" | grep -c "^$metric_name" || echo "0")
    if [ "$count" -gt 0 ]; then
        echo "✅ $description: $count métricas encontradas"
    else
        echo "❌ $description: métrica não encontrada"
    fi
}

# Verificar principais métricas
check_metric "http_requests_total" "Requisições HTTP totais"
check_metric "http_request_duration_seconds" "Duração das requisições"
check_metric "handler_duration_seconds" "Duração do handler"
check_metric "service_duration_seconds" "Duração do service"
check_metric "repository_duration_seconds" "Duração do repository"
check_metric "db_queries_total" "Queries do banco"
check_metric "business_operations_total" "Operações de negócio"
check_metric "handler_validation_errors_total" "Erros de validação"

# 11. Mostrar exemplo de métricas
echo ""
echo "📋 Exemplo de métricas coletadas:"
echo "================================"
curl -s "$METRICS_URL/metrics" | grep -E "(http_requests_total|handler_duration)" | head -5

echo ""
echo "🎉 Teste de observabilidade concluído!"
echo ""
echo "📊 Para ver todas as métricas: curl $METRICS_URL/metrics"
echo "🔍 Para ver health: curl $BASE_URL/health"
echo "📈 Para Grafana, importe o dashboard com as queries do README"