# CKAD Section 1: Practice Exercises

This file contains hands-on exercises to practice the concepts covered in Application Design and Build.

## Exercise 1: Basic Pod Management

### Task 1.1: Create a Simple Pod
Create a pod named `web-pod` with the following specifications:
- Image: `nginx:1.21`
- Container name: `nginx`
- Port: 80
- Labels: `app=web`, `env=dev`

**Solution:**
```bash
kubectl run web-pod --image=nginx:1.21 --port=80 --labels="app=web,env=dev"
```

Or using YAML:
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: web-pod
  labels:
    app: web
    env: dev
spec:
  containers:
  - name: nginx
    image: nginx:1.21
    ports:
    - containerPort: 80
```

### Task 1.2: Pod with Resource Limits
Create a pod named `resource-pod` with:
- Image: `busybox:1.35`
- Command: `sleep 3600`
- CPU request: 100m, limit: 200m
- Memory request: 128Mi, limit: 256Mi

**Commands to verify:**
```bash
kubectl describe pod resource-pod
kubectl top pod resource-pod
```

### Task 1.3: Pod with Environment Variables
Create a pod named `env-pod` with:
- Image: `busybox:1.35`
- Environment variables:
  - `DATABASE_URL=postgres://localhost:5432/mydb`
  - `LOG_LEVEL=debug`
- Command that prints the environment variables

**Verification:**
```bash
kubectl logs env-pod
```

---

## Exercise 2: Deployment Management

### Task 2.1: Create and Scale Deployment
1. Create a deployment named `web-app` with 3 replicas using `nginx:1.21`
2. Scale it to 5 replicas
3. Scale it back to 2 replicas

**Commands:**
```bash
kubectl create deployment web-app --image=nginx:1.21 --replicas=3
kubectl scale deployment web-app --replicas=5
kubectl scale deployment web-app --replicas=2
```

### Task 2.2: Rolling Updates
1. Create a deployment with `nginx:1.20`
2. Update the image to `nginx:1.21`
3. Check the rollout status
4. View rollout history
5. Rollback to the previous version

**Commands:**
```bash
kubectl create deployment rolling-app --image=nginx:1.20
kubectl set image deployment/rolling-app nginx=nginx:1.21
kubectl rollout status deployment/rolling-app
kubectl rollout history deployment/rolling-app
kubectl rollout undo deployment/rolling-app
```

### Task 2.3: Deployment with Custom Strategy
Create a deployment with the following rolling update strategy:
- Maximum unavailable: 1
- Maximum surge: 2
- Image: `nginx:1.21`
- Replicas: 4

---

## Exercise 3: Jobs and CronJobs

### Task 3.1: Simple Job
Create a job that:
- Runs a container with `perl:5.34.0` image
- Calculates first 1000 digits of Pi
- Has a backoff limit of 3

**Verification:**
```bash
kubectl logs job/pi-job
```

### Task 3.2: Parallel Job
Create a job that:
- Runs 3 pods in parallel
- Completes when 6 pods have finished successfully
- Each pod sleeps for 10 seconds

### Task 3.3: CronJob
Create a CronJob that:
- Runs every 5 minutes
- Uses `busybox:1.35` image
- Prints the current date and time
- Keeps history of 5 successful jobs and 2 failed jobs

---

## Exercise 4: Multi-Container Pods

### Task 4.1: Sidecar Pattern
Create a pod with two containers:
1. **Main container**: `nginx:1.21` serving on port 80
2. **Sidecar container**: `busybox:1.35` that tails nginx access logs

Both containers should share a volume for logs.

### Task 4.2: Init Container
Create a pod with:
1. **Init container**: Downloads data using `busybox:1.35` and `wget`
2. **Main container**: `nginx:1.21` that serves the downloaded data

### Task 4.3: Ambassador Pattern
Create a pod with:
1. **App container**: Simple application on port 8080
2. **Ambassador container**: Proxy that forwards traffic

---

## Exercise 5: Advanced Scenarios

### Task 5.1: StatefulSet
Create a StatefulSet for a database with:
- 3 replicas
- Persistent storage (1Gi per replica)
- Proper service for stable network identities

### Task 5.2: DaemonSet
Create a DaemonSet that:
- Runs on all nodes
- Mounts the host's `/var/log` directory
- Uses a log collection image

### Task 5.3: Health Checks
Create a deployment with:
- Liveness probe checking HTTP endpoint
- Readiness probe with different endpoint
- Startup probe for slow-starting applications

---

## Exercise 6: Configuration Management

### Task 6.1: ConfigMap and Secret
1. Create a ConfigMap with application configuration
2. Create a Secret with sensitive data
3. Create a pod that uses both ConfigMap and Secret as environment variables

### Task 6.2: Volume Mounts
Create a pod that:
- Mounts a ConfigMap as a file
- Mounts a Secret as a file
- Uses both in the application

---

