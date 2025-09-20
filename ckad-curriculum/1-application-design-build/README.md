# CKAD Section 1: Application Design and Build (20%)

This section covers the fundamental concepts of designing and building applications that run on Kubernetes. You'll learn about core workload resources, container management, and application architecture patterns.

## Table of Contents

1. [Pods](#pods)
2. [Deployments](#deployments)
3. [ReplicaSets](#replicasets)
4. [Jobs and CronJobs](#jobs-and-cronjobs)
5. [DaemonSets](#daemonsets)
6. [StatefulSets](#statefulsets)
7. [Multi-Container Pods](#multi-container-pods)
8. [Init Containers](#init-containers)
9. [Container Images and Build](#container-images-and-build)
10. [Application Architecture Patterns](#application-architecture-patterns)

---

## Pods

### Overview
Pods are the smallest deployable units in Kubernetes. They represent a group of one or more containers with shared storage/network and a specification for how to run the containers.

### Key Concepts
- **Ephemeral**: Pods are temporary and replaceable
- **Shared Context**: Containers in a pod share network and storage
- **Single Node**: A pod always runs on a single node
- **Atomic Unit**: Pod containers are scheduled together

### Pod Lifecycle
1. **Pending**: Pod accepted but not yet scheduled
2. **Running**: Pod bound to node, at least one container running
3. **Succeeded**: All containers terminated successfully
4. **Failed**: All containers terminated, at least one failed
5. **Unknown**: Pod state cannot be determined

### Basic Pod Specification

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx-pod
  labels:
    app: nginx
spec:
  containers:
  - name: nginx
    image: nginx:1.21
    ports:
    - containerPort: 80
    resources:
      requests:
        memory: "64Mi"
        cpu: "250m"
      limits:
        memory: "128Mi"
        cpu: "500m"
```

### Common kubectl Commands for Pods

```bash
# Create a pod
kubectl run nginx --image=nginx:1.21

# Create pod from YAML
kubectl apply -f pod.yaml

# Get pods
kubectl get pods
kubectl get pods -o wide

# Describe pod
kubectl describe pod nginx-pod

# Get pod logs
kubectl logs nginx-pod

# Execute command in pod
kubectl exec -it nginx-pod -- /bin/bash

# Delete pod
kubectl delete pod nginx-pod
```

---

## Deployments

### Overview
Deployments provide declarative updates for Pods and ReplicaSets. They manage the rollout of new versions, scaling, and rollback capabilities.

### Key Features
- **Declarative Updates**: Define desired state
- **Rolling Updates**: Gradual replacement of old pods
- **Rollback**: Return to previous versions
- **Scaling**: Increase/decrease replica count
- **Self-Healing**: Replace failed pods automatically

### Deployment Specification

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
  labels:
    app: nginx
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.21
        ports:
        - containerPort: 80
        resources:
          requests:
            memory: "64Mi"
            cpu: "250m"
          limits:
            memory: "128Mi"
            cpu: "500m"
```

### Rolling Update Strategy

```yaml
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 1
```

### Common kubectl Commands for Deployments

```bash
# Create deployment
kubectl create deployment nginx --image=nginx:1.21

# Apply deployment from file
kubectl apply -f deployment.yaml

# Get deployments
kubectl get deployments

# Scale deployment
kubectl scale deployment nginx-deployment --replicas=5

# Update deployment image
kubectl set image deployment/nginx-deployment nginx=nginx:1.22

# Rollout status
kubectl rollout status deployment/nginx-deployment

# Rollout history
kubectl rollout history deployment/nginx-deployment

# Rollback deployment
kubectl rollout undo deployment/nginx-deployment

# Rollback to specific revision
kubectl rollout undo deployment/nginx-deployment --to-revision=2
```

---

## ReplicaSets

### Overview
ReplicaSets ensure that a specified number of pod replicas are running at any given time. Usually managed by Deployments.

### Key Concepts
- **Desired State**: Maintains specified number of replicas
- **Selector**: Identifies pods to manage
- **Pod Template**: Template for creating new pods

### ReplicaSet Specification

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: nginx-replicaset
  labels:
    app: nginx
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.21
        ports:
        - containerPort: 80
```

### Common kubectl Commands

```bash
# Get ReplicaSets
kubectl get replicasets
kubectl get rs

# Describe ReplicaSet
kubectl describe rs nginx-replicaset

# Scale ReplicaSet
kubectl scale rs nginx-replicaset --replicas=5
```

---

## Jobs and CronJobs

### Jobs

Jobs create one or more pods and ensure they successfully terminate. Used for batch processing and one-time tasks.

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: pi-calculation
spec:
  template:
    spec:
      containers:
      - name: pi
        image: perl:5.34.0
        command: ["perl", "-Mbignum=bpi", "-wle", "print bpi(2000)"]
      restartPolicy: Never
  backoffLimit: 4
```

### CronJobs

CronJobs create Jobs on a time-based schedule, similar to cron in Unix systems.

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: backup-job
spec:
  schedule: "0 2 * * *"  # Every day at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: backup:latest
            command: ["/bin/sh", "-c", "backup-script.sh"]
          restartPolicy: OnFailure
```

### Common kubectl Commands

```bash
# Create Job
kubectl create job pi --image=perl:5.34.0 -- perl -Mbignum=bpi -wle 'print bpi(2000)'

# Get Jobs
kubectl get jobs

# Get CronJobs
kubectl get cronjobs

# Suspend CronJob
kubectl patch cronjob backup-job -p '{"spec":{"suspend":true}}'
```

---

## DaemonSets

### Overview
DaemonSets ensure that all (or some) nodes run a copy of a pod. Used for system-level services like log collection, monitoring agents, or network proxies.

```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: fluentd
  labels:
    app: fluentd
spec:
  selector:
    matchLabels:
      app: fluentd
  template:
    metadata:
      labels:
        app: fluentd
    spec:
      containers:
      - name: fluentd
        image: fluentd:v1.14
        resources:
          limits:
            memory: 200Mi
          requests:
            cpu: 100m
            memory: 200Mi
        volumeMounts:
        - name: varlog
          mountPath: /var/log
      volumes:
      - name: varlog
        hostPath:
          path: /var/log
```

---

## StatefulSets

### Overview
StatefulSets manage stateful applications, providing guarantees about ordering, stable network identities, and persistent storage.

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgres
spec:
  serviceName: postgres
  replicas: 3
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:13
        ports:
        - containerPort: 5432
        env:
        - name: POSTGRES_DB
          value: mydb
        - name: POSTGRES_USER
          value: admin
        - name: POSTGRES_PASSWORD
          value: password
        volumeMounts:
        - name: postgres-storage
          mountPath: /var/lib/postgresql/data
  volumeClaimTemplates:
  - metadata:
      name: postgres-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 1Gi
```

---

## Multi-Container Pods

### Sidecar Pattern

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: web-server
spec:
  containers:
  - name: web
    image: nginx:1.21
    ports:
    - containerPort: 80
    volumeMounts:
    - name: shared-logs
      mountPath: /var/log/nginx
  - name: log-agent
    image: fluent/fluent-bit:1.8
    volumeMounts:
    - name: shared-logs
      mountPath: /var/log/nginx
  volumes:
  - name: shared-logs
    emptyDir: {}
```

### Ambassador Pattern

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: app-with-proxy
spec:
  containers:
  - name: app
    image: myapp:latest
    ports:
    - containerPort: 8080
  - name: proxy
    image: envoyproxy/envoy:v1.19
    ports:
    - containerPort: 80
```

### Adapter Pattern

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: app-with-adapter
spec:
  containers:
  - name: app
    image: myapp:latest
    volumeMounts:
    - name: data
      mountPath: /data
  - name: adapter
    image: log-adapter:latest
    volumeMounts:
    - name: data
      mountPath: /data
  volumes:
  - name: data
    emptyDir: {}
```

---

## Init Containers

### Overview
Init containers run before app containers start. They're used for setup tasks, prerequisites, or initialization.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: app-with-init
spec:
  initContainers:
  - name: setup
    image: busybox:1.35
    command: ['sh', '-c', 'echo "Setting up..." && sleep 5']
  - name: download
    image: busybox:1.35
    command: ['sh', '-c', 'wget -O /shared/data.txt http://example.com/data.txt']
    volumeMounts:
    - name: shared-data
      mountPath: /shared
  containers:
  - name: app
    image: myapp:latest
    volumeMounts:
    - name: shared-data
      mountPath: /data
  volumes:
  - name: shared-data
    emptyDir: {}
```

---

## Container Images and Build

### Dockerfile Best Practices

```dockerfile
FROM node:16-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

RUN addgroup -g 1001 -S nodejs
RUN adduser -S nextjs -u 1001

USER nextjs

EXPOSE 3000

CMD ["npm", "start"]
```

### Multi-stage Build

```dockerfile
# Build stage
FROM node:16-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

# Production stage
FROM node:16-alpine AS production
WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY package*.json ./
RUN npm ci --only=production
USER node
EXPOSE 3000
CMD ["npm", "start"]
```

---

## Application Architecture Patterns

### 12-Factor App Principles for Kubernetes

1. **Codebase**: One codebase tracked in revision control
2. **Dependencies**: Explicitly declare and isolate dependencies
3. **Config**: Store config in environment variables
4. **Backing Services**: Treat backing services as attached resources
5. **Build, Release, Run**: Strictly separate build and run stages
6. **Processes**: Execute as one or more stateless processes
7. **Port Binding**: Export services via port binding
8. **Concurrency**: Scale out via the process model
9. **Disposability**: Maximize robustness with fast startup and graceful shutdown
10. **Dev/Prod Parity**: Keep development, staging, and production as similar as possible
11. **Logs**: Treat logs as event streams
12. **Admin Processes**: Run admin/management tasks as one-off processes

### Configuration Management

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
data:
  database_url: "postgres://localhost:5432/mydb"
  log_level: "info"
  feature_flag: "true"
---
apiVersion: v1
kind: Secret
metadata:
  name: app-secrets
type: Opaque
data:
  password: cGFzc3dvcmQ=  # base64 encoded
  api_key: YWJjZGVmZ2g=   # base64 encoded
```

### Health Checks

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: app-with-health-checks
spec:
  containers:
  - name: app
    image: myapp:latest
    ports:
    - containerPort: 8080
    livenessProbe:
      httpGet:
        path: /health
        port: 8080
      initialDelaySeconds: 30
      periodSeconds: 10
    readinessProbe:
      httpGet:
        path: /ready
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 5
    startupProbe:
      httpGet:
        path: /startup
        port: 8080
      failureThreshold: 30
      periodSeconds: 10
```

---

## Exam Tips and Common Scenarios

### Quick Pod Creation
```bash
# Create pod with dry-run and output YAML
kubectl run nginx --image=nginx:1.21 --dry-run=client -o yaml > pod.yaml

# Create pod with port and labels
kubectl run nginx --image=nginx:1.21 --port=80 --labels="app=web,env=prod"

# Create pod with environment variables
kubectl run nginx --image=nginx:1.21 --env="DB_HOST=localhost" --env="DB_PORT=5432"
```

### Quick Deployment Creation
```bash
# Create deployment
kubectl create deployment nginx --image=nginx:1.21

# Create deployment with replicas
kubectl create deployment nginx --image=nginx:1.21 --replicas=3

# Generate YAML
kubectl create deployment nginx --image=nginx:1.21 --dry-run=client -o yaml > deployment.yaml
```

### Resource Management
```bash
# Set resource requests and limits
kubectl set resources deployment nginx --requests=cpu=100m,memory=128Mi --limits=cpu=200m,memory=256Mi

# Auto-scale deployment
kubectl autoscale deployment nginx --min=2 --max=10 --cpu-percent=80
```

### Troubleshooting Commands
```bash
# Check pod events
kubectl describe pod <pod-name>

# Check logs
kubectl logs <pod-name>
kubectl logs <pod-name> -c <container-name>  # Multi-container pods
kubectl logs <pod-name> --previous           # Previous container

# Execute commands in pod
kubectl exec -it <pod-name> -- /bin/bash
kubectl exec -it <pod-name> -c <container-name> -- /bin/bash  # Multi-container

# Port forwarding for testing
kubectl port-forward pod/<pod-name> 8080:80
kubectl port-forward deployment/<deployment-name> 8080:80
```

This comprehensive guide covers the essential concepts for CKAD Section 1: Application Design and Build. Practice these examples and commands to build proficiency in designing and managing Kubernetes applications.