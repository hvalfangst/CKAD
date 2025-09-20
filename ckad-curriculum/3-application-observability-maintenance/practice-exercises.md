# CKAD Section 3: Application Observability and Maintenance - Practice Exercises

This file contains hands-on exercises to practice monitoring, logging, debugging, and maintaining applications in Kubernetes.

## Exercise 1: Basic Logging and Monitoring

### Task 1.1: Pod Logging
1. Create a pod that generates logs to stdout
2. View the logs using kubectl
3. Follow the logs in real-time
4. View logs from a previous container instance

**Commands:**
```bash
kubectl run log-pod --image=busybox:1.35 -- /bin/sh -c "while true; do echo 'Log entry $(date)'; sleep 5; done"
kubectl logs log-pod
kubectl logs -f log-pod
kubectl logs log-pod --previous
```

### Task 1.2: Multi-Container Logging
1. Create a pod with multiple containers
2. View logs from specific containers
3. View logs from all containers

**Verification:**
```bash
kubectl logs multi-container-pod -c container1
kubectl logs multi-container-pod --all-containers=true
```

### Task 1.3: Log Timestamps and Filtering
1. View logs with timestamps
2. View logs since a specific time
3. Tail specific number of log lines

**Commands:**
```bash
kubectl logs log-pod --timestamps
kubectl logs log-pod --since=1h
kubectl logs log-pod --tail=50
```

---

## Exercise 2: Health Checks and Probes

### Task 2.1: HTTP Liveness Probe
Create a pod with an HTTP liveness probe that:
- Checks the root path "/"
- Initial delay of 10 seconds
- Check every 5 seconds
- Timeout of 1 second
- Failure threshold of 3

**Solution:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: http-liveness
spec:
  containers:
  - name: web
    image: nginx:1.21
    ports:
    - containerPort: 80
    livenessProbe:
      httpGet:
        path: /
        port: 80
      initialDelaySeconds: 10
      periodSeconds: 5
      timeoutSeconds: 1
      failureThreshold: 3
```

### Task 2.2: Exec Readiness Probe
Create a pod with an exec readiness probe that:
- Checks for existence of /tmp/ready file
- Initial delay of 5 seconds
- Check every 5 seconds

### Task 2.3: TCP Startup Probe
Create a pod with a TCP startup probe for a slow-starting application:
- Check port 8080
- Failure threshold of 30
- Check every 10 seconds

### Task 2.4: Combined Probes
Create a deployment with all three probe types working together.

---

## Exercise 3: Resource Monitoring

### Task 3.1: Resource Usage Monitoring
1. Create a deployment with resource requests and limits
2. Monitor pod resource usage
3. Identify high CPU/memory consumers

**Commands:**
```bash
kubectl top pods
kubectl top pods --sort-by=cpu
kubectl top pods --sort-by=memory
kubectl top nodes
```

### Task 3.2: Resource Analysis
1. Create a pod that consumes high CPU
2. Monitor its resource usage over time
3. Identify when resource limits are reached

**High CPU Pod:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: cpu-stress
spec:
  containers:
  - name: stress
    image: stress:latest
    command: ["stress"]
    args: ["--cpu", "2", "--timeout", "300s"]
    resources:
      requests:
        cpu: 100m
        memory: 128Mi
      limits:
        cpu: 200m
        memory: 256Mi
```

### Task 3.3: Node Resource Analysis
1. Check node capacity and allocatable resources
2. Identify node resource utilization
3. Find pods consuming most resources on each node

---

## Exercise 4: Debugging Applications

### Task 4.1: Debug a Failing Pod
Debug this pod that won't start:
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: failing-pod
spec:
  containers:
  - name: app
    image: nonexistent:latest
    command: ["/nonexistent"]
```

**Debugging Steps:**
1. Check pod status
2. Describe the pod
3. Check events
4. Identify and fix issues

### Task 4.2: Debug CrashLoopBackOff
Debug this pod that keeps crashing:
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: crash-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ["/bin/sh", "-c", "exit 1"]
```

### Task 4.3: Debug Resource Issues
Debug this pod that can't be scheduled:
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: resource-pod
spec:
  containers:
  - name: app
    image: nginx:1.21
    resources:
      requests:
        memory: "100Gi"
        cpu: "100"
