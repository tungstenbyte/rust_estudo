#!/bin/bash

echo "🚀 Criando dashboards completos para Grafana..."

# Verificar se a pasta existe
if [ ! -d "observabilidade/grafana/dashboards" ]; then
    echo "❌ Pasta observabilidade/grafana/dashboards não encontrada!"
    echo "Execute primeiro o script de observabilidade principal."
    exit 1
fi

# Dashboard 1: Error Analysis
echo "📊 Criando Dashboard de Análise de Erros..."
cat > observabilidade/grafana/dashboards/error-analysis.json << 'EOF'
{
  "dashboard": {
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
            "expr": "rate(business_operations_total{success=\"false\"}[5m]) / rate(business_operations_total[5m]) * 100",
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
        "gridPos": {"h": 8, "w": 6, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "Top 10 Operações com Mais Erros",
        "type": "table",
        "targets": [
          {
            "expr": "topk(10, rate(business_operations_total{success=\"false\"}[5m]))",
            "legendFormat": "{{usecase}} - {{operation}}",
            "refId": "A",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 18, "x": 6, "y": 0}
      },
      {
        "id": 3,
        "title": "Erros HTTP por Status Code",
        "type": "graph",
        "targets": [
          {
            "expr": "sum by (status_code) (rate(http_errors_total[5m]))",
            "legendFormat": "{{status_code}}",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Errors/sec",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 8}
      },
      {
        "id": 4,
        "title": "Erros de Banco de Dados",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(db_query_errors_total[5m])",
            "legendFormat": "{{usecase}} - {{operation}} - {{table}}",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "DB Errors/sec",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 8}
      },
      {
        "id": 5,
        "title": "Comparação de Erros: Hoje vs Ontem",
        "type": "graph",
        "targets": [
          {
            "expr": "increase(business_operations_total{success=\"false\"}[1h])",
            "legendFormat": "Hoje",
            "refId": "A"
          },
          {
            "expr": "increase(business_operations_total{success=\"false\"}[1h] offset 1d)",
            "legendFormat": "Ontem (mesmo horário)",
            "refId": "B"
          }
        ],
        "yAxes": [
          {
            "label": "Total Errors",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 16}
      },
      {
        "id": 6,
        "title": "Heatmap de Erros por Hora",
        "type": "heatmap",
        "targets": [
          {
            "expr": "increase(business_operations_total{success=\"false\"}[1h])",
            "legendFormat": "{{usecase}} - {{operation}}",
            "refId": "A"
          }
        ],
        "gridPos": {"h": 10, "w": 24, "x": 0, "y": 24}
      }
    ]
  }
}
EOF

# Dashboard 2: Performance Analysis
echo "⚡ Criando Dashboard de Performance..."
cat > observabilidade/grafana/dashboards/performance-analysis.json << 'EOF'
{
  "dashboard": {
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
            "expr": "histogram_quantile(0.95, rate(handler_duration_seconds_bucket[5m])) * 1000",
            "legendFormat": "Handler P95",
            "refId": "A"
          },
          {
            "expr": "histogram_quantile(0.95, rate(service_duration_seconds_bucket[5m])) * 1000",
            "legendFormat": "Service P95",
            "refId": "B"
          },
          {
            "expr": "histogram_quantile(0.95, rate(repository_duration_seconds_bucket[5m])) * 1000",
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
        "alert": {
          "conditions": [
            {
              "evaluator": {
                "params": [1000],
                "type": "gt"
              },
              "operator": {
                "type": "and"
              },
              "query": {
                "params": ["A", "5m", "now"]
              },
              "reducer": {
                "params": [],
                "type": "last"
              },
              "type": "query"
            }
          ],
          "executionErrorState": "alerting",
          "for": "2m",
          "frequency": "10s",
          "handler": 1,
          "name": "Handler P95 Alto",
          "noDataState": "no_data",
          "notifications": []
        },
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "Rate de Operações Lentas",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(slow_operations_total{layer=\"handler\"}[5m])",
            "legendFormat": "Handler (>1s)",
            "refId": "A"
          },
          {
            "expr": "rate(slow_operations_total{layer=\"service\"}[5m])",
            "legendFormat": "Service (>500ms)",
            "refId": "B"
          },
          {
            "expr": "rate(slow_operations_total{layer=\"repository\"}[5m])",
            "legendFormat": "Repository (>200ms)",
            "refId": "C"
          }
        ],
        "yAxes": [
          {
            "label": "Slow Ops/sec",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 8}
      },
      {
        "id": 3,
        "title": "Top 10 Operações Mais Lentas",
        "type": "table",
        "targets": [
          {
            "expr": "topk(10, histogram_quantile(0.95, sum by (operation, usecase) (rate(business_operation_duration_seconds_bucket[5m]))))",
            "legendFormat": "{{usecase}} - {{operation}}",
            "refId": "A",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 8}
      },
      {
        "id": 4,
        "title": "Comparação de Performance: Hoje vs Ontem",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(handler_duration_seconds_bucket[1h])) * 1000",
            "legendFormat": "Hoje - Handler P95",
            "refId": "A"
          },
          {
            "expr": "histogram_quantile(0.95, rate(handler_duration_seconds_bucket[1h] offset 1d)) * 1000",
            "legendFormat": "Ontem - Handler P95",
            "refId": "B"
          }
        ],
        "yAxes": [
          {
            "label": "Latency (ms)",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 16}
      },
      {
        "id": 5,
        "title": "Distribuição de Latência por Percentil",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.50, rate(handler_duration_seconds_bucket[5m])) * 1000",
            "legendFormat": "P50",
            "refId": "A"
          },
          {
            "expr": "histogram_quantile(0.90, rate(handler_duration_seconds_bucket[5m])) * 1000",
            "legendFormat": "P90",
            "refId": "B"
          },
          {
            "expr": "histogram_quantile(0.95, rate(handler_duration_seconds_bucket[5m])) * 1000",
            "legendFormat": "P95",
            "refId": "C"
          },
          {
            "expr": "histogram_quantile(0.99, rate(handler_duration_seconds_bucket[5m])) * 1000",
            "legendFormat": "P99",
            "refId": "D"
          }
        ],
        "yAxes": [
          {
            "label": "Latency (ms)",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 24}
      }
    ]
  }
}
EOF

