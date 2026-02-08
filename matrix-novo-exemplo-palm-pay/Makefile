# <---- adicionado aqui: Makefile para automação de tarefas de observabilidade

.PHONY: help setup-obs start-monitoring stop-monitoring test-obs clean logs build run

# <---- adicionado aqui: Variáveis
DOCKER_DIR := ./docker
SCRIPTS_DIR := ./scripts
APP_PORT := 8080
METRICS_PORT := 2112

# <---- adicionado aqui: Help como target padrão
help: ## Mostra esta ajuda
	@echo "🔍 Palm Pay - Observabilidade"
	@echo "============================="
	@echo ""
	@echo "Comandos disponíveis:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# <---- adicionado aqui: Setup completo da observabilidade
setup-obs: ## Configura toda a observabilidade (dependências + monitoramento)
	@echo "🚀 Configurando observabilidade completa..."
	@chmod +x $(SCRIPTS_DIR)/setup_observability.sh
	@$(SCRIPTS_DIR)/setup_observability.sh

# <---- adicionado aqui: Gerenciamento do Docker
start-monitoring: ## Inicia stack de monitoramento (Prometheus + Grafana + AlertManager)
	@echo "🐳 Iniciando stack de monitoramento..."
	@cd $(DOCKER_DIR) && docker-compose -f docker-compose.monitoring.yml up -d
	@echo "✅ Stack iniciada!"
	@echo "📊 Prometheus: http://localhost:9090"
	@echo "📈 Grafana: http://localhost:3000 (admin/admin123)"
	@echo "🚨 AlertManager: http://localhost:9093"

stop-monitoring: ## Para stack de monitoramento
	@echo "🛑 Parando stack de monitoramento..."
	@cd $(DOCKER_DIR) && docker-compose -f docker-compose.monitoring.yml down
	@echo "✅ Stack parada!"

restart-monitoring: stop-monitoring start-monitoring ## Reinicia stack de monitoramento

# <---- adicionado aqui: Gerenciamento da aplicação
build: ## Compila a aplicação
	@echo "🔨 Compilando aplicação..."
	@go build -o bin/app main.go
	@echo "✅ Aplicação compilada em bin/app"

run: ## Executa a aplicação
	@echo "▶️  Iniciando aplicação..."
	@PORT=$(APP_PORT) go run main.go

deps: ## Instala dependências Go
	@echo "📦 Instalando dependências..."
	@go mod tidy
	@go mod download
	@echo "✅ Dependências instaladas!"

install-test-deps: ## Instala dependências de teste (corrige problemas)
	@echo "🔧 Instalando dependências de teste..."
	@chmod +x $(SCRIPTS_DIR)/install_test_deps.sh
	@$(SCRIPTS_DIR)/install_test_deps.sh

test-obs-simple: ## Teste simples da observabilidade (sem Docker)
	@echo "🧪 Teste simples da observabilidade..."
	@chmod +x $(SCRIPTS_DIR)/test_observability_simple.sh
	@$(SCRIPTS_DIR)/test_observability_simple.sh

# <---- adicionado aqui: Testes
test: ## Executa todos os testes
	@echo "🧪 Executando testes..."
	@go test -v ./...

test-obs: ## Testa especificamente a observabilidade
	@echo "🔍 Testando observabilidade..."
	@chmod +x $(SCRIPTS_DIR)/test_observability.sh
	@$(SCRIPTS_DIR)/test_observability.sh

test-coverage: ## Executa testes com coverage
	@echo "📊 Executando testes com coverage..."
	@go test -coverprofile=coverage.out ./...
	@go tool cover -html=coverage.out -o coverage.html
	@echo "✅ Coverage report gerado em coverage.html"

# <---- adicionado aqui: Verificações de saúde
health: ## Verifica saúde da aplicação
	@echo "🏥 Verificando saúde da aplicação..."
	@curl -s http://localhost:$(APP_PORT)/health | jq '.' || echo "❌ Aplicação não está respondendo"

metrics: ## Mostra métricas atuais
	@echo "📊 Métricas atuais:"
	@curl -s http://localhost:$(METRICS_PORT)/metrics | head -20

check-services: ## Verifica status de todos os serviços
	@echo "🔍 Verificando serviços..."
	@echo -n "App ($(APP_PORT)): "; curl -s -o /dev/null -w "%{http_code}" http://localhost:$(APP_PORT)/health || echo "DOWN"
	@echo -n "Metrics ($(METRICS_PORT)): "; curl -s -o /dev/null -w "%{http_code}" http://localhost:$(METRICS_PORT)/metrics || echo "DOWN"
	@echo -n "Prometheus (9090): "; curl -s -o /dev/null -w "%{http_code}" http://localhost:9090/-/ready || echo "DOWN"
	@echo -n "Grafana (3000): "; curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/api/health || echo "DOWN"
	@echo -n "AlertManager (9093): "; curl -s -o /dev/null -w "%{http_code}" http://localhost:9093/-/ready || echo "DOWN"

# <---- adicionado aqui: Logs e debugging
logs: ## Mostra logs da stack de monitoramento
	@echo "📋 Logs da stack de monitoramento:"
	@cd $(DOCKER_DIR) && docker-compose -f docker-compose.monitoring.yml logs -f

