---
id = "cmd/k8s/top"
kind = "command"
title = "kubectl top (usage)"
category = "reference/vendors/kubernetes"
tags = ["kubernetes", "kubectl", "cli", "metrics"]
risk = "info"
version = "Kubernetes 1.29"
---
Shows current CPU and memory usage of nodes and pods.

## Command

```bash
kubectl top nodes
kubectl top pods -A
```

## Notes

- Requires metrics-server (or a Prometheus-based adapter); without it you get Metrics API not available.
- Compare usage against requests/limits from `kubectl describe` to spot throttling.
- `pods -A` across all namespaces helps find the noisiest workloads quickly.

Source: Kubernetes Documentation — https://kubernetes.io/docs
