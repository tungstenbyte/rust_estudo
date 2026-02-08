# Várias requisições para ativar as métricas
echo "🔥 Gerando tráfego para criar métricas..."

for i in {1..1000}; do
  echo "Requisição $i"
  curl --location 'localhost:8000/api/meuexemplo?offset=0&limit=10000'
  curl -s "http://localhost:8000/api/meuexemplo/1" > /dev/null
  sleep 0.3  
done
