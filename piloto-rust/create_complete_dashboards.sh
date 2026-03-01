#!/bin/bash

echo "🔧 Corrigindo dashboards com estrutura JSON válida para Grafana..."

# Verificar se a pasta existe
if [ ! -d "observabilidade/grafana/dashboards" ]; then
    echo "❌ Pasta observabilidade/grafana/dashboards não encontrada!"
    exit 1
fi

# Remover dashboards com erro
echo "🗑️ Removendo dashboards com erro..."
rm -f observabilidade/grafana/dashboards/error-analysis.json
rm -f observabilidade/grafana/dashboards/performance-analysis.json
rm -f observabilidade/grafana/dashboards/troubleshooting.json
rm -f observabilidade/grafana/dashboards/business-metrics.json
rm -f observabilidade/grafana/dashboards/system-health.json
rm -f observabilidade/grafana/dashboards/multi-usecase.json

# Dashboard 1: Error Analysis - ESTRUTURA CORRETA
echo "📊 Criando Dashboard de Análise de Erros (corrigido)..."
cat > observabilidade/grafana/dashboards/error-analysis.json << 'EOF'
{
  "id": null,
  "title": "Error Analysis - Análise de Erros",
  "tags": ["errors", "troubleshooting", "palmay"],
  "timezone": "browser",
  "refresh": "5s",
  "time": {
    "from": "now-1h",
    "to": "now"
  },
  "panels": [
    {
      "id": 1,
      "title": "Taxa de Erro Total (%)",
      "type": "stat",
      "targets": [
        {
          "expr": "rate(business_operations_ratio_total{success=\"false\"}[5m]) / rate(business_operations_ratio_total[5m]) * 100",
          "legendFormat": "Error Rate %",
          "refId": "A"
        }
      ],
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "thresholds"
          },
          "thresholds": {
            "steps": [
              {"color": "green", "value": null},
              {"color": "yellow", "value": 1},
              {"color": "red", "value": 5}
            ]
          },
          "unit": "percent"
        }
      },
      "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
    },
    {
      "id": 2,
      "title": "Requests HTTP por Segundo",
      "type": "graph",
      "targets": [
        {
          "expr": "rate(http_requests_ratio_total{usecase=\"meuexemplo\"}[5m])",
          "legendFormat": "RPS",
          "refId": "A"
        }
      ],
      "yAxes": [
        {
          "label": "Requests/sec",
          "min": 0
        }
      ],
      "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
    },
    {
      "id": 3,
      "title": "Operações de Negócio por Segundo",
      "type": "graph",
      "targets": [
        {
          "expr": "rate(business_operations_ratio_total{usecase=\"meuexemplo\"}[5m])",
          "legendFormat": "{{operation}}",
          "refId": "A"
        }
      ],
      "yAxes": [
        {
          "label": "Operations/sec",
          "min": 0
        }
      ],
      "gridPos": {"h": 8, "w": 24, "x": 0, "y": 8}
    }
  ]
}
EOF

# Dashboard 2: Performance Analysis - ESTRUTURA CORRETA  
echo "⚡ Criando Dashboard de Performance (corrigido)..."
cat > observabilidade/grafana/dashboards/performance-analysis.json << 'EOF'
{
  "id": null,
  "title": "Performance Analysis - Análise de Performance",
  "tags": ["performance", "slow", "optimization"],
  "timezone": "browser",
  "refresh": "5s",
  "time": {
    "from": "now-1h",
    "to": "now"
  },
  "panels": [
    {
      "id": 1,
      "title": "P95 por Camada (ms)",
      "type": "graph",
      "targets": [
        {
          "expr": "histogram_quantile(0.95, rate(handler_duration_seconds_bucket{usecase=\"meuexemplo\"}[5m])) * 1000",
          "legendFormat": "Handler P95",
          "refId": "A"
        },
        {
          "expr": "histogram_quantile(0.95, rate(service_duration_seconds_bucket{usecase=\"meuexemplo\"}[5m])) * 1000",
          "legendFormat": "Service P95",
          "refId": "B"
        },
        {
          "expr": "histogram_quantile(0.95, rate(repository_duration_seconds_bucket{usecase=\"meuexemplo\"}[5m])) * 1000",
          "legendFormat": "Repository P95",
          "refId": "C"
        }
      ],
      "yAxes": [
        {
          "label": "Latency (ms)",
          "min": 0
        }
      ],
      "gridPos": {"h": 10, "w": 24, "x": 0, "y": 0}
    },
    {
      "id": 2,
      "title": "Tempo Médio por Camada (ms)",
      "type": "graph",
      "targets": [
        {
          "expr": "rate(handler_duration_seconds_sum{usecase=\"meuexemplo\"}[5m]) / rate(handler_duration_seconds_count{usecase=\"meuexemplo\"}[5m]) * 1000",
          "legendFormat": "Handler Avg",
          "refId": "A"
        },
        {
          "expr": "rate(service_duration_seconds_sum{usecase=\"meuexemplo\"}[5m]) / rate(service_duration_seconds_count{usecase=\"meuexemplo\"}[5m]) * 1000",
          "legendFormat": "Service Avg",
          "refId": "B"
        },
        {
          "expr": "rate(repository_duration_seconds_sum{usecase=\"meuexemplo\"}[5m]) / rate(repository_duration_seconds_count{usecase=\"meuexemplo\"}[5m]) * 1000",
          "legendFormat": "Repository Avg",
          "refId": "C"
        }
      ],
      "yAxes": [
        {
          "label": "Latency (ms)",
          "min": 0
        }
      ],
      "gridPos": {"h": 10, "w": 24, "x": 0, "y": 10}
    }
  ]
}
EOF