## Exercise 7: Troubleshooting Scenarios

### Task 7.1: Failed Pod
Create a pod that fails to start and practice troubleshooting:
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: failing-pod
spec:
  containers:
  - name: app
    image: nonexistent:latest
    command: ["/bin/nonexistent"]
```

**Troubleshooting steps:**
1. Check pod status
2. Describe the pod
3. Check events
4. Fix the issues

### Task 7.2: Resource Issues
Create a pod with impossible resource requirements and troubleshoot:
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: resource-issue
spec:
  containers:
  - name: app
    image: nginx:1.21
    resources:
      requests:
        memory: "100Gi"
        cpu: "100"
```

### Task 7.3: Image Pull Issues
Create a pod with an image that doesn't exist and practice troubleshooting.

---

## Exercise 8: Performance and Optimization

### Task 8.1: Resource Optimization
1. Create a deployment without resource limits
2. Monitor resource usage
3. Add appropriate resource requests and limits
4. Test the impact on scheduling

### Task 8.2: Horizontal Pod Autoscaler
1. Create a deployment with resource requests
2. Create an HPA for the deployment
3. Generate load and observe scaling behavior

---

## Exercise 9: Real-world Scenarios

### Task 9.1: Microservice Deployment
Deploy a simple microservice architecture:
1. Frontend service (nginx)
2. Backend API service
3. Database service (StatefulSet)
4. Background worker (Job)

### Task 9.2: Blue-Green Deployment
Implement a blue-green deployment strategy:
1. Deploy version 1 (blue)
2. Deploy version 2 (green) alongside
3. Switch traffic from blue to green
4. Remove blue deployment

### Task 9.3: Canary Deployment
Implement a canary deployment:
1. Deploy stable version with 90% traffic
2. Deploy canary version with 10% traffic
3. Monitor and gradually increase canary traffic

---

## Exam Simulation Exercises

### Scenario 1: Quick Pod Creation (2 minutes)
Create a pod named `exam-pod` with:
- Image: `redis:6-alpine`
- Port: 6379
- Labels: `app=redis`, `tier=cache`
- Resource limits: CPU 200m, Memory 256Mi

### Scenario 2: Deployment Update (3 minutes)
1. Create deployment `web-deploy` with `nginx:1.20`, 3 replicas
2. Update to `nginx:1.21`
3. If update fails, rollback to previous version

### Scenario 3: Multi-Container Setup (5 minutes)
Create a pod with:
1. Main container: `nginx:1.21`
2. Sidecar: `busybox:1.35` running `tail -f /var/log/nginx/access.log`
3. Shared volume between containers

### Scenario 4: Job Configuration (3 minutes)
Create a CronJob that:
- Runs every hour
- Uses `busybox:1.35`
- Executes: `date >> /tmp/hourly.log`
- Keeps 3 successful job history

### Scenario 5: Troubleshooting (4 minutes)
A pod named `broken-app` is failing to start. Identify and document the issues:
1. Check pod status and events
2. Examine logs
3. Identify root cause
4. Provide fix recommendations

---

## Solutions and Explanations

### Exercise 1 Solutions

**Task 1.2 Solution:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: resource-pod
spec:
  containers:
  - name: busybox
    image: busybox:1.35
    command: ["sleep", "3600"]
    resources:
      requests:
        cpu: "100m"
        memory: "128Mi"
      limits:
        cpu: "200m"
        memory: "256Mi"
```

**Task 1.3 Solution:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: env-pod
spec:
  containers:
  - name: busybox
    image: busybox:1.35
    command: ["sh", "-c", "env | grep -E '(DATABASE_URL|LOG_LEVEL)' && sleep 3600"]
    env:
    - name: DATABASE_URL
      value: "postgres://localhost:5432/mydb"
    - name: LOG_LEVEL
      value: "debug"
```

### Exercise 2 Solutions

**Task 2.3 Solution:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: custom-strategy-app
spec:
  replicas: 4
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 2
  selector:
    matchLabels:
      app: custom-strategy-app
  template:
    metadata:
      labels:
        app: custom-strategy-app
    spec:
      containers:
      - name: nginx
        image: nginx:1.21
        ports:
        - containerPort: 80
```

---

## Tips for CKAD Exam

1. **Time Management**: Practice creating resources quickly with `kubectl run` and `kubectl create`
2. **YAML Generation**: Use `--dry-run=client -o yaml` to generate templates
3. **Imperative Commands**: Master kubectl imperative commands for speed
4. **Documentation**: Know how to quickly find examples in Kubernetes docs
5. **Debugging**: Practice troubleshooting workflows systematically
6. **Resource Management**: Always include resource requests/limits in production scenarios
7. **Labels and Selectors**: Understand how controllers use labels to manage pods
8. **Multi-container Patterns**: Know when to use sidecar, ambassador, and adapter patterns

Remember: Practice these exercises multiple times to build muscle memory for the exam!