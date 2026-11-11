---
id = "doc/vendors/gcp/gcp"
kind = "vendor"
title = "Google Cloud (GCP)"
category = "reference/vendors/gcp"
tags = ["gcp", "gcloud", "cli", "networking", "model-doc"]
risk = "info"
version = "Google Cloud CLI (latest)"
---
The complete Google Cloud (GCP) document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Google Cloud
<!-- section-tags: gcp, gcloud, cli, networking -->


### gcloud compute networking basics

Lists and describes GCP compute instances and VPC networks.

#### Command

```bash
gcloud compute instances list
gcloud compute instances describe <instance-name> --zone <zone>
gcloud compute networks list
gcloud compute networks describe <network-name>
```

#### Notes

- `instances describe` shows networkInterfaces with internal IP, accessConfig (external IP) and subnetwork.
- `networks list` shows each VPC and its subnet-routing mode (auto/custom); `describe` adds routing details.
- Scope with `--project <project-id>` and `--zone <zone>` when your config spans multiple projects.
- All four are read-only and safe in production projects.

Source: Google Cloud CLI (gcloud compute) reference — https://cloud.google.com/docs

