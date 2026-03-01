# 🚀 Quick Start - Observabilidade Palm Pay

## ⚡ Setup em 3 Comandos

```bash
# 1. Setup automático completo
make setup-obs

# 2. Executar aplicação (em outro terminal)
make run

# 3. Testar observabilidade
make test-obs
```

**Pronto! 🎉 Observabilidade completa funcionando!**

---

## 📋 Checklist de Verificação

Após o setup, verifique se tudo está funcionando:

```bash
# ✅ Verificar serviços
make check-services

# ✅ Ver métricas
make metrics

# ✅ Verificar saúde
make health
```

**Resultado esperado:** Todos os serviços devem retornar status 200.

---

## 🔗 Acessos Rápidos

| Serviço | URL | Login |
|---------|-----|-------|
| 🔗 **Aplicação** | http://localhost:8080 | - |
| 🏥 **Health Check** | http://localhost:8080/health | - |
| 📊 **Métricas** | http://localhost:2112/metrics | - |
| 📈 **Grafana** | http://localhost:3000 | admin/admin123 |
| 🔍 **Prometheus** | http://localhost:9090 | - |
| 🚨 **AlertManager** | http://localhost:9093 | - |

---

## 🧪 Testando a Observabilidade

### **Teste Básico**
```bash
# Fazer algumas requisições
curl http://localhost:8080/api/meuexemplo?limit=10&offset=0
curl http://localhost:8080/api/meuexemplo/1
```

### **Gerar Carga**
```bash
make load-test
```

### **Gerar Erros (para testar alertas)**
```bash
make generate-errors
```

### **Ver Métricas Coletadas**
```bash
# Ver métricas direto no terminal
make metrics

# Ou abrir Grafana
open http://localhost:3000
```

---

## 📊 Dashboard no Grafana

1. **Acesse:** http://localhost:3000
2. **Login:** admin / admin123
3. **Dashboard:** Já configurado automaticamente como "Palm Pay - Observabilidade Completa"

### **Principais Métricas no Dashboard:**
- 📊 **HTTP Request Rate** - Requisições por segundo
- ⏱️ **HTTP P95 Latency** - Latência percentil 95
- 🔄 **Duration by Layer** - Tempo por camada (handler/service/repository)
- 🗄️ **Database Query Rate** - Taxa de queries no banco
- ❌ **Error Rate by Type** - Erros por tipo (validação/service/database)
- 🧵 **Active Goroutines** - Goroutines ativas
- 💾 **Memory Usage** - Uso de memória
- 🐌 **Slow Operations** - Operações lentas
- ⏰ **Timeouts** - Operações que deram timeout

---

## 🔧 Comandos Úteis

### **Gerenciamento**
```bash
make start-monitoring    # Iniciar monitoramento
make stop-monitoring     # Parar monitoramento
make restart-monitoring  # Reiniciar monitoramento
```

### **Logs**
```bash
make logs               # Todos os logs
make logs-prometheus    # Logs do Prometheus
make logs-grafana       # Logs do Grafana
```

### **Desenvolvimento**
```bash
make dev               # Setup completo para dev
make test              # Executar testes
make test-coverage     # Testes com coverage
make lint              # Linter
make fmt               # Formatar código
```

### **Limpeza**
```bash
make clean             # Limpeza básica
make clean-data        # Remove TODOS os dados (cuidado!)
```

---

## ❗ Solução de Problemas

### **Problema: Serviços não sobem**
```bash
# Verificar se portas estão livres
lsof -i :8080 -i :2112 -i :3000 -i :9090 -i :9093

# Parar tudo e tentar novamente
make clean
make setup-obs
```

### **Problema: Métricas não aparecem**
```bash
# Verificar se app está exposing métricas
curl http://localhost:2112/metrics | grep http_requests_total

# Se não aparecer, verificar logs da aplicação
make logs-app
```

### **Problema: Dashboard em branco**
```bash
# Verificar se Prometheus está coletando
curl http://localhost:9090/api/v1/targets

# Aguardar alguns minutos para métricas aparecerem
# Fazer algumas requisições: make load-test
```

