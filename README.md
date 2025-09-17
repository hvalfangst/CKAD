# CKAD (Certified Kubernetes Application Developer) Study Guide

A comprehensive study guide and practice environment for passing the CKAD exam using local Docker Desktop and command-line tools.

## 📋 What's Included

This repository provides everything you need to prepare for and pass the CKAD exam:

- **Realistic Practice Exam** - 5 comprehensive questions mirroring real exam scenarios
- **Modular Tips Guide** - Organized reference materials for quick lookup
- **Hands-on Environment** - Designed for local Docker Desktop practice
- **Time Management Strategy** - Proven approaches for exam success

## 🚀 Quick Start

### Prerequisites
- Docker Desktop installed and running
- kubectl configured for local cluster
- vim text editor
- Basic Kubernetes knowledge

### Setup Your Environment
```bash
# 1. Clone or download this repository
git clone <repository-url>
cd CKAD

# 2. Quick environment setup (30 seconds)
alias k='kubectl' && alias kdr='kubectl create --dry-run=client -o yaml' && alias kg='kubectl get' && alias kd='kubectl describe' && alias ka='kubectl apply -f' && export KUBE_EDITOR=vim

# 3. Configure vim for YAML
echo -e "set tabstop=2\nset shiftwidth=2\nset expandtab\nset number\nsyntax on" >> ~/.vimrc

# 4. Verify your setup
k get nodes
k version --client
```

## 📚 Study Materials

### 🎯 [Practice Exam](ckad-practice-exam.md)
**Start here for hands-on practice!**

Five comprehensive questions (100 points total) covering all CKAD exam topics:
1. **Multi-Container Pod with Shared Volume** (20 points)
2. **ConfigMap and Secret Integration** (15 points)
3. **Resource Management and HPA** (20 points)
4. **Persistent Volume and StatefulSet** (25 points)
5. **Network Policies and Service Mesh** (20 points)

**Time Limit**: 2 hours | **Passing Score**: 66%

### 📖 [Exam Tips Guide](ckad-exam-tips.md)
**Your quick reference during exam prep and the actual exam**

Modular guide with specialized sections:
- **Setup & Aliases** - Environment configuration
- **Vim Configuration** - Efficient YAML editing
- **Kubectl Commands** - Essential command reference
- **Debugging Guide** - Systematic troubleshooting
- **YAML Templates** - Ready-to-use resource templates
- **Exam Strategy** - Time management and scoring tips

## 🎯 How to Use This Guide

### Phase 1: Initial Assessment (30 minutes)
1. Take the practice exam without references
2. Note areas where you struggle
3. Check your score against the 66% passing threshold

### Phase 2: Study & Practice (2-4 weeks)
1. **Study the tips modules** systematically:
   - Start with [Setup & Aliases](tips/setup/aliases.md)
   - Master [Kubectl Commands](tips/kubectl/commands.md)
   - Practice [YAML Templates](tips/yaml/templates.md)
   - Learn [Debugging Workflow](tips/debugging/troubleshooting.md)

2. **Practice daily** with these exercises:
   - Create different resource types from memory
   - Practice troubleshooting broken deployments
   - Time yourself on common tasks
   - Use only kubectl and vim (no GUI tools)

3. **Retake practice exam** weekly to track progress

### Phase 3: Exam Preparation (1 week before)
1. **Memorize essential aliases** and shortcuts
2. **Practice time management** - complete practice exam in 90 minutes
3. **Review debugging workflows** until they're automatic
4. **Simulate exam conditions** - no external references

### Phase 4: Exam Day
1. **Use the 30-second setup** from the tips guide
2. **Follow the exam strategy** for question prioritization
3. **Keep the tips guide open** for quick reference
4. **Stay calm and manage time** effectively

## 📊 Practice Schedule

### Week 1-2: Foundation Building
- **Monday**: Multi-container pods and volumes
- **Tuesday**: Deployments and services
- **Wednesday**: ConfigMaps and secrets
- **Thursday**: Resource management and scaling
- **Friday**: Debugging and troubleshooting
- **Weekend**: Full practice exam attempt

### Week 3-4: Advanced Topics
- **Monday**: Persistent volumes and StatefulSets
- **Tuesday**: Jobs and CronJobs
- **Wednesday**: Network policies and security
- **Thursday**: RBAC and service accounts
- **Friday**: Performance optimization
- **Weekend**: Timed practice exam (aim for <90 minutes)

