# CKAD Section 5: Services and Networking - Practice Exercises

This file contains hands-on exercises to practice Kubernetes services, networking, ingress, and network policies.

## Exercise 1: Basic Services

### Task 1.1: ClusterIP Service
1. Create a deployment named `web-app` with 3 replicas using `nginx:1.21`
2. Expose it as a ClusterIP service on port 80
3. Test connectivity from another pod

**Commands:**
```bash
kubectl create deployment web-app --image=nginx:1.21 --replicas=3
kubectl expose deployment web-app --port=80 --target-port=80
kubectl run test-pod --image=busybox:1.35 -it --rm -- sh
# Inside pod: wget -qO- http://web-app
```

### Task 1.2: NodePort Service
1. Create a NodePort service for the web-app deployment
2. Access the service from outside the cluster
3. Verify the service is accessible on all nodes

**Solution:**
```bash
kubectl expose deployment web-app --type=NodePort --port=80 --target-port=80 --name=web-app-nodeport
kubectl get service web-app-nodeport
# Access via http://node-ip:node-port
```

### Task 1.3: LoadBalancer Service
1. Create a LoadBalancer service for the web-app
2. Wait for external IP assignment
3. Test external connectivity

### Task 1.4: Headless Service
1. Create a headless service for a StatefulSet
2. Test DNS resolution returning pod IPs
3. Verify individual pod DNS names

**Solution:**
```yaml
apiVersion: v1
kind: Service
metadata:
  name: headless-service
spec:
  clusterIP: None
  selector:
    app: web-app
  ports:
  - port: 80
```

---

## Exercise 2: Service Discovery

### Task 2.1: DNS Resolution
1. Create a service and test various DNS name formats
2. Test from pods in same and different namespaces
3. Verify environment variables are created

**DNS Testing:**
```bash
kubectl exec -it test-pod -- nslookup web-app
kubectl exec -it test-pod -- nslookup web-app.default
kubectl exec -it test-pod -- nslookup web-app.default.svc.cluster.local
```

### Task 2.2: Environment Variables
1. Create a service
2. Create a pod and check service-related environment variables
3. Understand the variable naming pattern

**Commands:**
```bash
kubectl run env-test --image=busybox:1.35 -- sleep 3600
kubectl exec env-test -- env | grep WEB_APP
```

### Task 2.3: Cross-Namespace Discovery
1. Create services in different namespaces
2. Test cross-namespace service discovery
3. Use fully qualified domain names

---

## Exercise 3: Advanced Service Configuration

### Task 3.1: Multi-Port Service
Create a service that exposes multiple ports:
- HTTP on port 80 → target port 8080
- HTTPS on port 443 → target port 8443
- Metrics on port 9090 → target port 9090

**Solution:**
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
  - name: https
    port: 443
    targetPort: 8443
  - name: metrics
    port: 9090
    targetPort: 9090
```

### Task 3.2: Session Affinity
1. Create a service with session affinity
2. Test that requests from same client go to same pod
3. Configure session timeout

### Task 3.3: External Name Service
1. Create an ExternalName service pointing to an external database
2. Test that pods can connect using the service name
3. Update the external name and verify changes

### Task 3.4: Service without Selector
1. Create a service without pod selector
2. Manually create endpoints pointing to external IPs
3. Test connectivity through the service

---

## Exercise 4: Ingress

### Task 4.1: Basic Ingress
1. Create an ingress that routes traffic to your web service
2. Configure host-based routing
3. Test the ingress (if ingress controller is available)

**Solution:**
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
            name: web-app
            port:
              number: 80
```

### Task 4.2: Path-based Routing
Create an ingress with path-based routing:
- `/app1` → app1-service
- `/app2` → app2-service
- `/api` → api-service

### Task 4.3: Host-based Routing
Create an ingress with host-based routing:
- `app1.example.com` → app1-service
- `app2.example.com` → app2-service
- `api.example.com` → api-service

### Task 4.4: TLS Ingress
1. Create a TLS secret with certificate and key
2. Create an ingress that uses TLS
3. Configure HTTPS redirection

**TLS Secret Creation:**
```bash
# Generate self-signed certificate (for testing)
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout tls.key -out tls.crt -subj "/CN=example.com"

kubectl create secret tls tls-secret --cert=tls.crt --key=tls.key
```

---

## Exercise 5: Network Policies

### Task 5.1: Default Deny All
1. Create a namespace called `secure-ns`
2. Apply a default deny-all network policy
3. Create two pods and verify they cannot communicate