```

### Task 4.4: Network Debugging
1. Create a debug pod with network utilities
2. Test connectivity to a service
3. Troubleshoot DNS resolution

**Debug Pod:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: debug-pod
spec:
  containers:
  - name: debug
    image: nicolaka/netshoot
    command: ["/bin/bash"]
    args: ["-c", "while true; do sleep 30; done"]
```

---

## Exercise 5: Advanced Debugging

### Task 5.1: Interactive Debugging
1. Execute commands inside a running container
2. Inspect process list
3. Check network connections
4. Examine file system

**Commands:**
```bash
kubectl exec -it pod-name -- /bin/bash
kubectl exec pod-name -- ps aux
kubectl exec pod-name -- netstat -tulpn
kubectl exec pod-name -- df -h
```

### Task 5.2: File Transfer
1. Copy files from a pod to local system
2. Copy files from local system to a pod
3. Extract logs to files

**Commands:**
```bash
kubectl cp pod-name:/path/to/file ./local-file
kubectl cp ./local-file pod-name:/path/to/file
kubectl logs pod-name > pod.log
```

### Task 5.3: Multi-Container Debugging
Debug a multi-container pod where:
- Main container is running
- Sidecar container is failing
- Shared volume has permission issues

---

## Exercise 6: Event Monitoring

### Task 6.1: Event Analysis
1. View all cluster events
2. Filter events by type (Warning/Normal)
3. Find events for specific objects

**Commands:**
```bash
kubectl get events
kubectl get events --field-selector type=Warning
kubectl get events --field-selector involvedObject.name=pod-name
```

### Task 6.2: Real-time Event Monitoring
1. Watch events in real-time
2. Create a pod and observe events
3. Delete a pod and observe events

### Task 6.3: Event Troubleshooting
Use events to troubleshoot:
- Pod scheduling issues
- Image pull problems
- Resource constraints

---

## Exercise 7: Application Performance

### Task 7.1: Performance Baseline
1. Create a simple web application
2. Measure baseline performance
3. Monitor resource usage under normal load

### Task 7.2: Load Testing
1. Generate load against your application
2. Monitor resource usage during load
3. Identify performance bottlenecks

**Load Generator:**
```bash
kubectl run load-generator --rm -i --tty --image=busybox:1.35 -- /bin/sh
# Inside pod: while true; do wget -q --spider http://service-name; done
```

### Task 7.3: Performance Optimization
1. Identify resource-constrained applications
2. Adjust resource requests and limits
3. Verify improved performance

---

## Exercise 8: Maintenance Operations

### Task 8.1: Rolling Restart
1. Perform rolling restart of a deployment
2. Monitor pod replacement
3. Verify application availability

**Commands:**
```bash
kubectl rollout restart deployment/app-name
kubectl rollout status deployment/app-name
```

### Task 8.2: Node Maintenance
1. Drain a node for maintenance
2. Observe pod migration
3. Restore node to service

**Commands:**
```bash
kubectl drain node-name --ignore-daemonsets --delete-emptydir-data
kubectl get pods -o wide
kubectl uncordon node-name
```

### Task 8.3: Resource Cleanup
1. Identify and clean up completed jobs
2. Remove failed pods
3. Clean up unused resources

---

## Exercise 9: Log Management

### Task 9.1: Sidecar Logging
Implement a sidecar logging pattern:
1. Main container writes to shared volume
2. Sidecar container reads and forwards logs
3. Verify log collection

### Task 9.2: Centralized Logging
Set up log aggregation:
1. Deploy log collector as DaemonSet
2. Configure log forwarding
3. Verify log collection

### Task 9.3: Log Analysis
1. Generate structured logs
2. Parse and analyze log data
3. Create log-based alerts

---

## Exercise 10: Monitoring Setup

### Task 10.1: Application Metrics
1. Create an application that exposes metrics
2. Configure service for metrics collection
3. Verify metrics endpoint

### Task 10.2: Custom Health Checks
Implement custom health check endpoints:
- /health - basic health status
- /ready - readiness status
- /metrics - application metrics

