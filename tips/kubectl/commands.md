# Essential Kubectl Commands for CKAD

## 🎯 Dry Run & YAML Generation (MOST IMPORTANT!)

### Generate Resource Templates
```bash
# Pod
kubectl run nginx --image=nginx --dry-run=client -o yaml > pod.yaml

# Deployment
kubectl create deployment nginx --image=nginx --replicas=3 --dry-run=client -o yaml > deployment.yaml

# Service
kubectl expose deployment nginx --port=80 --dry-run=client -o yaml > service.yaml

# ConfigMap
kubectl create configmap app-config --from-literal=key1=value1 --dry-run=client -o yaml > configmap.yaml

# Secret
kubectl create secret generic app-secret --from-literal=username=admin --dry-run=client -o yaml > secret.yaml

# Job
kubectl create job hello --image=busybox --dry-run=client -o yaml -- echo "Hello World" > job.yaml

# CronJob
kubectl create cronjob hello --image=busybox --schedule="*/1 * * * *" --dry-run=client -o yaml -- echo "Hello World" > cronjob.yaml
```

### Advanced Dry Run Techniques
```bash
# Generate with resource limits
kubectl run nginx --image=nginx --requests="cpu=100m,memory=128Mi" --limits="cpu=200m,memory=256Mi" --dry-run=client -o yaml

# Generate service with specific type
kubectl create service nodeport nginx --tcp=80:80 --dry-run=client -o yaml

# Generate deployment with labels
kubectl create deployment nginx --image=nginx --dry-run=client -o yaml | kubectl label --local -f - app=web --dry-run=client -o yaml
```

## 🔧 Quick Resource Creation

### Pods
```bash
# Simple pod
kubectl run nginx --image=nginx

# Pod with labels
kubectl run nginx --image=nginx --labels="app=web,env=prod"

# Pod with port
kubectl run nginx --image=nginx --port=80

# Pod with command
kubectl run busybox --image=busybox --command -- sleep 3600

# Pod with environment variables
kubectl run nginx --image=nginx --env="KEY1=value1" --env="KEY2=value2"
```

### Deployments
```bash
# Basic deployment
kubectl create deployment nginx --image=nginx

# Deployment with replicas
kubectl create deployment nginx --image=nginx --replicas=3

# Scale deployment
kubectl scale deployment nginx --replicas=5

# Rolling restart
kubectl rollout restart deployment nginx

# Check rollout status
kubectl rollout status deployment nginx
```

### Services
```bash
# ClusterIP service
kubectl expose deployment nginx --port=80 --target-port=8080

# NodePort service
kubectl expose deployment nginx --port=80 --type=NodePort

# LoadBalancer service
kubectl expose deployment nginx --port=80 --type=LoadBalancer

# Create service from scratch
kubectl create service clusterip nginx --tcp=80:8080
```

### ConfigMaps & Secrets
```bash
# ConfigMap from literals
kubectl create configmap app-config --from-literal=database.host=localhost --from-literal=database.port=5432

# ConfigMap from file
kubectl create configmap app-config --from-file=config.properties

# Secret from literals
kubectl create secret generic app-secret --from-literal=username=admin --from-literal=password=secret123

# Secret for Docker registry
kubectl create secret docker-registry regcred --docker-server=myregistry.com --docker-username=user --docker-password=pass
```

## 📋 Resource Management

### Get Resources
```bash
# Basic get commands
kubectl get pods
kubectl get deployments
kubectl get services
kubectl get all

# With additional information
kubectl get pods -o wide
kubectl get pods --show-labels
kubectl get pods --sort-by=.metadata.creationTimestamp

# Custom output
kubectl get pods -o custom-columns="NAME:.metadata.name,STATUS:.status.phase,NODE:.spec.nodeName"

# JSON/YAML output
kubectl get pod nginx -o yaml
kubectl get pod nginx -o json

# Watch resources
kubectl get pods -w
```

### Describe Resources
```bash
kubectl describe pod nginx
kubectl describe deployment nginx
kubectl describe service nginx
kubectl describe node worker-1
```

### Edit Resources
```bash
kubectl edit pod nginx
kubectl edit deployment nginx
kubectl edit service nginx

# Edit with specific editor
KUBE_EDITOR=nano kubectl edit pod nginx
```

