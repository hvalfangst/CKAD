# YAML Templates & Best Practices

## 🎯 Essential YAML Templates

### Pod Template
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: my-pod
  labels:
    app: my-app
    env: production
spec:
  containers:
  - name: main-container
    image: nginx:1.21
    ports:
    - containerPort: 80
    resources:
      requests:
        cpu: 100m
        memory: 128Mi
      limits:
        cpu: 200m
        memory: 256Mi
    env:
    - name: ENV_VAR
      value: "value"
    volumeMounts:
    - name: config-volume
      mountPath: /etc/config
  volumes:
  - name: config-volume
    configMap:
      name: my-config
```

### Deployment Template
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-deployment
  labels:
    app: my-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: my-app
  template:
    metadata:
      labels:
        app: my-app
        version: v1
    spec:
      containers:
      - name: main-container
        image: nginx:1.21
        ports:
        - containerPort: 80
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
          limits:
            cpu: 200m
            memory: 256Mi
        livenessProbe:
          httpGet:
            path: /
            port: 80
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /
            port: 80
          initialDelaySeconds: 5
          periodSeconds: 5
```

### Service Template
```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service
  labels:
    app: my-app
spec:
  type: ClusterIP  # ClusterIP, NodePort, LoadBalancer
  selector:
    app: my-app
  ports:
  - name: http
    protocol: TCP
    port: 80
    targetPort: 80
```

### ConfigMap Template
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: my-config
  labels:
    app: my-app
data:
  # Key-value pairs
  database.host: "localhost"
  database.port: "5432"

  # File content
  app.properties: |
    server.port=8080
    logging.level=INFO
    database.url=jdbc:postgresql://localhost:5432/mydb
```

### Secret Template
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: my-secret
  labels:
    app: my-app
type: Opaque
data:
  # Base64 encoded values
  username: YWRtaW4=        # admin
  password: cGFzc3dvcmQ=    # password
```

### PersistentVolumeClaim Template
```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: my-pvc
  labels:
    app: my-app
spec:
  accessModes:
  - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
  storageClassName: standard
```

## 🔧 Multi-Container Pod Templates

### Sidecar Pattern
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: sidecar-pod
spec:
  containers:
  - name: main-app
    image: nginx:1.21
    ports:
    - containerPort: 80
    volumeMounts:
    - name: shared-data
      mountPath: /usr/share/nginx/html

  - name: sidecar
    image: busybox
    command: ['sh', '-c', 'while true; do echo $(date) > /var/log/index.html; sleep 10; done']
    volumeMounts:
    - name: shared-data
      mountPath: /var/log

  volumes:
  - name: shared-data
    emptyDir: {}
```

### Init Container
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: init-pod
spec:
  initContainers:
  - name: init-setup
    image: busybox:1.35
    command: ['sh', '-c', 'echo "Initializing..." && sleep 5']
    volumeMounts:
    - name: workdir
      mountPath: /work-dir

  containers:
  - name: main-app
    image: nginx:1.21
    ports:
    - containerPort: 80
    volumeMounts:
    - name: workdir
      mountPath: /usr/share/nginx/html

  volumes:
  - name: workdir
    emptyDir: {}
```

## 🏗️ Advanced Templates

### StatefulSet Template
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: my-statefulset
spec:
  serviceName: my-headless-service
  replicas: 3
  selector:
    matchLabels:
      app: my-app
  template:
    metadata:
      labels:
        app: my-app
    spec:
      containers:
      - name: main-container
        image: nginx:1.21
        ports:
        - containerPort: 80
        volumeMounts:
        - name: data-volume
          mountPath: /var/lib/data
  volumeClaimTemplates:
  - metadata:
      name: data-volume
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 1Gi
```

### DaemonSet Template
```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: my-daemonset
spec:
  selector:
    matchLabels:
      app: my-daemon
  template:
    metadata:
      labels:
        app: my-daemon
    spec:
      containers:
      - name: main-container
        image: nginx:1.21
        resources:
          limits:
            memory: "128Mi"
            cpu: "100m"
        volumeMounts:
        - name: proc
          mountPath: /host/proc
          readOnly: true
        - name: sys
          mountPath: /host/sys
          readOnly: true
      volumes:
      - name: proc
        hostPath:
          path: /proc
      - name: sys
        hostPath:
          path: /sys
      hostNetwork: true
      hostPID: true
