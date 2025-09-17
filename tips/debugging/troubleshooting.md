# Debugging & Troubleshooting Guide

## 🔍 Essential Debugging Commands

### First Line of Defense
```bash
# Check resource status
kubectl get pods -o wide
kubectl get all
kubectl get events --sort-by=.metadata.creationTimestamp

# Describe resources (MOST IMPORTANT!)
kubectl describe pod <pod-name>
kubectl describe deployment <deployment-name>
kubectl describe service <service-name>
kubectl describe node <node-name>
```

### Logs Investigation
```bash
# Basic log commands
kubectl logs <pod-name>
kubectl logs <pod-name> -c <container-name>  # Multi-container pods
kubectl logs <pod-name> --previous           # Previous container instance
kubectl logs -f <pod-name>                   # Follow logs real-time
kubectl logs <pod-name> --tail=50            # Last 50 lines
kubectl logs <pod-name> --since=1h           # Last hour

# Advanced logging
kubectl logs -l app=nginx                    # All pods with label
kubectl logs deployment/nginx                # All pods in deployment
kubectl logs <pod-name> --timestamps=true    # With timestamps
```

## 🚨 Common Issues & Solutions

### Pod Issues

#### Pod Stuck in Pending
```bash
# Check events
kubectl describe pod <pod-name>

# Common causes:
# - Insufficient resources
kubectl describe nodes
kubectl top nodes

# - Node selector issues
kubectl get nodes --show-labels

# - PVC binding issues
kubectl get pv,pvc
```

#### Pod Stuck in ContainerCreating
```bash
# Check events
kubectl describe pod <pod-name>

# Common causes:
# - Image pull issues
# - Volume mount problems
# - Secret/ConfigMap missing

# Check image pull
kubectl describe pod <pod-name> | grep -A 5 "Events"
```

#### CrashLoopBackOff
```bash
# Check logs
kubectl logs <pod-name>
kubectl logs <pod-name> --previous

# Check resource limits
kubectl describe pod <pod-name> | grep -A 5 "Limits"

# Check liveness/readiness probes
kubectl describe pod <pod-name> | grep -A 10 "Liveness"
```

#### ImagePullBackOff
```bash
# Check image name and tag
kubectl describe pod <pod-name>

# Check secrets for private registries
kubectl get secrets
kubectl describe secret <registry-secret>

# Test image pull manually
docker pull <image-name>
```

### Service Issues

#### Service Not Accessible
```bash
# Check service and endpoints
kubectl get svc <service-name>
kubectl get endpoints <service-name>

# Check pod labels match service selector
kubectl describe svc <service-name>
kubectl get pods --show-labels

# Test service connectivity
kubectl run test --image=busybox --rm -it -- wget -qO- http://<service-name>:<port>
```

#### No Endpoints
```bash
# Check if pods are running and ready
kubectl get pods -l <selector-labels>

# Check pod labels
kubectl get pods --show-labels

# Verify service selector
kubectl describe svc <service-name>
```

### Deployment Issues

#### Deployment Not Rolling Out
```bash
# Check rollout status
kubectl rollout status deployment <deployment-name>

# Check replica set
kubectl get rs -l app=<app-label>

# Check deployment conditions
kubectl describe deployment <deployment-name>
```

#### Scaling Issues
```bash
# Check current replicas
kubectl get deployment <deployment-name>

# Check resource constraints
kubectl describe nodes | grep -A 5 "Allocated resources"

# Check HPA if configured
kubectl get hpa
```

## 🔧 Network Debugging

### DNS Resolution
```bash
# Test DNS from inside cluster
kubectl run test --image=busybox --rm -it -- nslookup kubernetes.default

# Test service DNS
kubectl run test --image=busybox --rm -it -- nslookup <service-name>

# Test cross-namespace DNS
kubectl run test --image=busybox --rm -it -- nslookup <service-name>.<namespace>.svc.cluster.local
```

### Network Connectivity
```bash
# Test service connectivity
kubectl run test --image=busybox --rm -it -- telnet <service-name> <port>

# Test external connectivity
kubectl run test --image=busybox --rm -it -- wget -qO- http://google.com

# Check network policies
kubectl get networkpolicies
kubectl describe networkpolicy <policy-name>
```

### Port Forwarding for Testing
```bash
# Forward pod port to localhost
kubectl port-forward pod/<pod-name> 8080:80

# Forward service port
kubectl port-forward service/<service-name> 8080:80

# Test from localhost
curl localhost:8080
```

## 📊 Resource & Performance Issues

