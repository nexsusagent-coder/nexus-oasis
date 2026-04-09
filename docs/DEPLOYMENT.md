# 🚀 Deployment Guide

This guide covers various deployment options for SENTIENT AI OS.

## Table of Contents

- [Docker Deployment](#docker-deployment)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Cloud Deployment](#cloud-deployment)
- [Edge Deployment](#edge-deployment)

---

## Docker Deployment

### Quick Start

```bash
# Clone repository
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f sentient
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection URL | Required |
| `REDIS_URL` | Redis connection URL | Required |
| `OPENAI_API_KEY` | OpenAI API key | Required |
| `ANTHROPIC_API_KEY` | Anthropic API key | Optional |
| `RUST_LOG` | Log level | `info` |

### Production Compose

```bash
# Production deployment
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

---

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (v1.28+)
- kubectl configured
- Helm v3 (optional)

### Using kubectl

```bash
# Create namespace
kubectl create namespace sentient

# Create secrets
kubectl create secret generic sentient-secrets \
  --from-literal=database-url='postgres://...' \
  --from-literal=openai-api-key='sk-...' \
  -n sentient

# Deploy
kubectl apply -f k8s/ -n sentient

# Check status
kubectl get pods -n sentient
```

### Using Helm

```bash
# Add repo
helm repo add sentient https://charts.sentient.ai

# Install
helm install sentient sentient/sentient \
  --namespace sentient \
  --create-namespace \
  --set secrets.openaiApiKey=sk-... \
  --set postgresql.enabled=true \
  --set redis.enabled=true
```

### Kubernetes Operator

```bash
# Install operator
kubectl apply -f https://github.com/nexsusagent-coder/SENTIENT_CORE/releases/latest/download/operator.yaml

# Create Agent resource
cat <<EOF | kubectl apply -f -
apiVersion: sentient.ai/v1
kind: Agent
metadata:
  name: my-agent
spec:
  replicas: 3
  model: gpt-4
  channels:
    - telegram
    - discord
  resources:
    limits:
      memory: "2Gi"
      cpu: "2000m"
    requests:
      memory: "1Gi"
      cpu: "1000m"
EOF
```

### Auto-Scaling

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: sentient-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: sentient-gateway
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

---

## Cloud Deployment

### AWS

#### ECS Fargate

```bash
# Build and push to ECR
aws ecr get-login-password | docker login --username AWS --password-stdin $ACCOUNT.dkr.ecr.$REGION.amazonaws.com
docker build -t sentient .
docker tag sentient:latest $ACCOUNT.dkr.ecr.$REGION.amazonaws.com/sentient:latest
docker push $ACCOUNT.dkr.ecr.$REGION.amazonaws.com/sentient:latest

# Deploy to ECS
aws ecs create-cluster --cluster-name sentient
# ... additional ECS configuration
```

#### EKS

```bash
# Create cluster
eksctl create cluster --name sentient --region us-east-1

# Deploy
kubectl apply -f k8s/
```

### Google Cloud

#### GKE

```bash
# Create cluster
gcloud container clusters create sentient \
  --num-nodes=3 \
  --zone=us-central1-a

# Get credentials
gcloud container clusters get-credentials sentient --zone us-central1-a

# Deploy
kubectl apply -f k8s/
```

#### Cloud Run

```bash
# Build and deploy
gcloud run deploy sentient \
  --source . \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated
```

### Azure

#### AKS

```bash
# Create cluster
az aks create \
  --resource-group sentient \
  --name sentient-cluster \
  --node-count 3

# Get credentials
az aks get-credentials --resource-group sentient --name sentient-cluster

# Deploy
kubectl apply -f k8s/
```

---

## Edge Deployment

### ARM64 Devices

```bash
# Build for ARM64
docker buildx build --platform linux/arm64 -t sentient:arm64 .

# Run on Raspberry Pi / Jetson
docker run -d \
  -p 8080:8080 \
  -e OPENAI_API_KEY=sk-... \
  sentient:arm64
```

### Minimal Image

For resource-constrained environments:

```bash
# Build minimal image (~50MB)
docker build -f Dockerfile.minimal -t sentient:minimal .

# Run with limited resources
docker run -d \
  --memory="512m" \
  --cpus="1" \
  -p 8080:8080 \
  sentient:minimal
```

---

## Monitoring & Observability

### Prometheus Metrics

```yaml
# Available metrics
- sentient_requests_total
- sentient_request_duration_seconds
- sentient_active_connections
- sentient_memory_usage_bytes
- sentient_agent_executions_total
- sentient_channel_messages_total
```

### Grafana Dashboard

Import the provided dashboard:
```bash
# Dashboard available at
http://localhost:3000/d/sentient
```

### Logging

```bash
# Configure log level
RUST_LOG=debug sentient gateway

# Structured logging (JSON)
RUST_LOG_FORMAT=json sentient gateway | jq
```

---

## Security Best Practices

### 1. Secrets Management

```bash
# Use Kubernetes secrets
kubectl create secret generic sentient-secrets \
  --from-literal=openai-api-key=$OPENAI_API_KEY

# Or use external secrets operator
# https://external-secrets.io/
```

### 2. Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: sentient-network-policy
spec:
  podSelector:
    matchLabels:
      app: sentient
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: sentient
```

### 3. RBAC

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: sentient-role
rules:
- apiGroups: [""]
  resources: ["pods", "secrets"]
  verbs: ["get", "list"]
```

---

## Troubleshooting

### Common Issues

**1. Database connection failed**
```bash
# Check connectivity
kubectl run pg-test --rm -it --image=postgres -- psql $DATABASE_URL
```

**2. Redis connection refused**
```bash
# Check Redis
kubectl run redis-test --rm -it --image=redis -- redis-cli -h redis ping
```

**3. Out of memory**
```bash
# Increase limits
kubectl set resources deployment/sentient \
  --limits=memory=2Gi \
  --requests=memory=1Gi
```

### Health Checks

```bash
# Check health endpoint
curl http://localhost:8080/health

# Check metrics
curl http://localhost:8080/metrics
```

---

## Performance Tuning

### Resource Recommendations

| Deployment Size | CPU | Memory | Replicas |
|----------------|-----|--------|----------|
| Small | 1 core | 1 GB | 2 |
| Medium | 2 cores | 2 GB | 3 |
| Large | 4 cores | 4 GB | 5 |

### Connection Pooling

```yaml
# config/sentient.toml
[database]
pool_size = 20
max_overflow = 10

[redis]
pool_size = 10
```

---

## Backup & Recovery

### Database Backup

```bash
# PostgreSQL backup
kubectl exec -it sentient-postgres-0 -- \
  pg_dump -U sentient sentient > backup.sql

# Restore
kubectl exec -i sentient-postgres-0 -- \
  psql -U sentient sentient < backup.sql
```

### Persistent Volumes

```bash
# Snapshot
kubectl create volume snapshot sentient-pvc-snapshot \
  --source sentient-pvc
```

---

## Next Steps

- [Configuration Guide](./CONFIGURATION.md)
- [API Documentation](./API.md)
- [Monitoring Guide](./MONITORING.md)
