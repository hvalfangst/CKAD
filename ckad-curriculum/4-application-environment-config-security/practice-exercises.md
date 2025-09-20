# CKAD Section 4: Application Environment, Configuration and Security - Practice Exercises

This file contains hands-on exercises to practice configuration management, security contexts, RBAC, and environmental aspects of Kubernetes applications.

## Exercise 1: ConfigMaps and Secrets

### Task 1.1: Create and Use ConfigMap
1. Create a ConfigMap named `app-config` with the following data:
   - `database_url`: `postgres://localhost:5432/mydb`
   - `log_level`: `info`
   - `debug_mode`: `false`
2. Create a pod that uses these values as environment variables
3. Verify the environment variables are set correctly

**Commands:**
```bash
kubectl create configmap app-config \
  --from-literal=database_url=postgres://localhost:5432/mydb \
  --from-literal=log_level=info \
  --from-literal=debug_mode=false

kubectl run config-pod --image=busybox:1.35 --dry-run=client -o yaml -- sleep 3600 > config-pod.yaml
# Edit to add env from configmap
kubectl apply -f config-pod.yaml
kubectl exec config-pod -- env | grep -E "(database_url|log_level|debug_mode)"
```

### Task 1.2: ConfigMap from File
1. Create a configuration file `app.properties`
2. Create a ConfigMap from this file
3. Mount the ConfigMap as a volume in a pod

### Task 1.3: Create and Use Secrets
1. Create a Secret named `db-secret` with:
   - `username`: `admin`
   - `password`: `secret123`
2. Create a pod that uses these as environment variables
3. Create another pod that mounts the secret as a file

**Commands:**
```bash
kubectl create secret generic db-secret \
  --from-literal=username=admin \
  --from-literal=password=secret123

# Verify secret
kubectl get secret db-secret -o yaml
```

### Task 1.4: TLS Secret
1. Generate a self-signed certificate
2. Create a TLS secret
3. Mount the TLS secret in an nginx pod

**Solution:**
```bash
# Generate certificate
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout tls.key -out tls.crt -subj "/CN=example.com"

# Create TLS secret
kubectl create secret tls tls-secret --cert=tls.crt --key=tls.key
```

---

## Exercise 2: Environment Variables

### Task 2.1: Multiple Environment Sources
Create a pod that gets environment variables from:
1. Direct values
2. ConfigMap
3. Secret
4. Field references (pod name, namespace, node name)
5. Resource field references (CPU/memory requests)

**Solution Template:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: multi-env-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'env | sort && sleep 3600']
    env:
    - name: DIRECT_VAR
      value: "direct-value"
    - name: POD_NAME
      valueFrom:
        fieldRef:
          fieldPath: metadata.name
    # Add more environment variables
    resources:
      requests:
        cpu: 100m
        memory: 128Mi
```

### Task 2.2: Environment Variable Precedence
1. Create overlapping environment variables from different sources
2. Understand which values take precedence
3. Test with conflicting variable names

### Task 2.3: Environment from Multiple ConfigMaps
1. Create two ConfigMaps with different data
2. Use both in a single pod with prefixes
3. Verify all variables are available

---

## Exercise 3: Security Contexts

### Task 3.1: Pod Security Context
Create a pod with the following security context:
- Run as user ID 1000
- Run as group ID 2000
- Run as non-root
- Set fsGroup to 3000
- Use runtime default seccomp profile

**Solution:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: security-pod
spec:
  securityContext:
    runAsUser: 1000
    runAsGroup: 2000
    runAsNonRoot: true
    fsGroup: 3000
    seccompProfile:
      type: RuntimeDefault
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'id && sleep 3600']
```

### Task 3.2: Container Security Context
Create a pod where:
- Pod runs as user 1000
- Container overrides to run as user 2000
- Container has read-only root filesystem
- Container drops all capabilities

### Task 3.3: Capabilities Management
Create a pod that:
- Drops all capabilities
- Adds only NET_BIND_SERVICE capability
- Verifies the container can bind to port 80

### Task 3.4: Privileged vs Unprivileged
1. Create a privileged container
2. Create an unprivileged container with security restrictions
3. Compare the capabilities and access levels

---

## Exercise 4: Service Accounts and RBAC

### Task 4.1: Custom Service Account
1. Create a ServiceAccount named `app-sa`
2. Create a pod that uses this ServiceAccount
3. Verify the ServiceAccount token is mounted

**Commands:**
```bash
kubectl create serviceaccount app-sa
kubectl get serviceaccount app-sa -o yaml
```

