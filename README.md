# Certified Kubernetes Application Developer (CKAD) Study Guide

A comprehensive study guide and practical resource for the Certified Kubernetes Application Developer (CKAD) exam. This repository contains complete documentation, examples, and practice exercises for all CKAD curriculum sections.

## 📚 Curriculum Overview

The CKAD exam covers 5 main domains with the following weightings:

| Section | Domain | Weight | Description |
|---------|--------|--------|-------------|
| 1 | [Application Design and Build](./ckad-curriculum/1-application-design-build/) | 20% | Pods, Deployments, Jobs, Multi-container patterns |
| 2 | [Application Deployment](./ckad-curriculum/2-application-deployment/) | 20% | Deployment strategies, Scaling, Rolling updates |
| 3 | [Application Observability and Maintenance](./ckad-curriculum/3-application-observability-maintenance/) | 15% | Monitoring, Logging, Debugging, Health checks |
| 4 | [Application Environment, Configuration and Security](./ckad-curriculum/4-application-environment-config-security/) | 25% | ConfigMaps, Secrets, RBAC, Security contexts |
| 5 | [Services and Networking](./ckad-curriculum/5-services-networking/) | 20% | Services, Ingress, Network policies, DNS |

## 🎯 What's Included

Each curriculum section contains:

### 📖 README.md
- **Comprehensive theory and concepts**
- **Official Kubernetes documentation references**
- **Best practices and patterns**
- **Quick reference commands**
- **Exam tips and common scenarios**

### 💻 examples.yaml
- **20+ practical YAML configurations per section**
- **Real-world examples you can apply immediately**
- **Progressive complexity from basic to advanced**
- **Production-ready configurations**
- **Common patterns and use cases**

### 🏋️ practice-exercises.md
- **10+ hands-on exercises per section**
- **Progressive difficulty levels**
- **Exam simulation scenarios with time limits**
- **Troubleshooting challenges**
- **Solutions and explanations**
- **Command references and debugging workflows**

## 🚀 Getting Started

### Prerequisites
- Basic Kubernetes knowledge
- Access to a Kubernetes cluster (local or cloud)
- kubectl installed and configured

### Study Approach

1. **Read the theory** in each section's README.md
2. **Practice with examples** from examples.yaml files
3. **Complete exercises** in practice-exercises.md
4. **Time yourself** on exam simulation scenarios
5. **Review and repeat** challenging areas

### Recommended Study Order

```
Section 1: Application Design and Build
   ↓
Section 4: Application Environment, Configuration and Security
   ↓
Section 2: Application Deployment
   ↓
Section 5: Services and Networking
   ↓
Section 3: Application Observability and Maintenance
```

## 📋 Quick Navigation

### Section 1: Application Design and Build (20%)
- **[📖 Theory Guide](./ckad-curriculum/1-application-design-build/README.md)** - Pods, Deployments, Jobs, Multi-container patterns
- **[💻 Examples](./ckad-curriculum/1-application-design-build/examples.yaml)** - 25+ YAML configurations
- **[🏋️ Exercises](./ckad-curriculum/1-application-design-build/practice-exercises.md)** - 9 exercise sets + exam scenarios

### Section 2: Application Deployment (20%)
- **[📖 Theory Guide](./ckad-curriculum/2-application-deployment/README.md)** - Deployment strategies, Scaling, HPA
- **[💻 Examples](./ckad-curriculum/2-application-deployment/examples.yaml)** - Rolling updates, Blue-green, Canary deployments
- **[🏋️ Exercises](./ckad-curriculum/2-application-deployment/practice-exercises.md)** - 10 exercise sets + performance scenarios

### Section 3: Application Observability and Maintenance (15%)
- **[📖 Theory Guide](./ckad-curriculum/3-application-observability-maintenance/README.md)** - Monitoring, Logging, Debugging
- **[💻 Examples](./ckad-curriculum/3-application-observability-maintenance/examples.yaml)** - Health checks, Logging patterns
- **[🏋️ Exercises](./ckad-curriculum/3-application-observability-maintenance/practice-exercises.md)** - Troubleshooting workflows

