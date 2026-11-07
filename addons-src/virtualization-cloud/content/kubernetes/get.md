---
id = "cmd/k8s/get"
kind = "command"
title = "kubectl get (resources)"
category = "reference/virtualization/kubernetes"
tags = ["kubernetes", "kubectl", "cli"]
risk = "info"
version = "Kubernetes 1.29"
---
Lists cluster resources: nodes, pods, services and more.

## Command

```bash
kubectl get nodes
kubectl get pods -A
kubectl get services -n <namespace>
```

## Notes

- `-A` (all namespaces) versus `-n <namespace>` controls scope; no -n means the current context namespace.
- `-o wide` adds IPs and node placement; `-o yaml` dumps the full object.
- A pod stuck in `Pending` usually lacks schedulable resources; `CrashLoopBackOff` needs `kubectl logs`.

Source: Kubernetes Documentation — https://kubernetes.io/docs
