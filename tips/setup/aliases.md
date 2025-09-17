# Essential Kubectl Aliases & Environment Setup

## 🚀 Quick Setup Script

Save this as `setup-ckad.sh` and run before your exam:

```bash
#!/bin/bash
# CKAD Exam Setup Script

# Essential kubectl aliases
alias k='kubectl'
alias kg='kubectl get'
alias kd='kubectl describe'
alias kdel='kubectl delete'
alias ka='kubectl apply'
alias kc='kubectl create'
alias ke='kubectl edit'
alias kl='kubectl logs'
alias kex='kubectl exec -it'

# Quick resource shortcuts
alias kgp='kubectl get pods'
alias kgs='kubectl get svc'
alias kgd='kubectl get deploy'
alias kgn='kubectl get nodes'
alias kgns='kubectl get namespaces'
alias kga='kubectl get all'

# Dry run aliases (MOST IMPORTANT!)
alias kdr='kubectl create --dry-run=client -o yaml'
alias kdrr='kubectl create --dry-run=client -o yaml >'

# Common combinations
alias kaf='kubectl apply -f'
alias kdelf='kubectl delete -f'

# Context and namespace shortcuts
alias kctx='kubectl config current-context'
alias kns='kubectl config set-context --current --namespace'

# Output format shortcuts
alias kgy='kubectl get -o yaml'
alias kgj='kubectl get -o json'
alias kgw='kubectl get -o wide'

echo "CKAD aliases loaded! 🎯"
```

## 📝 Manual Setup Commands

If you can't run scripts, execute these manually:

```bash
# Core aliases (MUST HAVE)
alias k='kubectl'
alias kdr='kubectl create --dry-run=client -o yaml'
alias kg='kubectl get'
alias kd='kubectl describe'

# Apply to current session
source ~/.bashrc
```

## 🔧 Environment Variables

```bash
# Set useful defaults
export KUBE_EDITOR=vim
export KUBECTL_EXTERNAL_DIFF=diff

# For faster completion
source <(kubectl completion bash)
complete -F __start_kubectl k
```

## ⚡ Context & Namespace Quick Setup

```bash
# Check current context
kubectl config current-context

# List all contexts
kubectl config get-contexts

# Switch context
kubectl config use-context <context-name>

# Set default namespace
kubectl config set-context --current --namespace=<namespace>

# Verify settings
kubectl config view --minify
```

## 🎯 One-Liner Setup

Copy and paste this at exam start:

```bash
alias k='kubectl' && alias kdr='kubectl create --dry-run=client -o yaml' && alias kg='kubectl get' && alias kd='kubectl describe' && alias ka='kubectl apply -f' && export KUBE_EDITOR=vim
```