### Resource Constraints
```bash
# Check node resources
kubectl top nodes
kubectl describe nodes | grep -A 5 "Allocated resources"

# Check pod resources
kubectl top pods
kubectl top pods -A

# Check resource requests/limits
kubectl describe pod <pod-name> | grep -A 5 "Requests"
kubectl describe pod <pod-name> | grep -A 5 "Limits"
```

### Storage Issues
```bash
# Check PV/PVC status
kubectl get pv,pvc
kubectl describe pv <pv-name>
kubectl describe pvc <pvc-name>

# Check storage class
kubectl get storageclass
kubectl describe storageclass <sc-name>

# Check volume mounts
kubectl describe pod <pod-name> | grep -A 10 "Mounts"
```

## 🔐 Security & RBAC Issues

### Permission Problems
```bash
# Check current user permissions
kubectl auth can-i create pods
kubectl auth can-i create pods --namespace=production

# Check service account permissions
kubectl auth can-i create pods --as=system:serviceaccount:default:my-sa

# Check RBAC
kubectl get roles,rolebindings
kubectl get clusterroles,clusterrolebindings
```

### Secret Issues
```bash
# Check secrets exist
kubectl get secrets
kubectl describe secret <secret-name>

# Check secret data (base64 encoded)
kubectl get secret <secret-name> -o yaml

# Decode secret data
kubectl get secret <secret-name> -o jsonpath='{.data.password}' | base64 -d
```

## 🚀 Advanced Debugging

### Exec into Containers
```bash
# Interactive shell
kubectl exec -it <pod-name> -- /bin/bash
kubectl exec -it <pod-name> -- /bin/sh

# Multi-container pods
kubectl exec -it <pod-name> -c <container-name> -- /bin/bash

# Run single command
kubectl exec <pod-name> -- ps aux
kubectl exec <pod-name> -- ls -la /app
```

### Debug Containers (Kubernetes 1.23+)
```bash
# Add debug container to running pod
kubectl debug <pod-name> -it --image=busybox --target=<container-name>

# Create debug copy of pod
kubectl debug <pod-name> -it --image=busybox --copy-to=<debug-pod-name>
```

### Events Analysis
```bash
# Get all events sorted by time
kubectl get events --sort-by=.metadata.creationTimestamp

# Events for specific object
kubectl get events --field-selector involvedObject.name=<pod-name>

# Events in specific namespace
kubectl get events -n <namespace>

# Watch events real-time
kubectl get events -w
```

## 🎯 Debugging Workflow

### Systematic Approach

1. **Check Status**
   ```bash
   kubectl get pods -o wide
   kubectl get all
   ```

2. **Get Events**
   ```bash
   kubectl get events --sort-by=.metadata.creationTimestamp
   kubectl describe pod <pod-name>
   ```

3. **Check Logs**
   ```bash
   kubectl logs <pod-name>
   kubectl logs <pod-name> --previous
   ```

4. **Verify Configuration**
   ```bash
   kubectl get <resource> <name> -o yaml
   kubectl describe <resource> <name>
   ```

5. **Test Connectivity**
   ```bash
   kubectl port-forward pod/<pod-name> 8080:80
   kubectl exec -it <pod-name> -- /bin/bash
   ```

### Quick Debug Checklist

- [ ] Pod status and phase
- [ ] Events and error messages
- [ ] Container logs (current and previous)
- [ ] Resource requests/limits
- [ ] Image pull status
- [ ] Volume mounts
- [ ] Service endpoints
- [ ] Network connectivity
- [ ] DNS resolution
- [ ] RBAC permissions

## 🚨 Emergency Commands

### Force Operations
```bash
# Force delete stuck pod
kubectl delete pod <pod-name> --force --grace-period=0

# Force delete namespace
kubectl delete namespace <namespace> --force --grace-period=0

# Restart deployment immediately
kubectl rollout restart deployment <deployment-name>
```

### Quick Fixes
```bash
# Scale to 0 and back
kubectl scale deployment <deployment-name> --replicas=0
kubectl scale deployment <deployment-name> --replicas=3

# Recreate resource
kubectl delete -f manifest.yaml && kubectl apply -f manifest.yaml

# Replace resource
kubectl replace --force -f manifest.yaml
```

## 📝 Debugging Best Practices

1. **Start with events** - `kubectl describe` shows recent events
2. **Check logs systematically** - current, previous, all containers
3. **Verify labels and selectors** - common source of issues
4. **Test step by step** - isolate problems
5. **Use port-forward** - test connectivity without services
6. **Check resource constraints** - CPU, memory, storage
7. **Validate YAML** - use dry-run before applying
8. **Document findings** - note what worked/didn't work

Remember: Most issues are configuration problems, not platform bugs!