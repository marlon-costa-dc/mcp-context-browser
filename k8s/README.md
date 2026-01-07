# 🚀 MCP Context Browser - Kubernetes Deployment

Esta documentação descreve como implantar o MCP Context Browser em um cluster Kubernetes com auto-scaling horizontal usando HPA (HorizontalPodAutoscaler).

## 📋 Pré-requisitos

- Kubernetes 1.24+
- Helm 3.x (opcional, para dependências)
- Cert-Manager (para TLS automático)
- NGINX Ingress Controller
- Prometheus Operator (para métricas e HPA customizado)
- Redis (para cache distribuído)
- PostgreSQL (para metadados)
- Milvus (para vector store)

## 🏗️ Arquitetura

```
Internet → Ingress → Service → Pods (2-10 replicas) → Dependencies
                       ↓
                   HPA (Auto-scaling)
                       ↓
                 Prometheus Metrics
```

### Componentes

- **Deployment**: Aplicação principal com health checks
- **HPA**: Auto-scaling baseado em CPU, memória e métricas customizadas
- **Service**: Load balancing interno
- **Ingress**: Exposição externa com TLS
- **ConfigMap**: Configurações da aplicação
- **Secrets**: Credenciais sensíveis
- **RBAC**: Controle de acesso
- **NetworkPolicy**: Segurança de rede
- **PodDisruptionBudget**: Alta disponibilidade

## 🚀 Deploy

### 1. Preparar Secrets

Antes do deployment, você precisa criar/popular os secrets com valores reais:

```bash
# Exemplo: Codificar Redis URL em base64
echo -n "redis://user:password@redis-service:6379/0" | base64

# Atualizar o secrets.yaml com os valores codificados
```

### 2. Deploy das Dependências

```bash
# Redis
helm repo add bitnami https://charts.bitnami.com/bitnami
helm install redis bitnami/redis -n default

# PostgreSQL
helm install postgresql bitnami/postgresql -n default

# Milvus (opcional, para vector store avançado)
helm repo add milvus https://milvus-io.github.io/milvus-helm/
helm install milvus milvus/milvus -n default

# Ollama (opcional, para embeddings locais)
helm repo add ollama https://otwld.github.io/ollama-helm/
helm install ollama ollama-ollama -n default
```

### 3. Deploy da Aplicação

```bash
# Deploy completo
./deploy.sh

# Ou aplicar manualmente
kubectl apply -f . -n default
```

### 4. Verificar Deploy

```bash
# Status dos pods
kubectl get pods -l app=mcp-context-browser

# Status do HPA
kubectl get hpa mcp-context-browser-hpa

# Logs da aplicação
kubectl logs -f deployment/mcp-context-browser

# Métricas
curl http://your-domain.com:3001/api/context/metrics
```

## ⚙️ Configuração

### Auto-scaling

O HPA está configurado para:

- **Mínimo**: 2 réplicas
- **Máximo**: 10 réplicas
- **Métricas**:
  - CPU: 70% utilização média
  - Memória: 80% utilização média
  - Requests/s: 100 requests por pod
  - Conexões ativas: 50 conexões por pod

### Resource Limits

```yaml
requests:
  cpu: 500m
  memory: 1Gi
limits:
  cpu: 2000m
  memory: 4Gi
```

### Health Checks

- **Liveness**: `/api/health` a cada 10s
- **Readiness**: `/api/health` a cada 5s
- **Startup**: `/api/health` com timeout de 6 tentativas

## 📊 Monitoramento

### Métricas Prometheus

O ServiceMonitor expõe métricas em `/api/context/metrics`:

- `mcp_http_requests_total`: Total de requests HTTP
- `mcp_http_request_duration_seconds`: Duração das requests
- `mcp_active_connections`: Conexões ativas
- `mcp_cache_hit_ratio`: Taxa de acerto do cache
- `mcp_resource_limits_*`: Limites de recursos

### Dashboards Grafana

Importe o dashboard fornecido em `docs/diagrams/grafana-dashboard.json`.

## 🔧 Troubleshooting

### Problemas Comuns

1. **Pods não iniciam**: Verificar secrets e configmaps
2. **HPA não escala**: Verificar métricas do Prometheus
3. **Timeouts**: Ajustar resource limits
4. **Cache errors**: Verificar conexão Redis

### Debug Commands

```bash
# Ver eventos
kubectl get events --sort-by=.metadata.creationTimestamp

# Descrever recursos
kubectl describe deployment mcp-context-browser
kubectl describe hpa mcp-context-browser-hpa

# Ver logs com contexto
kubectl logs -f deployment/mcp-context-browser --previous

# Port-forward para debug
kubectl port-forward svc/mcp-context-browser-service 3000:80
```

## 🔄 Updates

Para atualizar a aplicação:

```bash
# Build new image
docker build -t mcp-context-browser:v0.0.5 .

# Update deployment
kubectl set image deployment/mcp-context-browser mcp-context-browser=mcp-context-browser:v0.0.5

# Rollout
kubectl rollout status deployment/mcp-context-browser
```

## 🛡️ Segurança

- **RBAC**: ServiceAccount com permissões mínimas
- **NetworkPolicy**: Controle de tráfego de rede
- **Secrets**: Credenciais em base64
- **TLS**: Certificados automáticos via cert-manager
- **SecurityContext**: Execução como non-root

## 📈 Performance Tuning

### HPA Custom Metrics

Para métricas customizadas, adicione ao HPA:

```yaml
- type: Pods
  pods:
    metric:
      name: mcp_custom_metric
    target:
      type: AverageValue
      averageValue: "100"
```

### Resource Optimization

Ajuste os limites baseado no uso:

```bash
# Monitor resource usage
kubectl top pods -l app=mcp-context-browser

# Adjust limits
kubectl edit deployment mcp-context-browser
```

## 🤝 Suporte

Para issues, consulte:
- [GitHub Issues](https://github.com/mcp-context-browser/issues)
- [Documentation](https://docs.mcp-context-browser.com)
- [Kubernetes Best Practices](https://kubernetes.io/docs/concepts/)