### Section 4: Application Environment, Configuration and Security (25%)
- **[📖 Theory Guide](./ckad-curriculum/4-application-environment-config-security/README.md)** - ConfigMaps, Secrets, RBAC
- **[💻 Examples](./ckad-curriculum/4-application-environment-config-security/examples.yaml)** - Security contexts, Network policies
- **[🏋️ Exercises](./ckad-curriculum/4-application-environment-config-security/practice-exercises.md)** - Configuration and security scenarios

### Section 5: Services and Networking (20%)
- **[📖 Theory Guide](./ckad-curriculum/5-services-networking/README.md)** - Services, Ingress, Network policies
- **[💻 Examples](./ckad-curriculum/5-services-networking/examples.yaml)** - All service types, Ingress configurations
- **[🏋️ Exercises](./ckad-curriculum/5-services-networking/practice-exercises.md)** - Network troubleshooting scenarios

## 🛠️ Essential kubectl Commands

### Quick Resource Creation
```bash
# Pods
kubectl run nginx --image=nginx:1.21 --dry-run=client -o yaml > pod.yaml

# Deployments
kubectl create deployment nginx --image=nginx:1.21 --replicas=3 --dry-run=client -o yaml > deployment.yaml

# Services
kubectl expose deployment nginx --port=80 --target-port=80 --dry-run=client -o yaml > service.yaml

# ConfigMaps
kubectl create configmap app-config --from-literal=key=value --dry-run=client -o yaml > configmap.yaml

# Secrets
kubectl create secret generic app-secret --from-literal=password=secret --dry-run=client -o yaml > secret.yaml
```

### Debugging and Troubleshooting
```bash
# Pod investigation
kubectl describe pod <pod-name>
kubectl logs <pod-name> -f
kubectl exec -it <pod-name> -- /bin/bash

# Service connectivity
kubectl get endpoints
kubectl run test --image=busybox:1.35 -it --rm -- sh

# Resource monitoring
kubectl top pods
kubectl top nodes
kubectl get events --sort-by=.metadata.creationTimestamp
```

## 📝 Exam Tips

### Time Management
- **2 hours for 15-20 questions**
- **6-8 minutes average per question**
- **Practice with time limits**
- **Use kubectl shortcuts and aliases**

### Essential Skills
- **Fast YAML generation with --dry-run=client -o yaml**
- **Imperative commands for quick resource creation**
- **Systematic debugging workflows**
- **kubectl explain for field references**
- **vim/nano efficiency for YAML editing**

### Common Exam Scenarios
- Create and configure pods with specific requirements
- Set up deployments with rolling update strategies
- Configure ConfigMaps and Secrets with proper security
- Implement network policies and service configurations
- Debug failing applications and services
- Set up monitoring and health checks

## 🎓 CKAD Certification Details

### Exam Format
- **Performance-based exam** (hands-on lab environment)
- **2 hours duration**
- **15-20 practical questions**
- **66% passing score**
- **Valid for 3 years**

### Exam Environment
- **Pre-configured Kubernetes clusters**
- **kubectl and related tools available**
- **Web-based terminal**
- **Copy/paste enabled**
- **Kubernetes documentation accessible**

### Registration and Scheduling
- **Register at** [training.linuxfoundation.org](https://training.linuxfoundation.org)
- **Schedule through PSI Services**
- **Online proctored exam**
- **One free retake included**

## 🔗 Additional Resources

### Official Documentation
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [CKAD Exam Details](https://www.cncf.io/certification/ckad/)
- [Candidate Handbook](https://docs.linuxfoundation.org/tc-docs/certification/lf-handbook2)

### Practice Environments
- [Killer.sh](https://killer.sh) - CKAD exam simulator
- [Play with Kubernetes](https://labs.play-with-k8s.com/)
- Local clusters: minikube, kind, k3s

### Community Resources
- [Kubernetes Slack](https://kubernetes.slack.com)
- [CNCF Community](https://community.cncf.io/)
- [r/kubernetes](https://reddit.com/r/kubernetes)

## 🤝 Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add your improvements
4. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⭐ Acknowledgments

- Based on official Kubernetes documentation
- Inspired by the CNCF CKAD curriculum
- Community feedback and contributions
- Real-world Kubernetes experience and best practices

---

**Good luck with your CKAD certification journey! 🚀**

*Remember: Practice consistently, time yourself, and focus on hands-on experience. The CKAD exam tests practical skills, so the more you practice with real Kubernetes clusters, the better prepared you'll be.*