# Dashboard 3: Database Analysis - ESTRUTURA CORRETA
echo "🗄️ Criando Dashboard de Database..."
cat > observabilidade/grafana/dashboards/database-analysis.json << 'EOF'
{
  "id": null,
  "title": "Database Analysis - Análise de Banco de Dados",
  "tags": ["database", "db", "queries"],
  "timezone": "browser",
  "refresh": "5s",
  "time": {
    "from": "now-1h",
    "to": "now"
  },
  "panels": [
    {
      "id": 1,
      "title": "Queries por Segundo",
      "type": "graph",
      "targets": [
        {
          "expr": "rate(db_queries_ratio_total{usecase=\"meuexemplo\"}[5m])",
          "legendFormat": "{{operation}} - {{table}}",
          "refId": "A"
        }
      ],
      "yAxes": [
        {
          "label": "Queries/sec",
          "min": 0
        }
      ],
      "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
    },
    {
      "id": 2,
      "title": "Conexões Ativas",
      "type": "stat",
      "targets": [
        {
          "expr": "db_connections_active_ratio{usecase=\"meuexemplo\"}",
          "legendFormat": "Active Connections",
          "refId": "A"
        }
      ],
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "thresholds"
          },
          "thresholds": {
            "steps": [
              {"color": "green", "value": null},
              {"color": "yellow", "value": 50},
              {"color": "red", "value": 80}
            ]
          }
        }
      },
      "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
    },
    {
      "id": 3,
      "title": "Tempo de Query P95 (ms)",
      "type": "graph",
      "targets": [
        {
          "expr": "histogram_quantile(0.95, rate(db_query_duration_seconds_bucket{usecase=\"meuexemplo\"}[5m])) * 1000",
          "legendFormat": "DB Query P95",
          "refId": "A"
        }
      ],
      "yAxes": [
        {
          "label": "Query Time (ms)",
          "min": 0
        }
      ],
      "gridPos": {"h": 8, "w": 24, "x": 0, "y": 8}
    }
  ]
}
EOF

# Dashboard 4: System Health - ESTRUTURA CORRETA
echo "🖥️ Criando Dashboard de System Health..."
cat > observabilidade/grafana/dashboards/system-health.json << 'EOF'
{
  "id": null,
  "title": "System Health - Saúde do Sistema",
  "tags": ["system", "health", "infrastructure"],
  "timezone": "browser",
  "refresh": "5s",
  "time": {
    "from": "now-1h",
    "to": "now"
  },
  "panels": [
    {
      "id": 1,
      "title": "Status da Aplicação",
      "type": "stat",
      "targets": [
        {
          "expr": "up{job=\"novo-exemplo-palm-pay\"}",
          "legendFormat": "Service Status",
          "refId": "A"
        }
      ],
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "thresholds"
          },
          "thresholds": {
            "steps": [
              {"color": "red", "value": 0},
              {"color": "green", "value": 1}
            ]
          },
          "mappings": [
            {
              "options": {
                "0": {"text": "DOWN"},
                "1": {"text": "UP"}
              },
              "type": "value"
            }
          ]
        }
      },
      "gridPos": {"h": 6, "w": 8, "x": 0, "y": 0}
    },
    {
      "id": 2,
      "title": "Memory Usage (MB)",
      "type": "stat",
      "targets": [
        {
          "expr": "memory_usage_bytes{type=\"heap\"} / 1024 / 1024",
          "legendFormat": "Memory (MB)",
          "refId": "A"
        }
      ],
      "fieldConfig": {
        "defaults": {
          "unit": "decbytes"
        }
      },
      "gridPos": {"h": 6, "w": 8, "x": 8, "y": 0}
    },
    {
      "id": 3,
      "title": "Goroutines Ativas",
      "type": "stat",
      "targets": [
        {
          "expr": "goroutines_active_ratio{type=\"heap\"}",
          "legendFormat": "Goroutines",
          "refId": "A"
        }
      ],
      "gridPos": {"h": 6, "w": 8, "x": 16, "y": 0}
    },
    {
      "id": 4,
      "title": "Métricas do Sistema - Timeline",
      "type": "graph",
      "targets": [
        {
          "expr": "memory_usage_bytes{type=\"heap\"} / 1024 / 1024",
          "legendFormat": "Memory (MB)",
          "refId": "A"
        },
        {
          "expr": "goroutines_active_ratio{type=\"heap\"}",
          "legendFormat": "Goroutines",
          "refId": "B"
        }
      ],
      "yAxes": [
        {
          "label": "Mixed Metrics",
          "min": 0
        }
      ],
      "gridPos": {"h": 10, "w": 24, "x": 0, "y": 6}
    }
  ]
}
EOF

