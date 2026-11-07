---
id = "cmd/checkpoint/fw-ctl-pstat"
kind = "command"
title = "fw ctl pstat"
category = "reference/vendors/check-point"
tags = ["checkpoint", "cli", "kernel", "diagnostics"]
risk = "info"
version = "Gaia R81.x"
---
Shows kernel inspection statistics: free kernel memory, connections, NAT and VPN.

## Command

```text
fw ctl pstat
```

## Notes

- Run from Expert mode (enter `expert` from clish first).
- Watch `Free` kernel memory and the connections hash usage under load.
- A nearly full kernel memory or hash table shows up here before users report drops.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com