### Delete Resources
```bash
# Delete specific resource
kubectl delete pod nginx
kubectl delete deployment nginx

# Delete with file
kubectl delete -f deployment.yaml

# Force delete (immediate)
kubectl delete pod nginx --force --grace-period=0

# Delete with labels
kubectl delete pods -l app=nginx

# Delete all resources of a type
kubectl delete deployments --all
```

## 🔍 Filtering & Selection

### Label Selectors
```bash
# Get resources with specific label
kubectl get pods -l app=nginx
kubectl get pods -l "app=nginx,env=prod"
kubectl get pods -l "app in (nginx,apache)"

# Show labels
kubectl get pods --show-labels

# Add/remove labels
kubectl label pod nginx version=v1
kubectl label pod nginx version-  # Remove label
```

### Field Selectors
```bash
# Filter by field
kubectl get pods --field-selector status.phase=Running
kubectl get pods --field-selector metadata.namespace=default
kubectl get events --field-selector involvedObject.name=nginx
```

### Multiple Namespaces
```bash
# All namespaces
kubectl get pods --all-namespaces
kubectl get pods -A

# Specific namespace
kubectl get pods -n kube-system

# Multiple namespaces
kubectl get pods -A -l app=nginx
```

## ⚡ Context & Namespace Management

### Context Operations
```bash
# List contexts
kubectl config get-contexts

# Current context
kubectl config current-context

# Switch context
kubectl config use-context production

# Set namespace for context
kubectl config set-context --current --namespace=production

# View config
kubectl config view
kubectl config view --minify
```

### Namespace Operations
```bash
# Create namespace
kubectl create namespace production

# Set default namespace
kubectl config set-context --current --namespace=production

# Get resources from specific namespace
kubectl get pods -n production

# Get resources from all namespaces
kubectl get pods --all-namespaces
```

## 🚀 Advanced Commands

### Resource Quotas & Limits
```bash
# Check resource usage
kubectl top pods
kubectl top nodes
kubectl top pods -A

# Describe resource quota
kubectl describe quota -n production
```

### Port Forwarding
```bash
# Forward local port to pod
kubectl port-forward pod/nginx 8080:80

# Forward to service
kubectl port-forward service/nginx 8080:80

# Background port forward
kubectl port-forward pod/nginx 8080:80 &
```

### Exec & Logs
```bash
# Execute command in pod
kubectl exec nginx -- ls /
kubectl exec -it nginx -- /bin/bash

# Multi-container pod
kubectl exec -it nginx -c sidecar -- /bin/sh

# Get logs
kubectl logs nginx
kubectl logs nginx -c sidecar  # specific container
kubectl logs nginx --previous  # previous container
kubectl logs -f nginx          # follow logs
kubectl logs nginx --tail=50   # last 50 lines
```

### Events & Troubleshooting
```bash
# Get events
kubectl get events
kubectl get events --sort-by=.metadata.creationTimestamp
kubectl get events --field-selector involvedObject.name=nginx

# Cluster info
kubectl cluster-info
kubectl cluster-info dump

# Node information
kubectl describe nodes
kubectl get nodes -o wide
```

## 🎯 Exam-Specific Commands

### Quick Validation
```bash
# Validate YAML syntax
kubectl apply --dry-run=client -f manifest.yaml

# Check what would be created
kubectl create --dry-run=client -f manifest.yaml

# Apply and record (for rollbacks)
kubectl apply -f deployment.yaml --record
```

### Troubleshooting Commands
```bash
# Check pod status
kubectl get pods -o wide

# Check events for specific resource
kubectl describe pod nginx | grep Events -A 10

# Check resource constraints
kubectl describe nodes | grep -A 5 "Allocated resources"

# Test network connectivity
kubectl run test --image=busybox --rm -it -- wget -qO- http://service-name
```

## 📝 Command Chaining

### Useful Combinations
```bash
# Create and expose in one line
kubectl run nginx --image=nginx --port=80 --expose

# Scale and check status
kubectl scale deployment nginx --replicas=3 && kubectl rollout status deployment nginx

# Create resource and get details
kubectl apply -f deployment.yaml && kubectl get deployment nginx -o wide

# Delete and recreate
kubectl delete -f deployment.yaml && kubectl apply -f deployment.yaml
```