# Dashboard 5: Business Overview - ESTRUTURA CORRETA
echo "💼 Criando Dashboard de Business Overview..."
cat > observabilidade/grafana/dashboards/business-overview.json << 'EOF'
{
  "id": null,
  "title": "Business Overview - Visão Geral do Negócio",
  "tags": ["business", "overview", "kpi"],
  "timezone": "browser",
  "refresh": "5s",
  "time": {
    "from": "now-24h",
    "to": "now"
  },
  "panels": [
    {
      "id": 1,
      "title": "Total de Operações - Últimas 24h",
      "type": "stat",
      "targets": [
        {
          "expr": "sum(increase(business_operations_ratio_total{usecase=\"meuexemplo\"}[24h]))",
          "legendFormat": "Total Operations",
          "refId": "A"
        }
      ],
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "thresholds"
          },
          "thresholds": {
            "steps": [
              {"color": "green", "value": null}
            ]
          }
        }
      },
      "gridPos": {"h": 6, "w": 8, "x": 0, "y": 0}
    },
    {
      "id": 2,
      "title": "Taxa de Sucesso - 24h",
      "type": "stat",
      "targets": [
        {
          "expr": "sum(increase(business_operations_ratio_total{usecase=\"meuexemplo\", success=\"true\"}[24h])) / sum(increase(business_operations_ratio_total{usecase=\"meuexemplo\"}[24h])) * 100",
          "legendFormat": "Success Rate",
          "refId": "A"
        }
      ],
      "fieldConfig": {
        "defaults": {
          "unit": "percent",
          "color": {
            "mode": "thresholds"
          },
          "thresholds": {
            "steps": [
              {"color": "red", "value": null},
              {"color": "yellow", "value": 95},
              {"color": "green", "value": 99}
            ]
          }
        }
      },
      "gridPos": {"h": 6, "w": 8, "x": 8, "y": 0}
    },
    {
      "id": 3,
      "title": "P95 Latência - 24h",
      "type": "stat",
      "targets": [
        {
          "expr": "histogram_quantile(0.95, sum(rate(handler_duration_seconds_bucket{usecase=\"meuexemplo\"}[24h]))) * 1000",
          "legendFormat": "P95 Latency",
          "refId": "A"
        }
      ],
      "fieldConfig": {
        "defaults": {
          "unit": "ms",
          "color": {
            "mode": "thresholds"
          },
          "thresholds": {
            "steps": [
              {"color": "green", "value": null},
              {"color": "yellow", "value": 500},
              {"color": "red", "value": 1000}
            ]
          }
        }
      },
      "gridPos": {"h": 6, "w": 8, "x": 16, "y": 0}
    },
    {
      "id": 4,
      "title": "Operações por Hora",
      "type": "graph",
      "targets": [
        {
          "expr": "sum(increase(business_operations_ratio_total{usecase=\"meuexemplo\"}[1h]))",
          "legendFormat": "Operations/hour",
          "refId": "A"
        }
      ],
      "yAxes": [
        {
          "label": "Operations/hour",
          "min": 0
        }
      ],
      "gridPos": {"h": 10, "w": 24, "x": 0, "y": 6}
    }
  ]
}
EOF

echo ""
echo "🔄 Reiniciando Grafana para carregar dashboards corrigidos..."

# Reiniciar Grafana
docker compose -f docker-compose.observability.yml restart grafana

echo ""
echo "⏳ Aguardando Grafana reiniciar (30 segundos)..."
sleep 30

echo ""
echo "✅ Dashboards corrigidos criados com sucesso!"
echo ""
echo "📊 Dashboards disponíveis:"
echo "   ✅ Error Analysis - Análise de Erros"
echo "   ✅ Performance Analysis - Análise de Performance"
echo "   ✅ Database Analysis - Análise de Banco de Dados"
echo "   ✅ System Health - Saúde do Sistema"
echo "   ✅ Business Overview - Visão Geral do Negócio"
echo ""
echo "🌐 Acesse o Grafana: http://localhost:3000"
echo "🔑 Login: admin / Senha: admin123"
echo ""
echo "📈 Para ver dados nos dashboards, gere tráfego:"
echo "   for i in {1..20}; do curl -s 'http://localhost:8000/api/meuexemplo?limit=5&offset=0' > /dev/null; sleep 0.5; done"
echo ""
echo "🎯 Dashboards otimizados para suas métricas específicas!"