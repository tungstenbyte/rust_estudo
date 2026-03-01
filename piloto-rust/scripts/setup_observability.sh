#!/bin/bash

# <---- adicionado aqui: Script completo para setup da observabilidade

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DOCKER_DIR="$PROJECT_ROOT/docker"

echo "🚀 Setup da Observabilidade - Palm Pay"
echo "====================================="

# Função para verificar se um comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verificar dependências
echo "🔍 Verificando dependências..."

if ! command_exists docker; then
    echo "❌ Docker não encontrado. Instale o Docker primeiro."
    exit 1
fi

if ! command_exists docker-compose; then
    echo "❌ Docker Compose não encontrado. Instale o Docker Compose primeiro."
    exit 1
fi

if ! command_exists go; then
    echo "❌ Go não encontrado. Instale o Go primeiro."
    exit 1
fi

echo "✅ Todas as dependências encontradas"

# Criar diretórios necessários
echo ""
echo "📁 Criando estrutura de diretórios..."

mkdir -p "$DOCKER_DIR/grafana/provisioning/datasources"
mkdir -p "$DOCKER_DIR/grafana/provisioning/dashboards"
mkdir -p "$DOCKER_DIR/grafana/dashboards"

echo "✅ Diretórios criados"

# Criar arquivo de datasource do Grafana
echo ""
echo "⚙️ Configurando datasources do Grafana..."

cat > "$DOCKER_DIR/grafana/provisioning/datasources/prometheus.yml" << 'EOF'
apiVersion: 1

datasources:
  - name: prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
    editable: false
EOF

# Criar arquivo de provisioning de dashboards
cat > "$DOCKER_DIR/grafana/provisioning/dashboards/dashboards.yml" << 'EOF'
apiVersion: 1

providers:
  - name: 'default'
    orgId: 1
    folder: ''
    type: file
    disableDeletion: false
    updateIntervalSeconds: 10
    allowUiUpdates: true
    options:
      path: /var/lib/grafana/dashboards
EOF

echo "✅ Configurações do Grafana criadas"

# Instalar dependências Go
echo ""
echo "📦 Instalando dependências Go..."

cd "$PROJECT_ROOT"

# Verificar se go.mod existe
if [ ! -f "go.mod" ]; then
    echo "❌ go.mod não encontrado. Execute 'go mod init' primeiro."
    exit 1
fi

# Adicionar dependências de observabilidade
go get go.opentelemetry.io/otel@v1.21.0
go get go.opentelemetry.io/otel/metric@v1.21.0
go get go.opentelemetry.io/otel/sdk@v1.21.0
go get go.opentelemetry.io/otel/sdk/metric@v1.21.0
go get go.opentelemetry.io/otel/exporters/prometheus@v0.44.0
go get go.opentelemetry.io/otel/semconv/v1.17.0@v1.17.0
go get github.com/prometheus/client_golang@v1.17.0

go mod tidy

echo "✅ Dependências Go instaladas"

# Iniciar stack de monitoramento
echo ""
echo "🐳 Iniciando stack de monitoramento..."

cd "$DOCKER_DIR"

# Parar containers existentes (se houver)
docker-compose -f docker-compose.monitoring.yml down --remove-orphans 2>/dev/null || true

# Iniciar nova stack
docker-compose -f docker-compose.monitoring.yml up -d

echo "✅ Stack de monitoramento iniciada"

# Aguardar serviços ficarem prontos
echo ""
echo "⏳ Aguardando serviços ficarem prontos..."

wait_for_service() {
    local service_name=$1
    local url=$2
    local max_attempts=30
    local attempt=1

    echo "Aguardando $service_name..."
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s "$url" > /dev/null 2>&1; then
            echo "✅ $service_name está pronto"
            return 0
        fi
        
        echo "Tentativa $attempt/$max_attempts - $service_name ainda não está pronto..."
        sleep 2
        attempt=$((attempt + 1))
    done
    
    echo "❌ $service_name não ficou pronto a tempo"
    return 1
}

# Aguardar serviços
wait_for_service "Prometheus" "http://localhost:9090/-/ready"
wait_for_service "Grafana" "http://localhost:3000/api/health"
wait_for_service "AlertManager" "http://localhost:9093/-/ready"

# Verificar se a aplicação está rodando
echo ""
echo "🔍 Verificando aplicação..."

if curl -s "http://localhost:8080/health" > /dev/null 2>&1; then
    echo "✅ Aplicação está rodando e respondendo"
    
    # Verificar se métricas estão sendo coletadas
    if curl -s "http://localhost:2112/metrics" | grep -q "http_requests_total"; then
        echo "✅ Métricas estão sendo expostas"
    else
        echo "⚠️  Métricas não encontradas. Verifique se a aplicação está configurada corretamente."
    fi
else
    echo "⚠️  Aplicação não está rodando. Inicie-a com 'go run main.go'"
fi

# Mostrar informações de acesso
echo ""
echo "🎉 Setup concluído com sucesso!"
echo "=============================="
echo ""
echo "📊 Acessos disponíveis:"
echo "  🔗 Aplicação:     http://localhost:8080"
echo "  📈 Métricas:      http://localhost:2112/metrics"
echo "  🏥 Health Check:  http://localhost:8080/health"
echo ""
echo "🔍 Monitoramento:"
echo "  📊 Prometheus:    http://localhost:9090"
echo "  📈 Grafana:       http://localhost:3000 (admin/admin123)"
echo "  🚨 AlertManager:  http://localhost:9093"
echo "  🔍 Jaeger:        http://localhost:16686"
echo ""
echo "🧪 Para testar a observabilidade:"
echo "  ./scripts/test_observability.sh"
echo ""
echo "📚 Documentação completa no README"

# Verificar se jq está instalado para testes
if ! command_exists jq; then
    echo ""
    echo "💡 Dica: Instale 'jq' para melhor visualização dos JSONs:"
    echo "   Ubuntu/Debian: sudo apt-get install jq"
    echo "   MacOS: brew install jq"
    echo "   Windows: choco install jq"
fi

echo ""
echo "✨ Observabilidade completa configurada e pronta para uso!"