# Projeto Rust com Apache Pulsar

Projeto em Rust que **produz** e **consome** mensagens no Apache Pulsar, com Docker Compose (Pulsar + Pulsar Manager) e interface gráfica web.

## Estrutura do Projeto

| Item | Descrição |
|------|------------|
| `src/bin/producer_parallel.rs` | Produtor com **N producers em paralelo** (alto throughput) |
| `src/bin/consumer_workers.rs` | Consumidor com **N workers fixos** (estilo Go, alto throughput) |
| `docker-compose.yml` | Pulsar standalone + Pulsar Manager (interface web) |
| `.vscode/launch.json` | Configurações de debug para Producer Parallel e Consumer |

## Pré-requisitos

- **Rust** 1.70+
- **Docker** e **Docker Compose** (ou `docker compose` v2)
- **Cargo**

---

## Início rápido

### 1. Subir Pulsar e Pulsar Manager

```bash
docker compose up -d
```

Aguarde o Pulsar ficar saudável (~30 s). Verifique:

```bash
docker compose ps
```

### 2. Rodar o produtor

```bash
cargo run --bin producer_parallel
```

Envia 100.000 mensagens para o tópico `persistent://public/default/estudo_topic` (10 producers em paralelo).

### 3. Rodar o consumidor

Em outro terminal:

```bash
cargo run --bin consumer_workers
```

Recebe e exibe as mensagens (N workers em paralelo). Para encerrar após processar as pendentes, use `Ctrl+C` (o consumidor fica aguardando novas mensagens).

**Ordem não importa:** você pode **publicar primeiro e consumir depois** (as mensagens ficam no tópico até o consumidor rodar) ou **consumir primeiro e publicar depois** (o consumidor fica aguardando). Não é obrigatório ter o consumidor rodando quando você publica.

### O que o Pulsar cria automaticamente

Ao rodar a aplicação, o Pulsar **cria sozinho** (não precisa criar nada antes):

| Quando você roda… | O Pulsar cria… |
|-------------------|-----------------|
| **Produtor** (`cargo run --bin producer_parallel`) | O **tópico** `estudo_topic` no namespace `public/default` (e o tenant/namespace, se ainda não existirem). |
| **Consumidor** (`cargo run --bin consumer_workers`) | A **subscription** `rust-subscription-workers` no tópico `estudo_topic`. |

Ou seja: não é preciso criar tópico nem fila manualmente. Depois de rodar produtor e consumidor, no Pulsar Manager você verá em **Tenants → public → Namespaces → default → Topics** o tópico **estudo_topic** e, ao abri-lo, a subscription **rust-subscription-workers**.

### Concorrência, paralelismo e worker pool (estilo Go)

- **Concorrência:** várias tarefas fazendo progresso ao longo do tempo (podem ser entrelaçadas).
- **Paralelismo:** várias tarefas executando ao mesmo tempo (em vários núcleos). Com o runtime multi-thread do Tokio, tarefas assíncronas podem rodar em threads diferentes → temos **paralelismo** além de concorrência.

**O que temos neste projeto:**