logs-app: ## Mostra logs apenas da aplicação (se rodando via Docker)
	@echo "📋 Logs da aplicação:"
	@docker logs -f palm-pay-app 2>/dev/null || echo "❌ Container da aplicação não encontrado"

logs-prometheus: ## Mostra logs do Prometheus
	@echo "📋 Logs do Prometheus:"
	@docker logs -f palm-pay-prometheus 2>/dev/null || echo "❌ Prometheus não está rodando"

logs-grafana: ## Mostra logs do Grafana
	@echo "📋 Logs do Grafana:"
	@docker logs -f palm-pay-grafana 2>/dev/null || echo "❌ Grafana não está rodando"

# <---- adicionado aqui: Limpeza
clean: ## Remove containers, volumes e arquivos temporários
	@echo "🧹 Limpando ambiente..."
	@cd $(DOCKER_DIR) && docker-compose -f docker-compose.monitoring.yml down -v --remove-orphans
	@docker system prune -f
	@rm -f coverage.out coverage.html
	@rm -rf bin/
	@echo "✅ Limpeza concluída!"

clean-data: ## Remove TODOS os dados persistentes (cuidado!)
	@echo "⚠️  ATENÇÃO: Isso removerá TODOS os dados do Prometheus e Grafana!"
	@read -p "Tem certeza? [y/N] " -n 1 -r; echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		docker volume rm -f $$(docker volume ls -q | grep -E "(prometheus_data|grafana_data|alertmanager_data)") 2>/dev/null || true; \
		echo "✅ Dados removidos!"; \
	else \
		echo "❌ Operação cancelada"; \
	fi

# <---- adicionado aqui: Utilitários de desenvolvimento
load-test: ## Executa teste de carga simples
	@echo "🚀 Executando teste de carga..."
	@for i in $$(seq 1 50); do \
		curl -s -H "X-Request-ID: load-test-$$i" http://localhost:$(APP_PORT)/api/meuexemplo?limit=10&offset=0 > /dev/null & \
	done; \
	wait; \
	echo "✅ Teste de carga concluído! Verifique as métricas."

generate-errors: ## Gera erros intencionais para testar alertas
	@echo "💥 Gerando erros para testar alertas..."
	@for i in $$(seq 1 10); do \
		curl -s http://localhost:$(APP_PORT)/api/meuexemplo?limit=invalid > /dev/null; \
		curl -s http://localhost:$(APP_PORT)/api/meuexemplo/999999 > /dev/null; \
	done
	@echo "✅ Erros gerados! Verifique AlertManager em alguns minutos."

dashboard-import: ## Abre URL para importar dashboard no Grafana
	@echo "📈 Para importar o dashboard:"
	@echo "1. Acesse: http://localhost:3000"
	@echo "2. Login: admin/admin123"
	@echo "3. Dashboard já está configurado automaticamente!"
	@echo "4. Ou importe manualmente: docker/grafana/dashboards/palm-pay-observability.json"

# <---- adicionado aqui: Informações úteis
info: ## Mostra informações importantes do ambiente
	@echo "ℹ️  Informações do Ambiente"
	@echo "========================="
	@echo "🔗 URLs importantes:"
	@echo "  Aplicação:     http://localhost:$(APP_PORT)"
	@echo "  Health Check:  http://localhost:$(APP_PORT)/health"
	@echo "  Métricas:      http://localhost:$(METRICS_PORT)/metrics"
	@echo "  Prometheus:    http://localhost:9090"
	@echo "  Grafana:       http://localhost:3000 (admin/admin123)"
	@echo "  AlertManager:  http://localhost:9093"
	@echo ""
	@echo "📁 Arquivos importantes:"
	@echo "  Configuração:  $(DOCKER_DIR)/"
	@echo "  Scripts:       $(SCRIPTS_DIR)/"
	@echo "  Dashboards:    $(DOCKER_DIR)/grafana/dashboards/"
	@echo ""
	@echo "🧪 Comandos úteis:"
	@echo "  make setup-obs     # Setup inicial completo"
	@echo "  make start-monitoring  # Inicia monitoramento"
	@echo "  make test-obs      # Testa observabilidade"
	@echo "  make health        # Verifica saúde"

# <---- adicionado aqui: Target para desenvolvimento rápido
dev: deps start-monitoring ## Setup rápido para desenvolvimento
	@echo "🚀 Ambiente de desenvolvimento pronto!"
	@echo "Execute 'make run' em outro terminal para iniciar a aplicação"
	@echo "Execute 'make test-obs' para testar a observabilidade"

# <---- adicionado aqui: Atualização de dependências
update-deps: ## Atualiza todas as dependências
	@echo "🔄 Atualizando dependências..."
	@go get -u ./...
	@go mod tidy
	@echo "✅ Dependências atualizadas!"

# <---- adicionado aqui: Verificação de sintaxe
lint: ## Executa linter no código
	@echo "🔍 Executando linter..."
	@golangci-lint run ./... || echo "⚠️  golangci-lint não instalado. Execute: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest"

# <---- adicionado aqui: Formatação de código
fmt: ## Formata código Go
	@echo "🎨 Formatando código..."
	@go fmt ./...
	@echo "✅ Código formatado!"

# <---- adicionado aqui: Target all-in-one para CI/CD
ci: deps lint test build ## Pipeline completo de CI (deps + lint + test + build)
	@echo "✅ Pipeline de CI concluído com sucesso!"