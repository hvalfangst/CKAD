# CKAD Practice Exam - Local Docker Desktop Edition

**Time Limit: 2 hours**
**Passing Score: 66%**
**Environment: Local Docker Desktop + vim**

## Prerequisites
- Docker Desktop running
- kubectl configured for local cluster
- vim editor
- Basic understanding of Kubernetes concepts

---

## Question 1: Multi-Container Pod with Shared Volume (20 points)

**Scenario:** You need to create a multi-container pod that demonstrates inter-container communication using a shared volume.

**Tasks:**
1. Create a pod named `log-processor` in the `default` namespace with two containers:
   - **Container 1 (`writer`)**:
     - Image: `busybox:1.35`
     - Command: Write current timestamp to `/shared/app.log` every 5 seconds
     - Mount shared volume at `/shared`

   - **Container 2 (`reader`)**:
     - Image: `busybox:1.35`
     - Command: Continuously tail and display contents of `/shared/app.log`
     - Mount shared volume at `/shared`

2. Configure an `emptyDir` volume named `shared-storage`

3. Verify both containers are running and the reader shows timestamp logs

4. Save the pod manifest as `log-processor-pod.yaml`

**Expected Output:** The reader container should continuously display timestamps written by the writer container.

---

## Question 2: ConfigMap and Secret Integration (15 points)

**Scenario:** Deploy an application that uses both ConfigMaps for configuration and Secrets for sensitive data.

**Tasks:**
1. Create a ConfigMap named `app-config` with the following data:
   ```
   database.host=localhost
   database.port=5432
   database.name=myapp
   log.level=INFO
   ```

2. Create a Secret named `app-secrets` with the following data:
   ```
   database.username=admin
   database.password=secretpass123
   api.key=abc123xyz789
   ```

3. Create a deployment named `config-app` with:
   - Image: `nginx:1.21`
   - 2 replicas
   - Mount the ConfigMap as environment variables with prefix `CONFIG_`
   - Mount the Secret as environment variables with prefix `SECRET_`
   - Add a volume mount for the ConfigMap at `/etc/config`
   - Add a volume mount for the Secret at `/etc/secrets`

4. Verify the deployment and check that environment variables are properly set

**Validation:** Exec into a pod and verify both environment variables and mounted files are accessible.

---

## Question 3: Resource Management and Horizontal Pod Autoscaler (20 points)

**Scenario:** Deploy a resource-managed application with autoscaling capabilities.

**Tasks:**
1. Create a deployment named `resource-demo` with:
   - Image: `nginx:1.21`
   - Initial replicas: 2
   - Resource requests: CPU 100m, Memory 128Mi
   - Resource limits: CPU 200m, Memory 256Mi
   - Add label `app=resource-demo`

2. Create a Service named `resource-demo-service`:
   - Type: ClusterIP
   - Port: 80
   - Target port: 80
   - Selector: `app=resource-demo`

3. Create a Horizontal Pod Autoscaler named `resource-demo-hpa`:
   - Target deployment: `resource-demo`
   - Min replicas: 2
   - Max replicas: 5
   - Target CPU utilization: 70%

4. Generate load to test autoscaling:
   - Create a temporary pod with `busybox` image
   - Use wget/curl in a loop to generate traffic
   - Monitor HPA behavior

5. Save all manifests in separate files

**Validation:** HPA should scale up pods when CPU usage exceeds 70%.

---

## Question 4: Persistent Volume and StatefulSet (25 points)

**Scenario:** Deploy a stateful application that requires persistent storage.

**Tasks:**
1. Create a PersistentVolume named `app-pv` with:
   - Capacity: 1Gi
   - Access mode: ReadWriteOnce
   - Storage class: Use your local storage class
   - Host path: `/tmp/k8s-data` (create this directory first)

2. Create a PersistentVolumeClaim named `app-pvc`:
   - Request: 500Mi
   - Access mode: ReadWriteOnce
   - Should bind to the PV created above

3. Create a StatefulSet named `stateful-app` with:
   - Image: `postgres:13`
   - Replicas: 1
   - Service name: `postgres-service`
   - Environment variables:
     - `POSTGRES_DB=testdb`
     - `POSTGRES_USER=testuser`
     - `POSTGRES_PASSWORD=testpass`
   - Volume mount: PVC mounted at `/var/lib/postgresql/data`

4. Create a headless service named `postgres-service`:
   - Port: 5432
   - Selector: matches StatefulSet labels

5. Verify data persistence:
   - Connect to PostgreSQL and create a test table
   - Delete the pod and wait for recreation
   - Verify the test table still exists

**Validation:** Data should persist across pod deletions/recreations.

---

## Question 5: Network Policies and Service Mesh Basics (20 points)

**Scenario:** Implement network security policies to control traffic between applications.

**Tasks:**
1. Create two namespaces:
   - `frontend`
   - `backend`

2. Deploy applications in each namespace:

   **Frontend namespace:**
   - Deployment: `web-app` (nginx:1.21, replicas: 2)
   - Service: `web-service` (ClusterIP, port 80)
   - Labels: `app=web`, `tier=frontend`

   **Backend namespace:**
   - Deployment: `api-app` (nginx:1.21, replicas: 2)
   - Service: `api-service` (ClusterIP, port 80)
   - Labels: `app=api`, `tier=backend`

3. Create NetworkPolicies:

   **Policy 1:** `backend-policy` in `backend` namespace
   - Deny all ingress traffic by default
   - Allow traffic only from pods with label `tier=frontend`
   - Allow traffic only on port 80

   **Policy 2:** `frontend-policy` in `frontend` namespace
   - Allow all egress traffic to `backend` namespace
   - Deny egress traffic to external networks

4. Test network policies:
   - Verify frontend can reach backend service
   - Verify external pods cannot reach backend
   - Verify frontend cannot reach external services

5. Create a test pod in `default` namespace and verify it cannot access backend

**Validation:** Only frontend pods should be able to communicate with backend pods.

---

## Exam Tips

1. **Time Management:** Allocate time based on point values
2. **Validation:** Always test your solutions before moving on
3. **Documentation:** Use `kubectl explain` for resource specifications
4. **Debugging:** Use `kubectl describe`, `kubectl logs`, and `kubectl get events`
5. **Cleanup:** Remove test resources after validation to avoid conflicts

## Scoring Rubric

- **Question 1:** 20 points (Pod creation: 10, Volume configuration: 5, Verification: 5)
- **Question 2:** 15 points (ConfigMap: 4, Secret: 4, Deployment: 5, Verification: 2)
- **Question 3:** 20 points (Deployment: 6, Service: 4, HPA: 6, Testing: 4)
- **Question 4:** 25 points (PV: 5, PVC: 5, StatefulSet: 10, Service: 3, Persistence test: 2)
- **Question 5:** 20 points (Namespaces: 4, Deployments: 6, NetworkPolicies: 8, Testing: 2)

**Total: 100 points**
**Passing: 66 points**

Good luck with your CKAD preparation!