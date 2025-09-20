# CKAD Section 5: Services and Networking (20%)

This section covers Kubernetes networking concepts, services, ingress, and network policies. You'll learn how to expose applications, manage traffic routing, and implement network security.

## Table of Contents

1. [Services Overview](#services-overview)
2. [Service Types](#service-types)
3. [Service Discovery](#service-discovery)
4. [Endpoints and EndpointSlices](#endpoints-and-endpointslices)
5. [Ingress](#ingress)
6. [Network Policies](#network-policies)
7. [DNS and Service Names](#dns-and-service-names)
8. [Load Balancing](#load-balancing)
9. [Port Forwarding](#port-forwarding)
10. [Troubleshooting Networking](#troubleshooting-networking)

---

## Services Overview

### What are Services?
Services provide a stable network interface to a set of pods. They abstract the underlying pod IPs and provide load balancing, service discovery, and external access.

### Service Characteristics
- Stable IP address and DNS name
- Load balancing across multiple pods
- Service discovery through DNS
- Decoupling of clients from pod locations

### Basic Service Structure

```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service
  labels:
    app: my-service
spec:
  selector:
    app: my-app
  ports:
  - name: http
    protocol: TCP
    port: 80
    targetPort: 8080
  type: ClusterIP
```

---

## Service Types

### ClusterIP (Default)

Exposes the service on an internal IP in the cluster. Only accessible from within the cluster.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: clusterip-service
spec:
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

### NodePort

Exposes the service on each node's IP at a static port (30000-32767).

```yaml
apiVersion: v1
kind: Service
metadata:
  name: nodeport-service
spec:
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 8080
    nodePort: 30080
  type: NodePort
```

### LoadBalancer

Exposes the service externally using a cloud provider's load balancer.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: loadbalancer-service
spec:
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

### ExternalName

Maps the service to a DNS name, returns a CNAME record.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: external-service
spec:
  type: ExternalName
  externalName: example.com
```

### Headless Service

Service with `clusterIP: None` - no load balancing, returns pod IPs directly.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: headless-service
spec:
  clusterIP: None
  selector:
    app: database
  ports:
  - port: 5432
    targetPort: 5432
```

---

## Service Discovery

### DNS-based Discovery

```bash
# Service accessible via DNS
curl http://my-service.default.svc.cluster.local
curl http://my-service.default.svc
curl http://my-service.default
curl http://my-service  # If in same namespace
```

### Environment Variables

Kubernetes automatically creates environment variables for services:

```bash
# In pods, these environment variables are available:
MY_SERVICE_SERVICE_HOST=10.0.0.1
MY_SERVICE_SERVICE_PORT=80
MY_SERVICE_PORT_80_TCP=tcp://10.0.0.1:80
MY_SERVICE_PORT_80_TCP_PROTO=tcp
MY_SERVICE_PORT_80_TCP_PORT=80
MY_SERVICE_PORT_80_TCP_ADDR=10.0.0.1
```

### Service Discovery Example

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: client-pod
spec:
  containers:
  - name: client
    image: busybox:1.35
    command: ['sh', '-c']
    args:
    - |
      echo "Testing service discovery..."
      nslookup my-service
      curl http://my-service:80
      env | grep MY_SERVICE
      sleep 3600
```

---

## Endpoints and EndpointSlices

### Endpoints

Kubernetes automatically creates Endpoints objects that track the IPs of pods matching a service selector.

```bash
# View service endpoints
kubectl get endpoints
kubectl describe endpoints my-service
```

### Manual Endpoints

For services without selectors, you can manually define endpoints:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: external-database
spec:
  ports:
  - port: 5432
    targetPort: 5432
---
apiVersion: v1
kind: Endpoints
metadata:
  name: external-database
subsets:
- addresses:
  - ip: 192.168.1.100
  - ip: 192.168.1.101
  ports:
  - port: 5432
```

### EndpointSlices

Modern replacement for Endpoints, providing better scalability:

```yaml
apiVersion: discovery.k8s.io/v1
kind: EndpointSlice
metadata:
  name: my-service-1
  labels:
    kubernetes.io/service-name: my-service
addressType: IPv4
endpoints:
- addresses:
  - "10.1.2.3"
  conditions:
    ready: true
  hostname: pod-1
  nodeName: node-1
ports:
- name: http
  port: 80
  protocol: TCP
```

---

## Ingress

### Ingress Overview

Ingress manages external access to services, typically HTTP/HTTPS. Provides load balancing, SSL termination, and name-based virtual hosting.

### Basic Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: basic-ingress
spec:
  rules:
  - host: example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: web-service
            port:
              number: 80
```

### Path-based Routing

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: path-based-ingress
spec:
  rules:
  - host: example.com
    http:
      paths:
      - path: /app1
        pathType: Prefix
        backend:
          service:
            name: app1-service
            port:
              number: 80
      - path: /app2
        pathType: Prefix
        backend:
          service:
            name: app2-service
            port:
              number: 80
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: api-service
            port:
              number: 8080
```

### Host-based Routing

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: host-based-ingress
spec:
  rules:
  - host: app1.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: app1-service
            port:
              number: 80
  - host: app2.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: app2-service
            port:
              number: 80
```

### TLS Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: tls-ingress
spec:
  tls:
  - hosts:
    - example.com
    - www.example.com
    secretName: tls-secret
  rules:
  - host: example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: web-service
            port:
              number: 80
```

### Ingress with Annotations

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: annotated-ingress
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /$1
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/cors-allow-origin: "*"
    kubernetes.io/ingress.class: "nginx"
spec:
  rules:
  - host: api.example.com
    http:
      paths:
      - path: /v1/(.*)
        pathType: Prefix
        backend:
          service:
            name: api-v1-service
            port:
              number: 8080
```

### Path Types

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: path-types-ingress
spec:
  rules:
  - host: example.com
    http:
      paths:
      # Exact match
      - path: /exact-path
        pathType: Exact
        backend:
          service:
            name: exact-service
            port:
              number: 80
      # Prefix match
      - path: /prefix
        pathType: Prefix
        backend:
          service:
            name: prefix-service
            port:
              number: 80
      # Implementation-specific
      - path: /regex/*
        pathType: ImplementationSpecific
        backend:
          service:
            name: regex-service
            port:
              number: 80
```

---

## Network Policies

### Default Deny All

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-all
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
```

### Allow Specific Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-frontend
spec:
  podSelector:
    matchLabels:
      app: backend
  policyTypes:
  - Ingress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: frontend
    ports:
    - protocol: TCP
      port: 8080
```

### Allow Cross-Namespace Traffic

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-cross-namespace
spec:
  podSelector:
    matchLabels:
      app: api
  policyTypes:
  - Ingress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: frontend-ns
    - podSelector:
        matchLabels:
          app: frontend
    ports:
    - protocol: TCP
      port: 8080
```

### Complex Network Policy

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: complex-netpol
spec:
  podSelector:
    matchLabels:
      app: database
  policyTypes:
  - Ingress
  - Egress
  ingress:
  # Allow from backend pods
  - from:
    - podSelector:
        matchLabels:
          app: backend
    ports:
    - protocol: TCP
      port: 5432
  # Allow from monitoring namespace
  - from:
    - namespaceSelector:
        matchLabels:
          name: monitoring
    ports:
    - protocol: TCP
      port: 9090
  egress:
  # Allow DNS
  - to: []
    ports:
    - protocol: UDP
      port: 53
  # Allow backup service
  - to:
    - podSelector:
        matchLabels:
          app: backup
    ports:
    - protocol: TCP
      port: 3000
```

---

## DNS and Service Names

### Service DNS Names

Services get DNS names following this pattern:
- `<service-name>.<namespace>.svc.cluster.local`
- Short forms: `<service-name>.<namespace>`, `<service-name>` (same namespace)

### Headless Service DNS

For headless services, DNS returns individual pod IPs:
- `<pod-name>.<service-name>.<namespace>.svc.cluster.local`

### Pod DNS Names

Pods get DNS names when using headless services:
- `<pod-ip-with-dashes>.<service-name>.<namespace>.svc.cluster.local`

### DNS Configuration Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: dns-test-pod
spec:
  containers:
  - name: dns-test
    image: busybox:1.35
    command: ['sh', '-c']
    args:
    - |
      echo "Testing DNS resolution..."
      nslookup kubernetes.default
      nslookup my-service.default.svc.cluster.local
      nslookup my-service.default
      nslookup my-service
      sleep 3600
  dnsPolicy: ClusterFirst
  dnsConfig:
    options:
    - name: ndots
      value: "2"
    - name: edns0
```

---

## Load Balancing

### Service Load Balancing

Kubernetes services provide automatic load balancing across healthy pods:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: load-balanced-service
spec:
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 8080
  sessionAffinity: None  # Round-robin (default)
  # sessionAffinity: ClientIP  # Session affinity
```

### Session Affinity

```yaml
apiVersion: v1
kind: Service
metadata:
  name: sticky-service
spec:
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 8080
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 10800  # 3 hours
```

### External Traffic Policy

```yaml
apiVersion: v1
kind: Service
metadata:
  name: external-service
spec:
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
  externalTrafficPolicy: Local  # Preserves source IP
  # externalTrafficPolicy: Cluster  # Default, may SNAT
```

---

## Port Forwarding

### kubectl Port Forward

```bash
# Forward local port to pod
kubectl port-forward pod/my-pod 8080:80

# Forward to service
kubectl port-forward service/my-service 8080:80

# Forward to deployment
kubectl port-forward deployment/my-deployment 8080:80

# Bind to all interfaces
kubectl port-forward --address 0.0.0.0 pod/my-pod 8080:80

# Background port forward
kubectl port-forward pod/my-pod 8080:80 &
```

### Multi-Port Forwarding

```bash
# Forward multiple ports
kubectl port-forward pod/my-pod 8080:80 9090:9090
```

---

## Troubleshooting Networking

### Basic Network Debugging

```bash
# Check services
kubectl get services
kubectl describe service my-service

# Check endpoints
kubectl get endpoints
kubectl describe endpoints my-service

# Check network policies
kubectl get networkpolicies
kubectl describe networkpolicy my-netpol
```

### DNS Troubleshooting

```bash
# Test DNS from pod
kubectl exec -it my-pod -- nslookup kubernetes.default
kubectl exec -it my-pod -- nslookup my-service

# Check DNS configuration
kubectl exec -it my-pod -- cat /etc/resolv.conf

# Test connectivity
kubectl exec -it my-pod -- nc -zv my-service 80
kubectl exec -it my-pod -- wget -qO- http://my-service
```

### Network Debugging Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: network-debug-pod
spec:
  containers:
  - name: netshoot
    image: nicolaka/netshoot
    command: ["/bin/bash"]
    args: ["-c", "while true; do sleep 30; done"]
    securityContext:
      capabilities:
        add: ["NET_ADMIN", "NET_RAW"]
```

### Connectivity Testing

```bash
# Inside debug pod
# Test DNS
nslookup google.com
dig kubernetes.default.svc.cluster.local

# Test connectivity
nc -zv service-name 80
telnet service-name 80

# Test HTTP
curl -v http://service-name
wget -qO- http://service-name

# Network tools
ss -tulpn
netstat -tulpn
ip route
ip addr
```

### Service Troubleshooting Checklist

1. **Service exists and has correct selector**
   ```bash
   kubectl get service my-service
   kubectl describe service my-service
   ```

2. **Pods match service selector**
   ```bash
   kubectl get pods -l app=my-app
   kubectl get endpoints my-service
   ```

3. **Pods are ready and healthy**
   ```bash
   kubectl get pods -o wide
   kubectl describe pod my-pod
   ```

4. **Network policies allow traffic**
   ```bash
   kubectl get networkpolicies
   kubectl describe networkpolicy my-netpol
   ```

5. **DNS resolution works**
   ```bash
   kubectl exec -it test-pod -- nslookup my-service
   ```

---

## Advanced Networking Concepts

### Multi-Port Services

```yaml
apiVersion: v1
kind: Service
metadata:
  name: multi-port-service
spec:
  selector:
    app: multi-port-app
  ports:
  - name: http
    port: 80
    targetPort: 8080
    protocol: TCP
  - name: https
    port: 443
    targetPort: 8443
    protocol: TCP
  - name: grpc
    port: 9090
    targetPort: 9090
    protocol: TCP
```

### Service with Multiple Selectors

```yaml
apiVersion: v1
kind: Service
metadata:
  name: multi-selector-service
spec:
  selector:
    app: web-app
    version: stable
  ports:
  - port: 80
    targetPort: 8080
```

### External IPs

```yaml
apiVersion: v1
kind: Service
metadata:
  name: external-ip-service
spec:
  selector:
    app: web-app
  ports:
  - port: 80
    targetPort: 8080
  externalIPs:
  - 192.168.1.100
  - 192.168.1.101
```

---

## Exam Tips and Common Scenarios

### Quick Service Creation

```bash
# Expose deployment as service
kubectl expose deployment my-deployment --port=80 --target-port=8080

# Create service with specific type
kubectl expose deployment my-deployment --type=NodePort --port=80

# Create service imperatively
kubectl create service clusterip my-service --tcp=80:8080

# Generate service YAML
kubectl expose deployment my-deployment --port=80 --dry-run=client -o yaml
```

### Common Ingress Patterns

```bash
# Create basic ingress
kubectl create ingress my-ingress --rule="example.com/*=my-service:80"

# Generate ingress YAML
kubectl create ingress my-ingress --rule="example.com/*=my-service:80" --dry-run=client -o yaml
```

### Networking Verification

```bash
# Quick connectivity test
kubectl run test-pod --image=busybox:1.35 -it --rm -- sh
# Inside pod: wget -qO- http://service-name

# Check service endpoints
kubectl get endpoints service-name

# Test specific port
kubectl exec -it test-pod -- nc -zv service-name 80
```

### Network Policy Testing

```bash
# Test before applying policy
kubectl exec -it source-pod -- curl http://target-service

# Apply network policy
kubectl apply -f network-policy.yaml

# Test after applying policy
kubectl exec -it source-pod -- curl http://target-service
```

This comprehensive guide covers all essential networking concepts for CKAD Section 5. Master these service and networking patterns to effectively manage application connectivity in Kubernetes.