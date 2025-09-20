# CKAD Section 2: Application Deployment (20%)

This section covers deployment strategies, scaling, and lifecycle management of applications in Kubernetes. You'll learn about different deployment patterns, autoscaling, and managing application updates.

## Table of Contents

1. [Deployment Strategies](#deployment-strategies)
2. [Rolling Updates and Rollbacks](#rolling-updates-and-rollbacks)
3. [Scaling Applications](#scaling-applications)
4. [Horizontal Pod Autoscaler (HPA)](#horizontal-pod-autoscaler-hpa)
5. [Blue-Green Deployments](#blue-green-deployments)
6. [Canary Deployments](#canary-deployments)
7. [Deployment Patterns](#deployment-patterns)
8. [Application Lifecycle Management](#application-lifecycle-management)
9. [Resource Quotas and Limits](#resource-quotas-and-limits)
10. [Deployment Troubleshooting](#deployment-troubleshooting)

---

## Deployment Strategies

### Overview
Kubernetes supports various deployment strategies to minimize downtime and risk during application updates.

### Rolling Update (Default)
Gradually replaces old pods with new ones, ensuring service availability.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rolling-app
spec:
  replicas: 6
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 2      # Max pods that can be unavailable
      maxSurge: 2            # Max pods that can be created above desired
  selector:
    matchLabels:
      app: rolling-app
  template:
    metadata:
      labels:
        app: rolling-app
    spec:
      containers:
      - name: app
        image: nginx:1.21
        ports:
        - containerPort: 80
        readinessProbe:
          httpGet:
            path: /
            port: 80
          initialDelaySeconds: 5
          periodSeconds: 5
```

### Recreate Strategy
Terminates all existing pods before creating new ones.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: recreate-app
spec:
  replicas: 3
  strategy:
    type: Recreate
  selector:
    matchLabels:
      app: recreate-app
  template:
    metadata:
      labels:
        app: recreate-app
    spec:
      containers:
      - name: app
        image: nginx:1.21
```

---

## Rolling Updates and Rollbacks

### Performing Rolling Updates

```bash
# Update deployment image
kubectl set image deployment/myapp app=nginx:1.22

# Update with a new YAML file
kubectl apply -f updated-deployment.yaml

# Edit deployment directly
kubectl edit deployment myapp

# Update environment variables
kubectl set env deployment/myapp DATABASE_URL=postgres://new-host:5432/db
```

### Monitoring Rollout Status

```bash
# Check rollout status
kubectl rollout status deployment/myapp

# Watch rollout in real-time
kubectl rollout status deployment/myapp --watch=true

# Get rollout history
kubectl rollout history deployment/myapp

# Get details of specific revision
kubectl rollout history deployment/myapp --revision=3
```

### Rollback Operations

```bash
# Rollback to previous version
kubectl rollout undo deployment/myapp

# Rollback to specific revision
kubectl rollout undo deployment/myapp --to-revision=2

# Pause a rollout
kubectl rollout pause deployment/myapp

# Resume a paused rollout
kubectl rollout resume deployment/myapp
```

### Deployment with Revision History

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: versioned-app
  annotations:
    deployment.kubernetes.io/revision: "3"
spec:
  revisionHistoryLimit: 10  # Keep 10 old ReplicaSets
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 1
  selector:
    matchLabels:
      app: versioned-app
  template:
    metadata:
      labels:
        app: versioned-app
      annotations:
        version: "v1.2.0"
    spec:
      containers:
      - name: app
        image: nginx:1.21
        ports:
        - containerPort: 80
```

---

## Scaling Applications

### Manual Scaling

```bash
# Scale deployment
kubectl scale deployment myapp --replicas=5

# Scale multiple deployments
kubectl scale deployment myapp frontend backend --replicas=3

# Scale based on current replicas
kubectl scale --current-replicas=3 --replicas=5 deployment/myapp
```

### Declarative Scaling

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: scalable-app
spec:
  replicas: 5  # Update this value and apply
  selector:
    matchLabels:
      app: scalable-app
  template:
    metadata:
      labels:
        app: scalable-app
    spec:
      containers:
      - name: app
        image: nginx:1.21
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
          limits:
            cpu: 200m
            memory: 256Mi
```

---

## Horizontal Pod Autoscaler (HPA)

### Prerequisites
- Metrics Server must be installed
- Pods must have resource requests defined

### CPU-based HPA

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: cpu-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: web-app
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Pods
        value: 2
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
      - type: Pods
        value: 4
        periodSeconds: 60
```

### Memory-based HPA

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: memory-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: memory-app
  minReplicas: 1
  maxReplicas: 5
  metrics:
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Multiple Metrics HPA

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: multi-metric-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: multi-app
  minReplicas: 2
  maxReplicas: 15
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 60
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 70
```

### HPA Commands

```bash
# Create HPA
kubectl autoscale deployment web-app --cpu-percent=70 --min=2 --max=10

# Get HPA status
kubectl get hpa

# Describe HPA
kubectl describe hpa web-app

# Delete HPA
kubectl delete hpa web-app

# Check HPA events
kubectl get events --field-selector involvedObject.name=web-app-hpa
```

---

## Blue-Green Deployments

### Blue-Green Strategy Overview
Deploy new version alongside current version, then switch traffic.

### Blue Deployment (Current)

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-blue
  labels:
    app: myapp
    version: blue
spec:
  replicas: 3
  selector:
    matchLabels:
      app: myapp
      version: blue
  template:
    metadata:
      labels:
        app: myapp
        version: blue
    spec:
      containers:
      - name: app
        image: myapp:v1.0
        ports:
        - containerPort: 8080
```

### Green Deployment (New)

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-green
  labels:
    app: myapp
    version: green
spec:
  replicas: 3
  selector:
    matchLabels:
      app: myapp
      version: green
  template:
    metadata:
      labels:
        app: myapp
        version: green
    spec:
      containers:
      - name: app
        image: myapp:v2.0
        ports:
        - containerPort: 8080
```

### Service for Blue-Green

```yaml
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
spec:
  selector:
    app: myapp
    version: blue  # Switch to 'green' to cutover
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

### Blue-Green Deployment Process

```bash
# 1. Deploy green version
kubectl apply -f myapp-green-deployment.yaml

# 2. Test green deployment
kubectl port-forward deployment/myapp-green 8080:8080

# 3. Switch service to green
kubectl patch service myapp-service -p '{"spec":{"selector":{"version":"green"}}}'

# 4. Monitor and verify
kubectl get pods -l app=myapp

# 5. Remove blue deployment if successful
kubectl delete deployment myapp-blue
```

---

## Canary Deployments

### Canary Strategy Overview
Gradually roll out new version to a subset of users.

### Stable Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-stable
  labels:
    app: myapp
    track: stable
spec:
  replicas: 9  # 90% of traffic
  selector:
    matchLabels:
      app: myapp
      track: stable
  template:
    metadata:
      labels:
        app: myapp
        track: stable
    spec:
      containers:
      - name: app
        image: myapp:v1.0
        ports:
        - containerPort: 8080
```

### Canary Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-canary
  labels:
    app: myapp
    track: canary
spec:
  replicas: 1  # 10% of traffic
  selector:
    matchLabels:
      app: myapp
      track: canary
  template:
    metadata:
      labels:
        app: myapp
        track: canary
    spec:
      containers:
      - name: app
        image: myapp:v2.0
        ports:
        - containerPort: 8080
```

### Service for Canary

```yaml
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
spec:
  selector:
    app: myapp  # Selects both stable and canary
  ports:
  - port: 80
    targetPort: 8080
```

### Canary Deployment Process

```bash
# 1. Deploy canary with 10% traffic
kubectl apply -f myapp-canary.yaml

# 2. Monitor metrics and logs
kubectl logs -l track=canary
kubectl top pods -l track=canary

# 3. Gradually increase canary traffic
kubectl scale deployment myapp-canary --replicas=3
kubectl scale deployment myapp-stable --replicas=7

# 4. Complete rollout or rollback
# Success: Scale canary to 100%, remove stable
kubectl scale deployment myapp-canary --replicas=10
kubectl delete deployment myapp-stable

# Or rollback: Remove canary
kubectl delete deployment myapp-canary
```

---

## Deployment Patterns

### A/B Testing Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-variant-a
spec:
  replicas: 5
  selector:
    matchLabels:
      app: myapp
      variant: a
  template:
    metadata:
      labels:
        app: myapp
        variant: a
    spec:
      containers:
      - name: app
        image: myapp:variant-a
        env:
        - name: FEATURE_FLAG
          value: "false"
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-variant-b
spec:
  replicas: 5
  selector:
    matchLabels:
      app: myapp
      variant: b
  template:
    metadata:
      labels:
        app: myapp
        variant: b
    spec:
      containers:
      - name: app
        image: myapp:variant-b
        env:
        - name: FEATURE_FLAG
          value: "true"
```

### Feature Flag Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: feature-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: feature-app
  template:
    metadata:
      labels:
        app: feature-app
    spec:
      containers:
      - name: app
        image: myapp:latest
        env:
        - name: NEW_FEATURE_ENABLED
          valueFrom:
            configMapKeyRef:
              name: feature-flags
              key: new_feature
        - name: BETA_FEATURES
          valueFrom:
            configMapKeyRef:
              name: feature-flags
              key: beta_features
```

---

## Application Lifecycle Management

### Pod Disruption Budget

```yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: myapp-pdb
spec:
  minAvailable: 2  # Or use maxUnavailable: 1
  selector:
    matchLabels:
      app: myapp
```

### Graceful Shutdown

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: graceful-app
spec:
  containers:
  - name: app
    image: myapp:latest
    lifecycle:
      preStop:
        exec:
          command: ["/bin/sh", "-c", "sleep 15"]  # Grace period
  terminationGracePeriodSeconds: 30
```

### Startup and Shutdown Hooks

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: lifecycle-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: lifecycle-app
  template:
    metadata:
      labels:
        app: lifecycle-app
    spec:
      containers:
      - name: app
        image: nginx:1.21
        lifecycle:
          postStart:
            exec:
              command: ["/bin/sh", "-c", "echo 'Container started' >> /var/log/startup.log"]
          preStop:
            exec:
              command: ["/bin/sh", "-c", "nginx -s quit; while killall -0 nginx; do sleep 1; done"]
        volumeMounts:
        - name: log-volume
          mountPath: /var/log
      volumes:
      - name: log-volume
        emptyDir: {}
```

---

## Resource Quotas and Limits

### Namespace Resource Quota

```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: deployment-quota
  namespace: production
spec:
  hard:
    requests.cpu: "4"
    requests.memory: 8Gi
    limits.cpu: "8"
    limits.memory: 16Gi
    pods: "10"
    persistentvolumeclaims: "4"
    services: "5"
    secrets: "10"
    configmaps: "10"
```

### Limit Range

```yaml
apiVersion: v1
kind: LimitRange
metadata:
  name: deployment-limits
  namespace: production
spec:
  limits:
  - default:
      cpu: "200m"
      memory: "256Mi"
    defaultRequest:
      cpu: "100m"
      memory: "128Mi"
    type: Container
  - max:
      cpu: "1"
      memory: "1Gi"
    min:
      cpu: "50m"
      memory: "64Mi"
    type: Container
```

---

## Deployment Troubleshooting

### Common Issues and Solutions

#### ImagePullBackOff

```bash
# Check image name and tag
kubectl describe pod <pod-name>

# Verify image exists
docker pull <image-name>

# Check image pull secrets
kubectl get secrets
kubectl describe secret <image-pull-secret>
```

#### Insufficient Resources

```bash
# Check node resources
kubectl top nodes
kubectl describe nodes

# Check resource quotas
kubectl describe quota -n <namespace>

# Check pod resource requirements
kubectl describe pod <pod-name>
```

#### Failed Health Checks

```bash
# Check probe configuration
kubectl describe pod <pod-name>

# Test health endpoints manually
kubectl exec -it <pod-name> -- curl localhost:8080/health

# Check application logs
kubectl logs <pod-name> --previous
```

### Debugging Commands

```bash
# Check deployment status
kubectl get deployments
kubectl describe deployment <deployment-name>

# Check replica sets
kubectl get rs
kubectl describe rs <replicaset-name>

# Check events
kubectl get events --sort-by=.metadata.creationTimestamp

# Check pod status
kubectl get pods -o wide
kubectl describe pod <pod-name>

# Check resource usage
kubectl top pods
kubectl top nodes

# Debug networking
kubectl exec -it <pod-name> -- nslookup <service-name>
kubectl exec -it <pod-name> -- curl <service-name>:<port>
```

### Performance Monitoring

```bash
# Monitor deployment rollout
kubectl rollout status deployment/<deployment-name> --watch

# Check HPA metrics
kubectl get hpa
kubectl describe hpa <hpa-name>

# Monitor resource usage
watch kubectl top pods

# Check service endpoints
kubectl get endpoints
kubectl describe endpoints <service-name>
```

---

## Exam Tips and Common Scenarios

### Quick Deployment Operations

```bash
# Create deployment with specific image and replicas
kubectl create deployment myapp --image=nginx:1.21 --replicas=3

# Update deployment image
kubectl set image deployment/myapp nginx=nginx:1.22

# Scale deployment
kubectl scale deployment myapp --replicas=5

# Create HPA
kubectl autoscale deployment myapp --cpu-percent=70 --min=2 --max=10

# Rollback deployment
kubectl rollout undo deployment/myapp
```

### YAML Generation

```bash
# Generate deployment YAML
kubectl create deployment myapp --image=nginx:1.21 --dry-run=client -o yaml > deployment.yaml

# Generate HPA YAML
kubectl autoscale deployment myapp --cpu-percent=70 --min=2 --max=10 --dry-run=client -o yaml > hpa.yaml
```

### Monitoring and Verification

```bash
# Watch deployment rollout
kubectl rollout status deployment/myapp --watch

# Check all related resources
kubectl get all -l app=myapp

# Verify scaling behavior
kubectl get pods -l app=myapp --watch
```

This comprehensive guide covers all essential concepts for CKAD Section 2: Application Deployment. Practice these deployment strategies and scaling techniques to master application lifecycle management in Kubernetes.