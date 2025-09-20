# CKAD Section 4: Application Environment, Configuration and Security (25%)

This section covers configuration management, security contexts, secrets management, and environmental aspects of Kubernetes applications. This is the largest section of the CKAD exam.

## Table of Contents

1. [ConfigMaps](#configmaps)
2. [Secrets](#secrets)
3. [Environment Variables](#environment-variables)
4. [Security Contexts](#security-contexts)
5. [Service Accounts](#service-accounts)
6. [RBAC (Role-Based Access Control)](#rbac-role-based-access-control)
7. [Network Policies](#network-policies)
8. [Resource Quotas and Limits](#resource-quotas-and-limits)
9. [Pod Security Standards](#pod-security-standards)
10. [Volume Mounts and Persistent Storage](#volume-mounts-and-persistent-storage)
11. [Admission Controllers](#admission-controllers)
12. [Best Practices](#best-practices)

---

## ConfigMaps

### Overview
ConfigMaps store non-confidential configuration data in key-value pairs. They allow you to decouple configuration from container images.

### Creating ConfigMaps

```bash
# From literal values
kubectl create configmap app-config --from-literal=database_url=postgres://localhost:5432/db --from-literal=log_level=info

# From file
kubectl create configmap app-config --from-file=config.properties

# From directory
kubectl create configmap app-config --from-file=config-dir/

# From env file
kubectl create configmap app-config --from-env-file=app.env
```

### ConfigMap YAML

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
  namespace: default
data:
  # Simple key-value pairs
  database_url: "postgres://localhost:5432/mydb"
  log_level: "info"
  debug_mode: "false"

  # Multi-line data
  app.properties: |
    database.url=postgres://localhost:5432/mydb
    database.driver=org.postgresql.Driver
    app.name=MyApplication

  nginx.conf: |
    server {
        listen 80;
        server_name localhost;
        location / {
            root /usr/share/nginx/html;
            index index.html;
        }
    }
```

### Using ConfigMaps in Pods

#### As Environment Variables

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: configmap-env-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'echo "Database: $DATABASE_URL, Log Level: $LOG_LEVEL" && sleep 3600']
    env:
    # Single environment variable
    - name: DATABASE_URL
      valueFrom:
        configMapKeyRef:
          name: app-config
          key: database_url
    # Multiple environment variables
    envFrom:
    - configMapRef:
        name: app-config
```

#### As Volume Mounts

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: configmap-volume-pod
spec:
  containers:
  - name: app
    image: nginx:1.21
    volumeMounts:
    - name: config-volume
      mountPath: /etc/config
    - name: nginx-config
      mountPath: /etc/nginx/conf.d
  volumes:
  - name: config-volume
    configMap:
      name: app-config
  - name: nginx-config
    configMap:
      name: app-config
      items:
      - key: nginx.conf
        path: default.conf
```

### ConfigMap Best Practices

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config-versioned
  labels:
    version: "v1.2.0"
    app: myapp
data:
  config.yaml: |
    app:
      name: "MyApplication"
      version: "1.2.0"
      environment: "production"
    database:
      host: "postgres.example.com"
      port: 5432
      name: "myapp_prod"
    features:
      new_ui: true
      beta_features: false
    logging:
      level: "info"
      format: "json"
```

---

## Secrets

### Overview
Secrets store sensitive information such as passwords, OAuth tokens, SSH keys, and TLS certificates.

### Creating Secrets

```bash
# From literal values
kubectl create secret generic app-secret --from-literal=username=admin --from-literal=password=secret123

# From files
kubectl create secret generic app-secret --from-file=username.txt --from-file=password.txt

# TLS secret
kubectl create secret tls tls-secret --cert=cert.pem --key=key.pem

# Docker registry secret
kubectl create secret docker-registry registry-secret --docker-server=registry.example.com --docker-username=user --docker-password=pass --docker-email=user@example.com
```

### Secret Types

#### Generic Secret

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: app-secret
type: Opaque
data:
  username: YWRtaW4=        # base64 encoded "admin"
  password: c2VjcmV0MTIz    # base64 encoded "secret123"
  api-key: YWJjZGVmZ2g=     # base64 encoded "abcdefgh"
stringData:
  config.yaml: |            # Automatically base64 encoded
    database:
      host: db.example.com
      port: 5432
```

#### TLS Secret

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: tls-secret
type: kubernetes.io/tls
data:
  tls.crt: LS0tLS1CRUdJTi... # base64 encoded certificate
  tls.key: LS0tLS1CRUdJTi... # base64 encoded private key
```

#### Docker Registry Secret

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: registry-secret
type: kubernetes.io/dockerconfigjson
data:
  .dockerconfigjson: eyJhdXRocyI6eyJyZWdpc3RyeS5leGFtcGxlLmNvbSI6eyJ1c2VybmFtZSI6InVzZXIiLCJwYXNzd29yZCI6InBhc3MiLCJlbWFpbCI6InVzZXJAZXhhbXBsZS5jb20iLCJhdXRoIjoiZFhObGNqcHdZWE56In19fQ==
```

### Using Secrets in Pods

#### As Environment Variables

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: secret-env-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'echo "User: $USERNAME, API Key: $API_KEY" && sleep 3600']
    env:
    - name: USERNAME
      valueFrom:
        secretKeyRef:
          name: app-secret
          key: username
    - name: PASSWORD
      valueFrom:
        secretKeyRef:
          name: app-secret
          key: password
    envFrom:
    - secretRef:
        name: app-secret
```

#### As Volume Mounts

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: secret-volume-pod
spec:
  containers:
  - name: app
    image: nginx:1.21
    volumeMounts:
    - name: secret-volume
      mountPath: /etc/secrets
      readOnly: true
    - name: tls-volume
      mountPath: /etc/tls
      readOnly: true
  volumes:
  - name: secret-volume
    secret:
      secretName: app-secret
      defaultMode: 0400
  - name: tls-volume
    secret:
      secretName: tls-secret
      items:
      - key: tls.crt
        path: cert.pem
      - key: tls.key
        path: key.pem
        mode: 0600
```

#### Image Pull Secrets

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: private-image-pod
spec:
  containers:
  - name: app
    image: registry.example.com/myapp:latest
  imagePullSecrets:
  - name: registry-secret
```

---

## Environment Variables

### Direct Environment Variables

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: env-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'env && sleep 3600']
    env:
    - name: SIMPLE_VAR
      value: "simple-value"
    - name: NODE_NAME
      valueFrom:
        fieldRef:
          fieldPath: spec.nodeName
    - name: POD_NAME
      valueFrom:
        fieldRef:
          fieldPath: metadata.name
    - name: POD_IP
      valueFrom:
        fieldRef:
          fieldPath: status.podIP
    - name: CPU_REQUEST
      valueFrom:
        resourceFieldRef:
          containerName: app
          resource: requests.cpu
```

### Environment Variables from Multiple Sources

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
    - name: CONFIG_VAR
      valueFrom:
        configMapKeyRef:
          name: app-config
          key: log_level
    - name: SECRET_VAR
      valueFrom:
        secretKeyRef:
          name: app-secret
          key: api-key
    envFrom:
    - prefix: CONFIG_
      configMapRef:
        name: app-config
    - prefix: SECRET_
      secretRef:
        name: app-secret
```

---

## Security Contexts

### Pod Security Context

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: security-context-pod
spec:
  securityContext:
    runAsUser: 1000
    runAsGroup: 3000
    runAsNonRoot: true
    fsGroup: 2000
    fsGroupChangePolicy: "OnRootMismatch"
    seccompProfile:
      type: RuntimeDefault
    supplementalGroups: [4000]
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'id && ls -la /tmp && sleep 3600']
    securityContext:
      allowPrivilegeEscalation: false
      readOnlyRootFilesystem: true
      runAsUser: 2000
      capabilities:
        add: ["NET_ADMIN"]
        drop: ["ALL"]
    volumeMounts:
    - name: tmp-volume
      mountPath: /tmp
  volumes:
  - name: tmp-volume
    emptyDir: {}
```

### Container Security Context

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: container-security-pod
spec:
  containers:
  - name: secure-app
    image: nginx:1.21
    securityContext:
      runAsUser: 1001
      runAsGroup: 1001
      runAsNonRoot: true
      readOnlyRootFilesystem: true
      allowPrivilegeEscalation: false
      capabilities:
        drop:
        - ALL
        add:
        - CHOWN
        - SETUID
        - SETGID
      seLinuxOptions:
        level: "s0:c123,c456"
    volumeMounts:
    - name: cache
      mountPath: /var/cache/nginx
    - name: run
      mountPath: /var/run
  volumes:
  - name: cache
    emptyDir: {}
  - name: run
    emptyDir: {}
```

### Privileged and Unprivileged Containers

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: privilege-demo
spec:
  containers:
  # Unprivileged container (recommended)
  - name: unprivileged
    image: busybox:1.35
    command: ['sh', '-c', 'whoami && sleep 3600']
    securityContext:
      runAsUser: 1000
      runAsNonRoot: true
      allowPrivilegeEscalation: false
      readOnlyRootFilesystem: true
      capabilities:
        drop: ["ALL"]
  # Privileged container (use with caution)
  - name: privileged
    image: busybox:1.35
    command: ['sh', '-c', 'whoami && sleep 3600']
    securityContext:
      privileged: true
```

---

## Service Accounts

### Default Service Account

```bash
# View default service account
kubectl get serviceaccount default
kubectl describe serviceaccount default

# View service account token
kubectl get secret $(kubectl get serviceaccount default -o jsonpath='{.secrets[0].name}') -o jsonpath='{.data.token}' | base64 -d
```

### Custom Service Account

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: app-service-account
  namespace: default
automountServiceAccountToken: false
---
apiVersion: v1
kind: Pod
metadata:
  name: service-account-pod
spec:
  serviceAccountName: app-service-account
  automountServiceAccountToken: true
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'ls -la /var/run/secrets/kubernetes.io/serviceaccount/ && sleep 3600']
```

### Service Account with Image Pull Secrets

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: registry-service-account
imagePullSecrets:
- name: registry-secret
---
apiVersion: v1
kind: Pod
metadata:
  name: private-registry-pod
spec:
  serviceAccountName: registry-service-account
  containers:
  - name: app
    image: registry.example.com/private/myapp:latest
```

---

## RBAC (Role-Based Access Control)

### Role and RoleBinding

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: default
  name: pod-reader
rules:
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "watch", "list"]
- apiGroups: [""]
  resources: ["pods/log"]
  verbs: ["get"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: read-pods
  namespace: default
subjects:
- kind: ServiceAccount
  name: app-service-account
  namespace: default
roleRef:
  kind: Role
  name: pod-reader
  apiGroup: rbac.authorization.k8s.io
```

### ClusterRole and ClusterRoleBinding

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: node-reader
rules:
- apiGroups: [""]
  resources: ["nodes"]
  verbs: ["get", "list", "watch"]
- apiGroups: ["metrics.k8s.io"]
  resources: ["nodes", "pods"]
  verbs: ["get", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: read-nodes
subjects:
- kind: ServiceAccount
  name: monitoring-service-account
  namespace: monitoring
roleRef:
  kind: ClusterRole
  name: node-reader
  apiGroup: rbac.authorization.k8s.io
```

### Testing RBAC

```bash
# Check if service account can perform action
kubectl auth can-i get pods --as=system:serviceaccount:default:app-service-account

# Check permissions for current user
kubectl auth can-i get pods
kubectl auth can-i create deployments
kubectl auth can-i "*" "*"

# List permissions for service account
kubectl auth can-i --list --as=system:serviceaccount:default:app-service-account
```

---

## Network Policies

### Default Deny All Network Policy

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-all
  namespace: production
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
```

### Allow Specific Traffic

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: web-app-netpol
  namespace: production
spec:
  podSelector:
    matchLabels:
      app: web-app
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: frontend
    - namespaceSelector:
        matchLabels:
          name: production
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: database
    ports:
    - protocol: TCP
      port: 5432
  - to: []  # Allow DNS
    ports:
    - protocol: UDP
      port: 53
```

### Complex Network Policy

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-server-netpol
spec:
  podSelector:
    matchLabels:
      app: api-server
      tier: backend
  policyTypes:
  - Ingress
  - Egress
  ingress:
  # Allow from frontend in same namespace
  - from:
    - podSelector:
        matchLabels:
          app: frontend
    ports:
    - protocol: TCP
      port: 8080
  # Allow from monitoring namespace
  - from:
    - namespaceSelector:
        matchLabels:
          name: monitoring
    ports:
    - protocol: TCP
      port: 9090
  egress:
  # Allow to database
  - to:
    - podSelector:
        matchLabels:
          app: database
    ports:
    - protocol: TCP
      port: 5432
  # Allow external API calls
  - to: []
    ports:
    - protocol: TCP
      port: 443
  # Allow DNS
  - to: []
    ports:
    - protocol: UDP
      port: 53
```

---

## Resource Quotas and Limits

### Namespace Resource Quota

```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: production-quota
  namespace: production
spec:
  hard:
    # Compute resources
    requests.cpu: "10"
    requests.memory: 20Gi
    limits.cpu: "20"
    limits.memory: 40Gi

    # Storage resources
    requests.storage: 100Gi
    persistentvolumeclaims: "10"

    # Object counts
    pods: "50"
    services: "20"
    secrets: "30"
    configmaps: "40"
    deployments.apps: "20"
```

### Limit Range

```yaml
apiVersion: v1
kind: LimitRange
metadata:
  name: production-limits
  namespace: production
spec:
  limits:
  # Container limits
  - type: Container
    default:
      cpu: "200m"
      memory: "256Mi"
    defaultRequest:
      cpu: "100m"
      memory: "128Mi"
    max:
      cpu: "2"
      memory: "2Gi"
    min:
      cpu: "50m"
      memory: "64Mi"
  # Pod limits
  - type: Pod
    max:
      cpu: "4"
      memory: "4Gi"
  # PVC limits
  - type: PersistentVolumeClaim
    max:
      storage: "10Gi"
    min:
      storage: "1Gi"
```

---

## Pod Security Standards

### Pod Security Policy Replacement

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: secure-namespace
  labels:
    pod-security.kubernetes.io/enforce: restricted
    pod-security.kubernetes.io/audit: restricted
    pod-security.kubernetes.io/warn: restricted
```

### Secure Pod Example

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: secure-pod
  namespace: secure-namespace
spec:
  securityContext:
    runAsNonRoot: true
    runAsUser: 1000
    runAsGroup: 1000
    fsGroup: 1000
    seccompProfile:
      type: RuntimeDefault
  containers:
  - name: app
    image: nginx:1.21
    securityContext:
      allowPrivilegeEscalation: false
      readOnlyRootFilesystem: true
      runAsNonRoot: true
      runAsUser: 1000
      capabilities:
        drop:
        - ALL
    volumeMounts:
    - name: cache
      mountPath: /var/cache/nginx
    - name: run
      mountPath: /var/run
  volumes:
  - name: cache
    emptyDir: {}
  - name: run
    emptyDir: {}
```

---

## Volume Mounts and Persistent Storage

### EmptyDir Volume

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: emptydir-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'echo "Hello" > /tmp/data/hello.txt && sleep 3600']
    volumeMounts:
    - name: temp-storage
      mountPath: /tmp/data
  volumes:
  - name: temp-storage
    emptyDir:
      sizeLimit: 1Gi
```

### HostPath Volume

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: hostpath-pod
spec:
  containers:
  - name: app
    image: busybox:1.35
    command: ['sh', '-c', 'ls -la /host/var/log && sleep 3600']
    volumeMounts:
    - name: host-logs
      mountPath: /host/var/log
      readOnly: true
  volumes:
  - name: host-logs
    hostPath:
      path: /var/log
      type: Directory
```

### Persistent Volume Claim

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: app-storage-claim
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 5Gi
  storageClassName: standard
---
apiVersion: v1
kind: Pod
metadata:
  name: pvc-pod
spec:
  containers:
  - name: app
    image: nginx:1.21
    volumeMounts:
    - name: app-storage
      mountPath: /usr/share/nginx/html
  volumes:
  - name: app-storage
    persistentVolumeClaim:
      claimName: app-storage-claim
```

---

## Best Practices

### Configuration Management

1. **Separation of Concerns**
   ```yaml
   # Separate ConfigMaps for different purposes
   apiVersion: v1
   kind: ConfigMap
   metadata:
     name: app-config
   data:
     app.properties: |
       app.name=MyApp
       app.version=1.0.0
   ---
   apiVersion: v1
   kind: ConfigMap
   metadata:
     name: nginx-config
   data:
     nginx.conf: |
       server {
         listen 80;
         location / {
           proxy_pass http://backend;
         }
       }
   ```

2. **Immutable ConfigMaps**
   ```yaml
   apiVersion: v1
   kind: ConfigMap
   metadata:
     name: app-config-v1
   immutable: true
   data:
     config.yaml: |
       version: "1.0.0"
       database:
         host: "db.example.com"
   ```

### Security Best Practices

1. **Least Privilege**
   ```yaml
   apiVersion: v1
   kind: Pod
   spec:
     securityContext:
       runAsNonRoot: true
       runAsUser: 1000
       fsGroup: 1000
     containers:
     - name: app
       securityContext:
         allowPrivilegeEscalation: false
         readOnlyRootFilesystem: true
         capabilities:
           drop: ["ALL"]
   ```

2. **Resource Limits**
   ```yaml
   apiVersion: v1
   kind: Pod
   spec:
     containers:
     - name: app
       resources:
         requests:
           cpu: 100m
           memory: 128Mi
         limits:
           cpu: 500m
           memory: 512Mi
   ```

---

## Exam Tips and Common Scenarios

### Quick Commands

```bash
# Create ConfigMap from literals
kubectl create configmap app-config --from-literal=key1=value1 --from-literal=key2=value2

# Create Secret from literals
kubectl create secret generic app-secret --from-literal=username=admin --from-literal=password=secret

# Create ServiceAccount
kubectl create serviceaccount my-sa

# Check RBAC permissions
kubectl auth can-i get pods --as=system:serviceaccount:default:my-sa

# Apply resource quota
kubectl create quota my-quota --hard=cpu=2,memory=4Gi,pods=10
```

### YAML Generation

```bash
# Generate ConfigMap YAML
kubectl create configmap app-config --from-literal=key=value --dry-run=client -o yaml

# Generate Secret YAML
kubectl create secret generic app-secret --from-literal=password=secret --dry-run=client -o yaml

# Generate ServiceAccount YAML
kubectl create serviceaccount my-sa --dry-run=client -o yaml
```

This comprehensive guide covers all essential concepts for CKAD Section 4: Application Environment, Configuration and Security. Master these configuration and security patterns to effectively manage Kubernetes applications.