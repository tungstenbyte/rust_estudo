# Makefile – Rust + observabilidade

.PHONY: help build run test fmt lint clean deps \
	setup-obs start-monitoring stop-monitoring restart-monitoring \
	health metrics check-services logs clean-data info docker-build docker-run

DOCKER_DIR := ./docker
SCRIPTS_DIR := ./scripts
APP_PORT ?= 8000
METRICS_PORT ?= 2112

help: ## Mostra esta ajuda
	@echo "Rust - piloto-rust"
	@echo "============================="
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# --- Rust ---
deps: ## Baixa dependências (cargo fetch)
	@cargo fetch

build: ## Compila em modo release
	@cargo build --release

run: ## Roda a aplicação (usa .env)
	@cargo run

test: ## Roda os testes
	@cargo test

fmt: ## Formata o código
	@cargo fmt

lint: ## Roda clippy
	@cargo clippy -- -D warnings

clean: ## Remove target/ e artefatos
	@cargo clean
	@rm -rf bin/
	@echo "Limpeza concluída."

# --- Docker ---
docker-build: ## Build da imagem Docker
	@docker build -t piloto-rust .

docker-run: ## Sobe o container (usa .env)
	@docker run --rm --env-file .env -p $(APP_PORT):$(APP_PORT) -p $(METRICS_PORT):$(METRICS_PORT) piloto-rust

# --- Observabilidade (Prometheus, Grafana, etc.) ---
setup-obs: ## Configura observabilidade (scripts)
	@chmod +x $(SCRIPTS_DIR)/setup_observability.sh 2>/dev/null || true
	@$(SCRIPTS_DIR)/setup_observability.sh 2>/dev/null || echo "Script não encontrado."

start-monitoring: ## Sobe stack de monitoramento
	@cd $(DOCKER_DIR) && docker compose -f docker-compose.monitoring.yml up -d
	@echo "Prometheus: http://localhost:9090"
	@echo "Grafana:    http://localhost:3000 (admin/admin123)"
	@echo "AlertManager: http://localhost:9093"

stop-monitoring: ## Para stack de monitoramento
	@cd $(DOCKER_DIR) && docker compose -f docker-compose.monitoring.yml down

restart-monitoring: stop-monitoring start-monitoring ## Reinicia monitoramento

health: ## Health check da aplicação
	@curl -s http://localhost:$(APP_PORT)/health | jq '.' 2>/dev/null || curl -s http://localhost:$(APP_PORT)/health

metrics: ## Métricas (Prometheus)
	@curl -s http://localhost:$(METRICS_PORT)/metrics | head -30

check-services: ## Status dos serviços
	@echo -n "App ($(APP_PORT)):      "; curl -s -o /dev/null -w "%{http_code}\n" http://localhost:$(APP_PORT)/health || echo "DOWN"
	@echo -n "Metrics ($(METRICS_PORT)): "; curl -s -o /dev/null -w "%{http_code}\n" http://localhost:$(METRICS_PORT)/metrics || echo "DOWN"

logs: ## Logs da stack de monitoramento
	@cd $(DOCKER_DIR) && docker compose -f docker-compose.monitoring.yml logs -f

info: ## URLs e informações
	@echo "App:        http://localhost:$(APP_PORT)"
	@echo "Health:     http://localhost:$(APP_PORT)/health"
	@echo "API:        http://localhost:$(APP_PORT)/api/meuexemplo | /api/segundominio"
	@echo "Métricas:   http://localhost:$(METRICS_PORT)/metrics"
