# 🔧 Troubleshooting - Observabilidade

## ❗ Problemas Comuns e Soluções

### 1. **Erro no observability_test.go**

#### **Problema:** Imports não encontrados
```
package novo-exemplo-palm-pay/utils/observabilidade is not in GOROOT
```

#### **✅ Solução:**
```bash
# 1. Verificar se os arquivos estão nos locais corretos
ls -la utils/observabilidade/

# 2. Instalar dependências de teste
make install-test-deps

# 3. Verificar go.mod
go mod tidy

# 4. Teste simples
make test-obs-simple
```

---

### 2. **Estrutura de Arquivos Incorreta**

#### **Problema:** Arquivos não encontrados
```
❌ utils/observabilidade/tracker.go - ARQUIVO FALTANDO!
```

#### **✅ Solução:**
Verifique se TODOS os arquivos estão nos locais corretos:

```
projeto/
├── utils/observabilidade/
│   ├── init.go                    # ✅ Deve existir
│   ├── tracker.go                 # ✅ Deve existir  
│   ├── wrappers.go               # ✅ Deve existir
│   ├── metrics.go                # ✅ Deve existir
│   ├── layer_metrics.go          # ✅ Deve existir
│   ├── context_middleware.go     # ✅ Deve existir
│   └── middleware.go             # ✅ Deve existir
├── tests/
│   └── observability_test.go     # ✅ Deve existir
├── app/meuexemplo/
│   ├── handler/meuexemplo.go     # ✅ Deve existir
│   ├── service/meuexemplo.go     # ✅ Deve existir
│   └── repository/meuexemplo.go  # ✅ Deve existir
└── scripts/
    ├── install_test_deps.sh      # ✅ Deve existir
    └── test_observability_simple.sh # ✅ Deve existir
```

---

### 3. **Dependências Não Instaladas**

#### **Problema:** Testify não encontrado
```
package github.com/stretchr/testify/assert is not in GOROOT
```

#### **✅ Solução:**
```bash
# Instalar dependências automático
make install-test-deps

# OU manual:
go get github.com/stretchr/testify@v1.8.4
go get go.opentelemetry.io/otel@v1.21.0
go mod tidy
```

---

### 4. **Erros de Compilação**

#### **Problema:** Package conflicts
```
import cycle not allowed
```

#### **✅ Solução:**
```bash
# 1. Limpar cache do Go
go clean -modcache

# 2. Reinstalar dependências
rm go.sum
go mod tidy
go mod download

# 3. Verificar imports circulares
go list -deps ./...
```

---

### 5. **Teste Simples para Verificar**

Execute este comando para verificar se tudo está funcionando:

```bash
# Teste mínimo
make test-obs-simple
```

**Resultado esperado:**
```
✅ utils/observabilidade/init.go
✅ utils/observabilidade/tracker.go
✅ utils/observabilidade/wrappers.go
✅ Pacote observabilidade compila corretamente
✅ Testes compilam corretamente
✅ Observabilidade inicializada com sucesso!
✅ Inicialização básica funciona!
🎉 Todos os testes básicos passaram!
```

---

### 6. **Verificação Manual Passo a Passo**

#### **Passo 1: Verificar arquivos**
```bash
find . -name "*.go" | grep observabilidade
```

#### **Passo 2: Verificar go.mod**
```bash
cat go.mod | grep -E "(otel|prometheus|testify)"
```

#### **Passo 3: Tentar compilar**
```bash
go build ./utils/observabilidade/...
```

#### **Passo 4: Tentar testar**
```bash
go test -v ./tests/
```

---

### 7. **Reset Completo (Se nada funcionar)**

```bash
# 1. Backup do seu código
cp -r . ../backup_projeto

# 2. Reset completo
make clean
rm -rf vendor/ go.sum

# 3. Reinstalar tudo
make install-test-deps
make test-obs-simple

# 4. Se ainda não funcionar, verifique se todos 
#    os 24 arquivos foram copiados corretamente
```

---

### 8. **Logs de Debug**

Para ver exatamente onde está o problema:

```bash
# Compilação com verbose
go build -v ./utils/observabilidade/...

# Teste com verbose
go test -v ./tests/

# Verificar módulos
go list -m all | grep -E "(otel|prometheus|testify)"
```

---

### 9. **Comandos de Diagnóstico**

```bash
# Verificar estrutura
make info

# Verificar se Docker está funcionando  
make check-services

# Verificar dependências
go mod verify

# Verificar sintaxe
go vet ./...
```

---

### 10. **Configuração Mínima para Funcionar**

Se você quer apenas o básico funcionando:

```bash
# 1. Apenas instalar observabilidade básica
go get go.opentelemetry.io/otel@v1.21.0
go get github.com/prometheus/client_golang@v1.17.0

# 2. Testar apenas inicialização
go run -c 'import "novo-exemplo-palm-pay/utils/observabilidade"; observabilidade.InitObservability("test", "1.0")'

# 3. Se funcionar, adicionar resto das dependências
make install-test-deps
```

---

## 🆘 Se Ainda Não Funcionar

1. **Verifique sua versão do Go:**
   ```bash
   go version  # Deve ser >= 1.19
   ```

2. **Verifique se você está na raiz do projeto:**
   ```bash
   ls go.mod  # Deve existir
   ```

3. **Verifique se tem permissões:**
   ```bash
   chmod +x scripts/*.sh
   ```

4. **Execute o diagnóstico completo:**
   ```bash
   make test-obs-simple 2>&1 | tee debug.log
   ```

5. **Envie o debug.log para análise**

---

## ✅ Verificação Final

Quando tudo estiver funcionando, você deve conseguir:

```bash
✅ make install-test-deps     # Sem erros
✅ make test-obs-simple       # Todos os testes passam  
✅ go test ./tests/           # Testes compilam e executam
✅ make setup-obs             # Setup completo funciona
```

**🎉 Agora a observabilidade está funcionando perfeitamente!**