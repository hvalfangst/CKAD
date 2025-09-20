# CKAD Section 3: Application Observability and Maintenance (15%)

This section covers monitoring, logging, debugging, and maintaining applications in Kubernetes. You'll learn how to troubleshoot issues, collect metrics, manage logs, and ensure application health.

## Table of Contents

1. [Application Monitoring](#application-monitoring)
2. [Logging and Log Management](#logging-and-log-management)
3. [Debugging Applications](#debugging-applications)
4. [Health Checks and Probes](#health-checks-and-probes)
5. [Metrics and Monitoring Tools](#metrics-and-monitoring-tools)
6. [Troubleshooting Workflows](#troubleshooting-workflows)
7. [Application Performance](#application-performance)
8. [Maintenance Operations](#maintenance-operations)
9. [Event Monitoring](#event-monitoring)
10. [Best Practices](#best-practices)

---

## Application Monitoring

### Overview
Monitoring provides visibility into application health, performance, and resource usage in Kubernetes clusters.

### Pod Monitoring

```bash
# Check pod status
kubectl get pods
kubectl get pods -o wide
kubectl get pods --show-labels

# Monitor pod resources
kubectl top pods
kubectl top pods --sort-by=cpu
kubectl top pods --sort-by=memory

# Watch pod changes
kubectl get pods --watch
kubectl get pods -w -l app=myapp
```

### Node Monitoring

```bash
# Check node status
kubectl get nodes
kubectl get nodes -o wide

# Monitor node resources
kubectl top nodes
kubectl describe nodes

# Check node capacity
kubectl describe node <node-name> | grep -A5 "Capacity\|Allocatable"
```

### Namespace Monitoring

```bash
# Monitor namespace resources
kubectl top pods -n <namespace>
kubectl get events -n <namespace>

# Check resource quotas
kubectl describe quota -n <namespace>
kubectl get limitrange -n <namespace>
```

---

## Logging and Log Management

### Container Logs

```bash
# Basic log viewing
kubectl logs <pod-name>
kubectl logs <pod-name> -c <container-name>  # Multi-container pods

# Follow logs in real-time
kubectl logs -f <pod-name>
kubectl logs -f <pod-name> -c <container-name>

# Previous container logs
kubectl logs <pod-name> --previous
kubectl logs <pod-name> -c <container-name> --previous

# Tail logs
kubectl logs <pod-name> --tail=100
kubectl logs <pod-name> --since=1h

# Logs with timestamps
kubectl logs <pod-name> --timestamps
```

### Advanced Log Queries

```bash
# Logs from multiple pods
kubectl logs -l app=myapp
kubectl logs -l app=myapp --all-containers=true

# Logs from all containers in a pod
kubectl logs <pod-name> --all-containers=true

# Logs since specific time
kubectl logs <pod-name> --since-time=2023-01-01T00:00:00Z

# Export logs to file
kubectl logs <pod-name> > app.log
```

### Sidecar Logging Pattern

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: counter
spec:
  containers:
  - name: count
    image: busybox:1.35
    command: ["/bin/sh", "-c", "i=0; while true; do echo \"$i: $(date)\" >> /var/log/1.log; i=$((i+1)); sleep 1; done"]
    volumeMounts:
    - name: varlog
      mountPath: /var/log
  - name: count-log
    image: busybox:1.35
    command: ["/bin/sh", "-c", "tail -n+1 -f /var/log/1.log"]
    volumeMounts:
    - name: varlog
      mountPath: /var/log
  volumes:
  - name: varlog
    emptyDir: {}
```

### Log Aggregation Configuration

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: fluent-bit-config
data:
  fluent-bit.conf: |
    [SERVICE]
        Flush         1
        Log_Level     info
        Daemon        off
        Parsers_File  parsers.conf

    [INPUT]
        Name              tail
        Path              /var/log/containers/*.log
        Parser            docker
        Tag               kube.*
        Refresh_Interval  5

    [OUTPUT]
        Name  es
        Match *
        Host  elasticsearch
        Port  9200
        Index kubernetes
```

---

## Debugging Applications

### Pod State Diagnosis

```bash
# Get detailed pod information
kubectl describe pod <pod-name>

# Check pod events
kubectl get events --field-selector involvedObject.name=<pod-name>

# Check pod specifications
kubectl get pod <pod-name> -o yaml
kubectl get pod <pod-name> -o json
```

### Common Pod Issues

#### ImagePullBackOff
```bash
# Check image details
kubectl describe pod <pod-name>

# Verify image exists
docker pull <image-name>

# Check image pull secrets
kubectl get secrets
kubectl describe secret <secret-name>
```

#### CrashLoopBackOff
```bash
# Check container logs
kubectl logs <pod-name> --previous

# Examine container configuration
kubectl describe pod <pod-name>

# Check resource limits
kubectl top pod <pod-name>
```

#### Pending Pods
```bash
# Check scheduling issues
kubectl describe pod <pod-name>

# Check node resources
kubectl describe nodes
kubectl top nodes

# Check pod requirements
kubectl get pod <pod-name> -o yaml | grep -A10 resources
```

### Interactive Debugging

```bash
# Execute commands in running container
kubectl exec -it <pod-name> -- /bin/bash
kubectl exec -it <pod-name> -c <container-name> -- /bin/sh

# Run debug commands
kubectl exec <pod-name> -- ps aux
kubectl exec <pod-name> -- netstat -tulpn
kubectl exec <pod-name> -- cat /etc/hosts

# Copy files to/from container
kubectl cp <pod-name>:/path/to/file ./local-file
kubectl cp ./local-file <pod-name>:/path/to/file
```

### Debug Utilities Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: debug-utils
spec:
  containers:
  - name: debug
    image: nicolaka/netshoot
    command: ["/bin/bash"]
    args: ["-c", "while true; do sleep 30; done"]
    securityContext:
      capabilities:
        add: ["NET_ADMIN"]
```

---

## Health Checks and Probes

### Liveness Probes

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: liveness-http
spec:
  containers:
  - name: liveness
    image: nginx:1.21
    ports:
    - containerPort: 80
    livenessProbe:
      httpGet:
        path: /
        port: 80
      initialDelaySeconds: 30
      periodSeconds: 10
      timeoutSeconds: 5
      failureThreshold: 3
      successThreshold: 1
```

### Readiness Probes

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: readiness-exec
spec:
  containers:
  - name: readiness
    image: nginx:1.21
    readinessProbe:
      exec:
        command:
        - cat
        - /tmp/healthy
      initialDelaySeconds: 5
      periodSeconds: 5
      timeoutSeconds: 1
      failureThreshold: 3
      successThreshold: 1
```

### Startup Probes

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: startup-tcp
spec:
  containers:
  - name: app
    image: myapp:latest
    ports:
    - containerPort: 8080
    startupProbe:
      tcpSocket:
        port: 8080
      failureThreshold: 30
      periodSeconds: 10
    livenessProbe:
      tcpSocket:
        port: 8080
      periodSeconds: 10
    readinessProbe:
      tcpSocket:
        port: 8080
      periodSeconds: 5
```

### Combined Health Checks

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: healthy-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: healthy-app
  template:
    metadata:
      labels:
        app: healthy-app
    spec:
      containers:
      - name: app
        image: nginx:1.21
        ports:
        - containerPort: 80
        startupProbe:
          httpGet:
            path: /
            port: 80
          failureThreshold: 30
          periodSeconds: 10
        livenessProbe:
          httpGet:
            path: /health
            port: 80
          initialDelaySeconds: 0
          periodSeconds: 10
          timeoutSeconds: 1
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /ready
            port: 80
          initialDelaySeconds: 0
          periodSeconds: 5
          timeoutSeconds: 1
          failureThreshold: 3
```

---

## Metrics and Monitoring Tools

### Metrics Server

```bash
# Check metrics server
kubectl get apiservice v1beta1.metrics.k8s.io

# Install metrics server (if needed)
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml

# Verify metrics availability
kubectl top nodes
kubectl top pods
```

### Resource Monitoring

```bash
# Pod resource usage
kubectl top pods --all-namespaces
kubectl top pods --sort-by=cpu
kubectl top pods --sort-by=memory

# Node resource usage
kubectl top nodes

# Namespace resource usage
kubectl top pods -n kube-system
```

### Custom Metrics Example

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: metrics-app
  annotations:
    prometheus.io/scrape: "true"
    prometheus.io/port: "8080"
    prometheus.io/path: "/metrics"
spec:
  containers:
  - name: app
    image: prom/node-exporter:latest
    ports:
    - containerPort: 9100
      name: metrics
```

---

## Troubleshooting Workflows

### Systematic Debugging Approach

1. **Gather Information**
   ```bash
   kubectl get pods
   kubectl describe pod <pod-name>
   kubectl logs <pod-name>
   ```

2. **Check Events**
   ```bash
   kubectl get events --sort-by=.metadata.creationTimestamp
   kubectl get events --field-selector involvedObject.name=<pod-name>
   ```

3. **Verify Configuration**
   ```bash
   kubectl get pod <pod-name> -o yaml
   kubectl explain pod.spec.containers
   ```

4. **Test Connectivity**
   ```bash
   kubectl exec -it <pod-name> -- nslookup kubernetes.default
   kubectl exec -it <pod-name> -- curl <service-name>
   ```

### Network Troubleshooting

```bash
# Check service endpoints
kubectl get endpoints
kubectl describe endpoints <service-name>

# Test DNS resolution
kubectl exec -it <pod-name> -- nslookup <service-name>
kubectl exec -it <pod-name> -- dig <service-name>

# Check network policies
kubectl get networkpolicies
kubectl describe networkpolicy <policy-name>

# Port connectivity test
kubectl exec -it <pod-name> -- telnet <service-name> <port>
kubectl exec -it <pod-name> -- nc -zv <service-name> <port>
```

### Storage Troubleshooting

```bash
# Check persistent volumes
kubectl get pv
kubectl describe pv <pv-name>

# Check persistent volume claims
kubectl get pvc
kubectl describe pvc <pvc-name>

# Check mount points
kubectl exec -it <pod-name> -- df -h
kubectl exec -it <pod-name> -- mount | grep <volume-name>
```

---

## Application Performance

### Resource Utilization Analysis

```bash
# Monitor resource usage over time
watch kubectl top pods

# Check resource requests vs usage
kubectl describe pod <pod-name> | grep -A10 "Requests\|Limits"

# Identify resource-intensive pods
kubectl top pods --sort-by=cpu --no-headers | head -10
kubectl top pods --sort-by=memory --no-headers | head -10
```

### Performance Profiling

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: performance-test
spec:
  containers:
  - name: app
    image: nginx:1.21
    resources:
      requests:
        cpu: 100m
        memory: 128Mi
      limits:
        cpu: 500m
        memory: 512Mi
    readinessProbe:
      httpGet:
        path: /
        port: 80
      initialDelaySeconds: 5
      periodSeconds: 1
    livenessProbe:
      httpGet:
        path: /
        port: 80
      initialDelaySeconds: 10
      periodSeconds: 10
```

### Load Testing

```bash
# Create load testing pod
kubectl run load-test --image=busybox:1.35 -it --rm --restart=Never -- /bin/sh

# Inside the pod, generate load
while true; do wget -q --spider http://your-service; done
```

---

## Maintenance Operations

### Rolling Restarts

```bash
# Restart deployment
kubectl rollout restart deployment/<deployment-name>

# Restart daemonset
kubectl rollout restart daemonset/<daemonset-name>

# Restart statefulset
kubectl rollout restart statefulset/<statefulset-name>
```

### Draining Nodes

```bash
# Drain node for maintenance
kubectl drain <node-name> --ignore-daemonsets --delete-emptydir-data

# Cordon node (mark unschedulable)
kubectl cordon <node-name>

# Uncordon node (mark schedulable)
kubectl uncordon <node-name>
```

### Resource Cleanup

```bash
# Delete completed jobs
kubectl delete jobs --field-selector status.successful=1

# Delete evicted pods
kubectl delete pods --field-selector=status.phase=Failed

# Clean up orphaned resources
kubectl get all --all-namespaces | grep -i terminating
```

---

## Event Monitoring

### Event Types and Analysis

```bash
# Get all events
kubectl get events

# Sort events by timestamp
kubectl get events --sort-by=.metadata.creationTimestamp

# Filter events by type
kubectl get events --field-selector type=Warning
kubectl get events --field-selector type=Normal

# Filter events by reason
kubectl get events --field-selector reason=Failed
kubectl get events --field-selector reason=SuccessfulCreate

# Watch events in real-time
kubectl get events --watch
```

### Event Details

```bash
# Get events for specific object
kubectl get events --field-selector involvedObject.name=<pod-name>
kubectl get events --field-selector involvedObject.namespace=<namespace>

# Get events with custom output
kubectl get events -o custom-columns=LAST-SEEN:.lastTimestamp,TYPE:.type,REASON:.reason,OBJECT:.involvedObject.name,MESSAGE:.message

# Export events
kubectl get events -o json > events.json
kubectl get events -o yaml > events.yaml
```

---

## Best Practices

### Logging Best Practices

1. **Structured Logging**
   ```yaml
   env:
   - name: LOG_FORMAT
     value: "json"
   - name: LOG_LEVEL
     value: "info"
   ```

2. **Log Rotation**
   ```bash
   # Configure kubelet log rotation
   --container-log-max-size=10Mi
   --container-log-max-files=5
   ```

3. **Centralized Logging**
   - Use log aggregation systems (ELK, Fluentd)
   - Implement log forwarding
   - Set up log retention policies

### Monitoring Best Practices

1. **Health Checks**
   - Always implement readiness probes
   - Use appropriate liveness probe timeouts
   - Implement startup probes for slow-starting apps

2. **Resource Monitoring**
   - Set appropriate resource requests and limits
   - Monitor resource utilization trends
   - Implement alerting on resource thresholds

3. **Application Metrics**
   - Expose application metrics
   - Use consistent metric naming
   - Implement business-level monitoring

### Troubleshooting Best Practices

1. **Documentation**
   - Maintain runbooks for common issues
   - Document application dependencies
   - Keep debugging commands handy

2. **Automation**
   - Automate health checks
   - Implement self-healing mechanisms
   - Use monitoring automation

3. **Preparation**
   - Have debug tools readily available
   - Maintain access to log aggregation
   - Keep contact information updated

---

## Exam Tips and Common Scenarios

### Quick Debugging Commands

```bash
# Fast pod status check
kubectl get pods | grep -v Running

# Quick error identification
kubectl describe pod <pod-name> | grep -A5 -B5 -i error

# Recent events
kubectl get events --sort-by=.metadata.creationTimestamp | tail -10

# Resource usage summary
kubectl top pods --sort-by=cpu
```

### Common Troubleshooting Scenarios

1. **Pod Won't Start**
   - Check events: `kubectl describe pod`
   - Verify image: `kubectl get pod -o yaml`
   - Check resources: `kubectl top nodes`

2. **Service Not Accessible**
   - Check endpoints: `kubectl get endpoints`
   - Verify labels: `kubectl get pods --show-labels`
   - Test connectivity: `kubectl exec -it pod -- curl service`

3. **High Resource Usage**
   - Monitor usage: `kubectl top pods`
   - Check limits: `kubectl describe pod`
   - Review logs: `kubectl logs pod`

### Monitoring Workflows

```bash
# Complete pod health check
kubectl get pods
kubectl describe pod <pod-name>
kubectl logs <pod-name>
kubectl exec -it <pod-name> -- ps aux

# Service connectivity check
kubectl get svc
kubectl get endpoints
kubectl exec -it <pod-name> -- nslookup <service>
kubectl exec -it <pod-name> -- curl <service>

# Resource utilization check
kubectl top nodes
kubectl top pods
kubectl describe nodes | grep -A5 "Allocated resources"
```

This comprehensive guide covers all essential aspects of application observability and maintenance for CKAD. Practice these monitoring, logging, and debugging techniques to effectively manage Kubernetes applications.