**Solution:**
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-all
  namespace: secure-ns
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
```

### Task 5.2: Allow Specific Communication
1. In the secure namespace, create pods with labels `app=frontend` and `app=backend`
2. Create a network policy allowing frontend to communicate with backend on port 8080
3. Test the policy works correctly

### Task 5.3: Cross-Namespace Policy
1. Create pods in different namespaces
2. Create a network policy allowing communication between specific namespaces
3. Test allowed and denied connections

### Task 5.4: Database Access Policy
Create a network policy that:
- Allows ingress to database pods only from backend pods
- Allows egress from database pods only for DNS and backup
- Denies all other traffic

**Solution:**
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: database-policy
spec:
  podSelector:
    matchLabels:
      app: database
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: backend
    ports:
    - protocol: TCP
      port: 5432
  egress:
  - to: []
    ports:
    - protocol: UDP
      port: 53
  - to:
    - podSelector:
        matchLabels:
          app: backup
    ports:
    - protocol: TCP
      port: 3000
```

---

## Exercise 6: Endpoints and EndpointSlices

### Task 6.1: Manual Endpoints
1. Create a service without a selector
2. Manually create endpoints pointing to external servers
3. Test connectivity through the service

### Task 6.2: Endpoint Troubleshooting
1. Create a service with incorrect selector
2. Identify why endpoints are empty
3. Fix the selector and verify endpoints are populated

### Task 6.3: EndpointSlice Management
1. Examine EndpointSlices for existing services
2. Understand the relationship between services and EndpointSlices
3. Monitor changes when pods are added/removed

**Commands:**
```bash
kubectl get endpoints
kubectl get endpointslices
kubectl describe endpointslice <endpointslice-name>
```

---

## Exercise 7: Load Balancing and Traffic Management

### Task 7.1: Load Balancing Verification
1. Create a service with multiple backend pods
2. Make multiple requests and verify load distribution
3. Scale the deployment and observe load balancing changes

**Testing Script:**
```bash
for i in {1..20}; do
  kubectl exec test-pod -- wget -qO- http://web-app | grep hostname
done
```

### Task 7.2: Session Affinity
1. Configure a service with session affinity
2. Test that requests from the same source go to the same pod
3. Compare behavior with and without session affinity

### Task 7.3: External Traffic Policy
1. Create a LoadBalancer service with `externalTrafficPolicy: Local`
2. Compare with `externalTrafficPolicy: Cluster`
3. Understand the impact on source IP preservation

---

## Exercise 8: DNS and Service Names

### Task 8.1: DNS Resolution Testing
1. Create services in multiple namespaces
2. Test various DNS name formats
3. Verify DNS from different pod locations

### Task 8.2: Custom DNS Configuration
1. Create a pod with custom DNS configuration
2. Test resolution behavior with different settings
3. Configure DNS search domains

**Custom DNS Pod:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: custom-dns-pod
spec:
  containers:
  - name: test
    image: busybox:1.35
    command: ['sleep', '3600']
  dnsPolicy: ClusterFirst
  dnsConfig:
    nameservers:
    - 1.1.1.1
    searches:
    - custom.local
    options:
    - name: ndots
      value: "2"
```

### Task 8.3: Headless Service DNS
1. Create a headless service for a StatefulSet
2. Test that DNS returns individual pod IPs
3. Verify pod-specific DNS names work

---

## Exercise 9: Network Troubleshooting

### Task 9.1: Service Connectivity Issues
Debug this scenario:
- Service exists but pods can't connect
- Service has no endpoints
- DNS resolution fails

**Debugging Steps:**
```bash
kubectl get services
kubectl get endpoints
kubectl describe service <service-name>
kubectl get pods --show-labels
kubectl exec -it test-pod -- nslookup <service-name>
```

### Task 9.2: Ingress Troubleshooting
Debug common ingress issues:
- Ingress created but not accessible
- Wrong backend service
- TLS certificate problems

### Task 9.3: Network Policy Debugging
Debug network policy issues:
- Policy applied but traffic still blocked/allowed
- Understanding policy precedence
- Testing policy changes

**Network Policy Testing:**
```bash
# Test before policy
kubectl exec source-pod -- curl http://target-service

# Apply policy
kubectl apply -f network-policy.yaml