### Task 10.3: Monitoring Dashboard
1. Set up metrics collection
2. Create monitoring queries
3. Build basic dashboard

---

## Exam Simulation Exercises

### Scenario 1: Pod Debugging (5 minutes)
A pod named `broken-app` is in CrashLoopBackOff state:
1. Identify the root cause
2. Check logs and events
3. Provide solution recommendations

### Scenario 2: Resource Monitoring (3 minutes)
1. Find the pod consuming most CPU in the cluster
2. Find the pod consuming most memory
3. Check if any nodes are under resource pressure

### Scenario 3: Health Check Setup (4 minutes)
Add health checks to an existing deployment:
1. HTTP liveness probe on port 8080, path /health
2. HTTP readiness probe on port 8080, path /ready
3. Initial delay 10s, check every 5s

### Scenario 4: Log Investigation (3 minutes)
A service is receiving complaints about errors:
1. Check recent logs for error messages
2. Identify which pods are generating errors
3. Determine error frequency

### Scenario 5: Network Debugging (4 minutes)
A pod cannot connect to a service:
1. Test DNS resolution
2. Check service endpoints
3. Verify network connectivity

---

## Solutions and Best Practices

### Exercise 1 Solutions

**Task 1.2 Multi-Container Pod:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: multi-container-pod
spec:
  containers:
  - name: container1
    image: busybox:1.35
    command: ["/bin/sh", "-c", "while true; do echo 'Container 1 log'; sleep 5; done"]
  - name: container2
    image: busybox:1.35
    command: ["/bin/sh", "-c", "while true; do echo 'Container 2 log'; sleep 3; done"]
```

### Exercise 2 Solutions

**Task 2.2 Exec Readiness Probe:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: exec-readiness
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ["/bin/sh", "-c", "sleep 30; touch /tmp/ready; sleep 600"]
    readinessProbe:
      exec:
        command:
        - cat
        - /tmp/ready
      initialDelaySeconds: 5
      periodSeconds: 5
```

### Exercise 4 Solutions

**Task 4.1 Debugging Steps:**
```bash
# Check pod status
kubectl get pods

# Describe pod for detailed info
kubectl describe pod failing-pod

# Check events
kubectl get events --field-selector involvedObject.name=failing-pod

# Fix: Update image to existing one
kubectl patch pod failing-pod -p '{"spec":{"containers":[{"name":"app","image":"busybox:1.35","command":["/bin/sh","-c","sleep 3600"]}]}}'
```

### Common Debugging Commands

```bash
# Quick health check
kubectl get pods | grep -v Running | grep -v Completed

# Resource usage summary
kubectl top pods --sort-by=cpu | head -10

# Recent events
kubectl get events --sort-by=.metadata.creationTimestamp | tail -10

# Pod details
kubectl describe pod <pod-name> | grep -A10 -B10 -i error

# Network connectivity test
kubectl exec -it <pod-name> -- nc -zv <service-name> <port>

# DNS resolution test
kubectl exec -it <pod-name> -- nslookup <service-name>
```

---

## Troubleshooting Checklists

### Pod Won't Start Checklist
- [ ] Check pod events: `kubectl describe pod`
- [ ] Verify image exists and is accessible
- [ ] Check resource requests vs node capacity
- [ ] Verify security contexts and permissions
- [ ] Check for node selector/affinity issues

### Service Not Working Checklist
- [ ] Check service exists: `kubectl get svc`
- [ ] Verify endpoints: `kubectl get endpoints`
- [ ] Check pod labels match service selector
- [ ] Test DNS resolution: `nslookup service-name`
- [ ] Test connectivity: `telnet service-name port`

### High Resource Usage Checklist
- [ ] Monitor resource usage: `kubectl top pods`
- [ ] Check resource limits: `kubectl describe pod`
- [ ] Review application logs for errors
- [ ] Check for memory leaks or CPU loops
- [ ] Verify resource requests are appropriate

### Performance Issues Checklist
- [ ] Monitor response times
- [ ] Check resource utilization
- [ ] Review application metrics
- [ ] Analyze log patterns
- [ ] Test under different load conditions

Remember: Practice these debugging scenarios repeatedly to build systematic troubleshooting skills for the CKAD exam!