# Dashboard 3: Troubleshooting
echo "🔍 Criando Dashboard de Troubleshooting..."
cat > observabilidade/grafana/dashboards/troubleshooting.json << 'EOF'
{
  "dashboard": {
    "id": null,
    "title": "Troubleshooting - Resolução de Problemas",
    "tags": ["troubleshooting", "debug", "issues"],
    "timezone": "browser",
    "refresh": "5s",
    "time": {
      "from": "now-1h",
      "to": "now"
    },
    "templating": {
      "list": [
        {
          "name": "usecase",
          "type": "query",
          "query": "label_values(business_operations_total, usecase)",
          "current": {
            "value": "meuexemplo",
            "text": "meuexemplo"
          },
          "hide": 0,
          "includeAll": true,
          "multi": false,
          "options": [],
          "refresh": 1,
          "regex": "",
          "sort": 1
        },
        {
          "name": "operation",
          "type": "query",
          "query": "label_values(business_operations_total{usecase=\"$usecase\"}, operation)",
          "current": {
            "value": "all",
            "text": "All"
          },
          "hide": 0,
          "includeAll": true,
          "multi": true,
          "options": [],
          "refresh": 1,
          "regex": "",
          "sort": 1
        }
      ]
    },
    "panels": [
      {
        "id": 1,
        "title": "Health Check - Status Geral",
        "type": "stat",
        "targets": [
          {
            "expr": "rate(business_operations_total{usecase=\"$usecase\", success=\"true\"}[5m]) / rate(business_operations_total{usecase=\"$usecase\"}[5m]) * 100",
            "legendFormat": "Success Rate %",
            "refId": "A"
          },
          {
            "expr": "rate(timeout_operations_total{usecase=\"$usecase\"}[5m])",
            "legendFormat": "Timeouts/sec",
            "refId": "B"
          },
          {
            "expr": "rate(slow_operations_total{usecase=\"$usecase\"}[5m])",
            "legendFormat": "Slow Ops/sec",
            "refId": "C"
          }
        ],
        "fieldConfig": {
          "defaults": {
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
        "gridPos": {"h": 6, "w": 24, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "Timeline de Problemas - Últimas 24h",
        "type": "graph",
        "targets": [
          {
            "expr": "increase(business_operations_total{usecase=\"$usecase\", success=\"false\"}[1h])",
            "legendFormat": "Erros por hora",
            "refId": "A"
          },
          {
            "expr": "increase(timeout_operations_total{usecase=\"$usecase\"}[1h])",
            "legendFormat": "Timeouts por hora",
            "refId": "B"
          },
          {
            "expr": "increase(slow_operations_total{usecase=\"$usecase\"}[1h])",
            "legendFormat": "Operações lentas por hora",
            "refId": "C"
          }
        ],
        "yAxes": [
          {
            "label": "Occurrences",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 6}
      },
      {
        "id": 3,
        "title": "Drill Down - Por Operação",
        "type": "table",
        "targets": [
          {
            "expr": "sum by (operation) (rate(business_operations_total{usecase=\"$usecase\", operation=~\"$operation\"}[5m]))",
            "legendFormat": "RPS",
            "refId": "A",
            "format": "table"
          },
          {
            "expr": "sum by (operation) (rate(business_operations_total{usecase=\"$usecase\", operation=~\"$operation\", success=\"false\"}[5m]))",
            "legendFormat": "Error Rate",
            "refId": "B",
            "format": "table"
          },
          {
            "expr": "histogram_quantile(0.95, sum by (operation) (rate(business_operation_duration_seconds_bucket{usecase=\"$usecase\", operation=~\"$operation\"}[5m])))",
            "legendFormat": "P95 Latency",
            "refId": "C",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 14}
      },
      {
        "id": 4,
        "title": "Correlação: Tráfego vs Performance",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(business_operations_total{usecase=\"$usecase\"}[5m])",
            "legendFormat": "RPS",
            "refId": "A",
            "yAxis": 1
          },
          {
            "expr": "histogram_quantile(0.95, rate(handler_duration_seconds_bucket{usecase=\"$usecase\"}[5m])) * 1000",
            "legendFormat": "P95 Latency (ms)",
            "refId": "B",
            "yAxis": 2
          }
        ],
        "yAxes": [
          {
            "label": "Requests/sec",
            "min": 0,
            "position": "left"
          },
          {
            "label": "Latency (ms)",
            "min": 0,
            "position": "right"
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 22}
      },
      {
        "id": 5,
        "title": "Database Performance Impact",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(db_query_duration_seconds_bucket{usecase=\"$usecase\"}[5m])) * 1000",
            "legendFormat": "DB Query P95 (ms)",
            "refId": "A"
          },
          {
            "expr": "rate(db_query_errors_total{usecase=\"$usecase\"}[5m])",
            "legendFormat": "DB Errors/sec",
            "refId": "B"
          },
          {
            "expr": "db_connections_active{usecase=\"$usecase\"}",
            "legendFormat": "Active DB Connections",
            "refId": "C"
          }
        ],
        "yAxes": [
          {
            "label": "Mixed Metrics",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 30}
      }
    ]
  }
}
EOF

# Dashboard 4: Multi-Usecase Comparison
echo "🌐 Criando Dashboard Multi-Usecase..."
cat > observabilidade/grafana/dashboards/multi-usecase.json << 'EOF'
{
  "dashboard": {
    "id": null,
    "title": "Multi-Usecase Comparison - Comparação entre Usecases",
    "tags": ["comparison", "multi-usecase", "overview"],
    "timezone": "browser",
    "refresh": "5s",
    "time": {
      "from": "now-1h",
      "to": "now"
    },
    "panels": [
      {
        "id": 1,
        "title": "RPS por Usecase",
        "type": "graph",
        "targets": [
          {
            "expr": "sum by (usecase) (rate(business_operations_total[5m]))",
            "legendFormat": "{{usecase}}",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Requests/sec",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "P95 Latency por Usecase",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, sum by (usecase) (rate(handler_duration_seconds_bucket[5m]))) * 1000",
            "legendFormat": "{{usecase}}",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Latency (ms)",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
      },
      {
        "id": 3,
        "title": "Taxa de Erro por Usecase",
        "type": "graph",
        "targets": [
          {
            "expr": "sum by (usecase) (rate(business_operations_total{success=\"false\"}[5m])) / sum by (usecase) (rate(business_operations_total[5m])) * 100",
            "legendFormat": "{{usecase}}",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Error Rate (%)",
            "min": 0,
            "max": 100
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 8}
      },
      {
        "id": 4,
        "title": "Timeouts por Usecase",
        "type": "graph",
        "targets": [
          {
            "expr": "sum by (usecase) (rate(timeout_operations_total[5m]))",
            "legendFormat": "{{usecase}}",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Timeouts/sec",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 8}
      },
      {
        "id": 5,
        "title": "Ranking de Usecases por Volume",
        "type": "table",
        "targets": [
          {
            "expr": "topk(10, sum by (usecase) (rate(business_operations_total[5m])))",
            "legendFormat": "{{usecase}}",
            "refId": "A",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 8, "x": 0, "y": 16}
      },
      {
        "id": 6,
        "title": "Ranking de Usecases por Erros",
        "type": "table",
        "targets": [
          {
            "expr": "topk(10, sum by (usecase) (rate(business_operations_total{success=\"false\"}[5m])))",
            "legendFormat": "{{usecase}}",
            "refId": "A",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 8, "x": 8, "y": 16}
      },
      {
        "id": 7,
        "title": "Ranking de Usecases por Latência",
        "type": "table",
        "targets": [
          {
            "expr": "topk(10, histogram_quantile(0.95, sum by (usecase) (rate(handler_duration_seconds_bucket[5m]))))",
            "legendFormat": "{{usecase}}",
            "refId": "A",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 8, "x": 16, "y": 16}
      },
      {
        "id": 8,
        "title": "Resource Usage por Usecase",
        "type": "graph",
        "targets": [
          {
            "expr": "sum by (usecase) (db_connections_active)",
            "legendFormat": "DB Connections - {{usecase}}",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Active Connections",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 24}
      }
    ]
  }
}
EOF

# Dashboard 5: Business Metrics
echo "💼 Criando Dashboard de Business Metrics..."
cat > observabilidade/grafana/dashboards/business-metrics.json << 'EOF'
{
  "dashboard": {
    "id": null,
    "title": "Business Metrics - Métricas de Negócio",
    "tags": ["business", "kpi", "metrics"],
    "timezone": "browser",
    "refresh": "5s",
    "time": {
      "from": "now-24h",
      "to": "now"
    },
    "panels": [
      {
        "id": 1,
        "title": "Operações por Hora - Últimas 24h",
        "type": "graph",
        "targets": [
          {
            "expr": "sum(increase(business_operations_total[1h]))",
            "legendFormat": "Total Operations/hour",
            "refId": "A"
          },
          {
            "expr": "sum(increase(business_operations_total{success=\"true\"}[1h]))",
            "legendFormat": "Successful Operations/hour",
            "refId": "B"
          }
        ],
        "yAxes": [
          {
            "label": "Operations/hour",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "SLA Compliance - Success Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "sum(rate(business_operations_total{success=\"true\"}[24h])) / sum(rate(business_operations_total[24h])) * 100",
            "legendFormat": "24h Success Rate",
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
                {"color": "red", "value": null},
                {"color": "yellow", "value": 95},
                {"color": "green", "value": 99}
              ]
            },
            "unit": "percent"
          }
        },
        "gridPos": {"h": 6, "w": 8, "x": 0, "y": 8}
      },
      {
        "id": 3,
        "title": "Response Time SLA",
        "type": "stat",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, sum(rate(handler_duration_seconds_bucket[24h]))) * 1000",
            "legendFormat": "24h P95 Latency",
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
                {"color": "yellow", "value": 500},
                {"color": "red", "value": 1000}
              ]
            },
            "unit": "ms"
          }
        },
        "gridPos": {"h": 6, "w": 8, "x": 8, "y": 8}
      },
      {
        "id": 4,
        "title": "Availability %",
        "type": "stat",
        "targets": [
          {
            "expr": "(1 - (sum(rate(timeout_operations_total[24h])) + sum(rate(business_operations_total{success=\"false\"}[24h]))) / sum(rate(business_operations_total[24h]))) * 100",
            "legendFormat": "24h Availability",
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
                {"color": "red", "value": null},
                {"color": "yellow", "value": 99},
                {"color": "green", "value": 99.9}
              ]
            },
            "unit": "percent"
          }
        },
        "gridPos": {"h": 6, "w": 8, "x": 16, "y": 8}
      },
      {
        "id": 5,
        "title": "Top 10 Operações Mais Usadas",
        "type": "table",
        "targets": [
          {
            "expr": "topk(10, sum by (operation, usecase) (increase(business_operations_total[24h])))",
            "legendFormat": "{{usecase}} - {{operation}}",
            "refId": "A",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 14}
      },
      {
        "id": 6,
        "title": "Operações Críticas - Health Check",
        "type": "table",
        "targets": [
          {
            "expr": "sum by (operation, usecase) (rate(business_operations_total{success=\"false\"}[1h])) > 0",
            "legendFormat": "{{usecase}} - {{operation}}",
            "refId": "A",
            "format": "table"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 14}
      },
      {
        "id": 7,
        "title": "Throughput Trend - 7 dias",
        "type": "graph",
        "targets": [
          {
            "expr": "sum(increase(business_operations_total[1d]))",
            "legendFormat": "Operations/day",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Operations/day",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 22}
      },
      {
        "id": 8,
        "title": "Peak Hours Analysis",
        "type": "heatmap",
        "targets": [
          {
            "expr": "sum by (hour) (increase(business_operations_total[1h]))",
            "legendFormat": "{{hour}}h",
            "refId": "A"
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 30}
      }
    ]
  }
}
EOF

# Dashboard 6: System Health
echo "🖥️ Criando Dashboard de System Health..."
cat > observabilidade/grafana/dashboards/system-health.json << 'EOF'
{
  "dashboard": {
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
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "memory_usage_bytes{type=\"heap\"}",
            "legendFormat": "Heap Memory",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Bytes",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "Goroutines Active",
        "type": "graph",
        "targets": [
          {
            "expr": "goroutines_active{type=\"heap\"}",
            "legendFormat": "Active Goroutines",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Count",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
      },
      {
        "id": 3,
        "title": "Garbage Collection Duration",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(gc_duration_seconds_sum[5m]) / rate(gc_duration_seconds_count[5m]) * 1000",
            "legendFormat": "Average GC Duration",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Duration (ms)",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 8}
      },
      {
        "id": 4,
        "title": "Database Connections",
        "type": "graph",
        "targets": [
          {
            "expr": "db_connections_active",
            "legendFormat": "{{usecase}} - Active Connections",
            "refId": "A"
          }
        ],
        "yAxes": [
          {
            "label": "Connections",
            "min": 0
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 8}
      },
      {
        "id": 5,
        "title": "HTTP Server Status",
        "type": "stat",
        "targets": [
          {
            "expr": "up{job=\"novo-exemplo-palm-pay\"}",
            "legendFormat": "Service Up",
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
        "gridPos": {"h": 6, "w": 24, "x": 0, "y": 16}
      }
    ]
  }
}
EOF

echo ""
echo "🔄 Reiniciando Grafana para carregar novos dashboards..."

# Reiniciar apenas o Grafana para carregar os novos dashboards
docker-compose -f docker-compose.observability.yml restart grafana

echo ""
echo "✅ Dashboards completos criados com sucesso!"
echo ""
echo "📊 Dashboards adicionados:"
echo "   ✅ Error Analysis - Análise de Erros"
echo "   ✅ Performance Analysis - Análise de Performance"
echo "   ✅ Troubleshooting - Resolução de Problemas"
echo "   ✅ Multi-Usecase Comparison - Comparação entre Usecases"
echo "   ✅ Business Metrics - Métricas de Negócio"
echo "   ✅ System Health - Saúde do Sistema"
echo ""
echo "🌐 Acesse o Grafana: http://localhost:3000"
echo "🔑 Login: admin / Senha: admin123"
echo ""
echo "📈 Aguarde 1-2 minutos para o Grafana carregar os novos dashboards!"
echo "💡 Dica: Gere algumas requisições na API para ver dados nos dashboards"
echo ""
echo "🚀 Agora você tem observabilidade completa com todos os dashboards do README!"