### **Problema: Alertas não funcionam**
```bash
# Gerar erros intencionais
make generate-errors

# Verificar AlertManager
open http://localhost:9093

# Alertas levam 2-5 minutos para disparar
```

---

## 🔥 Dicas Pro

### **1. Request ID Tracking**
Todas as requisições têm Request ID automático:
```bash
curl -H "X-Request-ID: minha-req-123" http://localhost:8080/api/meuexemplo
```

### **2. Queries Prometheus Úteis**
```promql
# Taxa de requisições
rate(http_requests_total[5m])

# Latência P95
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))

# Taxa de erro
rate(http_errors_total[5m]) / rate(http_requests_total[5m])
```

### **3. Monitorar Performance**
```bash
# Ver operações lentas
curl -s http://localhost:9090/api/v1/query?query=slow_operations_total

# Ver timeouts
curl -s http://localhost:9090/api/v1/query?query=timeout_operations_total
```

### **4. Alerts Personalizados**
Edite `./docker/alert_rules.yml` e reinicie:
```bash
make restart-monitoring
```

---

## 📚 Arquivos Importantes

| Arquivo | Descrição |
|---------|-----------|
| `./utils/observabilidade/` | 📁 Código da observabilidade |
| `./docker/prometheus.yml` | ⚙️ Config do Prometheus |
| `./docker/alert_rules.yml` | 🚨 Regras de alerta |
| `./docker/grafana/dashboards/` | 📊 Dashboards |
| `./scripts/test_observability.sh` | 🧪 Script de teste |
| `./Makefile` | 🔧 Automação de tarefas |

---

## 🎯 Próximos Passos

1. **Personalizar Alertas:** Edite `alert_rules.yml`
2. **Criar Dashboards:** Use o Grafana
3. **Integrar Slack:** Configure webhook no `alertmanager.yml`
4. **Monitoring de Produção:** Ajuste thresholds para seu ambiente
5. **Logs Centralizados:** Adicione ELK Stack ou similar

---

## 🆘 Precisa de Ajuda?

```bash
# Ver todos os comandos disponíveis
make help

# Ver informações do ambiente
make info

# Verificar status de tudo
make check-services
```

---

**🎉 Agora você tem observabilidade de nível produção com código limpo!**

**Próximo passo:** Explore o Grafana e personalize os alertas para suas necessidades.




🎉 SISTEMA COMPLETO DE OBSERVABILIDADE FINALIZADO!
📁 Todos os Arquivos Criados (24 arquivos):
🔧 Core da Observabilidade:

./utils/observabilidade/tracker.go - Sistema principal de tracking com defer
./utils/observabilidade/wrappers.go - Wrappers para handler/service/repository
./utils/observabilidade/context_middleware.go - Request ID e contexto
./utils/observabilidade/middleware.go - Middlewares HTTP
./utils/observabilidade/metrics.go - Métricas básicas
./utils/observabilidade/layer_metrics.go - Métricas por camada
./utils/observabilidade/init.go - Inicialização

💻 Código da Aplicação (Limpo com Defer):

./app/meuexemplo/handler/meuexemplo.go - Handler super limpo
./app/meuexemplo/service/meuexemplo.go - Service limpo
./app/meuexemplo/repository/meuexemplo.go - Repository limpo
./app/meuexemplo/handler/register.go - Registro de rotas
./server/app.go - App com middlewares

🐳 Infraestrutura de Monitoramento:

./docker/docker-compose.monitoring.yml - Stack completa (Prometheus + Grafana + AlertManager)
./docker/prometheus.yml - Configuração do Prometheus
./docker/alert_rules.yml - Regras de alertas
./docker/alertmanager.yml - Configuração de notificações
./docker/grafana/dashboards/palm-pay-observability.json - Dashboard completo

🧪 Testes e Scripts:

./tests/observability_test.go - Testes da observabilidade
./scripts/setup_observability.sh - Setup automático
./scripts/test_observability.sh - Teste completo

⚙️ Automação e Documentação:

./Makefile - Automação de todas as tarefas
QUICK_START.md - Guia de início rápido
README.md - Documentação completa (já mostrado anteriormente)
Dependências Go - Lista de dependências necessárias


🚀 COMO USAR (3 comandos):
1. Setup Inicial:
