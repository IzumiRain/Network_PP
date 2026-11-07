---
id = "cmd/f5-bigip/show-sys-performance"
kind = "command"
title = "show sys performance"
category = "reference/vendors/f5/bigip/system"
tags = ["f5", "bigip", "tmsh", "performance"]
risk = "info"
version = "BIG-IP 16.1"
---
Shows system performance counters: CPU utilization, memory usage and uptime for the appliance.

## Command

```bash
tmsh show sys performance
```

## Notes

- Quick capacity check during slowness reports before diving into per-object statistics.
- Sustained high CPU with normal traffic usually means a heavy iRule, a profiling module, or logging overhead.
- Pair with show sys hardware to rule out physical problems (fans, temperature) behind the numbers.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