### Task 4.2: RBAC Setup
1. Create a Role that allows:
   - Get, list, watch pods
   - Get pod logs
2. Create a RoleBinding to bind the role to your ServiceAccount
3. Test the permissions

**Solution:**
```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: default
  name: pod-reader
rules:
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "list", "watch"]
- apiGroups: [""]
  resources: ["pods/log"]
  verbs: ["get"]
```

### Task 4.3: Test RBAC Permissions
Test the ServiceAccount permissions:
```bash
kubectl auth can-i get pods --as=system:serviceaccount:default:app-sa
kubectl auth can-i create pods --as=system:serviceaccount:default:app-sa
kubectl auth can-i --list --as=system:serviceaccount:default:app-sa
```

### Task 4.4: ClusterRole and ClusterRoleBinding
1. Create a ClusterRole for reading nodes
2. Create a ClusterRoleBinding
3. Test cross-namespace access

---

## Exercise 5: Network Policies

### Task 5.1: Default Deny Policy
1. Create a namespace `secure-ns`
2. Apply a default deny-all NetworkPolicy
3. Test that pods cannot communicate

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

### Task 5.2: Allow Specific Traffic
1. Create two pods in the secure namespace
2. Create a NetworkPolicy allowing traffic between them
3. Test connectivity

### Task 5.3: Cross-Namespace Communication
1. Create pods in different namespaces
2. Create NetworkPolicy allowing specific cross-namespace traffic
3. Verify the policy works correctly

### Task 5.4: Complex Network Policy
Create a NetworkPolicy that:
- Allows ingress on port 8080 from pods with label `app=frontend`
- Allows egress to pods with label `app=database` on port 5432
- Allows DNS (port 53)

---

## Exercise 6: Resource Management

### Task 6.1: Resource Quotas
1. Create a namespace with ResourceQuota limiting:
   - CPU requests: 2 cores
   - Memory requests: 4Gi
   - Number of pods: 10
2. Try to create pods that exceed the quota
3. Observe the behavior

**Solution:**
```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: test-quota
  namespace: test-ns
spec:
  hard:
    requests.cpu: "2"
    requests.memory: 4Gi
    pods: "10"
```

### Task 6.2: Limit Ranges
1. Create a LimitRange with default resource requests/limits
2. Create a pod without specifying resources
3. Verify default values are applied

### Task 6.3: Resource Quota Enforcement
1. Create a pod that exceeds CPU limits
2. Create a pod that exceeds memory limits
3. Understand the error messages

---

## Exercise 7: Persistent Storage

### Task 7.1: EmptyDir Volume
1. Create a pod with two containers sharing an emptyDir volume
2. Have one container write to the shared volume
3. Have the other container read from it

### Task 7.2: HostPath Volume
1. Create a pod that mounts a host directory
2. Write a file from the container
3. Verify the file exists on the host

### Task 7.3: Persistent Volume Claim
1. Create a PVC requesting 1Gi of storage
2. Create a pod that uses the PVC
3. Write data to the persistent volume
4. Delete and recreate the pod
5. Verify data persists

**Solution:**
```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: my-pvc
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```

---

## Exercise 8: Advanced Configuration

### Task 8.1: Configuration Versioning
1. Create a ConfigMap with version v1
2. Create a pod using it
3. Create ConfigMap v2 with different values
4. Update the pod to use v2
5. Implement rolling update strategy

### Task 8.2: Immutable ConfigMaps
1. Create an immutable ConfigMap
2. Try to modify it
3. Understand the benefits and limitations

### Task 8.3: Projected Volumes
Create a pod with projected volume containing:
- ConfigMap data
- Secret data
- ServiceAccount token

**Solution:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: projected-volume-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ['sleep', '3600']
    volumeMounts:
    - name: projected-vol
      mountPath: /projected
  volumes:
  - name: projected-vol
    projected:
      sources:
      - configMap:
          name: app-config
      - secret:
          name: app-secret
      - serviceAccountToken:
          path: token
          expirationSeconds: 7200