```

### Job Template
```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: my-job
spec:
  completions: 1
  parallelism: 1
  backoffLimit: 3
  template:
    metadata:
      labels:
        app: my-job
    spec:
      restartPolicy: Never
      containers:
      - name: job-container
        image: busybox
        command: ['sh', '-c', 'echo "Job completed" && sleep 30']
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
```

### CronJob Template
```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: my-cronjob
spec:
  schedule: "0 2 * * *"  # Every day at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          restartPolicy: OnFailure
          containers:
          - name: cronjob-container
            image: busybox
            command: ['sh', '-c', 'echo "CronJob executed at $(date)"']
```

## 🔐 Security Templates

### NetworkPolicy Template
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: deny-all-ingress
  namespace: production
spec:
  podSelector: {}  # Applies to all pods in namespace
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: allowed-app
    ports:
    - protocol: TCP
      port: 80
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: database
    ports:
    - protocol: TCP
      port: 5432
```

### ServiceAccount Template
```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: my-service-account
  namespace: default
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: default
  name: my-role
rules:
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "watch", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: my-role-binding
  namespace: default
subjects:
- kind: ServiceAccount
  name: my-service-account
  namespace: default
roleRef:
  kind: Role
  name: my-role
  apiGroup: rbac.authorization.k8s.io
```

## 📊 Monitoring Templates

### HorizontalPodAutoscaler Template
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: my-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: my-deployment
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### PodDisruptionBudget Template
```yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: my-pdb
spec:
  minAvailable: 2
  # Or use maxUnavailable: 1
  selector:
    matchLabels:
      app: my-app
```

## 🎯 Common Patterns & Snippets

### Environment Variables from ConfigMap/Secret
```yaml
# All keys from ConfigMap
envFrom:
- configMapRef:
    name: my-config
- secretRef:
    name: my-secret

# Individual keys
env:
- name: DATABASE_HOST
  valueFrom:
    configMapKeyRef:
      name: my-config
      key: database.host
- name: DATABASE_PASSWORD
  valueFrom:
    secretKeyRef:
      name: my-secret
      key: password
```

### Volume Mounts
```yaml
volumeMounts:
- name: config-volume
  mountPath: /etc/config
  readOnly: true
- name: secret-volume
  mountPath: /etc/secrets
  readOnly: true
- name: data-volume
  mountPath: /var/lib/data

volumes:
- name: config-volume
  configMap:
    name: my-config
- name: secret-volume
  secret:
    secretName: my-secret
- name: data-volume
  persistentVolumeClaim:
    claimName: my-pvc
```

### Health Probes
```yaml
livenessProbe:
  httpGet:
    path: /healthz
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10
  timeoutSeconds: 5
  failureThreshold: 3

readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 5
  timeoutSeconds: 3
  failureThreshold: 3

# TCP probe
livenessProbe:
  tcpSocket:
    port: 8080
  initialDelaySeconds: 15
  periodSeconds: 20

# Command probe
livenessProbe:
  exec:
    command:
    - cat
    - /tmp/healthy
  initialDelaySeconds: 5
  periodSeconds: 5
```

## ✅ YAML Best Practices

### Naming Conventions
- Use kebab-case for names
- Include app/component labels
- Use descriptive names
- Include version/environment info

### Resource Management
- Always set resource requests
- Set appropriate limits
- Use namespace for organization
- Apply consistent labeling

### Security
- Use non-root containers
- Set security context
- Limit privileges
- Use service accounts

### Configuration
- Externalize config with ConfigMaps
- Store secrets securely
- Use environment-specific values
- Version your configurations

## 🚀 Quick Generation Commands

```bash
# Generate and modify templates
kubectl create deployment nginx --image=nginx --dry-run=client -o yaml | kubectl set resources --local --limits=cpu=200m,memory=256Mi --requests=cpu=100m,memory=128Mi --dry-run=client -o yaml -f -

# Add labels to generated YAML
kubectl create deployment nginx --image=nginx --dry-run=client -o yaml | kubectl label --local -f - app=web env=prod --dry-run=client -o yaml

# Create service for deployment
kubectl expose deployment nginx --port=80 --dry-run=client -o yaml
```