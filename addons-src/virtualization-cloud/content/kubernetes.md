---
id = "doc/vendors/kubernetes/kubernetes"
kind = "vendor"
title = "Kubernetes"
category = "reference/vendors/kubernetes"
tags = ["kubernetes", "kubectl", "cli", "troubleshooting", "logs", "metrics", "model-doc"]
risk = "info"
version = "Kubernetes 1.29"
---
The complete Kubernetes document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Commands
<!-- section-tags: kubernetes, kubectl, cli, troubleshooting, logs, metrics -->


### kubectl describe

Shows the full event-annotated detail of one resource.

#### Command

```bash
kubectl describe pod <pod-name> -n <namespace>
kubectl describe node <node-name>
```

#### Notes

- The Events section at the bottom is where scheduling and image-pull failures appear.
- Use it after `kubectl get` when a resource is not in the expected state.
- Works for any resource type: pod, service, deployment, node, pvc.

Source: Kubernetes Documentation — https://kubernetes.io/docs


### kubectl get (resources)

Lists cluster resources: nodes, pods, services and more.

#### Command

```bash
kubectl get nodes
kubectl get pods -A
kubectl get services -n <namespace>
```

#### Notes

- `-A` (all namespaces) versus `-n <namespace>` controls scope; no -n means the current context namespace.
- `-o wide` adds IPs and node placement; `-o yaml` dumps the full object.
- A pod stuck in `Pending` usually lacks schedulable resources; `CrashLoopBackOff` needs `kubectl logs`.

Source: Kubernetes Documentation — https://kubernetes.io/docs


### kubectl logs

Prints the logs of a container running in a pod.

#### Command

```bash
kubectl logs <pod-name> -n <namespace>
kubectl logs -f --tail 100 <pod-name>
```

#### Notes

- Add `-c <container>` when the pod runs multiple containers (sidecars).
- `--previous` shows the logs of the last crashed container instance - key for CrashLoopBackOff.
- `-f` streams like tail -f; `--tail` limits the initial output.
- Pod must exist; for deleted pods use your log aggregation stack.

Source: Kubernetes Documentation — https://kubernetes.io/docs


### kubectl top (usage)

Shows current CPU and memory usage of nodes and pods.

#### Command

```bash
kubectl top nodes
kubectl top pods -A
```

#### Notes

- Requires metrics-server (or a Prometheus-based adapter); without it you get Metrics API not available.
- Compare usage against requests/limits from `kubectl describe` to spot throttling.
- `pods -A` across all namespaces helps find the noisiest workloads quickly.

Source: Kubernetes Documentation — https://kubernetes.io/docs

