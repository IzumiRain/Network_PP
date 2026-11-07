---
id = "cmd/gcp/compute-networking-basics"
kind = "command"
title = "gcloud compute networking basics"
category = "reference/vendors/google-cloud"
tags = ["gcp", "gcloud", "cli", "networking"]
risk = "info"
version = "Google Cloud CLI (latest)"
---
Lists and describes GCP compute instances and VPC networks.

## Command

```bash
gcloud compute instances list
gcloud compute instances describe <instance-name> --zone <zone>
gcloud compute networks list
gcloud compute networks describe <network-name>
```

## Notes

- `instances describe` shows networkInterfaces with internal IP, accessConfig (external IP) and subnetwork.
- `networks list` shows each VPC and its subnet-routing mode (auto/custom); `describe` adds routing details.
- Scope with `--project <project-id>` and `--zone <zone>` when your config spans multiple projects.
- All four are read-only and safe in production projects.

Source: Google Cloud CLI (gcloud compute) reference — https://cloud.google.com/docs
