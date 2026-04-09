# SENTIENT AI - Kubernetes Deployment Guide

Deploy and scale SENTIENT agents on Kubernetes.

---

## Overview

SENTIENT provides a Kubernetes operator for:

- **Distributed Agent Deployment**: Scale across clusters
- **Task Dispatch**: Distribute workloads to agents
- **Auto-scaling**: Scale based on load
- **Monitoring**: Prometheus metrics
- **High Availability**: Multi-replica deployments

---

## Prerequisites

- Kubernetes 1.25+
- kubectl configured
- Helm 3.0+ (optional)

---

## Quick Start

### Install Operator

```bash
# Apply CRDs
kubectl apply -f https://sentient.ai/crds.yaml

# Deploy operator
kubectl apply -f https://sentient.ai/operator.yaml
```

### Deploy First Agent

```bash
kubectl apply -f - <<EOF
apiVersion: sentient.ai/v1
kind: SentientAgent
metadata:
  name: hello-agent
spec:
  replicas: 1
  agentType: Worker
  model:
    provider: openai
    model: gpt-4o
EOF
```

---

## Custom Resource Definitions

### SentientAgent

```yaml
apiVersion: sentient.ai/v1
kind: SentientAgent
metadata:
  name: production-agent
  namespace: sentient
spec:
  # Number of replicas
  replicas: 3
  
  # Agent type: Worker, Orchestrator, Gateway, Voice
  agentType: Worker
  
  # Model configuration
  model:
    provider: openai           # openai, anthropic, google, ollama
    model: gpt-4o
    apiKeySecret: openai-key   # Kubernetes secret name
    parameters:
      temperature: 0.7
      max_tokens: 4096
  
  # Enabled channels
  channels:
    - telegram
    - discord
    - slack
  
  # Resource requirements
  resources:
    memory: 2Gi
    cpu: "1"
    gpu: "1"     # Optional: for local models
  
  # Voice support
  voiceEnabled: true
  
  # Skills to install
  skills:
    - translator
    - code-reviewer
    - weather
  
  # Environment variables
  env:
    LOG_LEVEL: info
    MAX_TOKENS: "4096"
  
  # Secret references
  secrets:
    - name: openai-key
      key: api-key
    - name: telegram-token
      key: token

status:
  currentReplicas: 3
  readyReplicas: 3
  phase: Running
  conditions:
    - type: Ready
      status: "True"
```

### SentientTask

```yaml
apiVersion: sentient.ai/v1
kind: SentientTask
metadata:
  name: process-query
  namespace: sentient
spec:
  # Task type: Chat, Voice, Channel, Skill, Custom
  taskType: Chat
  
  # Task input
  input:
    message: "What is the weather in Istanbul?"
    context:
      user: "john"
      session: "abc123"
  
  # Target specific agents (empty = any available)
  targetAgents:
    - production-agent
  
  # Priority (1-10, default 5)
  priority: 7
  
  # Timeout in seconds
  timeout: 300
  
  # Retry count
  retries: 3
  
  # Callback URL for result
  callbackUrl: https://api.example.com/callback

status:
  phase: Completed          # Pending, Queued, Running, Completed, Failed
  result:
    response: "Currently 25°C in Istanbul"
  assignedAgent: production-agent-1
  startedAt: "2024-01-15T10:30:00Z"
  completedAt: "2024-01-15T10:30:05Z"
```

---

## Deployment Examples

### High-Availability Setup

```yaml
apiVersion: sentient.ai/v1
kind: SentientAgent
metadata:
  name: ha-agent
spec:
  replicas: 5
  agentType: Worker
  model:
    provider: openai
    model: gpt-4o
    apiKeySecret: openai-key
  resources:
    memory: 4Gi
    cpu: "2"
---
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: ha-agent-pdb
spec:
  minAvailable: 3
  selector:
    matchLabels:
      app: sentient-agent
```

### Voice Agent with GPU

```yaml
apiVersion: sentient.ai/v1
kind: SentientAgent
metadata:
  name: voice-agent
spec:
  replicas: 2
  agentType: Voice
  voiceEnabled: true
  model:
    provider: ollama
    model: llama3.3
  resources:
    memory: 8Gi
    cpu: "4"
    gpu: "1"
  env:
    WHISPER_MODEL: whisper-medium
    TTS_PROVIDER: openai
```

### Multi-Channel Agent

