---
id = "cmd/checkpoint/cpstat"
kind = "command"
title = "cpstat (status probes)"
category = "reference/vendors/check-point"
tags = ["checkpoint", "cli", "diagnostics"]
risk = "info"
version = "Gaia R81.x"
---
Compact one-shot status probes for OS health and gateway components.

## Command

```text
cpstat os -f all
cpstat fw
```

## Notes

- `cpstat os -f all` reports OS-level health: CPU, memory, disks and hardware sensors.
- `cpstat fw` reports security gateway status in a compact form.
- Output is short and scriptable - handy for cron-based health checks.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com
