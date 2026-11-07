---
id = "cmd/checkpoint/cpview"
kind = "command"
title = "cpview (performance view)"
category = "reference/firewalls/checkpoint"
tags = ["checkpoint", "gaia", "cli", "performance"]
risk = "info"
version = "Gaia R81.x"
---
Launches the Check Point performance view TUI: CPU, memory, connections, throughput, VPN.

## Command

```text
cpview
```

## Notes

- Best first tool for "gateway is slow" triage on Gaia.
- Navigate the metric tree with the on-screen menu; exit from the File menu.
- Watch FW connections vs. capacity and CPU per core under load.
- Works locally via SSH and is read-only.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com