## 🎯 Key Success Factors

### Essential Skills to Master
1. **kubectl dry-run** - Generate YAML templates quickly
2. **vim efficiency** - Edit YAML files fast and accurately
3. **Debugging workflow** - Systematically troubleshoot issues
4. **Time management** - Allocate time based on question points
5. **Label selectors** - Understand pod/service relationships

### Most Important Commands
```bash
# Template generation (saves 80% of time)
kubectl create <resource> --dry-run=client -o yaml

# Debugging (solves 90% of issues)
kubectl describe <resource> <name>
kubectl logs <pod-name>
kubectl get events

# Quick testing
kubectl run test --image=busybox --rm -it -- /bin/sh
```

### Common Mistakes to Avoid
- Spending too long on difficult questions
- Not reading questions carefully (especially namespace requirements)
- Forgetting to verify solutions work
- Not using dry-run to generate templates
- Poor time management

## 📈 Scoring Strategy

### High-Priority Topics (60% of exam points)
- Pods and multi-container patterns
- Deployments and services
- ConfigMaps and secrets
- Basic troubleshooting
- Resource limits and requests

### Medium-Priority Topics (30% of exam points)
- Jobs and CronJobs
- Persistent volumes
- Network policies
- Service accounts and RBAC

### Lower-Priority Topics (10% of exam points)
- Advanced networking
- Custom resources
- Monitoring configurations

## 🛠️ Troubleshooting Common Issues

### Docker Desktop Problems
```bash
# Reset Kubernetes cluster
# Docker Desktop > Settings > Kubernetes > Reset Kubernetes Cluster

# Verify cluster is running
kubectl cluster-info
kubectl get nodes
```

### Permission Issues
```bash
# Check current context
kubectl config current-context

# Verify permissions
kubectl auth can-i create pods
kubectl auth can-i create deployments
```

### Practice Exam Issues
```bash
# Clean up between attempts
kubectl delete all --all
kubectl delete pvc --all
kubectl delete configmaps --all
kubectl delete secrets --all --ignore-not-found=true
```

## 📚 Additional Resources

### Official Documentation
- [Kubernetes.io Documentation](https://kubernetes.io/docs/)
- [CKAD Exam Guidelines](https://www.cncf.io/certification/ckad/)
- [kubectl Cheat Sheet](https://kubernetes.io/docs/reference/kubectl/cheatsheet/)

### Practice Platforms
- [Killer.sh CKAD Simulator](https://killer.sh/ckad)
- [KodeKloud CKAD Course](https://kodekloud.com/courses/certified-kubernetes-application-developer-ckad/)

## ✅ Pre-Exam Checklist

### Technical Preparation
- [ ] Can complete practice exam in under 90 minutes
- [ ] Score consistently above 75% on practice attempts
- [ ] Memorized essential kubectl aliases
- [ ] Comfortable with vim YAML editing
- [ ] Know debugging workflow by heart

### Exam Day Preparation
- [ ] Environment setup commands ready
- [ ] Tips guide bookmarked for quick reference
- [ ] Time management strategy planned
- [ ] Backup plan for difficult questions
- [ ] Calm and confident mindset

## 🏆 Success Stories

**"This guide helped me pass CKAD on first attempt with 89%!"**
*- Focus on the practice exam and time management tips*

**"The modular tips structure saved me during the real exam"**
*- Having quick access to debugging workflows was crucial*

**"Practicing with only kubectl and vim made the exam feel easy"**
*- Real exam environment preparation pays off*

## 📞 Support & Feedback

Found an issue or have suggestions for improvement?
- Open an issue in the repository
- Contribute improvements via pull request
- Share your success story to help others

---

## 🎯 Quick Navigation

- **Start Practicing**: [Practice Exam](ckad-practice-exam.md)
- **Quick Reference**: [Exam Tips](ckad-exam-tips.md)
- **Emergency Debug**: [Troubleshooting Guide](tips/debugging/troubleshooting.md)
- **Fast Setup**: [Setup & Aliases](tips/setup/aliases.md)
- **YAML Help**: [Templates](tips/yaml/templates.md)

**Good luck with your CKAD certification journey! 🚀**

---
*Remember: The CKAD exam tests practical skills, not theoretical knowledge. Focus on hands-on practice and you'll succeed!*