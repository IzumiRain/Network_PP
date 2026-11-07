---
id = "cmd/k8s/describe"
kind = "command"
title = "kubectl describe"
category = "reference/vendors/kubernetes"
tags = ["kubernetes", "kubectl", "cli", "troubleshooting"]
risk = "info"
version = "Kubernetes 1.29"
---
Shows the full event-annotated detail of one resource.

## Command

```bash
kubectl describe pod <pod-name> -n <namespace>
kubectl describe node <node-name>
```

## Notes

- The Events section at the bottom is where scheduling and image-pull failures appear.
- Use it after `kubectl get` when a resource is not in the expected state.
- Works for any resource type: pod, service, deployment, node, pvc.

Source: Kubernetes Documentation — https://kubernetes.io/docs
