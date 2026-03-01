#!/bin/bash

echo "🔧 Corrigindo target do Prometheus para acessar a aplicação..."

# 1. Descobrir o IP correto do host
echo "🔍 Descobrindo IP do host..."

# Opção 1: Gateway do Docker (mais comum)
DOCKER_GATEWAY=$(docker network inspect bridge | grep -A 4 '"IPAM"' | grep '"Gateway"' | head -1 | cut -d'"' -f4)
echo "Gateway Docker: $DOCKER_GATEWAY"

# Opção 2: IP da interface principal
HOST_IP=$(hostname -I | awk '{print $1}')
echo "IP do Host: $HOST_IP"

# Opção 3: IP específico do Docker
DOCKER_HOST_IP=$(ip route | grep docker0 | awk '{print $NF}' | head -1)
echo "Docker Host IP: $DOCKER_HOST_IP"

# 2. Testar qual IP funciona
echo ""
echo "🧪 Testando conectividade..."

IPS_TO_TEST=("$DOCKER_GATEWAY" "$HOST_IP" "$DOCKER_HOST_IP" "172.17.0.1" "192.168.65.1")
WORKING_IP=""

for ip in "${IPS_TO_TEST[@]}"; do
    if [ ! -z "$ip" ]; then
        echo "Testando IP: $ip"
        if curl -s --connect-timeout 3 "http://$ip:2112/metrics" > /dev/null 2>&1; then
            echo "✅ IP $ip FUNCIONOU!"
            WORKING_IP="$ip"
            break
        else
            echo "❌ IP $ip não funcionou"
        fi
    fi
done

if [ -z "$WORKING_IP" ]; then
    echo ""
    echo "🚨 Nenhum IP funcionou automaticamente."
    echo "Vamos usar uma abordagem alternativa..."
    
    # Opção: network_mode host
    echo ""
    echo "📝 Criando docker-compose com network_mode: host..."
    
    cat > docker-compose.observability.yml << 'EOF'
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    container_name: prometheus
    network_mode: host
    volumes:
      - ./observabilidade/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
      - '--web.enable-lifecycle'
      - '--web.listen-address=0.0.0.0:9090'

  grafana:
    image: grafana/grafana:latest
    container_name: grafana
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin123
    volumes:
      - grafana_data:/var/lib/grafana
      - ./observabilidade/grafana/provisioning:/etc/grafana/provisioning
      - ./observabilidade/grafana/dashboards:/var/lib/grafana/dashboards
    networks:
      - monitoring

volumes:
  prometheus_data:
  grafana_data:

networks:
  monitoring:
    driver: bridge
EOF

    # Configuração do Prometheus para localhost
    cat > observabilidade/prometheus.yml << 'EOF'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'novo-exemplo-palm-pay'
    static_configs:
      - targets: ['localhost:2112']
    metrics_path: '/metrics'
    scrape_interval: 5s
EOF

    echo "✅ Configuração atualizada para usar network_mode: host"
    WORKING_IP="localhost"
else
    # Usar o IP que funcionou
    echo ""
    echo "✅ Usando IP que funcionou: $WORKING_IP"
    
    cat > observabilidade/prometheus.yml << EOF
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'novo-exemplo-palm-pay'
    static_configs:
      - targets: ['$WORKING_IP:2112']
    metrics_path: '/metrics'
    scrape_interval: 5s
EOF
fi

echo ""
echo "🔄 Reiniciando containers..."

# Parar containers
docker-compose -f docker-compose.observability.yml down

# Aguardar um pouco
sleep 3

# Subir novamente
docker-compose -f docker-compose.observability.yml up -d

echo ""
echo "⏳ Aguardando containers subirem (30 segundos)..."
sleep 30

echo ""
echo "🧪 Testando conectividade final..."

# Testar se Prometheus consegue acessar
if [ "$WORKING_IP" = "localhost" ]; then
    TEST_URL="http://localhost:9090/api/v1/targets"
else
    TEST_URL="http://localhost:9090/api/v1/targets"
fi

if curl -s "$TEST_URL" | grep -q "novo-exemplo-palm-pay"; then
    echo "✅ Prometheus configurado com sucesso!"
    echo ""
    echo "🎯 Próximos passos:"
    echo "1. Abrir Prometheus: http://localhost:9090/targets"
    echo "2. Verificar se target está UP"
    echo "3. Testar query: up{job=\"novo-exemplo-palm-pay\"}"
    echo ""
else
    echo "⚠️  Verificação automática falhou, mas pode estar funcionando."
    echo "Verifique manualmente em: http://localhost:9090/targets"
fi

echo ""
echo "📊 URLs para verificar:"
echo "- Prometheus Targets: http://localhost:9090/targets"
echo "- Grafana: http://localhost:3000"
echo "- Métricas App: http://localhost:2112/metrics"

echo ""
echo "🔧 Se ainda não funcionar, execute:"
echo "docker logs prometheus"
echo "curl http://localhost:2112/metrics | head -5"