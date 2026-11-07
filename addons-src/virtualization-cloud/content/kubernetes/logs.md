---
id = "cmd/k8s/logs"
kind = "command"
title = "kubectl logs"
category = "reference/vendors/kubernetes"
tags = ["kubernetes", "kubectl", "cli", "logs"]
risk = "info"
version = "Kubernetes 1.29"
---
Prints the logs of a container running in a pod.

## Command

```bash
kubectl logs <pod-name> -n <namespace>
kubectl logs -f --tail 100 <pod-name>
```

## Notes

- Add `-c <container>` when the pod runs multiple containers (sidecars).
- `--previous` shows the logs of the last crashed container instance - key for CrashLoopBackOff.
- `-f` streams like tail -f; `--tail` limits the initial output.
- Pod must exist; for deleted pods use your log aggregation stack.

Source: Kubernetes Documentation — https://kubernetes.io/docs