| Binário | Concorrência | Paralelismo | Padrão |
|---------|--------------|-------------|--------|
| **producer_parallel** | Sim | Sim (Tokio) | **N producers** em paralelo (cada task com seu producer; mensagens distribuídas entre as tasks). Veja [Produtor](#produtor). |
| **consumer_workers** | Sim | Sim (Tokio) | Pool **fixo** estilo Go: N workers fixos que leem de um canal (cada mensagem vai para um worker). Veja [Consumidor](#consumidor). |

### Produtor

O binário **`producer_parallel`** é o produtor do projeto: **N producers em paralelo** (ex.: 10), cada um enviando mensagens ao mesmo tópico. As mensagens são distribuídas entre as tasks (task 0 envia ids 1, 11, 21, ...; task 1 envia 2, 12, 22, ...; etc.).

- **Concorrência e paralelismo:** Tokio multi-thread; até N mensagens sendo enviadas ao mesmo tempo.
- **Constantes:** `NUM_WORKERS` (ex.: 10) e `TOTAL_MESSAGES` (ex.: 100_000) em `producer_parallel.rs`.

Como rodar:

```bash
cargo run --bin producer_parallel
```

Para testar: em outro terminal rode `cargo run --bin consumer_workers`; as mensagens chegarão em ordem não necessariamente sequencial.

### Como ver as mensagens sendo consumidas

**1. No terminal (mais direto)**  
Ao rodar o consumidor, cada mensagem é impressa no terminal:

```bash
cargo run --bin consumer_workers
```

Se os logs não aparecerem: o consumidor usa nível `info` por padrão. Rode primeiro o produtor para enviar mensagens. Para ver em tempo real: terminal 1 = `cargo run --bin consumer_workers`, terminal 2 = `cargo run --bin producer_parallel`.

Exemplo de saída:

```
INFO consumer_workers: worker 0 mensagem #1: ID=1, Content='Mensagem número 1 (worker 0)'
...
```

Para ver mensagens em tempo real: em um terminal rode `cargo run --bin producer_parallel` e em outro `cargo run --bin consumer_workers`; as mensagens aparecem no terminal do consumidor conforme são recebidas.

**2. No Pulsar Manager (estatísticas e inspeção)**  
- Acesse **http://localhost:9527** → entre no ambiente → **Tenants** → **public** → **Namespaces** → **default** → **Topics** → **estudo_topic**.  
- No tópico você vê: **Subscriptions** (ex.: `rust-subscription-workers`), número de mensagens publicadas, atraso (backlog), etc.  
- Em algumas versões do Pulsar Manager há opção **Peek** ou **Messages** na subscription para espiar mensagens; se não aparecer, use o terminal para ver o conteúdo.

**3. Via Pulsar Admin API**  
Para inspecionar mensagens pela API REST (ex.: com `curl`), use o endpoint do Pulsar em `http://localhost:8080` (consulte a documentação do Pulsar para peek de mensagens por subscription).

### Consumidor

O binário **`consumer_workers`** é o consumidor do projeto: **N workers fixos** (estilo Go) que ficam em loop recebendo mensagens de um canal; um receptor recebe do Pulsar e um **dispatcher** distribui em **round-robin** para os N canais (um por worker). Cada worker processa e envia a mensagem de volta para o receptor fazer o ack.

- **Subscription:** `rust-subscription-workers`.
- **N workers:** constante `NUM_WORKERS` (ex.: 10) em `consumer_workers.rs`.
- **Concorrência e paralelismo:** sim (Tokio multi-thread); até N mensagens sendo processadas ao mesmo tempo.

Equivalente em Go:

```go
jobs := make(chan Message, cap)
for i := 0; i < N; i++ {
    go func() { for msg := range jobs { process(msg); ack(msg) } }()
}
for msg := range pulsar.Messages() { jobs <- msg }
```

Como rodar:

```bash
cargo run --bin consumer_workers
```

---

## Interface gráfica (Pulsar Manager)

Acesse **http://localhost:9527** para administrar o Pulsar pela web.

### Primeira vez: criar usuário admin

O Pulsar Manager exige um e-mail válido (evite `admin@localhost`). Execute **apenas uma vez**:

```bash
# Obter token CSRF
CSRF_TOKEN=$(curl -s http://localhost:7750/pulsar-manager/csrf-token)

# Criar usuário admin / senha apachepulsar (use e-mail válido, ex: admin@example.com)
curl -H "X-XSRF-TOKEN: $CSRF_TOKEN" \
     -H "Cookie: XSRF-TOKEN=$CSRF_TOKEN" \
     -H "Content-Type: application/json" \
     -X PUT http://localhost:7750/pulsar-manager/users/superuser \
     -d '{"name": "admin", "password": "apachepulsar", "description": "Admin", "email": "admin@example.com"}'
```

Resposta esperada: `{"message":"Add super user success, please login"}`.

### Login e ambiente

1. Abra **http://localhost:9527**
2. Login: **admin** | Senha: **apachepulsar**
3. Clique em **"New Environment"**
4. Preencha os campos conforme a tabela abaixo e salve.

#### Configuração do ambiente (New Environment)

Use estes valores ao criar um novo ambiente no Pulsar Manager (todos acessam o Pulsar pela rede Docker):

| Campo | Valor | Observação |
|-------|--------|------------|
| **Environment Name** | `local` (ou outro nome) | Nome que identifica o ambiente na interface |
| **Service URL** | `http://pulsar:8080` | API HTTP/Admin do Pulsar (obrigatório usar `pulsar`, não `localhost`) |
| **Bookie URL** | `http://pulsar:3181` | BookKeeper no standalone usa a porta 3181 no mesmo container |

Depois disso é possível ver tópicos, subscriptions, tenants e namespaces.

### O que fazer na console (passo a passo)

Depois de logado, para ver o cluster e os tópicos:

1. **Adicionar o ambiente (se ainda não fez)**  
   - Clique em **"Environments"** no menu (ou **"New Environment"**).  
   - **Environment Name:** `local` (qualquer nome).  
   - **Service URL:** `http://pulsar:8080`.  
   - **Bookie URL:** `http://pulsar:3181`.  
   - Clique em **"Confirm"** / **"Save"**.

2. **Entrar no ambiente**  
   - Clique no ambiente que você criou (ex.: `local`).  
   - Você entra na visão do cluster.

3. **Ver tenants**  
   - No menu lateral, acesse **Tenants**.  
   - Deve aparecer o tenant **public** (usado pelo nosso tópico).

4. **Ver namespaces e tópicos**  
   - Clique no tenant **public** → **Namespaces** → **default**.  
   - Em **Topics** você verá o tópico **estudo_topic** (o que o producer/consumer usam).  
   - Ao clicar em **estudo_topic** dá para ver subscriptions (ex.: `rust-subscription-workers`), mensagens, estatísticas, etc.

5. **Resumo**  
   - **Environments** = clusters Pulsar que você gerencia.  
   - **Tenants** → **Namespaces** → **Topics** = hierarquia dos tópicos.  
   - Em um tópico você vê producers, consumers, subscriptions e estatísticas.

### Ver o conteúdo das mensagens na console do Pulsar

**No Pulsar Manager (http://localhost:9527):**

- Você **consegue ver** na console: tópicos, subscriptions, **estatísticas** (número de mensagens publicadas, backlog, etc.) e metadados.
- Ver o **conteúdo (payload)** de cada mensagem na interface web **depende da versão** do Pulsar Manager: em algumas versões há opção **Peek** ou **Messages** no tópico ou na subscription (procure no menu do tópico **estudo_topic** ou ao abrir uma subscription).
- Se não aparecer "Peek" ou "Messages", a console não exibe o corpo das mensagens; use uma das opções abaixo.

**Formas de ver as mensagens de fato:**

1. **Terminal (mais simples)**  
   Rode o consumidor e veja as mensagens nos logs:
   ```bash
   cargo run --bin consumer_workers
   ```
   Em outro terminal: `cargo run --bin producer_parallel` para enviar; as linhas "worker N mensagem #M: ..." mostram o conteúdo.

2. **CLI dentro do container Pulsar**  
   Para espiar mensagens sem consumir (peek) pela linha de comando:
   ```bash
   docker exec -it pulsar bin/pulsar-client consume "persistent://public/default/estudo_topic" -s "peek-sub" -n 0
   ```
   (Cria uma subscription temporária e lê mensagens; ajuste o nome do container se for outro.)

3. **API REST do Pulsar (peek)**  
   O Pulsar expõe endpoints de admin para peek por ledger/entry; a documentação oficial descreve a API (por exemplo, endpoints em `http://localhost:8080/admin/v2/...`).

### Quantas mensagens tem na fila (backlog)

O **backlog** é a quantidade de mensagens ainda não consumidas em uma subscription. Formas de ver:

**1. Pulsar Manager (http://localhost:9527)**  
- Entre no ambiente → **Tenants** → **public** → **Namespaces** → **default** → **Topics** → **estudo_topic**.  
- Ao abrir o tópico ou uma **subscription** (ex.: `rust-subscription-workers`), a interface mostra estatísticas como **backlog** (mensagens na fila), mensagens publicadas, etc.

**2. API REST do Pulsar (curl)**  
Exige `jq`. Base URL do namespace: `http://localhost:8080/admin/v2/persistent/public/default`.

**Por tópico e por subscription (fila)** — tabela: tópico | subscription | backlog:

```bash
# Um tópico (ex.: estudo_topic): mostra cada subscription e a quantidade
curl -s http://localhost:8080/admin/v2/persistent/public/default/estudo_topic/stats | \
  jq -r '"TOPIC\tSUBSCRIPTION\tBACKLOG", (.subscriptions | to_entries[] | "estudo_topic\t\(.key)\t\(.value.msgBacklog)")'
```

Exemplo de saída:
```
TOPIC    SUBSCRIPTION                BACKLOG
estudo_topic rust-subscription-workers   304492
```

**Todos os tópicos do namespace** — quantidade em cada tópico e em cada subscription (fila):

```bash
# Lista todos os tópicos do namespace e, para cada um, subscription + backlog
echo "TOPIC	SUBSCRIPTION	BACKLOG"
for t in $(curl -s http://localhost:8080/admin/v2/persistent/public/default | jq -r '.[]'); do
  curl -s "http://localhost:8080/admin/v2/persistent/public/default/${t}/stats" | \
    jq -r --arg topic "$t" '.subscriptions | to_entries[] | "\($topic)\t\(.key)\t\(.value.msgBacklog)"'
done
```

**Total geral** (soma do backlog de todas as subscriptions de um tópico):

```bash
curl -s http://localhost:8080/admin/v2/persistent/public/default/estudo_topic/stats | jq '[.subscriptions[].msgBacklog] | add'
```

**Uma subscription específica** (ex.: `rust-subscription-workers`):

```bash
curl -s http://localhost:8080/admin/v2/persistent/public/default/estudo_topic/stats | jq '.subscriptions["rust-subscription-workers"].msgBacklog'
```

**JSON completo do tópico** (para inspecionar todos os campos):

```bash
curl -s http://localhost:8080/admin/v2/persistent/public/default/estudo_topic/stats | jq .
```

Se não tiver `jq`, use o último `curl` e procure por `msgBacklog` no JSON.

**Se o backlog não diminuir:** o consumidor usa subscription **Exclusive** (só **um** consumidor por subscription). Se outro processo estiver rodando `consumer_workers` (ou outro consumidor na mesma subscription), o novo não recebe mensagens. Feche todos os consumidores e rode **apenas um**. O consumidor loga a cada 10 s: *"Ainda aguardando mensagens... (recebidas até agora: N)"* — se N continua 0, confira que não há outro consumidor ativo.

**Por que o consumidor não lê desde o início?**  
No Pulsar, **Earliest/Latest só vale na criação da subscription**. Depois que a subscription já existe, o consumidor sempre continua a partir do **cursor** (última posição confirmada). Ou seja: se a subscription `rust-subscription-workers` já existia de uma execução anterior, ele não “volta ao início”; ele retoma de onde parou.

**Passo a passo confiável para ler desde o início** (faça nesta ordem):

1. **Pare o consumidor** (Ctrl+C no processo que está rodando `consumer_workers`). Se o consumidor estiver conectado, resetar cursor ou apagar subscription pode não ter efeito ou dar erro.

2. **Apague a subscription** — é o método mais confiável; a subscription é recriada com Earliest quando o consumidor subir de novo:

   ```bash
   docker exec pulsar bin/pulsar-admin topics unsubscribe --subscription rust-subscription-workers persistent://public/default/estudo_topic
   ```

3. **Suba o consumidor de novo:**

   ```bash
   cargo run --bin consumer_workers
   ```

   A subscription `rust-subscription-workers` será criada de novo com Earliest e o consumidor receberá as mensagens desde o início.

**Alternativa: resetar o cursor** (mantém a subscription; use só se não quiser apagá-la). Faça com o consumidor **parado**.

- **pulsar-admin** (tempo relativo: “X atrás”; use `d` = dias, `w` = semanas):

  ```bash
  docker exec pulsar bin/pulsar-admin topics reset-cursor --subscription rust-subscription-workers --time 10000d persistent://public/default/estudo_topic
  ```

  (10000d ≈ 27 anos atrás; na prática equivale a “desde o início” do tópico.)

- **REST** (timestamp em **milissegundos** desde 1970-01-01; 0 = início):

  ```bash
  curl -X POST "http://localhost:8080/admin/v2/persistent/public/default/estudo_topic/subscription/rust-subscription-workers/resetcursor/0"
  ```

  Se retornar erro (por exemplo 404 ou 405), use o método de **apagar a subscription** acima.

**3. CLI dentro do container Pulsar**  
Estatísticas do tópico:

```bash
docker exec pulsar bin/pulsar-admin topics stats persistent://public/default/estudo_topic
```

Estatísticas de uma subscription:

```bash
docker exec pulsar bin/pulsar-admin topics subscriptions persistent://public/default/estudo_topic
```

---

## Apagar um tópico

Para remover um tópico (ex.: o antigo `my-topic` ou o atual `estudo_topic`):

1. Apague as subscriptions do tópico (senão o delete pode falhar):
   ```bash
   docker exec pulsar bin/pulsar-admin topics unsubscribe --subscription rust-subscription-workers persistent://public/default/NOME_DO_TOPIC
   ```
   Repita para cada subscription listada em `topics subscriptions persistent://public/default/NOME_DO_TOPIC`.

2. Apague o tópico:
   ```bash
   docker exec pulsar bin/pulsar-admin topics delete persistent://public/default/NOME_DO_TOPIC
   ```

Exemplo para apagar o tópico antigo `my-topic` (troque o nome se for outro):
```bash
docker exec pulsar bin/pulsar-admin topics unsubscribe --subscription rust-subscription-workers persistent://public/default/my-topic
docker exec pulsar bin/pulsar-admin topics unsubscribe --subscription rust-subscription persistent://public/default/my-topic
docker exec pulsar bin/pulsar-admin topics delete persistent://public/default/my-topic
```

O tópico **estudo_topic** é criado automaticamente quando você roda o produtor ou o consumidor (não precisa criar à mão).

---

## Variáveis de ambiente

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `PULSAR_URL` | `pulsar://localhost:6650` | URL do broker Pulsar |
| `PULSAR_TOPIC` | `persistent://public/default/estudo_topic` | Tópico usado pelo produtor/consumidor |

Exemplo:

```bash
PULSAR_TOPIC=persistent://public/default/outro-topic cargo run --bin producer_parallel
```

---

## Portas e URLs

| Serviço | Porta | URL |
|---------|--------|-----|
| Pulsar (binário) | 6650 | `pulsar://localhost:6650` |
| Pulsar Admin API | 8080 | http://localhost:8080 |
| Pulsar Manager (web) | 9527 | http://localhost:9527 |
| Pulsar Manager (API) | 7750 | http://localhost:7750 |

---

## Compilar e executar (release)

```bash
cargo build --release
./target/release/producer_parallel
./target/release/consumer_workers
```

---

## Parar os serviços

```bash
docker compose down
```

Remover volumes (dados):

```bash
docker compose down -v
```

---

## Estrutura das mensagens (JSON)

```json
{
  "id": 1,
  "content": "Mensagem número 1",
  "timestamp": 1769703960
}
```

---

## Notas

- Consumidor usa subscription **Exclusive** (uma instância por vez).
- Mensagens são confirmadas (ACK) após o processamento.
- Para ocultar logs do crate `pulsar` no produtor: o filtro padrão já usa `pulsar=off`; para ver tudo: `RUST_LOG=debug cargo run --bin producer_parallel`.
- **Backlog por tópico e por subscription (fila)** e total: ver [Quantas mensagens tem na fila (backlog)](#quantas-mensagens-tem-na-fila-backlog) (curl com `jq`).





curl -s http://localhost:8080/admin/v2/persistent/public/default/estudo_topic/stats | \
  jq -r '"TOPIC\tSUBSCRIPTION\tBACKLOG", (.subscriptions | to_entries[] | "estudo_topic\t\(.key)\t\(.value.msgBacklog)")'