```yaml
apiVersion: sentient.ai/v1
kind: SentientAgent
metadata:
  name: channel-agent
spec:
  replicas: 3
  agentType: Worker
  channels:
    - telegram
    - discord
    - slack
    - whatsapp
    - matrix
  model:
    provider: anthropic
    model: claude-3-5-sonnet
    apiKeySecret: anthropic-key
  secrets:
    - name: telegram-token
      key: token
    - name: discord-token
      key: token
    - name: slack-token
      key: token
    - name: whatsapp-credentials
      key: credentials
```

---

## Scaling

### Manual Scaling

```bash
# Scale up
kubectl scale sentientagent production-agent --replicas=10

# Scale down
kubectl scale sentientagent production-agent --replicas=2
```

### Auto-Scaling

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: agent-hpa
spec:
  scaleTargetRef:
    apiVersion: sentient.ai/v1
    kind: SentientAgent
    name: production-agent
  minReplicas: 2
  maxReplicas: 20
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Pods
      pods:
        metric:
          name: sentient_tasks_pending
        target:
          type: AverageValue
          averageValue: "10"
```

---

## Monitoring

### Prometheus Metrics

```bash
# Port forward metrics
kubectl port-forward svc/sentient-operator 9090:9090

# Available metrics:
# - sentient_tasks_total
# - sentient_tasks_pending
# - sentient_agents_active
# - sentient_task_duration_seconds
# - sentient_messages_by_channel_total
# - sentient_errors_total
# - sentient_memory_bytes
```

### Grafana Dashboard

```bash
# Import dashboard
kubectl apply -f https://sentient.ai/grafana-dashboard.yaml
```

### Alerts

```yaml
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: sentient-alerts
spec:
  groups:
    - name: sentient
      rules:
        - alert: AgentDown
          expr: sentient_agents_active < 1
          for: 5m
          labels:
            severity: critical
          annotations:
            summary: "No active SENTIENT agents"
            
        - alert: HighTaskLatency
          expr: histogram_quantile(0.95, sentient_task_duration_seconds) > 30
          for: 10m
          labels:
            severity: warning
          annotations:
            summary: "High task latency detected"
```

---

## Secrets Management

### Create Secrets

```bash
# OpenAI API key
kubectl create secret generic openai-key \
  --from-literal=api-key=sk-...

# Anthropic API key
kubectl create secret generic anthropic-key \
  --from-literal=api-key=sk-ant-...

# Channel tokens
kubectl create secret generic telegram-token \
  --from-literal=token=123456:ABC...

kubectl create secret generic discord-token \
  --from-literal=token=Bot xyz...
```

### External Secrets Operator

```yaml
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: sentient-secrets
spec:
  refreshInterval: 1h
  secretStoreRef:
    name: aws-secretsmanager
    kind: SecretStore
  target:
    name: sentient-api-keys
  data:
    - secretKey: openai-key
      remoteRef:
        key: sentient/openai-api-key
    - secretKey: anthropic-key
      remoteRef:
        key: sentient/anthropic-api-key
```

---

## Helm Chart

### Install

```bash
helm repo add sentient https://charts.sentient.ai
helm install sentient sentient/sentient-operator \
  --namespace sentient \
  --create-namespace
```

### values.yaml

```yaml
replicaCount: 1

image:
  repository: sentientai/operator
  tag: latest
  pullPolicy: IfNotPresent

resources:
  limits:
    cpu: 500m
    memory: 256Mi
  requests:
    cpu: 100m
    memory: 128Mi

# Default agents to deploy
agents:
  - name: default-agent
    replicas: 2
    agentType: Worker
    model:
      provider: openai
      model: gpt-4o
    channels:
      - telegram

# Secrets (will be created)
secrets:
  openaiKey: ""
  telegramToken: ""

monitoring:
  enabled: true
  serviceMonitor:
    enabled: true
```

---

## Operations

### View Logs

```bash
# Operator logs
kubectl logs -l app=sentient-operator -n sentient

# Agent logs
kubectl logs -l app=sentient-agent,agent=production-agent

# Follow logs
kubectl logs -f deployment/production-agent
```

### Debug

```bash
# Get agent status
kubectl get sentientagent production-agent -o yaml

# Describe agent
kubectl describe sentientagent production-agent

# Check events
kubectl get events --field-selector reason=AgentReconcile
```

### Upgrade

```bash
# Upgrade operator
kubectl apply -f https://sentient.ai/operator.yaml

# Rolling restart agents
kubectl rollout restart sentientagent production-agent
```

---

## Best Practices

1. **Use Resource Limits**: Always set memory/CPU limits
2. **Secrets Management**: Never hardcode API keys
3. **Health Checks**: Configure liveness/readiness probes
4. **Monitoring**: Set up alerts for critical metrics
5. **Backup**: Regular backups of configuration

---

**Scale your AI with SENTIENT on Kubernetes! ☸️**