```

---

## Exercise 9: Security Best Practices

### Task 9.1: Secure Pod Template
Create a pod template that follows security best practices:
- Non-root user
- Read-only root filesystem
- No privilege escalation
- Dropped capabilities
- Resource limits

### Task 9.2: Pod Security Standards
1. Create a namespace with restricted pod security standards
2. Try to create pods that violate the standards
3. Create compliant pods

### Task 9.3: Security Scanning
1. Create a pod with known security issues
2. Use tools to scan for vulnerabilities
3. Fix the identified issues

---

## Exercise 10: Troubleshooting Configuration

### Task 10.1: ConfigMap Issues
Debug these common ConfigMap problems:
- Missing keys
- Incorrect volume mounts
- Environment variable conflicts

### Task 10.2: Secret Issues
Debug these Secret problems:
- Base64 encoding issues
- Mount permission problems
- Image pull secret failures

### Task 10.3: RBAC Issues
Debug RBAC problems:
- Missing permissions
- Incorrect ServiceAccount
- Wrong namespace bindings

---

## Exam Simulation Exercises

### Scenario 1: Configuration Setup (8 minutes)
1. Create a ConfigMap named `web-config` with:
   - `server.port`: `8080`
   - `log.level`: `info`
2. Create a Secret named `web-secret` with:
   - `api.key`: `abc123`
   - `db.password`: `secret`
3. Create a deployment using both ConfigMap and Secret

### Scenario 2: Security Context (5 minutes)
Create a pod that:
- Runs as user 1000, group 2000
- Has read-only root filesystem
- Drops all capabilities except NET_BIND_SERVICE
- Uses a non-root user

### Scenario 3: RBAC Setup (6 minutes)
1. Create ServiceAccount `monitor-sa`
2. Create Role allowing read access to pods and services
3. Create RoleBinding connecting them
4. Test permissions

### Scenario 4: Network Policy (4 minutes)
Create a NetworkPolicy that:
- Applies to pods with label `app=database`
- Allows ingress only from pods with label `app=backend`
- Allows egress only for DNS

### Scenario 5: Resource Management (3 minutes)
1. Create namespace with ResourceQuota limiting 2 CPU, 4Gi memory
2. Create LimitRange with default 100m CPU, 128Mi memory
3. Create a pod and verify defaults are applied

---

## Solutions and Best Practices

### Exercise 1 Solutions

**Task 1.2 ConfigMap from File:**
```bash
# Create config file
cat > app.properties << EOF
database.host=localhost
database.port=5432
app.name=MyApp
EOF

# Create ConfigMap
kubectl create configmap file-config --from-file=app.properties

# Use in pod
kubectl run file-pod --image=busybox:1.35 --dry-run=client -o yaml -- sleep 3600 > file-pod.yaml
# Edit to add volume mount
```

### Exercise 3 Solutions

**Task 3.2 Container Override:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: override-security-pod
spec:
  securityContext:
    runAsUser: 1000
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'id && sleep 3600']
    securityContext:
      runAsUser: 2000
      readOnlyRootFilesystem: true
      capabilities:
        drop: ["ALL"]
```

### Exercise 4 Solutions

**Task 4.2 RBAC Complete Example:**
```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: read-pods-binding
  namespace: default
subjects:
- kind: ServiceAccount
  name: app-sa
  namespace: default
roleRef:
  kind: Role
  name: pod-reader
  apiGroup: rbac.authorization.k8s.io
```

---

## Common Commands Reference

### ConfigMap Commands
```bash
# Create from literals
kubectl create configmap my-config --from-literal=key1=value1

# Create from file
kubectl create configmap my-config --from-file=config.txt

# Create from directory
kubectl create configmap my-config --from-file=config-dir/

# View ConfigMap
kubectl get configmap my-config -o yaml
```

### Secret Commands
```bash
# Create generic secret
kubectl create secret generic my-secret --from-literal=password=secret

# Create TLS secret
kubectl create secret tls tls-secret --cert=cert.pem --key=key.pem

# Create Docker registry secret
kubectl create secret docker-registry reg-secret \
  --docker-server=registry.com \
  --docker-username=user \
  --docker-password=pass
```

### RBAC Commands
```bash
# Create ServiceAccount
kubectl create serviceaccount my-sa

# Create Role
kubectl create role pod-reader --verb=get,list,watch --resource=pods

# Create RoleBinding
kubectl create rolebinding read-pods --role=pod-reader --serviceaccount=default:my-sa

# Test permissions
kubectl auth can-i get pods --as=system:serviceaccount:default:my-sa
```

### Security Commands
```bash
# Check pod security context
kubectl describe pod my-pod | grep -A10 "Security Context"

# View effective permissions
kubectl auth can-i --list --as=system:serviceaccount:default:my-sa

# Check resource usage
kubectl top pods
kubectl describe quota
```

Remember: Practice these configuration and security scenarios extensively to master this important section of the CKAD exam!