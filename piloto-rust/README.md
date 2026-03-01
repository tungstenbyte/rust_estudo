# piloto-rust

Projeto **piloto** em Rust: referência para todos os demais projetos (arquitetura, convenções, infra).

## Estrutura

```
src/
├── main.rs, lib.rs
├── config.rs          # .env
├── server.rs           # App, rotas, health, métricas
├── shared/             # ItemsPage, tipos comuns
├── meuexemplo/         # Domínio 1
│   ├── mod.rs, model.rs, ddl.rs, repository.rs, service.rs, handler.rs
├── segundominio/       # Domínio 2
│   └── ...
└── ...
```

- **Um domínio** = uma pasta com model, ddl, repository, service, handler.
- **Rotas**: `/api/{dominio}` (ex.: `/api/meuexemplo`, `/api/segundominio`).
- **Server** só orquestra: cria repo/service por domínio e monta as rotas.

## Como rodar

```bash
# .env com POSTGRESQL_READ_URL, POSTGRESQL_WRITE_URL (e PORT opcional)
make run
# ou
cargo run
```

- Health: `http://localhost:8000/health`
- API: `http://localhost:8000/api/meuexemplo`, `/api/segundominio`
- Métricas: `http://localhost:2112/metrics`

## Comandos úteis

| Comando | Descrição |
|---------|------------|
| `make build` | Build release |
| `make run` | Sobe a aplicação |
| `make test` | Testes |
| `make fmt` / `make lint` | Formatar / clippy |
| `make docker-build` / `make docker-run` | Docker |
| `make start-monitoring` | Prometheus + Grafana + AlertManager |
| `make help` | Lista todos os alvos |

## O que já tem (piloto)

- [x] Arquitetura por domínios (múltiplos domínios)
- [x] Shared (ItemsPage, etc.)
- [x] Config via .env (dotenvy)
- [x] Health e métricas Prometheus
- [x] Graceful shutdown
- [x] Dockerfile multi-stage Rust
- [x] Makefile para Rust + observabilidade
- [x] K8s (deployment, service, HPA)
- [x] Migrations SQL (ex.: segundominio)

## O que falta (para virar piloto completo)

- [ ] **Testes**: pelo menos um teste por domínio (handler ou service) ou smoke test.
- [ ] **CI**: pipeline (build + test + lint), ex. GitHub Actions.
- [ ] **Cargo.lock** commitado para build reproduzível.
- [ ] **README** de “como clonar e virar novo projeto” (passo a passo).
- [ ] **CORS** configurado no server (se a API for chamada pelo browser).
- [ ] **Documentar** em `docker/` e `observabilidade/`: trocar `novo-exemplo-palm-pay` por `piloto-rust` onde for nome de serviço/job (Prometheus, Alertmanager, dashboards).

## Como adicionar um novo domínio

1. Criar pasta `src/novodominio/` com: `mod.rs`, `model.rs`, `ddl.rs`, `repository.rs`, `service.rs`, `handler.rs` (copiar de `segundominio` e adaptar).
2. Em `server.rs`: instanciar repo e service do domínio e fazer `.nest("/novodominio", novodominio::routes(svc))` no router da API.
3. Criar tabela no banco (migration em `migrations/`).

## Nome do projeto

- **Pasta**: `piloto-rust`
- **Crate**: `piloto-rust` (binário: `piloto_rust`)
- **Serviço** (health, logs): `piloto-rust`
