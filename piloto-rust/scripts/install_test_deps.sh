#!/bin/bash

# <---- corrigido aqui: Script para instalar dependências de teste

set -e

echo "📦 Instalando dependências de teste..."

# <---- corrigido aqui: Verificar se estamos no diretório do projeto
if [ ! -f "go.mod" ]; then
    echo "❌ go.mod não encontrado. Execute este script na raiz do projeto."
    exit 1
fi

# <---- corrigido aqui: Instalar dependências de observabilidade
echo "🔍 Instalando dependências de observabilidade..."
go get go.opentelemetry.io/otel@v1.21.0
go get go.opentelemetry.io/otel/metric@v1.21.0
go get go.opentelemetry.io/otel/sdk@v1.21.0
go get go.opentelemetry.io/otel/sdk/metric@v1.21.0
go get go.opentelemetry.io/otel/exporters/prometheus@v0.44.0
go get go.opentelemetry.io/otel/semconv/v1.17.0@v1.17.0
go get github.com/prometheus/client_golang@v1.17.0

# <---- corrigido aqui: Instalar dependências de teste
echo "🧪 Instalando dependências de teste..."
go get github.com/stretchr/testify@v1.8.4

# <---- corrigido aqui: Instalar dependências do Echo se necessário
echo "🌐 Verificando dependências do Echo..."
go get github.com/labstack/echo/v4@v4.11.4
go get github.com/labstack/gommon@v0.4.0

# <---- corrigido aqui: Limpar e baixar todas as dependências
echo "🧹 Limpando e organizando dependências..."
go mod tidy
go mod download

# <---- corrigido aqui: Verificar se as dependências estão corretas
echo "✅ Verificando instalação..."

# Tentar compilar os testes
if go test -c ./tests/ > /dev/null 2>&1; then
    echo "✅ Dependências de teste instaladas com sucesso!"
else
    echo "❌ Erro ao compilar testes. Verificando problemas..."
    go test -c ./tests/
    exit 1
fi

# Verificar se pode importar observabilidade
if go list -f '{{.Dir}}' novo-exemplo-palm-pay/utils/observabilidade > /dev/null 2>&1; then
    echo "✅ Pacote de observabilidade encontrado!"
else
    echo "❌ Pacote de observabilidade não encontrado. Verifique se os arquivos estão nos locais corretos."
    exit 1
fi

echo ""
echo "🎉 Todas as dependências instaladas com sucesso!"
echo ""
echo "📝 Próximos passos:"
echo "1. Execute: make test"
echo "2. Ou execute: go test ./tests/"
echo "3. Para testar observabilidade: make test-obs"