# Test after policy
kubectl exec source-pod -- curl http://target-service
```

---

## Exercise 10: Advanced Networking

### Task 10.1: Multi-Cluster Services
1. Set up services that can be accessed across clusters
2. Configure DNS for cross-cluster discovery
3. Test connectivity between clusters

### Task 10.2: Service Mesh Integration
1. Deploy a simple service mesh (if available)
2. Configure services with mesh features
3. Test advanced routing and policies

### Task 10.3: CNI and Network Plugins
1. Examine the current CNI configuration
2. Understand pod networking setup
3. Test network connectivity at different layers

---

## Exam Simulation Exercises

### Scenario 1: Service Exposure (4 minutes)
1. Create deployment `exam-app` with 3 replicas using `nginx:1.21`
2. Expose it as ClusterIP service on port 80
3. Create NodePort service on port 30080
4. Test both services work correctly

### Scenario 2: Ingress Setup (5 minutes)
1. Create two services: `app1-service` and `app2-service`
2. Create ingress with path-based routing:
   - `/app1` → app1-service
   - `/app2` → app2-service
3. Verify ingress configuration

### Scenario 3: Network Policy (6 minutes)
1. Create namespace `secure-app`
2. Apply default deny-all network policy
3. Create policy allowing specific pod-to-pod communication
4. Test policy enforcement

### Scenario 4: Service Discovery (3 minutes)
1. Create service in namespace `backend`
2. Create pod in namespace `frontend`
3. Test cross-namespace service discovery using FQDN

### Scenario 5: Troubleshooting (4 minutes)
A service named `broken-service` is not working:
1. Identify why pods can't connect to the service
2. Check service configuration and endpoints
3. Fix the issues

---

## Solutions and Best Practices

### Exercise 1 Solutions

**Task 1.4 Headless Service Testing:**
```bash
# Create StatefulSet for testing
kubectl create statefulset web --image=nginx:1.21 --replicas=3

# Test DNS resolution
kubectl exec -it test-pod -- nslookup headless-service
kubectl exec -it test-pod -- nslookup web-0.headless-service
kubectl exec -it test-pod -- nslookup web-1.headless-service
```

### Exercise 4 Solutions

**Task 4.2 Path-based Routing:**
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: path-based-ingress
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /$2
spec:
  rules:
  - host: example.com
    http:
      paths:
      - path: /app1(/|$)(.*)
        pathType: Prefix
        backend:
          service:
            name: app1-service
            port:
              number: 80
      - path: /app2(/|$)(.*)
        pathType: Prefix
        backend:
          service:
            name: app2-service
            port:
              number: 80
```

### Exercise 5 Solutions

**Task 5.2 Allow Specific Communication:**
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-frontend-to-backend
  namespace: secure-ns
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

---

## Common Commands Reference

### Service Commands
```bash
# Create service
kubectl expose deployment my-app --port=80 --target-port=8080
kubectl create service clusterip my-service --tcp=80:8080

# Get services
kubectl get services
kubectl get svc -o wide

# Describe service
kubectl describe service my-service

# Test service
kubectl run test --image=busybox:1.35 -it --rm -- sh
```

### Ingress Commands
```bash
# Create ingress
kubectl create ingress my-ingress --rule="example.com/*=my-service:80"

# Get ingress
kubectl get ingress
kubectl describe ingress my-ingress

# Generate YAML
kubectl create ingress my-ingress --rule="example.com/*=my-service:80" --dry-run=client -o yaml
```

### Network Policy Commands
```bash
# Get network policies
kubectl get networkpolicies
kubectl get netpol

# Describe policy
kubectl describe networkpolicy my-policy

# Test connectivity
kubectl exec -it source-pod -- curl target-service
kubectl exec -it source-pod -- nc -zv target-service 80
```

### Troubleshooting Commands
```bash
# Check endpoints
kubectl get endpoints
kubectl describe endpoints my-service

# DNS testing
kubectl exec -it test-pod -- nslookup my-service
kubectl exec -it test-pod -- dig my-service

# Connectivity testing
kubectl exec -it test-pod -- wget -qO- http://my-service
kubectl exec -it test-pod -- curl -v http://my-service
kubectl exec -it test-pod -- nc -zv my-service 80
```

### Network Debugging
```bash
# Network debug pod
kubectl run netshoot --image=nicolaka/netshoot -it --rm -- bash

# Inside netshoot pod:
nslookup my-service
dig my-service
curl -v http://my-service
nc -zv my-service 80
ss -tulpn
ip route
```

Remember: Practice these networking scenarios extensively to master service discovery, ingress configuration, and network policy implementation for the CKAD exam!