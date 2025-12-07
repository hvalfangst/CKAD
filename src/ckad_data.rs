#[derive(Clone, PartialEq)]
pub struct Concept {
    pub title: String,
    pub command: String,
    pub description: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Category {
    pub name: String,
    pub concepts: Vec<Concept>,
}

pub fn get_ckad_concepts() -> Vec<Category> {
    vec![
        Category {
            name: "🚀 Quick Start & Context".to_string(),
            concepts: vec![
                Concept {
                    title: "Set namespace for current context".to_string(),
                    command: "k config set-context --current --namespace NAMESPACE".to_string(),
                    description: Some("⚡ EXAM TIP: Set this early to avoid typing -n on every command!".to_string()),
                },
                Concept {
                    title: "Deploy resource from manifest".to_string(),
                    command: "k apply -f manifest.yaml -n NAMESPACE".to_string(),
                    description: None,
                },
                Concept {
                    title: "Validate YAML before applying".to_string(),
                    command: "k apply -f app.yaml --dry-run=client".to_string(),
                    description: Some("Check for errors without creating resources".to_string()),
                },
                Concept {
                    title: "Apply all manifests in directory".to_string(),
                    command: "k apply -f /path/to/manifests/".to_string(),
                    description: Some("Useful for multi-file deployments".to_string()),
                },
            ],
        },
        Category {
            name: "📦 Pods & Deployments".to_string(),
            concepts: vec![
                Concept {
                    title: "Create pod manifest with labels".to_string(),
                    command: "k run random-pod --image=nginx:alpine --labels='id=awesome-pod' --dry-run=client -oyaml > pod.yaml".to_string(),
                    description: Some("⚡ EXAM TIP: Always use --dry-run=client -oyaml for manifests".to_string()),
                },
                Concept {
                    title: "Create deployment manifest".to_string(),
                    command: "k create deployment random-deployment --image=nginx:alpine --replicas=3 --dry-run=client -oyaml > deployment.yaml".to_string(),
                    description: Some("Creates deployment with 3 replicas".to_string()),
                },
                Concept {
                    title: "Create deployment with env vars".to_string(),
                    command: "k create deploy envtest --image=nginx --env APP_ENV=prod --env LOG_LEVEL=debug".to_string(),
                    description: Some("Quickly add environment variables".to_string()),
                },
                Concept {
                    title: "Add label to existing deployment".to_string(),
                    command: "k label deployment random-deployment id=awesome-deployment".to_string(),
                    description: None,
                },
                Concept {
                    title: "Update deployment image".to_string(),
                    command: "k set image deployment/api-deploy httpd=httpd:2.4.58".to_string(),
                    description: Some("⚡ Fast way to update image in exam".to_string()),
                },
                Concept {
                    title: "Scale deployment".to_string(),
                    command: "k scale deployment/myapp --replicas=5".to_string(),
                    description: None,
                },
                Concept {
                    title: "Quick edit running resource".to_string(),
                    command: r#"k edit deploy NAME
k edit pod NAME
k edit svc NAME"#.to_string(),
                    description: Some("Opens resource in $EDITOR (usually vi)".to_string()),
                },
                Concept {
                    title: "Get deployment with wide output".to_string(),
                    command: "k get deploy -o wide".to_string(),
                    description: Some("Shows additional columns like images, selectors".to_string()),
                },
            ],
        },
        Category {
            name: "🔐 Secrets & ConfigMaps".to_string(),
            concepts: vec![
                Concept {
                    title: "Create secret from literals".to_string(),
                    command: "k create secret generic SECRET_NAME --from-literal=KEY1=VALUE1 --from-literal=KEY2=VALUE2".to_string(),
                    description: None,
                },
                Concept {
                    title: "Create secret from file".to_string(),
                    command: "k create secret generic db-secret --from-file=./password.txt".to_string(),
                    description: None,
                },
                Concept {
                    title: "Base64 encode for secrets".to_string(),
                    command: "echo -n 'mypassword' | base64".to_string(),
                    description: Some("Get base64 value for manual secret creation".to_string()),
                },
                Concept {
                    title: "Map secret to env vars".to_string(),
                    command: r#"env:
  - name: SECRET_USERNAME
    valueFrom:
      secretKeyRef:
        name: mysecret
        key: username
  - name: SECRET_PASSWORD
    valueFrom:
      secretKeyRef:
        name: mysecret
        key: password"#.to_string(),
                    description: Some("⚡ EXAM TIP: Common pattern - replace hardcoded env with secrets".to_string()),
                },
                Concept {
                    title: "Load all secret keys as env vars".to_string(),
                    command: "k set env deploy/myapp --from=secret/db-pass".to_string(),
                    description: Some("Imports all keys from secret as environment variables".to_string()),
                },
                Concept {
                    title: "Mount secret as volume".to_string(),
                    command: r#"spec:
  volumes:
  - name: secret-volume
    secret:
      secretName: my-secret
  containers:
  - name: container
    volumeMounts:
    - name: secret-volume
      mountPath: /tmp/secret"#.to_string(),
                    description: Some("Secret appears as files in /tmp/secret".to_string()),
                },
                Concept {
                    title: "Create configmap from literals".to_string(),
                    command: "k create cm my-config --from-literal=key1=value1 --from-literal=key2=value2".to_string(),
                    description: None,
                },
                Concept {
                    title: "Create configmap from file".to_string(),
                    command: "k create cm app-config --from-file=KEY=/opt/data/config.txt".to_string(),
                    description: None,
                },
                Concept {
                    title: "Create configmap with custom key name".to_string(),
                    command: "k create configmap CM_NAME --from-file=index.html=/opt/course/file.html".to_string(),
                    description: Some("File content stored under custom key 'index.html'".to_string()),
                },
                Concept {
                    title: "Get decoded ServiceAccount token".to_string(),
                    command: "k -n NAMESPACE describe secret SECRET_NAME".to_string(),
                    description: Some("Shows decoded token and CA certificate".to_string()),
                },
            ],
        },
        Category {
            name: "🔍 Debugging & Inspection".to_string(),
            concepts: vec![
                Concept {
                    title: "Get pod IPs and node placement".to_string(),
                    command: "k get po -o wide".to_string(),
                    description: Some("Shows IP, NODE, NOMINATED NODE, READINESS GATES".to_string()),
                },
                Concept {
                    title: "Create temp debug pod".to_string(),
                    command: r#"k run debug --rm -it --image=nginx:alpine -- sh
# curl http://service
        OR
# curl 10.244.1.5:80"#.to_string(),
                    description: Some("⚡ EXAM TIP: Quick way to test connectivity, --rm auto-deletes".to_string()),
                },
                Concept {
                    title: "Debug with busybox".to_string(),
                    command: "k run tmp --rm -it --image=busybox -- sh".to_string(),
                    description: Some("Lightweight container for testing".to_string()),
                },
                Concept {
                    title: "Debug with curl".to_string(),
                    command: "k run tmp --rm -it --image=alpine/curl -- sh".to_string(),
                    description: Some("Has curl pre-installed for API testing".to_string()),
                },
                Concept {
                    title: "Explain resource fields".to_string(),
                    command: r#"k explain pvc
k explain pvc.spec"#.to_string(),
                    description: Some("⚡ EXAM TIP: Your best friend for discovering field names!".to_string()),
                },
                Concept {
                    title: "Recursive explain (show all fields)".to_string(),
                    command: r#"k explain pod.spec --recursive
k explain pod.spec --recursive | grep -i volume"#.to_string(),
                    description: Some("Search for specific fields in nested structures".to_string()),
                },
                Concept {
                    title: "Find field paths quickly".to_string(),
                    command: r#"k explain deploy.spec --recursive | grep -i replicas
k explain pod.spec.containers --recursive | grep -i probe
k explain pod.spec --recursive | grep -i serviceAccount"#.to_string(),
                    description: Some("Grep to find exact field location".to_string()),
                },
                Concept {
                    title: "Check container logs".to_string(),
                    command: "k logs POD_NAME -c CONTAINER_NAME".to_string(),
                    description: Some("Specify container for multi-container pods".to_string()),
                },
                Concept {
                    title: "Follow logs in real-time".to_string(),
                    command: "k logs -f POD_NAME".to_string(),
                    description: Some("Stream logs (like tail -f)".to_string()),
                },
                Concept {
                    title: "Previous container logs".to_string(),
                    command: "k logs POD_NAME --previous".to_string(),
                    description: Some("View logs from crashed container".to_string()),
                },
                Concept {
                    title: "Execute command in pod".to_string(),
                    command: r#"k exec POD_NAME -- ls /
k exec -it POD_NAME -- sh"#.to_string(),
                    description: Some("-it for interactive shell".to_string()),
                },
                Concept {
                    title: "Attach to running container".to_string(),
                    command: "k attach POD_NAME -c CONTAINER".to_string(),
                    description: Some("Attach to stdout/stdin of running process".to_string()),
                },
                Concept {
                    title: "Copy files to/from pod".to_string(),
                    command: r#"k cp ./local-file POD:/tmp/file
k cp POD:/var/log/app.log ./app.log"#.to_string(),
                    description: Some("Transfer files for debugging".to_string()),
                },
                Concept {
                    title: "Port forward to pod".to_string(),
                    command: r#"k port-forward pod/POD 8080:80
k port-forward svc/SVC 8080:80"#.to_string(),
                    description: Some("Access pod/service from localhost:8080".to_string()),
                },
                Concept {
                    title: "Debug with ephemeral container".to_string(),
                    command: "k debug -it pod/broken --image=busybox --target=broken -- /bin/sh".to_string(),
                    description: Some("⚡ EXAM TIP: Debug CrashLoopBackOff pods without restart".to_string()),
                },
                Concept {
                    title: "Get pod status with jsonpath".to_string(),
                    command: r#"k get pod POD_NAME -o jsonpath="{.status.phase}""#.to_string(),
                    description: Some("Extract specific fields programmatically".to_string()),
                },
                Concept {
                    title: "Get container images".to_string(),
                    command: r#"k get pod POD -o jsonpath='{.spec.containers[*].image}'"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Get pod status with describe".to_string(),
                    command: "k describe pod POD_NAME | grep -i status:".to_string(),
                    description: None,
                },
                Concept {
                    title: "Search in all pod YAMLs".to_string(),
                    command: "k -n NAMESPACE get pod -o yaml | grep SEARCH_TEXT -A10".to_string(),
                    description: Some("Search across all pods in namespace".to_string()),
                },
                Concept {
                    title: "Filter events by pod".to_string(),
                    command: "k get events --field-selector involvedObject.name=POD".to_string(),
                    description: Some("See only events related to specific pod".to_string()),
                },
                Concept {
                    title: "Watch resources in real-time".to_string(),
                    command: r#"k get po -w
k get deploy -w"#.to_string(),
                    description: Some("Live updates as resources change".to_string()),
                },
                Concept {
                    title: "Get resource usage (requires metrics-server)".to_string(),
                    command: r#"k top pod
k top node"#.to_string(),
                    description: Some("Show CPU/memory usage".to_string()),
                },
            ],
        },
        Category {
            name: "⚙️ Jobs & CronJobs".to_string(),
            concepts: vec![
                Concept {
                    title: "Create job manifest".to_string(),
                    command: r#"k create job JOB_NAME --image=busybox:1.31.0 --dry-run=client -oyaml -- sh -c "sleep 2 && echo done" > job.yaml"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Create cronjob manifest".to_string(),
                    command: r#"k create cronjob my-cronjob --image=busybox:1.31.0 --schedule="*/5 * * * *" --dry-run=client -oyaml -- sh -c "sleep 2 && echo done" > cronjob.yaml"#.to_string(),
                    description: Some("Runs every 5 minutes".to_string()),
                },
                Concept {
                    title: "Manually trigger job from cronjob".to_string(),
                    command: "k create job my-job --from=cronjob/my-cronjob".to_string(),
                    description: Some("⚡ EXAM TIP: Test cronjobs without waiting for schedule".to_string()),
                },
                Concept {
                    title: "Job with completions and parallelism".to_string(),
                    command: r#"spec:
  completions: 3
  parallelism: 2
  template:
    metadata:
      labels:
        id: awesome-job
    spec:
      containers:
      - command:
        - sh
        - -c
        - sleep 2 && echo done
        image: busybox:1.31.0
        name: job-container
      restartPolicy: Never"#.to_string(),
                    description: Some("Run 3 completions with max 2 parallel pods".to_string()),
                },
                Concept {
                    title: "CronJob with completions/parallelism".to_string(),
                    command: r#"apiVersion: batch/v1
kind: CronJob
metadata:
  name: my-cronjob
spec:
  schedule: "*/5 * * * *"
  jobTemplate:
    spec:
      completions: 3
      parallelism: 2
      template:
        metadata:
          labels:
            id: awesome-job
        spec:
          containers:
          - name: container
            image: busybox:1.31.0
            command:
            - sh
            - -c
            - sleep 2 && echo done
          restartPolicy: Never"#.to_string(),
                    description: Some("Complete CronJob with parallelism and labels".to_string()),
                },
                Concept {
                    title: "Delete completed jobs".to_string(),
                    command: "k delete pod --field-selector=status.phase==Succeeded".to_string(),
                    description: Some("Clean up completed job pods".to_string()),
                },
            ],
        },
        Category {
            name: "🎩 Helm".to_string(),
            concepts: vec![
                Concept {
                    title: "List releases in namespace".to_string(),
                    command: r#"helm -n NAMESPACE ls
helm -n NAMESPACE ls -a    # includes pending-install states"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Uninstall release".to_string(),
                    command: "helm -n NAMESPACE uninstall RELEASE_NAME".to_string(),
                    description: None,
                },
                Concept {
                    title: "Update repos and search charts".to_string(),
                    command: r#"helm repo update
helm search repo nginx --versions"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Upgrade release".to_string(),
                    command: "helm -n NAMESPACE upgrade RELEASE_NAME REPO/CHART".to_string(),
                    description: None,
                },
                Concept {
                    title: "Show chart values".to_string(),
                    command: "helm show values REPO/CHART".to_string(),
                    description: Some("See all configurable values for a chart".to_string()),
                },
                Concept {
                    title: "Install with custom values".to_string(),
                    command: "helm -n NAMESPACE install RELEASE_NAME REPO/CHART --set replicaCount=2 --set image.debug=true".to_string(),
                    description: Some("Override default chart values".to_string()),
                },
                Concept {
                    title: "Install with values file".to_string(),
                    command: "helm install RELEASE CHART -f values.yaml".to_string(),
                    description: None,
                },
            ],
        },
        Category {
            name: "❤️ Probes & Health Checks".to_string(),
            concepts: vec![
                Concept {
                    title: "Readiness probe - exec command".to_string(),
                    command: r#"readinessProbe:
  exec:
    command:
    - sh
    - -c
    - cat /tmp/ready
  initialDelaySeconds: 5
  periodSeconds: 10"#.to_string(),
                    description: Some("Container ready when command exits 0".to_string()),
                },
                Concept {
                    title: "Readiness probe - HTTP GET".to_string(),
                    command: r#"readinessProbe:
  httpGet:
    path: /healthz
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 10"#.to_string(),
                    description: Some("Check HTTP endpoint for readiness".to_string()),
                },
                Concept {
                    title: "Liveness probe - TCP socket".to_string(),
                    command: r#"livenessProbe:
  tcpSocket:
    port: 80
  initialDelaySeconds: 10
  periodSeconds: 15"#.to_string(),
                    description: Some("Restart container if TCP connection fails".to_string()),
                },
                Concept {
                    title: "Liveness probe - HTTP".to_string(),
                    command: r#"livenessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5"#.to_string(),
                    description: Some("⚡ EXAM TIP: Most common liveness probe type".to_string()),
                },
                Concept {
                    title: "Startup probe (for slow-starting containers)".to_string(),
                    command: r#"startupProbe:
  httpGet:
    path: /startup
    port: 8080
  failureThreshold: 30
  periodSeconds: 10"#.to_string(),
                    description: Some("Gives container up to 300s to start before liveness checks".to_string()),
                },
            ],
        },
        Category {
            name: "🔄 Deployments & Rollouts".to_string(),
            concepts: vec![
                Concept {
                    title: "View rollout history".to_string(),
                    command: r#"k rollout history deploy DEPLOY_NAME
k rollout history deploy DEPLOY_NAME --revision 2"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Rollback to previous version".to_string(),
                    command: "k rollout undo deploy DEPLOY_NAME".to_string(),
                    description: Some("⚡ EXAM TIP: Quick fix for bad deployments".to_string()),
                },
                Concept {
                    title: "Rollback to specific revision".to_string(),
                    command: "k rollout undo deploy DEPLOY_NAME --to-revision=3".to_string(),
                    description: None,
                },
                Concept {
                    title: "Check rollout status".to_string(),
                    command: "k rollout status deploy DEPLOY_NAME".to_string(),
                    description: Some("Wait for rollout to complete".to_string()),
                },
                Concept {
                    title: "Pause rollout".to_string(),
                    command: "k rollout pause deploy/myapp".to_string(),
                    description: Some("⚡ EXAM TIP: Pause, make changes, resume for atomic update".to_string()),
                },
                Concept {
                    title: "Resume paused rollout".to_string(),
                    command: "k rollout resume deploy/myapp".to_string(),
                    description: None,
                },
                Concept {
                    title: "Restart deployment (trigger rollout)".to_string(),
                    command: "k rollout restart deploy DEPLOY_NAME".to_string(),
                    description: Some("Force pod recreation without config changes".to_string()),
                },
                Concept {
                    title: "Get ReplicaSets with images".to_string(),
                    command: "k get rs -o wide | grep DEPLOY_NAME".to_string(),
                    description: Some("See old and new ReplicaSets during rollout".to_string()),
                },
                Concept {
                    title: "Patch deployment image".to_string(),
                    command: r#"k patch deploy myapp -p '{"spec":{"template":{"spec":{"containers":[{"name":"myapp","image":"nginx:alpine"}]}}}}"#.to_string(),
                    description: Some("Fast JSON-based patching".to_string()),
                },
            ],
        },
        Category {
            name: "🔒 Security & RBAC".to_string(),
            concepts: vec![
                Concept {
                    title: "SecurityContext - run as user".to_string(),
                    command: r#"securityContext:
  runAsUser: 1001
  runAsGroup: 3000
  fsGroup: 2000
  allowPrivilegeEscalation: false
  capabilities:
    drop: ["ALL"]
    add: ["NET_ADMIN"]"#.to_string(),
                    description: Some("⚡ EXAM TIP: Container-level security settings".to_string()),
                },
                Concept {
                    title: "Container-level security context".to_string(),
                    command: r#"containers:
- name: container
  image: nginx
  securityContext:
    allowPrivilegeEscalation: false
    privileged: false"#.to_string(),
                    description: Some("Minimal security settings".to_string()),
                },
                Concept {
                    title: "Create ServiceAccount".to_string(),
                    command: "k create sa backend-sa".to_string(),
                    description: None,
                },
                Concept {
                    title: "Set ServiceAccount for deployment".to_string(),
                    command: r#"spec:
  template:
    spec:
      serviceAccountName: my-sa"#.to_string(),
                    description: Some("⚡ EXAM TIP: Pods recreate when SA changes in deployment".to_string()),
                },
                Concept {
                    title: "Create Role".to_string(),
                    command: "k create role pod-reader --verb=get,list,watch --resource=pods".to_string(),
                    description: Some("Namespace-scoped permissions".to_string()),
                },
                Concept {
                    title: "Create RoleBinding".to_string(),
                    command: r#"k create rolebinding pod-reader-binding \
  --role=pod-reader \
  --serviceaccount=default:backend-sa"#.to_string(),
                    description: Some("Bind role to ServiceAccount".to_string()),
                },
                Concept {
                    title: "Create ClusterRole".to_string(),
                    command: "k create clusterrole node-reader --verb=get,list --resource=nodes".to_string(),
                    description: Some("Cluster-wide permissions".to_string()),
                },
                Concept {
                    title: "Create ClusterRoleBinding".to_string(),
                    command: "k create clusterrolebinding node-reader-binding --clusterrole=node-reader --serviceaccount=default:my-sa".to_string(),
                    description: None,
                },
            ],
        },
        Category {
            name: "🌐 Services & Networking".to_string(),
            concepts: vec![
                Concept {
                    title: "Expose pod as ClusterIP".to_string(),
                    command: "k expose pod POD_NAME --name SERVICE_NAME --port 3333 --target-port 80".to_string(),
                    description: Some("port: service port, target-port: container port".to_string()),
                },
                Concept {
                    title: "Expose deployment".to_string(),
                    command: "k expose deployment api-deploy --port=80 --target-port=80 --name=api-svc --type=ClusterIP".to_string(),
                    description: None,
                },
                Concept {
                    title: "Create ClusterIP service manifest".to_string(),
                    command: "k create service clusterip SERVICE_NAME --tcp 3333:80 --dry-run=client -oyaml".to_string(),
                    description: Some("Then edit selector to match pods".to_string()),
                },
                Concept {
                    title: "Create NodePort service".to_string(),
                    command: "k create service nodeport myapp --tcp=8080:80 --node-port=30100".to_string(),
                    description: None,
                },
                Concept {
                    title: "Change service type to NodePort".to_string(),
                    command: r#"# k edit svc SERVICE_NAME
spec:
  type: NodePort
  ports:
  - port: 8080
    targetPort: 80
    nodePort: 30100"#.to_string(),
                    description: Some("Edit existing service to expose externally".to_string()),
                },
                Concept {
                    title: "Create ExternalName service".to_string(),
                    command: r#"apiVersion: v1
kind: Service
metadata:
  name: external-api
spec:
  type: ExternalName
  externalName: api.example.com"#.to_string(),
                    description: Some("⚡ EXAM TIP: DNS CNAME redirect, no ports/selectors!".to_string()),
                },
                Concept {
                    title: "Check service endpoints".to_string(),
                    command: r#"k get endpointslice
k describe svc SERVICE_NAME | grep Endpoints"#.to_string(),
                    description: Some("Verify pods are backing the service".to_string()),
                },
                Concept {
                    title: "Get service details".to_string(),
                    command: "k get svc SERVICE_NAME -o wide".to_string(),
                    description: None,
                },
                Concept {
                    title: "NetworkPolicy - egress restriction".to_string(),
                    command: r#"apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: np1
  namespace: default
spec:
  podSelector:
    matchLabels:
      id: frontend
  policyTypes:
  - Egress
  egress:
  - to:
    - podSelector:
        matchLabels:
          id: api
  - ports:
    - port: 53
      protocol: UDP
    - port: 53
      protocol: TCP"#.to_string(),
                    description: Some("⚡ EXAM TIP: Allow DNS (port 53) or pods can't resolve names!".to_string()),
                },
                Concept {
                    title: "NetworkPolicy - ingress restriction".to_string(),
                    command: r#"apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: db-allow
spec:
  podSelector:
    matchLabels:
      app: db
  ingress:
  - from:
    - podSelector:
        matchLabels:
          role: backend
    ports:
    - protocol: TCP
      port: 5432
  policyTypes:
  - Ingress"#.to_string(),
                    description: Some("Only allow backend pods to connect to db".to_string()),
                },
                Concept {
                    title: "Fix NetworkPolicy by labeling pod".to_string(),
                    command: "k label pod worker role=backend --overwrite".to_string(),
                    description: Some("⚡ EXAM TIP: Often told NOT to modify NP, label pods instead!".to_string()),
                },
                Concept {
                    title: "Create Ingress".to_string(),
                    command: r#"apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: web-ingress
spec:
  rules:
  - http:
      paths:
      - path: /app
        pathType: Prefix
        backend:
          service:
            name: api-svc
            port:
              number: 80"#.to_string(),
                    description: Some("Route HTTP /app path to service".to_string()),
                },
                Concept {
                    title: "Create Ingress imperatively".to_string(),
                    command: r#"k create ingress NAME --rule="host/path=service:80" -oyaml --dry-run=client"#.to_string(),
                    description: None,
                },
            ],
        },
        Category {
            name: "💾 Storage".to_string(),
            concepts: vec![
                Concept {
                    title: "PersistentVolume manifest".to_string(),
                    command: r#"apiVersion: v1
kind: PersistentVolume
metadata:
  name: my-pv
spec:
  capacity:
    storage: 2Gi
  accessModes:
    - ReadWriteOnce
  hostPath:
    path: "/Volumes/Data""#.to_string(),
                    description: None,
                },
                Concept {
                    title: "PersistentVolumeClaim manifest".to_string(),
                    command: r#"apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: my-pvc
  namespace: default
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 2Gi"#.to_string(),
                    description: Some("Request persistent storage from cluster".to_string()),
                },
                Concept {
                    title: "Mount PVC in deployment".to_string(),
                    command: r#"spec:
  volumes:
  - name: data
    persistentVolumeClaim:
      claimName: my-pvc
  containers:
  - name: container
    volumeMounts:
    - name: data
      mountPath: /tmp/project-data"#.to_string(),
                    description: Some("⚡ EXAM TIP: Define volume at spec level, mount in container".to_string()),
                },
                Concept {
                    title: "EmptyDir volume (temporary)".to_string(),
                    command: r#"volumes:
- name: shared-data
  emptyDir: {}
containers:
- name: app
  volumeMounts:
  - mountPath: /shared
    name: shared-data"#.to_string(),
                    description: Some("Shared storage between containers, deleted with pod".to_string()),
                },
                Concept {
                    title: "HostPath volume".to_string(),
                    command: r#"volumes:
- name: host-volume
  hostPath:
    path: /data
    type: Directory"#.to_string(),
                    description: Some("Mount directory from node filesystem".to_string()),
                },
                Concept {
                    title: "StorageClass with Retain policy".to_string(),
                    command: r#"apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: my-storage-class
provisioner: my-provisioner
reclaimPolicy: Retain"#.to_string(),
                    description: Some("Retain: PV not deleted when PVC deleted".to_string()),
                },
                Concept {
                    title: "Check PVC status".to_string(),
                    command: "k get pvc".to_string(),
                    description: Some("Verify PVC is Bound before using".to_string()),
                },
            ],
        },
        Category {
            name: "🚀 InitContainers & Sidecars".to_string(),
            concepts: vec![
                Concept {
                    title: "InitContainer example".to_string(),
                    command: r#"initContainers:
- name: init-con
  image: busybox:1.31.0
  command: ['sh', '-c', 'echo "content" > /tmp/web-content/index.html']
  volumeMounts:
  - name: web-content
    mountPath: /tmp/web-content"#.to_string(),
                    description: Some("Runs before main containers, must complete successfully".to_string()),
                },
                Concept {
                    title: "Sidecar logging container".to_string(),
                    command: r#"initContainers:
  - name: init
    image: bash:5.0.11
    command: ['bash', '-c', 'echo init > /var/log/cleaner/cleaner.log']
    volumeMounts:
      - name: logs
        mountPath: /var/log/cleaner
  - name: logger
    image: bash:5.0.11
    restartPolicy: Always
    command: ['bash', '-c', 'tail -f /var/log/cleaner/cleaner.log']
    volumeMounts:
      - name: logs
        mountPath: /var/log/cleaner"#.to_string(),
                    description: Some("Sidecar that tails log file and outputs to stdout".to_string()),
                },
                Concept {
                    title: "Multi-container pod with shared volume".to_string(),
                    command: r#"apiVersion: v1
kind: Pod
metadata:
  name: multi
spec:
  containers:
  - name: app
    image: nginx
    volumeMounts:
    - mountPath: /shared
      name: shared-data
  - name: sidecar
    image: busybox
    command: ["/bin/sh","-c","while true; do ls /shared; sleep 5; done"]
    volumeMounts:
    - mountPath: /shared
      name: shared-data
  volumes:
  - name: shared-data
    emptyDir: {}"#.to_string(),
                    description: Some("⚡ EXAM TIP: Common pattern for log collection, monitoring".to_string()),
                },
            ],
        },
        Category {
            name: "⚡ Resource Management".to_string(),
            concepts: vec![
                Concept {
                    title: "Resource requests and limits".to_string(),
                    command: r#"containers:
- name: container
  image: nginx
  resources:
    requests:
      cpu: "100m"
      memory: "128Mi"
    limits:
      cpu: "200m"
      memory: "256Mi""#.to_string(),
                    description: Some("requests: guaranteed, limits: maximum allowed".to_string()),
                },
                Concept {
                    title: "ResourceQuota for namespace".to_string(),
                    command: r#"k create quota team-a \
  --hard=cpu=2,memory=2Gi,pods=10 -n dev"#.to_string(),
                    description: Some("Limit total resources in namespace".to_string()),
                },
                Concept {
                    title: "LimitRange for namespace".to_string(),
                    command: r#"apiVersion: v1
kind: LimitRange
metadata:
  name: mem-limit-range
spec:
  limits:
  - default:
      memory: 512Mi
    defaultRequest:
      memory: 256Mi
    type: Container"#.to_string(),
                    description: Some("Set default limits for pods without explicit limits".to_string()),
                },
                Concept {
                    title: "Force replace resource".to_string(),
                    command: "k replace -f manifest.yaml --force --grace-period=0".to_string(),
                    description: Some("Delete and recreate resource immediately".to_string()),
                },
            ],
        },
        Category {
            name: "🏷️ Labels & Annotations".to_string(),
            concepts: vec![
                Concept {
                    title: "Label multiple pods by selector".to_string(),
                    command: r#"k label pod -l "type in (worker,runner)" protected=true"#.to_string(),
                    description: Some("Bulk label pods matching selector".to_string()),
                },
                Concept {
                    title: "Remove label".to_string(),
                    command: "k label pod myapp version-".to_string(),
                    description: Some("Trailing dash removes label".to_string()),
                },
                Concept {
                    title: "Annotate pods by selector".to_string(),
                    command: r#"k annotate pod -l protected=true protected="do not delete this pod""#.to_string(),
                    description: Some("Add metadata annotation to pods".to_string()),
                },
                Concept {
                    title: "Show labels".to_string(),
                    command: "k get pod --show-labels".to_string(),
                    description: None,
                },
                Concept {
                    title: "Filter by label".to_string(),
                    command: r#"k get pod -l app=frontend
k get pod -l 'env in (prod,staging)'"#.to_string(),
                    description: Some("Query resources by label selectors".to_string()),
                },
                Concept {
                    title: "Custom columns with labels".to_string(),
                    command: "k get pod -o custom-columns=NAME:.metadata.name,LABELS:.metadata.labels".to_string(),
                    description: None,
                },
            ],
        },
        Category {
            name: "🐳 Container Tools".to_string(),
            concepts: vec![
                Concept {
                    title: "Docker - build and push".to_string(),
                    command: r#"sudo docker build -t registry.example.com:5000/image:tag .
sudo docker push registry.example.com:5000/image:tag"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Docker - tag image".to_string(),
                    command: "docker tag myapp:1.0 myrepo/myapp:1.0".to_string(),
                    description: None,
                },
                Concept {
                    title: "Docker - save as tar".to_string(),
                    command: "docker save myrepo/myapp:1.0 -o myimage.tar".to_string(),
                    description: Some("⚡ EXAM TIP: Export image for transfer".to_string()),
                },
                Concept {
                    title: "Docker - load from tar".to_string(),
                    command: "docker load -i myimage.tar".to_string(),
                    description: None,
                },
                Concept {
                    title: "Podman - run detached".to_string(),
                    command: "sudo podman run -d --name CONTAINER_NAME IMAGE".to_string(),
                    description: None,
                },
                Concept {
                    title: "Podman - get logs".to_string(),
                    command: "sudo podman logs CONTAINER_NAME > /opt/logs.txt".to_string(),
                    description: None,
                },
            ],
        },
        Category {
            name: "📚 Kubectl Explain Cheat Sheet".to_string(),
            concepts: vec![
                Concept {
                    title: "Basic explain usage".to_string(),
                    command: r#"k explain pod
k explain pod.spec
k explain pod.spec.containers"#.to_string(),
                    description: Some("Navigate resource structure hierarchically".to_string()),
                },
                Concept {
                    title: "Recursive explain".to_string(),
                    command: r#"k explain pod.spec --recursive
k explain pod.spec --recursive | grep -i volume"#.to_string(),
                    description: Some("Show all nested fields at once".to_string()),
                },
                Concept {
                    title: "Common probe paths".to_string(),
                    command: r#"k explain pod.spec.containers.readinessProbe
k explain pod.spec.containers.livenessProbe
k explain pod.spec.containers.startupProbe"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Common resource paths".to_string(),
                    command: r#"k explain pod.spec.containers.resources
k explain pod.spec.containers.securityContext
k explain pod.spec.volumes
k explain pod.spec.initContainers"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Job/CronJob paths".to_string(),
                    command: r#"k explain job.spec.completions
k explain job.spec.parallelism
k explain cronjob.spec.schedule"#.to_string(),
                    description: None,
                },
                Concept {
                    title: "Network/Storage paths".to_string(),
                    command: r#"k explain networkpolicy.spec.egress
k explain networkpolicy.spec.ingress
k explain pv.spec
k explain pvc.spec"#.to_string(),
                    description: None,
                },
            ],
        },
        Category {
            name: "🎯 Advanced Exam Patterns".to_string(),
            concepts: vec![
                Concept {
                    title: "Export pod to different namespace".to_string(),
                    command: r#"k -n SOURCE_NS get pod POD_NAME -o yaml > pod.yaml
# Edit: change namespace, remove status, nodeName, token volumes
k -n TARGET_NS create -f pod.yaml
k -n SOURCE_NS delete pod POD_NAME --force --grace-period=0"#.to_string(),
                    description: Some("⚡ EXAM TIP: Move pods between namespaces".to_string()),
                },
                Concept {
                    title: "Get all resources in namespace".to_string(),
                    command: "k get all -n NAMESPACE".to_string(),
                    description: Some("Quick overview of pods, services, deployments, etc.".to_string()),
                },
                Concept {
                    title: "Delete all resources in namespace".to_string(),
                    command: "k delete all --all -n dev".to_string(),
                    description: Some("⚠️ Careful: deletes everything in namespace!".to_string()),
                },
                Concept {
                    title: "Get node IPs".to_string(),
                    command: "k get nodes -o wide".to_string(),
                    description: Some("Shows internal/external IPs, OS, kernel version".to_string()),
                },
                Concept {
                    title: "Drain node for maintenance".to_string(),
                    command: "k drain NODE --ignore-daemonsets --force --delete-emptydir-data".to_string(),
                    description: Some("Evict pods from node safely".to_string()),
                },
                Concept {
                    title: "Uncordon node".to_string(),
                    command: "k uncordon NODE".to_string(),
                    description: Some("Allow pod scheduling again".to_string()),
                },
                Concept {
                    title: "Taint node".to_string(),
                    command: r#"k taint nodes node1 key=value:NoSchedule
k taint nodes node1 key=value:NoSchedule-"#.to_string(),
                    description: Some("Add/remove node taint (trailing - removes)".to_string()),
                },
                Concept {
                    title: "Toleration for tainted nodes".to_string(),
                    command: r#"tolerations:
- key: "key"
  operator: "Equal"
  value: "value"
  effect: "NoSchedule""#.to_string(),
                    description: Some("Allow pod to schedule on tainted nodes".to_string()),
                },
                Concept {
                    title: "Get API resources".to_string(),
                    command: "k api-resources".to_string(),
                    description: Some("List all resource types in cluster".to_string()),
                },
                Concept {
                    title: "Get API versions".to_string(),
                    command: "k api-versions".to_string(),
                    description: Some("Useful when fixing deprecated APIs".to_string()),
                },
                Concept {
                    title: "Convert deprecated API".to_string(),
                    command: "k convert -f old.yaml --output-version apps/v1".to_string(),
                    description: Some("⚡ EXAM TIP: Update old extensions/v1beta1 to apps/v1".to_string()),
                },
                Concept {
                    title: "Canary deployment pattern".to_string(),
                    command: r#"k create deploy web --image=nginx:1.23 --replicas=3
k create deploy web-canary --image=nginx:1.25 --replicas=1
# Both use same service selector (e.g., app: web)"#.to_string(),
                    description: Some("25% traffic to new version (1 of 4 pods)".to_string()),
                },
                Concept {
                    title: "Save cluster resources to files".to_string(),
                    command: r#"k get ns > /opt/course/1/namespaces
k get all -o yaml > /opt/backup.yaml"#.to_string(),
                    description: Some("Export for backup or review".to_string()),
                },
                Concept {
                    title: "Field selector queries".to_string(),
                    command: r#"k get pods --field-selector status.phase=Running
k get pods --field-selector metadata.namespace=default"#.to_string(),
                    description: Some("Filter by resource fields instead of labels".to_string()),
                },
            ],
        },
        Category {
            name: "⚡ Time-Saving Aliases".to_string(),
            concepts: vec![
                Concept {
                    title: "Essential kubectl aliases".to_string(),
                    command: r#"alias k=kubectl
alias kgp='kubectl get pods'
alias kgs='kubectl get svc'
alias kgd='kubectl get deploy'
alias kdp='kubectl describe pod'
alias kds='kubectl describe svc'
alias kdd='kubectl describe deploy'
alias kaf='kubectl apply -f'
alias kdf='kubectl delete -f'
alias ksys='kubectl -n kube-system'"#.to_string(),
                    description: Some("⚡ EXAM TIP: Set these at start of exam in ~/.bashrc".to_string()),
                },
                Concept {
                    title: "Enable kubectl autocompletion".to_string(),
                    command: r#"source <(kubectl completion bash)
echo "source <(kubectl completion bash)" >> ~/.bashrc
complete -F __start_kubectl k"#.to_string(),
                    description: Some("Tab completion for kubectl commands".to_string()),
                },
                Concept {
                    title: "Set default editor".to_string(),
                    command: r#"export KUBE_EDITOR=nano
# or
export KUBE_EDITOR=vi"#.to_string(),
                    description: Some("Choose editor for kubectl edit".to_string()),
                },
            ],
        },
    ]
}
