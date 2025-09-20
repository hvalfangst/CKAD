# CKAD Section 2: Application Deployment - Practice Exercises

This file contains hands-on exercises to practice deployment strategies, scaling, and application lifecycle management.

## Exercise 1: Basic Deployment Operations

### Task 1.1: Create and Scale Deployment
1. Create a deployment named `web-server` with `nginx:1.20` image and 2 replicas
2. Scale it to 5 replicas
3. Update the image to `nginx:1.21`
4. Scale back to 3 replicas

**Commands:**
```bash
kubectl create deployment web-server --image=nginx:1.20 --replicas=2
kubectl scale deployment web-server --replicas=5
kubectl set image deployment/web-server nginx=nginx:1.21
kubectl scale deployment web-server --replicas=3
```

### Task 1.2: Deployment with Rolling Update Strategy
Create a deployment with the following specifications:
- Name: `rolling-app`
- Image: `nginx:1.20`
- Replicas: 6
- Rolling update strategy: maxUnavailable=2, maxSurge=2

**Verification:**
```bash
kubectl describe deployment rolling-app
kubectl get rs -l app=rolling-app
```

### Task 1.3: Monitor and Rollback
1. Create a deployment with `nginx:1.20`
2. Update to `nginx:1.21` and monitor rollout
3. Update to a non-existent image `nginx:broken`
4. Check rollout status and rollback to previous version

**Commands:**
```bash
kubectl rollout status deployment/rolling-app
kubectl rollout history deployment/rolling-app
kubectl rollout undo deployment/rolling-app
```

---

## Exercise 2: Horizontal Pod Autoscaler

### Task 2.1: CPU-based HPA
1. Create a deployment with resource requests (CPU: 100m, Memory: 128Mi)
2. Create an HPA targeting 70% CPU utilization, min 2, max 10 replicas
3. Generate load and observe scaling

**Prerequisites:**
```bash
# Ensure metrics server is running
kubectl top nodes
```

### Task 2.2: Memory-based HPA
Create an HPA based on memory utilization (80%) for a memory-intensive application.

### Task 2.3: Multi-metric HPA
Create an HPA that scales based on both CPU (60%) and memory (70%) metrics.

**Solution Template:**
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: multi-metric-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: your-app
  minReplicas: 2
  maxReplicas: 10
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

---

## Exercise 3: Blue-Green Deployment

### Task 3.1: Implement Blue-Green Deployment
1. Deploy blue version with `nginx:1.20`
2. Deploy green version with `nginx:1.21` alongside blue
3. Create a service pointing to blue
4. Test green deployment separately
5. Switch service to green
6. Remove blue deployment

**Steps:**
```bash
# 1. Deploy blue
kubectl create deployment myapp-blue --image=nginx:1.20
kubectl label deployment myapp-blue version=blue

# 2. Deploy green
kubectl create deployment myapp-green --image=nginx:1.21
kubectl label deployment myapp-green version=green

# 3. Create service pointing to blue
kubectl expose deployment myapp-blue --port=80 --name=myapp-service

# 4. Test green deployment
kubectl port-forward deployment/myapp-green 8080:80

# 5. Switch service to green
kubectl patch service myapp-service -p '{"spec":{"selector":{"app":"myapp-green"}}}'

# 6. Clean up blue
kubectl delete deployment myapp-blue
```

### Task 3.2: Blue-Green with Health Checks
Implement blue-green deployment with proper readiness and liveness probes.

---

## Exercise 4: Canary Deployment

### Task 4.1: Basic Canary Deployment
1. Deploy stable version with 9 replicas (90% traffic)
2. Deploy canary version with 1 replica (10% traffic)
3. Gradually increase canary traffic
4. Complete rollout or rollback

### Task 4.2: Canary with Monitoring
Implement canary deployment with:
- Monitoring metrics
- Automatic rollback on failure
- Progressive traffic shifting

**Implementation:**
```bash
# Deploy stable version
kubectl create deployment api-stable --image=myapi:v1.0 --replicas=9
kubectl label deployment api-stable track=stable

# Deploy canary version
kubectl create deployment api-canary --image=myapi:v2.0 --replicas=1
kubectl label deployment api-canary track=canary

# Create service selecting both
kubectl create service clusterip api-service --tcp=80:8080
kubectl label service api-service app=api

# Monitor and adjust
kubectl logs -l track=canary
kubectl scale deployment api-canary --replicas=3
kubectl scale deployment api-stable --replicas=7
```

---

## Exercise 5: Advanced Deployment Strategies

### Task 5.1: A/B Testing
Deploy two variants of an application:
- Variant A: Feature flag disabled
- Variant B: Feature flag enabled
- Equal traffic distribution

### Task 5.2: Feature Flag Deployment
Create a deployment that uses ConfigMap for feature flags and can toggle features without redeployment.

### Task 5.3: Rolling Update with Readiness Gates
Implement a deployment with custom readiness gates for external validation.

---

## Exercise 6: Resource Management

### Task 6.1: Resource Quotas
1. Create a namespace with resource quota
2. Try to deploy applications that exceed quota
3. Observe behavior and adjust resources

**Resource Quota:**
```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: test-quota
  namespace: test-namespace
spec:
  hard:
    requests.cpu: "2"
    requests.memory: 4Gi
    limits.cpu: "4"
    limits.memory: 8Gi
    pods: "10"
```

### Task 6.2: Limit Ranges
Create limit ranges that enforce default resource requests and limits.

### Task 6.3: Pod Disruption Budget
Create PDB to ensure minimum availability during updates.

---

## Exercise 7: Lifecycle Management

### Task 7.1: Graceful Shutdown
Create a deployment with:
- PreStop hook for graceful shutdown
- Appropriate termination grace period
- Proper signal handling

### Task 7.2: Startup and Shutdown Hooks
Implement lifecycle hooks for:
- Application initialization
- Cleanup procedures
- Health state validation

**Example:**
```yaml
lifecycle:
  postStart:
    exec:
      command: ["/bin/sh", "-c", "echo 'Starting up' > /tmp/startup.log"]
  preStop:
    exec:
      command: ["/bin/sh", "-c", "sleep 30"]
```

---

## Exercise 8: Troubleshooting Scenarios

### Task 8.1: Failed Deployment
Debug a deployment that fails to roll out:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: broken-deploy
spec:
  replicas: 3
  selector:
    matchLabels:
      app: broken-app
  template:
    metadata:
      labels:
        app: broken-app
    spec:
      containers:
      - name: app
        image: nonexistent:latest
        resources:
          requests:
            memory: "100Gi"  # Impossible request
```

**Troubleshooting Steps:**
1. Check deployment status
2. Examine events
3. Check resource availability
4. Fix issues

### Task 8.2: HPA Not Scaling
Debug an HPA that's not scaling:
- Check metrics server
- Verify resource requests
- Examine HPA conditions

### Task 8.3: Rollout Stuck
A deployment rollout is stuck at 2/3 replicas. Identify and fix the issue.

---

## Exercise 9: Performance Optimization

### Task 9.1: Optimize Rolling Updates
Configure rolling update strategy for:
- Zero-downtime deployment
- Fast rollout
- Resource efficiency

### Task 9.2: HPA Tuning
Fine-tune HPA for:
- Stable scaling behavior
- Quick response to load
- Cost optimization

### Task 9.3: Resource Right-sizing
1. Deploy application without resource limits
2. Monitor actual usage
3. Set appropriate requests and limits
4. Test under load

---

## Exercise 10: Real-world Scenarios

### Task 10.1: Microservices Deployment
Deploy a microservices architecture:
1. Frontend service (3 replicas)
2. API service (5 replicas, with HPA)
3. Background worker (2 replicas)
4. Database (StatefulSet, 1 replica)

### Task 10.2: Multi-environment Deployment
Set up deployments for:
- Development (1 replica, no resource limits)
- Staging (2 replicas, basic resources)
- Production (5 replicas, full resources, HPA, PDB)

### Task 10.3: Disaster Recovery
Implement a deployment strategy that can:
- Survive node failures
- Handle zone outages
- Recover from image registry issues

---

## Exam Simulation Exercises

### Scenario 1: Quick Deployment (3 minutes)
Create a deployment named `exam-app` with:
- Image: `nginx:1.21`
- 4 replicas
- CPU request: 100m, limit: 200m
- Memory request: 128Mi, limit: 256Mi
- Rolling update: maxUnavailable=1, maxSurge=1

### Scenario 2: HPA Setup (2 minutes)
Create an HPA for the above deployment:
- CPU target: 70%
- Min replicas: 2
- Max replicas: 8

### Scenario 3: Update and Rollback (3 minutes)
1. Update `exam-app` to `nginx:1.22`
2. Monitor rollout status
3. If issues occur, rollback to previous version

### Scenario 4: Blue-Green Switch (4 minutes)
1. Deploy green version alongside existing blue
2. Test green version
3. Switch service from blue to green
4. Verify traffic is flowing to green

### Scenario 5: Canary Analysis (5 minutes)
1. Deploy canary version (10% traffic)
2. Monitor logs and metrics
3. Decide to either promote or rollback
4. Execute the decision

---

## Solutions and Best Practices

### Exercise 1 Solutions

**Task 1.2 Solution:**
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
      maxUnavailable: 2
      maxSurge: 2
  selector:
    matchLabels:
      app: rolling-app
  template:
    metadata:
      labels:
        app: rolling-app
    spec:
      containers:
      - name: nginx
        image: nginx:1.20
        ports:
        - containerPort: 80
```

### Exercise 2 Solutions

**Task 2.1 Setup:**
```bash
# Create deployment with resources
kubectl create deployment cpu-app --image=nginx:1.21
kubectl patch deployment cpu-app -p '{"spec":{"template":{"spec":{"containers":[{"name":"nginx","resources":{"requests":{"cpu":"100m","memory":"128Mi"}}}]}}}}'

# Create HPA
kubectl autoscale deployment cpu-app --cpu-percent=70 --min=2 --max=10
```

### Load Testing Commands

```bash
# Generate CPU load
kubectl run -i --tty load-generator --rm --image=busybox:1.35 --restart=Never -- /bin/sh -c "while true; do wget -q --spider http://cpu-app; done"

# Monitor HPA
watch kubectl get hpa

# Monitor pods
watch kubectl get pods -l app=cpu-app
```

---

## Common Pitfalls and Tips

### Deployment Issues
1. **Image Pull Errors**: Always verify image exists and is accessible
2. **Resource Constraints**: Check node capacity and quotas
3. **Selector Mismatch**: Ensure deployment selector matches pod labels
4. **Health Check Failures**: Configure appropriate probe timeouts

### HPA Issues
1. **Missing Metrics**: Ensure metrics server is running
2. **No Resource Requests**: HPA requires resource requests
3. **Scaling Thrashing**: Use appropriate stabilization windows
4. **Multiple HPAs**: Don't create multiple HPAs for same deployment

### Rolling Update Best Practices
1. Always use readiness probes
2. Set appropriate resource requests
3. Use PodDisruptionBudgets for critical apps
4. Monitor rollout status
5. Have rollback plan ready

### Blue-Green Best Practices
1. Use separate deployments with version labels
2. Test thoroughly before switching
3. Have monitoring in place
4. Plan for quick rollback
5. Consider database migration implications

### Canary Best Practices
1. Start with small percentage
2. Monitor key metrics continuously
3. Automate promotion/rollback decisions
4. Use feature flags for quick toggles
5. Plan traffic distribution carefully

Remember: Practice these scenarios repeatedly to build confidence for the CKAD exam!